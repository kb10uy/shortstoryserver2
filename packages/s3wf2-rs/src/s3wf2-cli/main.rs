use clap::{load_yaml, App};
use std::error::Error;

mod sub_debug;
mod sub_format;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("./cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    match matches.subcommand() {
        ("debug", Some(args)) => {
            sub_debug::subcommand_debug(args)?;
        }
        ("format", Some(args)) => {
            sub_format::subcommand_format(args)?;
        }
        ("help", _) => {}
        _ => {}
    }
    Ok(())
}
