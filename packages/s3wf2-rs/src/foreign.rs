use crate::{
    document::Document,
    emitter::{html::HtmlEmitter, Emit},
    error::Error,
    parser::Parser,
};
use libc::{c_char, c_int, c_void, size_t};
use std::{
    ffi::{CStr, CString},
    io::{Error as IoError, ErrorKind, Write},
};

/// Buffered Writer that has hook object.
struct BufHookWriter<H, S> {
    buffer: Vec<u8>,
    written: usize,
    hook: H,
    state: S,
}

impl<S, H: Fn(&[u8], &S) -> Result<(), IoError>> BufHookWriter<H, S> {
    /// Creates a new instance.
    pub fn new(hook: H, state: S) -> BufHookWriter<H, S> {
        BufHookWriter {
            buffer: vec![0; 8192],
            written: 0,
            hook,
            state,
        }
    }

    /// Call the hook function.
    fn call_hook(&mut self) -> Result<(), IoError> {
        let hook = &self.hook;
        hook(&self.buffer[..self.written], &self.state)
    }
}

impl<S, H: Fn(&[u8], &S) -> Result<(), IoError>> Write for BufHookWriter<H, S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        let available = self.buffer.len() - self.written;
        let mut target = &mut self.buffer[self.written..];

        if available <= buf.len() {
            target.write_all(&buf[..available])?;
            self.call_hook()?;

            self.written = 0;
            Ok(available)
        } else {
            target.write_all(buf)?;

            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<(), IoError> {
        self.call_hook()?;
        self.written = 0;

        Ok(())
    }
}

/// Represents the status of foreign APIs.
#[repr(C)]
pub enum Status {
    /// Operation was done successfully.
    Success = 0,

    /// In `s3wf2_get_next_error`, no more error was found.
    NoMoreError,

    /// Some error occurred.
    Error,

    /// The `Environment` is invalid.
    InvalidEnvironment,

    /// The source text is invalid.
    /// It may be null or has invalid UTF-8 sequences.
    InvalidSource,

    /// An error occurred in parseing.
    ParseError,
}

/// The wrapper of Parser and Document.
pub struct Environment<'a> {
    parser: Parser,
    document: Option<Document<'a>>,
    errors: Option<Vec<Error>>,
    error_position: Option<usize>,
}

impl<'a> Environment<'a> {
    /// Resurrects the `Environment` instance from pointer.
    unsafe fn resurrect(pointer: *mut Environment) -> Result<&mut Environment, Status> {
        if pointer.is_null() {
            Err(Status::Error)
        } else {
            Ok(&mut *pointer)
        }
    }
}

/// Creates an `Environment` instance, and return its pointer.
#[no_mangle]
pub extern "C" fn s3wf2_init<'a>() -> *mut Environment<'a> {
    let environment = Box::new(Environment {
        parser: Parser::new(),
        document: None,
        errors: None,
        error_position: None,
    });

    Box::into_raw(environment)
}

/// Releases the allocated area of `Environment`.
///
/// # Safety
/// This function dereferences the `environment` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_free(environment: *mut Environment) -> Status {
    let owned_box = Box::from_raw(if environment.is_null() {
        return Status::InvalidEnvironment;
    } else {
        environment
    });

    drop(owned_box);
    Status::Success
}

/// Parses the `source` text.
///
/// # Safety
/// This function dereferences the `environment` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_parse(
    environment: *mut Environment,
    source: *const c_char,
) -> Status {
    let mut environment = match Environment::resurrect(environment) {
        Ok(env) => env,
        Err(error) => return error,
    };
    let source = match CStr::from_ptr(source).to_str() {
        Ok(s) => s,
        Err(_) => return Status::InvalidSource,
    };

    let parse_result = environment.parser.parse(source);
    match parse_result {
        Ok(document) => {
            environment.document = Some(document);
            environment.errors = None;
            environment.error_position = None;

            Status::Success
        }
        Err(errors) => {
            environment.document = None;
            environment.errors = Some(errors);
            environment.error_position = Some(0);

            Status::ParseError
        }
    }
}

