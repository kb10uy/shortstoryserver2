use clap::ArgMatches;
use std::{error::Error, fs::File, io, io::prelude::*};

use crate::util::exit_document_errors;
use s3wf2::{
    emitter::{html::HtmlEmitter, Emit},
    parser::Parser,
};

pub fn subcommand_format(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
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
    let document = match parser.parse(&text) {
        Ok(document) => document,
        Err(errors) => {
            exit_document_errors(&errors);
        }
    };

    let formatted = match args.value_of("type") {
        Some("html") => {
            let emitter = HtmlEmitter::new(4);
            emitter
                .emit(&document)
                .expect("Unexpected HTML error occured")
        }
        _ => panic!("Invalid format type"),
    };

    let stdout = io::stdout();
    let mut locked = io::BufWriter::new(stdout.lock());
    write!(locked, "{}", formatted)?;
    Ok(())
}
