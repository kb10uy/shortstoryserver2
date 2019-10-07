use clap::ArgMatches;
use std::{error::Error, fs::File, io, io::prelude::*, time::Instant};

use crate::util::exit_document_errors;
use s3wf2::{
    document::Document,
    emitter::{console::ConsoleEmitter, html::HtmlEmitter, Emit},
    parser::Parser,
};

pub fn subcommand_format(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let verbose = args.is_present("verbose");

    let stdin;
    let mut source: Box<dyn io::Read> = match args.value_of("INPUT") {
        None | Some("-") => {
            stdin = io::stdin();
            Box::new(io::BufReader::new(stdin.lock()))
        }
        Some(filename) => Box::new(io::BufReader::new(File::open(filename)?)),
    };
    let mut text = String::new();
    source.read_to_string(&mut text)?;

    let parser = Parser::new();
    let parse_started = Instant::now();
    let document = match parser.parse(&text) {
        Ok(document) => document,
        Err(errors) => {
            exit_document_errors(&errors);
        }
    };
    let parse_time = parse_started.elapsed();
    if verbose {
        use ansi_term::Color::Green;
        eprintln!(
            "{} It took {}ms.",
            Green.bold().paint("Parse succeeded."),
            parse_time.as_millis()
        );
    }

    match args.value_of("type") {
        Some("html") => {
            emit_html(&document, args)?;
        }
        Some("console") => {
            emit_console(&document)?;
        }
        _ => panic!("Invalid format type"),
    }

    Ok(())
}

fn emit_html(document: &Document, args: &ArgMatches) -> Result<(), io::Error> {
    let mut emitter = HtmlEmitter::new(4);
    match args.value_of("output") {
        None | Some("-") => {
            let stdout = io::stdout();
            let mut writer = io::BufWriter::with_capacity(1 << 16, stdout.lock());
            emitter.emit(&mut writer, document)?;
            writer.flush()
        }
        Some(file) => {
            let mut writer = io::BufWriter::with_capacity(1 << 16, File::create(file)?);
            emitter.emit(&mut writer, document)?;
            writer.flush()
        }
    }
}

fn emit_console(document: &Document) -> Result<(), io::Error> {
    let mut emitter = ConsoleEmitter::new();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::with_capacity(1 << 16, stdout.lock());
    emitter.emit(&mut writer, document)?;
    writer.flush()
}
