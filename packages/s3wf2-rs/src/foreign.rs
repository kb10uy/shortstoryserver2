use std::ffi::CStr;
use libc::{c_char, c_int};
use crate::{
    parser::Parser,
    document::Document,
    error::Error,
};

/// C API の処理結果を表す enum
#[repr(C)]
pub enum Status {
    Success = 0,
    Invalid,
    InvalidSource,
    ParseError,
}

/// C API で取り回すための [Parser] とゆかいな仲間たち
pub struct ParserWrapper<'a> {
    parser: Parser,
    parse_result: Option<Result<Document<'a>, Vec<Error>>>,
    errors_position: Option<usize>,
}

impl<'a> ParserWrapper<'a> {
    fn new() -> ParserWrapper<'a> {
        ParserWrapper {
            parser: Parser::new(),
            parse_result: None,
            errors_position: None,
        }
    }
}

/// 新しい ParserWrapper の
#[no_mangle]
pub extern fn s3wf2_parser_new<'a>() -> *mut ParserWrapper<'a> {
    let parser = Box::new(ParserWrapper::new());

    Box::into_raw(parser)
}


#[no_mangle]
pub unsafe extern fn s3wf2_parser_free(parser: *mut ParserWrapper) {
    if parser.is_null() {
        return;
    }
    let _destroyed = Box::from_raw(parser);
}

#[no_mangle]
pub unsafe extern fn s3wf2_parser_parse(parser: *mut ParserWrapper, source: *const c_char) -> Status {
    if parser.is_null() {
        return Status::Invalid;
    }
    let parser: &mut ParserWrapper = &mut *parser;

    let src_str = match CStr::from_ptr(source).to_str() {
        Ok(s) => s,
        Err(_) => {
            return Status::InvalidSource;
        }
    };
    let result = parser.parser.parse(src_str);
    let status = if result.is_ok() {
        Status::Success
    } else {
        Status::ParseError
    };
    parser.parse_result = Some(result);

    status
}

#[no_mangle]
pub unsafe extern fn s3wf2_parser_next_error(parser: *mut ParserWrapper) -> c_int {
    if parser.is_null() {
        return 0;
    }
    let parser: &mut ParserWrapper = &mut *parser;

    match parser.errors_position {
        Some(next) => {
            1
        }
        None => 0
    }
}
