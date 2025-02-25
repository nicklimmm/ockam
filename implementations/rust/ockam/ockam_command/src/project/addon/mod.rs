use core::fmt::Write;

use clap::{Args, Subcommand};

use ockam_api::cloud::addon::Addon;
use ockam_api::cloud::project::Projects;
use ockam_api::nodes::InMemoryNode;
use ockam_node::Context;

use crate::operation::util::check_for_operation_completion;
use crate::output::Output;
use crate::project::addon::configure_confluent::AddonConfigureConfluentSubcommand;
use crate::project::addon::configure_influxdb::AddonConfigureInfluxdbSubcommand;
use crate::project::addon::configure_okta::AddonConfigureOktaSubcommand;
use crate::project::addon::disable::AddonDisableSubcommand;
use crate::project::addon::list::AddonListSubcommand;
use crate::project::util::check_project_readiness;
use crate::util::api::CloudOpts;
use crate::{CommandGlobalOpts, Result};

mod configure_confluent;
mod configure_influxdb;
mod configure_okta;
mod disable;
mod list;

/// Manage addons for a project
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help = true, subcommand_required = true)]
pub struct AddonCommand {
    #[command(subcommand)]
    subcommand: AddonSubcommand,
    #[command(flatten)]
    cloud_opts: CloudOpts,
}

#[derive(Clone, Debug, Subcommand)]
pub enum AddonSubcommand {
    List(AddonListSubcommand),
    Disable(AddonDisableSubcommand),
    #[command(subcommand)]
    Configure(ConfigureAddonCommand),
}

impl AddonCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        match self.subcommand {
            AddonSubcommand::List(cmd) => cmd.run(opts),
            AddonSubcommand::Disable(cmd) => cmd.run(opts),
            AddonSubcommand::Configure(cmd) => cmd.run(opts),
        }
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum ConfigureAddonCommand {
    Okta(AddonConfigureOktaSubcommand),
    Influxdb(AddonConfigureInfluxdbSubcommand),
    Confluent(AddonConfigureConfluentSubcommand),
}

impl ConfigureAddonCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        match self {
            ConfigureAddonCommand::Okta(cmd) => cmd.run(opts),
            ConfigureAddonCommand::Influxdb(cmd) => cmd.run(opts),
            ConfigureAddonCommand::Confluent(cmd) => cmd.run(opts),
        }
    }
}

impl Output for Addon {
    fn output(&self) -> Result<String> {
        let mut w = String::new();
        write!(w, "Addon:")?;
        write!(w, "\n  Id: {}", self.id)?;
        write!(w, "\n  Enabled: {}", self.enabled)?;
        write!(w, "\n  Description: {}", self.description)?;
        writeln!(w)?;
        Ok(w)
    }
}

async fn check_configuration_completion(
    opts: &CommandGlobalOpts,
    ctx: &Context,
    node: &InMemoryNode,
    project_id: &str,
    operation_id: &str,
) -> Result<()> {
    check_for_operation_completion(opts, ctx, node, operation_id, "the addon configuration")
        .await?;
    let project = node.get_project(ctx, project_id).await?;
    let _ = check_project_readiness(opts, ctx, node, project).await?;
    Ok(())
}
