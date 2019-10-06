use clap::ArgMatches;
use std::{error::Error, fs::File, io, io::prelude::*};

use crate::util::exit_document_errors;
use s3wf2::{
    emitter::{html::HtmlEmitter, Emit},
    parser::Parser,
};

pub fn subcommand_format(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let (stdin, stdout);
    let mut source: Box<dyn io::Read> = match args.value_of("INPUT") {
        None | Some("-") => {
            stdin = io::stdin();
            Box::new(io::BufReader::new(stdin.lock()))
        }
        Some(filename) => Box::new(io::BufReader::new(File::open(filename)?)),
    };
    let mut destination: Box<dyn io::Write> = match args.value_of("output") {
        None | Some("-") => {
            stdout = io::stdout();
            Box::new(stdout.lock())
        }
        Some(filename) => Box::new(io::BufWriter::new(File::create(filename)?)),
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

    match args.value_of("type") {
        Some("html") => {
            let emitter = HtmlEmitter::new(4);
            emitter.emit(&mut destination, &document)?;
        }
        _ => panic!("Invalid format type"),
    }

    Ok(())
}
