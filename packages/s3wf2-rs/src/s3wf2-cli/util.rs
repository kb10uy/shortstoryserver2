use s3wf2::error::Error;
use std::process::exit;

/// Shows document errors, and exit(1).
pub fn exit_document_errors(errors: &[Error]) -> ! {
    use ansi_term::Colour::{Red, White};

    eprintln!("{}", Red.bold().paint("Failed to parse!"));
    for error in errors {
        eprintln!(
            "{} {}: {}",
            Red.bold().paint("Error"),
            White.paint(format!("{}", error.line())),
            format!("{}", error.reason())
        );
    }

    exit(1);
}
