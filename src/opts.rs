use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "caniuse", about = "A CLI client for caniuse.com")]
pub struct Opts {
    #[structopt(short, long, help = "Print the version and exit")]
    pub version: bool,
    #[structopt(short, long, help = "Force-update cached data before querying")]
    pub update: bool,
    #[structopt(
        short,
        long,
        help = "Print currently cached data and exit instead of using fuzzy-finder"
    )]
    pub dump: bool,
    #[structopt(
        short,
        long,
        help = "Pretty-print JSON output, must be combined with --dump or --query option"
    )]
    pub pretty: bool,
    #[structopt(
        long,
        help = "Transform JSON structure for use in Alfred workflow. See: https://www.alfredapp.com/help/workflows/inputs/script-filter/json/"
    )]
    pub alfred: bool,
    #[structopt(
        short,
        long,
        help = "Run a query and output the JSON results instead of using the fuzzy-finder"
    )]
    pub query: Option<String>,
}
