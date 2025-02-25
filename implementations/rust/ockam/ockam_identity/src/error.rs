use ockam_core::compat::string::String;
use ockam_core::{
    errcode::{Kind, Origin},
    Error,
};

/// Identity crate error
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum IdentityError {
    /// Invalid key type
    InvalidKeyType = 1,
    /// Invalid Key Data
    InvalidKeyData,
    /// Invalid Identifier format
    InvalidIdentifier(String),
    /// Identity Change History is empty
    EmptyIdentity,
    /// Identity Verification Failed
    IdentityVerificationFailed,
    /// PurposeKeyAttestation Verification Failed
    PurposeKeyAttestationVerificationFailed,
    /// Credential Verification Failed
    CredentialVerificationFailed,
    /// Error occurred while getting current UTC Timestamp
    UnknownTimestamp,
    /// Unknown Authority
    UnknownAuthority,
    /// No CredentialsRetriever
    NoCredentialsRetriever,
    /// No Credentials set on a trust context
    NoCredentialsSet,
    /// Unknown version of the Credential
    UnknownCredentialVersion,
    /// Invalid data_type value for Credential
    InvalidCredentialDataType,
    /// Unknown version of the Identity
    UnknownIdentityVersion,
    /// Invalid data_type value for Identity
    InvalidIdentityDataType,
    /// Unknown version of the PurposeKeyAttestation
    UnknownPurposeKeyAttestationVersion,
    /// Invalid data_type value for PurposeKeyAttestation
    InvalidPurposeKeyAttestationDataType,
    /// A credential was rejected by the trust context
    SecureChannelVerificationFailedIncorrectCredential,
    /// Credentials could not be checked because the trust context is missing
    SecureChannelVerificationFailedMissingTrustContext,
    /// SecureChannelTrustCheckFailed
    SecureChannelTrustCheckFailed,
    /// Invalid Nonce value
    InvalidNonce,
    /// Nonce overflow
    NonceOverflow,
    /// Unknown message destination
    UnknownChannelMsgDestination,
    /// Invalid LocalInfo type
    InvalidLocalInfoType,
    /// Duplicate Secure Channel
    DuplicateSecureChannel,
    /// Consistency Error
    ConsistencyError,
    /// Invalid Hex
    InvalidHex,
    /// Secret Key doesn't correspond to the Identity
    WrongSecretKey,
}

impl ockam_core::compat::error::Error for IdentityError {}

impl core::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl From<IdentityError> for Error {
    #[track_caller]
    fn from(err: IdentityError) -> Self {
        let kind = Kind::Unknown; // FIXME: fill these in with more
                                  // meaningful error kinds
        Error::new(Origin::Identity, kind, err)
    }
}
