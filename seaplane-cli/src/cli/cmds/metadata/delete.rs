use clap::{ArgMatches, Command};
use serde_json::json;

use crate::{
    cli::cmds::metadata::{
        build_config_request_key, common, common::SeaplaneMetadataCommonArgMatches, CliCommand,
    },
    context::{Ctx, MetadataCtx},
    error::Result,
    printer::OutputFormat,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataDelete;

impl SeaplaneMetadataDelete {
    pub fn command() -> Command<'static> {
        Command::new("delete")
            .visible_aliases(&["del", "remove", "rm"])
            .override_usage("seaplane metadata delete <KEY>... [OPTIONS]")
            .about("Delete one or more metadata key-value pairs")
            .args(common::args())
    }
}

impl CliCommand for SeaplaneMetadataDelete {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let mut len = 0;
        for kv in ctx.md_ctx().kvs.iter_mut() {
            let key = kv.key.as_ref().unwrap().to_string();
            build_config_request_key(&key, ctx)?.delete_value()?;
            if ctx.out_format == OutputFormat::Table {
                cli_println!("Removed {key}");
            }
            len += 1;
        }

        if ctx.out_format == OutputFormat::Table {
            cli_println!(
                "\nSuccessfully removed {len} item{}",
                if len > 1 { "s" } else { "" }
            );
        } else {
            cli_println!(
                "{}",
                json!({"removed": ctx.md_ctx().kvs.keys().collect::<Vec<_>>() })
            )
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.init_md(MetadataCtx::from_md_common(
            &SeaplaneMetadataCommonArgMatches(matches),
        )?);
        ctx.out_format = matches.value_of_t_or_exit("format");
        Ok(())
    }
}