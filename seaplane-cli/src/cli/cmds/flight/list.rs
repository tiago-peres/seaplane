use clap::{ArgMatches, Command};
use strum::VariantNames;

use crate::{
    cli::{cmds::formation::SeaplaneFormationFetch, CliCommand},
    error::Result,
    printer::Output,
    Ctx, OutputFormat,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightList;

impl SeaplaneFlightList {
    pub fn command() -> Command<'static> {
        // TODO: add sorting
        // TODO: add filtering
        Command::new("list")
            .visible_alias("ls")
            .about("List the current Flight definitions")
            .arg(arg!(--fetch - ('F')).help("Fetch remote Flight definitions prior to listing (by default only local state is considered)"))
            .arg(
                arg!(--format =["FORMAT"=>"table"])
                    .help("Change the output format")
                    .possible_values(OutputFormat::VARIANTS),
            )
    }
}

impl CliCommand for SeaplaneFlightList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.stateless && !ctx.args.fetch {
            cli_eprint!(@Red, "error: ");
            cli_eprint!("'seaplane flight list ");
            cli_eprint!(@Yellow, "--stateless");
            cli_eprint!("' does nothing without also adding '");
            cli_eprint!(@Green, "--fetch");
            cli_eprintln!("'");
            cli_eprintln!("(hint: 'seaplane flight list' only looks at local state, but '--stateless' ignores local state)");
            cli_eprint!("(hint: 'seaplane flight list ");
            cli_eprint!(@Green, "--fetch");
            cli_eprintln!("' will download remote references as well)");
            std::process::exit(1);
        }

        if ctx.args.fetch {
            SeaplaneFormationFetch.run(ctx)?;
        }

        match ctx.args.out_format {
            OutputFormat::Json => ctx.db.flights.print_json(ctx)?,
            OutputFormat::Table => ctx.db.flights.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.out_format = matches.value_of_t("format").unwrap_or_default();
        ctx.args.fetch = matches.is_present("fetch");
        Ok(())
    }
}
