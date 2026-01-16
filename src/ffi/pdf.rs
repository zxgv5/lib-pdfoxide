//! Pdf and PdfBuilder C API for creating and universal access
//!
//! Provides FFI functions for:
//! - Creating PDFs from various sources (Markdown, HTML, Text, Images)
//! - Universal Pdf API combining read/create/edit
//! - Builder pattern for complex PDF construction

use crate::api::Pdf as RustPdf;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use super::exceptions::{pdf_error_to_code, ErrorCode};

/// Opaque handle for Pdf universal API
pub struct PdfHandle(RustPdf);

/// Create a PDF from Markdown text
///
/// # Arguments
/// * `markdown` - UTF-8 null-terminated Markdown content
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// Opaque handle to Pdf, or null on error
#[no_mangle]
pub unsafe extern "C" fn pdf_from_markdown(
    markdown: *const c_char,
    error_code: *mut i32,
) -> *mut PdfHandle {
    if error_code.is_null() {
        return ptr::null_mut();
    }

    let markdown_str = match CStr::from_ptr(markdown).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return ptr::null_mut();
        },
    };

    match RustPdf::from_markdown(markdown_str) {
        Ok(pdf) => {
            *error_code = ErrorCode::Success as i32;
            Box::into_raw(Box::new(PdfHandle(pdf)))
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Create a PDF from HTML content
///
/// # Arguments
/// * `html` - UTF-8 null-terminated HTML content
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// Opaque handle to Pdf, or null on error
#[no_mangle]
pub unsafe extern "C" fn pdf_from_html(
    html: *const c_char,
    error_code: *mut i32,
) -> *mut PdfHandle {
    if error_code.is_null() {
        return ptr::null_mut();
    }

    let html_str = match CStr::from_ptr(html).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return ptr::null_mut();
        },
    };

    match RustPdf::from_html(html_str) {
        Ok(pdf) => {
            *error_code = ErrorCode::Success as i32;
            Box::into_raw(Box::new(PdfHandle(pdf)))
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Create a PDF from plain text
///
/// # Arguments
/// * `text` - UTF-8 null-terminated text content
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// Opaque handle to Pdf, or null on error
#[no_mangle]
pub unsafe extern "C" fn pdf_from_text(
    text: *const c_char,
    error_code: *mut i32,
) -> *mut PdfHandle {
    if error_code.is_null() {
        return ptr::null_mut();
    }

    let text_str = match CStr::from_ptr(text).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return ptr::null_mut();
        },
    };

    match RustPdf::from_text(text_str) {
        Ok(pdf) => {
            *error_code = ErrorCode::Success as i32;
            Box::into_raw(Box::new(PdfHandle(pdf)))
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Save PDF to file
///
/// # Arguments
/// * `handle` - The PDF handle
/// * `path` - UTF-8 null-terminated output file path
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// 0 on success, non-zero on error
#[no_mangle]
pub unsafe extern "C" fn pdf_save(
    handle: *mut PdfHandle,
    path: *const c_char,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return -1;
        },
    };

    let pdf = &mut (*handle).0;
    match pdf.save(path_str) {
        Ok(_) => {
            *error_code = ErrorCode::Success as i32;
            0
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            -1
        },
    }
}

/// Save PDF to bytes buffer
///
/// # Arguments
/// * `handle` - The PDF handle
/// * `output_ptr` - Output parameter for byte buffer pointer
/// * `output_len` - Output parameter for buffer size
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// 0 on success, non-zero on error
/// The output buffer must be freed with FreeBytes
#[no_mangle]
pub unsafe extern "C" fn pdf_save_to_bytes(
    handle: *const PdfHandle,
    output_ptr: *mut *mut u8,
    output_len: *mut usize,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || output_ptr.is_null() || output_len.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let pdf = &(*handle).0;
    let bytes = pdf.as_bytes().to_vec();
    let len = bytes.len();
    let boxed: Box<[u8]> = bytes.into_boxed_slice();
    *output_ptr = Box::into_raw(boxed) as *mut u8;
    *output_len = len;
    *error_code = ErrorCode::Success as i32;
    0
}

/// Get the number of pages in the PDF
#[no_mangle]
pub unsafe extern "C" fn pdf_get_page_count(handle: *mut PdfHandle, error_code: *mut i32) -> i32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let pdf = &mut (*handle).0;
    match pdf.page_count() {
        Ok(count) => {
            *error_code = ErrorCode::Success as i32;
            count as i32
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            -1
        },
    }
}

/// Free a Pdf handle
///
/// # Safety
/// The handle must be valid and not used after this call.
#[no_mangle]
pub unsafe extern "C" fn pdf_free(handle: *mut PdfHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}
