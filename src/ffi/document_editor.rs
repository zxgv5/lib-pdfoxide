//! DocumentEditor C API for editing and manipulating PDF documents
//!
//! Provides FFI functions for:
//! - Opening PDFs for editing
//! - Modifying document metadata (title, author, etc.)
//! - Managing pages (add, remove, reorder)
//! - Saving changes
//! - Accessing and modifying page content

use crate::editor::DocumentEditor as RustDocumentEditor;
use crate::editor::EditableDocument;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

use super::exceptions::{pdf_error_to_code, ErrorCode};
use super::utils::rust_string_to_c;

/// Opaque handle for DocumentEditor
pub struct DocumentEditorHandle(RustDocumentEditor);

/// Open a PDF document for editing
///
/// # Arguments
/// * `path` - UTF-8 null-terminated file path
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// Opaque handle to DocumentEditor, or null on error
#[no_mangle]
pub unsafe extern "C" fn document_editor_open(
    path: *const c_char,
    error_code: *mut i32,
) -> *mut DocumentEditorHandle {
    if error_code.is_null() {
        return ptr::null_mut();
    }

    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return ptr::null_mut();
        },
    };

    match RustDocumentEditor::open(path_str) {
        Ok(editor) => {
            *error_code = ErrorCode::Success as i32;
            Box::into_raw(Box::new(DocumentEditorHandle(editor)))
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Free a DocumentEditor handle
///
/// # Safety
/// The handle must be valid and not used after this call.
#[no_mangle]
pub unsafe extern "C" fn document_editor_free(handle: *mut DocumentEditorHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// Check if the document has been modified
#[no_mangle]
pub unsafe extern "C" fn document_editor_is_modified(handle: *const DocumentEditorHandle) -> bool {
    if handle.is_null() {
        return false;
    }

    let editor = &(*handle).0;
    editor.is_modified()
}

/// Get the source file path
///
/// # Returns
/// UTF-8 null-terminated string pointer. Must be freed with free_string().
#[no_mangle]
pub unsafe extern "C" fn document_editor_get_source_path(
    handle: *const DocumentEditorHandle,
    error_code: *mut i32,
) -> *mut c_char {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return ptr::null_mut();
    }

    let editor = &(*handle).0;
    let path = editor.source_path();
    *error_code = ErrorCode::Success as i32;
    rust_string_to_c(path.to_string())
}

/// Get the PDF version as (major, minor)
#[no_mangle]
pub unsafe extern "C" fn document_editor_get_version(
    handle: *const DocumentEditorHandle,
    major: *mut u8,
    minor: *mut u8,
) {
    if handle.is_null() || major.is_null() || minor.is_null() {
        return;
    }

    let editor = &(*handle).0;
    let version = editor.version();
    *major = version.0;
    *minor = version.1;
}

/// Get the number of pages in the document
#[no_mangle]
pub unsafe extern "C" fn document_editor_get_page_count(
    handle: *mut DocumentEditorHandle,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let editor = &mut (*handle).0;
    match editor.page_count() {
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

/// Get document title
///
/// # Returns
/// UTF-8 null-terminated string pointer. Must be freed with free_string().
/// Returns null if title is not set or on error.
#[no_mangle]
pub unsafe extern "C" fn document_editor_get_title(
    handle: *mut DocumentEditorHandle,
    error_code: *mut i32,
) -> *mut c_char {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return ptr::null_mut();
    }

    let editor = &mut (*handle).0;
    match editor.title() {
        Ok(Some(title)) => {
            *error_code = ErrorCode::Success as i32;
            rust_string_to_c(title)
        },
        Ok(None) => {
            *error_code = ErrorCode::Success as i32;
            ptr::null_mut()
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Set document title
#[no_mangle]
pub unsafe extern "C" fn document_editor_set_title(
    handle: *mut DocumentEditorHandle,
    title: *const c_char,
    error_code: *mut i32,
) {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return;
    }

    let title_str = match CStr::from_ptr(title).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return;
        },
    };

    let editor = &mut (*handle).0;
    editor.set_title(title_str);
    *error_code = ErrorCode::Success as i32;
}

/// Get document author
///
/// # Returns
/// UTF-8 null-terminated string pointer. Must be freed with free_string().
/// Returns null if author is not set or on error.
#[no_mangle]
pub unsafe extern "C" fn document_editor_get_author(
    handle: *mut DocumentEditorHandle,
    error_code: *mut i32,
) -> *mut c_char {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return ptr::null_mut();
    }

    let editor = &mut (*handle).0;
    match editor.author() {
        Ok(Some(author)) => {
            *error_code = ErrorCode::Success as i32;
            rust_string_to_c(author)
        },
        Ok(None) => {
            *error_code = ErrorCode::Success as i32;
            ptr::null_mut()
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Set document author
#[no_mangle]
pub unsafe extern "C" fn document_editor_set_author(
    handle: *mut DocumentEditorHandle,
    author: *const c_char,
    error_code: *mut i32,
) {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return;
    }

    let author_str = match CStr::from_ptr(author).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return;
        },
    };

    let editor = &mut (*handle).0;
    editor.set_author(author_str);
    *error_code = ErrorCode::Success as i32;
}

/// Get document subject
///
/// # Returns
/// UTF-8 null-terminated string pointer. Must be freed with free_string().
/// Returns null if subject is not set or on error.
#[no_mangle]
pub unsafe extern "C" fn document_editor_get_subject(
    handle: *mut DocumentEditorHandle,
    error_code: *mut i32,
) -> *mut c_char {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return ptr::null_mut();
    }

    let editor = &mut (*handle).0;
    match editor.subject() {
        Ok(Some(subject)) => {
            *error_code = ErrorCode::Success as i32;
            rust_string_to_c(subject)
        },
        Ok(None) => {
            *error_code = ErrorCode::Success as i32;
            ptr::null_mut()
        },
        Err(e) => {
            *error_code = pdf_error_to_code(&e);
            ptr::null_mut()
        },
    }
}

/// Set document subject
#[no_mangle]
pub unsafe extern "C" fn document_editor_set_subject(
    handle: *mut DocumentEditorHandle,
    subject: *const c_char,
    error_code: *mut i32,
) {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return;
    }

    let subject_str = match CStr::from_ptr(subject).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = ErrorCode::ParseError as i32;
            return;
        },
    };

    let editor = &mut (*handle).0;
    editor.set_subject(subject_str);
    *error_code = ErrorCode::Success as i32;
}

/// Save document to file
///
/// # Arguments
/// * `handle` - The DocumentEditor handle
/// * `path` - UTF-8 null-terminated output file path
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// 0 on success, non-zero on error
#[no_mangle]
pub unsafe extern "C" fn document_editor_save(
    handle: *mut DocumentEditorHandle,
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

    let editor = &mut (*handle).0;
    match editor.save(Path::new(path_str)) {
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
