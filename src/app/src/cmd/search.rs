use std::io::stdout;

use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::package::PackageSummary;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::{ItemOperationTrait, ItemSearchTrait};

pub(crate) const CMD_NAME: &str = "search";

pub(crate) struct SearchCmd;

impl SearchCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for SearchCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Searches package").args(&[
            Arg::with_name("name")
                .value_name("string")
                .short("n")
                .long("name")
                .help("Package name")
                .takes_value(true),
            Arg::with_name("owner")
                .value_name("string")
                .short("r")
                .long("owner")
                .help("Package owner")
                .takes_value(true),
            Arg::with_name("pattern")
                .value_name("string")
                .short("p")
                .long("pattern")
                .help("Regex pattern")
                .takes_value(true),
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Show all release versions of package given '--name' specified)"),
        ])
    }

    fn run(&self, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let pkg_service = container.get::<PackageService>().unwrap();

        if matches.is_present("name") && matches.is_present("all") {
            let pkgs: Vec<PackageSummary> = pkg_service
                .find(&matches.value_of("name").unwrap().to_string())?
                .into_iter()
                .map(|it| PackageSummary::from(it))
                .collect();

            return output!(config.output_format, .display(
                stdout(),
                &pkgs,
                None,
                None,
            ));
        }

        let pkgs: Vec<PackageSummary> = pkg_service
            .search(
                matches.value_of("name"),
                matches.value_of("pattern"),
                matches.value_of("owner"),
            )?
            .into_iter()
            .map(|it| PackageSummary::from(it))
            .collect();

        output!(config.output_format, .display(
            stdout(),
            &pkgs,
            None,
            None,
        ))
    }
}