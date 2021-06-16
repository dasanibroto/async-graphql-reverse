use async_graphql_reverse::*;
use clap::Clap;
use env_logger;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "tacogips")]
struct Opts {
    #[clap(long, short)]
    input_schema: String,

    #[clap(long, short)]
    output_dir: String,
}

fn setup_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init();
}

fn main() {
    setup_logger();
    let opts: Opts = Opts::parse();
    let structured_schema = parse_schema_file(&opts.input_schema).unwrap();

    let config = RendererConfig {
        custom_datasource_using: None,
    };
    output(&opts.output_dir, structured_schema, config).expect("error ")
}
