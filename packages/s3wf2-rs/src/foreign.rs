use crate::{
    document::Document,
    emitter::{html::HtmlEmitter, Emit},
    error::Error,
    parser::Parser,
};
use libc::{c_char, c_int, c_uchar, size_t};
use std::{
    ffi::{CStr, CString},
    mem::forget,
    io::Write,
    slice::from_raw_parts_mut,
};

#[repr(C)]
pub enum Status {
    Success = 0,
    Invalid,
    InvalidSource,
    ParseError,
}

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

    fn reset(&mut self) {
        self.parse_result = None;
        self.errors_position = None;
    }
}

#[no_mangle]
pub extern "C" fn s3wf2_parser_new<'a>() -> *mut ParserWrapper<'a> {
    let parser = Box::new(ParserWrapper::new());

    Box::into_raw(parser)
}

#[no_mangle]
pub unsafe extern "C" fn s3wf2_parser_free(parser: *mut ParserWrapper) {
    if parser.is_null() {
        return;
    }
    let _destroyed = Box::from_raw(parser);
}

#[no_mangle]
pub unsafe extern "C" fn s3wf2_parser_reset(parser: *mut ParserWrapper) -> Status {
    if parser.is_null() {
        return Status::Invalid;
    }
    let parser: &mut ParserWrapper = &mut *parser;

    parser.reset();
    Status::Success
}

#[no_mangle]
pub unsafe extern "C" fn s3wf2_parser_parse(
    parser: *mut ParserWrapper,
    source: *const c_char,
) -> Status {
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
        parser.errors_position = Some(0);
        Status::ParseError
    };
    parser.parse_result = Some(result);

    status
}

#[no_mangle]
pub unsafe extern "C" fn s3wf2_parser_next_error(
    parser: *mut ParserWrapper,
    buffer: *mut c_char,
    buffer_length: size_t,
) -> c_int {
    if parser.is_null() {
        return 0;
    }
    let parser: &mut ParserWrapper = &mut *parser;

    let errors = match &parser.parse_result {
        Some(Err(errors)) => errors,
        _ => return 0,
    };

    match parser.errors_position {
        Some(next) => {
            if next < errors.len() {
                let mut sized_buffer = from_raw_parts_mut(buffer as *mut c_uchar, buffer_length);
                let message = CString::new(format!("{}", errors[next])).unwrap();
                match sized_buffer.write_all(&message.to_bytes_with_nul()) {
                    Ok(_) => {
                        parser.errors_position = Some(next + 1);
                        1
                    }
                    Err(_) => {
                        parser.errors_position = None;
                        0
                    }
                }
            } else {
                parser.errors_position = None;
                0
            }
        }
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn s3wf2_emit_html(
    parser: *const ParserWrapper,
    result_ptr: *mut *const c_char,
) -> Status {
    if parser.is_null() {
        return Status::Invalid;
    }
    let parser: &ParserWrapper = &*parser;

    let document = match &parser.parse_result {
        Some(Ok(document)) => document,
        _ => return Status::Invalid,
    };
    let mut html_buffer: Vec<u8> = Vec::with_capacity(1 << 16);
    let emitter = HtmlEmitter::new(4);
    match emitter.emit(&mut html_buffer, document) {
        Ok(()) => {
            let result = CString::from_vec_unchecked(html_buffer);
            *result_ptr = result.as_ptr();
            forget(result);
            Status::Success
        }
        Err(_) => Status::Invalid,
    }
}