/// Converts the held `Document` into HTML, and return it.
///
/// The returned pointer of `ptr` is allocated in library,
/// therefore you should not release it.
/// Instead, use `s3wf2_free_string`.
///
/// # Safety
/// This function dereferences the `environment` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_get_document_string(
    environment: *mut Environment,
    ptr: *mut *mut c_char,
) -> Status {
    let environment = match Environment::resurrect(environment) {
        Ok(env) => env,
        Err(error) => return error,
    };

    let document = match &environment.document {
        Some(doc) => doc,
        None => return Status::Error,
    };

    let mut emitter = HtmlEmitter::new(4);
    let mut document_buffer = Vec::with_capacity(8192);
    if emitter.emit(&mut document_buffer, document).is_err() {
        return Status::Error;
    };

    let document_string = CString::from_vec_unchecked(document_buffer);
    *ptr = document_string.into_raw();

    Status::Success
}

/// Converts the held `Document` into HTML, and return it via the callback function.
///
/// The returned pointer of `ptr` is allocated in library,
/// therefore you should not release it.
/// Instead, use `s3wf2_free_string`.
///
/// # Safety
/// This function dereferences the `environment` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_get_document_buffered(
    environment: *mut Environment,
    callback: fn(*const c_char, size_t, *mut c_void) -> c_int,
    state: *mut c_void,
) -> Status {
    let environment = match Environment::resurrect(environment) {
        Ok(env) => env,
        Err(error) => return error,
    };

    let document = match &environment.document {
        Some(doc) => doc,
        None => return Status::Error,
    };

    let mut emitter = HtmlEmitter::new(4);
    let hook = |buf: &[u8], state: &*mut c_void| {
        let cb_result = callback(buf.as_ptr() as *mut c_char, buf.len(), *state);

        match cb_result {
            0 => Ok(()),
            _ => {
                let error = IoError::from(ErrorKind::Other);
                Err(error)
            }
        }
    };

    let mut writer = BufHookWriter::new(&hook, state);
    if emitter.emit(&mut writer, document).is_err() {
        return Status::Error;
    };

    Status::Success
}

/// Releases the allocated buffer given from libs3wf2.
///
/// The returned pointer of `ptr` is allocated in library,
/// therefore you should not release it.
/// Instead, use `s3wf2_free_string`.
///
/// # Safety
/// This function dereferences the `ptr` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_free_string(ptr: *mut c_char) -> Status {
    if ptr.is_null() {
        Status::Error
    } else {
        drop(CString::from_raw(ptr));
        Status::Success
    }
}

/// Gets the message string of following error.
/// If all the errors are read, it returns `Status::NoMoreError`.
///
/// # Safety
/// This function dereferences the `environment` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_get_next_error(
    environment: *mut Environment,
    ptr: *mut *mut c_char,
) -> Status {
    let environment = match Environment::resurrect(environment) {
        Ok(env) => env,
        Err(error) => return error,
    };

    let errors = match &environment.errors {
        Some(errors) => errors,
        None => return Status::NoMoreError,
    };
    let next_index = environment.error_position.expect("Error position not set");

    match errors.get(next_index) {
        Some(error) => {
            let message = error.to_string().into_bytes();
            *ptr = CString::new(message)
                .expect("Error message contains a NUL byte")
                .into_raw();
            environment.error_position = Some(next_index + 1);

            Status::Success
        }
        None => Status::NoMoreError,
    }
}

/// Resets the position of error list.
///
/// # Safety
/// This function dereferences the `environment` pointer,
/// so may cause access violation error.
#[no_mangle]
pub unsafe extern "C" fn s3wf2_reset_error(environment: *mut Environment) -> Status {
    let environment = match Environment::resurrect(environment) {
        Ok(env) => env,
        Err(error) => return error,
    };

    if environment.error_position.is_some() {
        environment.error_position = Some(0);
    }

    Status::Success
}
