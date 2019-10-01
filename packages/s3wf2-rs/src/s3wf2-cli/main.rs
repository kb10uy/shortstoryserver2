use std::error::Error;
use clap::{App, load_yaml};

mod sub_debug;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("./cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    match matches.subcommand() {
        ("debug", Some(args)) => {
            sub_debug::subcommand_debug(args)?;
        }
        ("help", _) => {}
        _ => {}
    }
    Ok(())
}
