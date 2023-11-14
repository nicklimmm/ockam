use std::collections::HashMap;

use minicbor::Decoder;
use tracing::trace;

use ockam::identity::utils::now;
use ockam::identity::AttributesEntry;
use ockam::identity::{secure_channel_required, IdentityAttributesRepository, TRUST_CONTEXT_ID};
use ockam::identity::{Identifier, IdentitySecureChannelLocalInfo};
use ockam_core::api::{Method, RequestHeader, Response};
use ockam_core::compat::sync::Arc;
use ockam_core::{CowStr, Result, Routed, Worker};
use ockam_node::Context;

use crate::authenticator::direct::types::AddMember;

pub struct DirectAuthenticator {
    trust_context: String,
    identity_attributes_repository: Arc<dyn IdentityAttributesRepository>,
}

impl DirectAuthenticator {
    pub async fn new(
        trust_context: String,
        identity_attributes_repository: Arc<dyn IdentityAttributesRepository>,
    ) -> Result<Self> {
        Ok(Self {
            trust_context,
            identity_attributes_repository,
        })
    }

    async fn add_member<'a>(
        &self,
        enroller: &Identifier,
        id: &Identifier,
        attrs: &HashMap<CowStr<'a>, CowStr<'a>>,
    ) -> Result<()> {
        let auth_attrs = attrs
            .iter()
            .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec()))
            .chain(
                [(
                    TRUST_CONTEXT_ID.to_owned(),
                    self.trust_context.as_bytes().to_vec(),
                )]
                .into_iter(),
            )
            .collect();
        let entry = AttributesEntry::new(auth_attrs, now()?, None, Some(enroller.clone()));
        self.identity_attributes_repository
            .put_attributes(id, entry)
            .await
    }

    async fn list_members(&self) -> Result<HashMap<Identifier, AttributesEntry>> {
        let all_attributes = self
            .identity_attributes_repository
            .list_attributes_by_identifier()
            .await?;
        let attested_by_me = all_attributes.into_iter().collect();
        Ok(attested_by_me)
    }
}

#[ockam_core::worker]
impl Worker for DirectAuthenticator {
    type Context = Context;
    type Message = Vec<u8>;

    async fn handle_message(&mut self, c: &mut Context, m: Routed<Self::Message>) -> Result<()> {
        if let Ok(i) = IdentitySecureChannelLocalInfo::find_info(m.local_message()) {
            let from = i.their_identity_id();
            let mut dec = Decoder::new(m.as_body());
            let req: RequestHeader = dec.decode()?;
            trace! {
                target: "ockam_api::authenticator::direct::direct_authenticator",
                from   = %from,
                id     = %req.id(),
                method = ?req.method(),
                path   = %req.path(),
                body   = %req.has_body(),
                "request"
            }
            let path_segments = req.path_segments::<5>();
            let res = match (req.method(), path_segments.as_slice()) {
                (Some(Method::Post), [""]) | (Some(Method::Post), ["members"]) => {
                    let add: AddMember = dec.decode()?;
                    self.add_member(&from, add.member(), add.attributes())
                        .await?;
                    Response::ok().with_headers(&req).to_vec()?
                }
                (Some(Method::Get), ["member_ids"]) => {
                    let entries = self.list_members().await?;
                    let ids: Vec<Identifier> = entries.into_keys().collect();
                    Response::ok().with_headers(&req).body(ids).to_vec()?
                }
                (Some(Method::Get), [""]) | (Some(Method::Get), ["members"]) => {
                    let entries = self.list_members().await?;

                    Response::ok().with_headers(&req).body(entries).to_vec()?
                }
                (Some(Method::Delete), [id]) | (Some(Method::Delete), ["members", id]) => {
                    let identifier = Identifier::try_from(id.to_string())?;
                    self.identity_attributes_repository
                        .delete(&identifier)
                        .await?;

                    Response::ok().with_headers(&req).to_vec()?
                }

                _ => Response::unknown_path(&req).to_vec()?,
            };
            c.send(m.return_route(), res).await
        } else {
            secure_channel_required(c, m).await
        }
    }
}
