use clap::ArgMatches;
use std::{error::Error, fs::File, io};

use crate::util::exit_document_errors;
use s3wf2::parser::Parser;

pub fn subcommand_debug(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let (mut stdin, mut file);
    let source: &mut dyn io::Read = match args.value_of("INPUT") {
        None | Some("-") => {
            stdin = io::stdin();
            &mut stdin
        }
        Some(filename) => {
            file = io::BufReader::new(File::open(filename)?);
            &mut file
        }
    };

    let mut text = String::new();
    source.read_to_string(&mut text)?;

    let parser = Parser::new();
    match parser.parse(&text) {
        Ok(document) => {
            println!("{}", document);
            Ok(())
        }
        Err(errors) => {
            exit_document_errors(&errors);
        }
    }
}
