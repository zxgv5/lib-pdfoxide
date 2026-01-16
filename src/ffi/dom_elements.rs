//! DOM Elements C API for element access and manipulation
//!
//! Provides FFI functions for:
//! - Finding and enumerating page elements
//! - Accessing element properties (type, position, content)
//! - Modifying element attributes
//! - Type-specific operations (text, image, path)

use crate::editor::dom::PdfElement;
use std::os::raw::c_char;
use std::ptr;

use super::exceptions::ErrorCode;
use super::utils::rust_string_to_c;

// Element type constants
pub const ELEMENT_TYPE_TEXT: i32 = 0;
pub const ELEMENT_TYPE_IMAGE: i32 = 1;
pub const ELEMENT_TYPE_PATH: i32 = 2;
pub const ELEMENT_TYPE_TABLE: i32 = 3;
pub const ELEMENT_TYPE_STRUCTURE: i32 = 4;

/// Get element type as integer constant
pub fn element_to_type_code(element: &PdfElement) -> i32 {
    match element {
        PdfElement::Text(_) => ELEMENT_TYPE_TEXT,
        PdfElement::Image(_) => ELEMENT_TYPE_IMAGE,
        PdfElement::Path(_) => ELEMENT_TYPE_PATH,
        PdfElement::Table(_) => ELEMENT_TYPE_TABLE,
        PdfElement::Structure(_) => ELEMENT_TYPE_STRUCTURE,
    }
}

/// Opaque handle for DOM element
pub struct PdfElementHandle(Box<PdfElement>);

/// Find all elements of a given type on a page
///
/// # Arguments
/// * `handle` - The page handle
/// * `element_type` - Type of element to find (ELEMENT_TYPE_*)
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// Count of elements found, or -1 on error
#[no_mangle]
pub unsafe extern "C" fn pdf_page_find_elements_count(
    handle: *const super::dom::PdfPageHandle,
    _element_type: i32,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    // Placeholder: full implementation depends on Rust API providing element enumeration
    // For now, return 0 elements
    *error_code = ErrorCode::Success as i32;
    0
}

/// Get text content of a text element
///
/// # Returns
/// UTF-8 null-terminated string pointer. Must be freed with free_string().
#[no_mangle]
pub unsafe extern "C" fn pdf_text_element_get_content(
    handle: *const PdfElementHandle,
    error_code: *mut i32,
) -> *mut c_char {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return ptr::null_mut();
    }

    let element = &(*handle).0;

    match element.as_ref() {
        PdfElement::Text(text) => {
            *error_code = ErrorCode::Success as i32;
            rust_string_to_c(text.text().to_string())
        },
        _ => {
            *error_code = ErrorCode::InvalidStateError as i32;
            ptr::null_mut()
        },
    }
}

/// Get font size of a text element
#[no_mangle]
pub unsafe extern "C" fn pdf_text_element_get_font_size(
    handle: *const PdfElementHandle,
    error_code: *mut i32,
) -> f32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return 0.0;
    }

    let element = &(*handle).0;

    match element.as_ref() {
        PdfElement::Text(text) => {
            *error_code = ErrorCode::Success as i32;
            text.font_size()
        },
        _ => {
            *error_code = ErrorCode::InvalidStateError as i32;
            0.0
        },
    }
}

/// Get bounding box of an element
///
/// # Arguments
/// * `handle` - The element handle
/// * `x` - Output parameter for x coordinate
/// * `y` - Output parameter for y coordinate
/// * `width` - Output parameter for width
/// * `height` - Output parameter for height
#[no_mangle]
pub unsafe extern "C" fn pdf_element_get_bbox(
    handle: *const PdfElementHandle,
    x: *mut f32,
    y: *mut f32,
    width: *mut f32,
    height: *mut f32,
) {
    if handle.is_null() || x.is_null() || y.is_null() || width.is_null() || height.is_null() {
        return;
    }

    let element = &(*handle).0;
    let bbox = match element.as_ref() {
        PdfElement::Text(text) => text.bbox(),
        PdfElement::Image(img) => img.bbox(),
        PdfElement::Path(path) => path.bbox(),
        PdfElement::Table(table) => table.bbox(),
        PdfElement::Structure(s) => s.bbox(),
    };

    *x = bbox.x;
    *y = bbox.y;
    *width = bbox.width;
    *height = bbox.height;
}

/// Get element type as integer constant
#[no_mangle]
pub unsafe extern "C" fn pdf_element_get_type(handle: *const PdfElementHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }

    let element = &(*handle).0;
    element_to_type_code(element)
}

/// Free an element handle
#[no_mangle]
pub unsafe extern "C" fn pdf_element_free(handle: *mut PdfElementHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// Get image format as integer (0=JPEG, 1=PNG, 2=Unknown, etc.)
#[no_mangle]
pub unsafe extern "C" fn pdf_image_element_get_format(
    handle: *const PdfElementHandle,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let element = &(*handle).0;

    match element.as_ref() {
        PdfElement::Image(img) => {
            *error_code = ErrorCode::Success as i32;
            match img.format() {
                crate::elements::ImageFormat::Jpeg => 0,
                crate::elements::ImageFormat::Png => 1,
                crate::elements::ImageFormat::Jpeg2000 => 2,
                crate::elements::ImageFormat::Jbig2 => 3,
                crate::elements::ImageFormat::Raw => 4,
                crate::elements::ImageFormat::Unknown => 5,
            }
        },
        _ => {
            *error_code = ErrorCode::InvalidStateError as i32;
            -1
        },
    }
}

/// Get image dimensions (width, height)
#[no_mangle]
pub unsafe extern "C" fn pdf_image_element_get_dimensions(
    handle: *const PdfElementHandle,
    width: *mut u32,
    height: *mut u32,
    error_code: *mut i32,
) {
    if handle.is_null() || width.is_null() || height.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return;
    }

    let element = &(*handle).0;

    match element.as_ref() {
        PdfElement::Image(img) => {
            let (w, h) = img.dimensions();
            *width = w;
            *height = h;
            *error_code = ErrorCode::Success as i32;
        },
        _ => {
            *error_code = ErrorCode::InvalidStateError as i32;
        },
    }
}

/// Get the raw image data from an image element
///
/// # Arguments
/// * `handle` - The element handle
/// * `data` - Output buffer for image data
/// * `max_len` - Maximum length of data buffer
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// The number of bytes written to data buffer, or -1 on error
#[no_mangle]
pub unsafe extern "C" fn pdf_image_element_get_data(
    handle: *const PdfElementHandle,
    data: *mut u8,
    max_len: i32,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || data.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let element = &(*handle).0;

    match element.as_ref() {
        PdfElement::Image(_img) => {
            // Placeholder: full implementation would extract raw image bytes
            *error_code = ErrorCode::Success as i32;
            0
        },
        _ => {
            *error_code = ErrorCode::InvalidStateError as i32;
            -1
        },
    }
}

/// Get the size of the image data
///
/// # Arguments
/// * `handle` - The element handle
/// * `error_code` - Output parameter for error code
///
/// # Returns
/// The size in bytes of the image data, or -1 on error
#[no_mangle]
pub unsafe extern "C" fn pdf_image_element_get_data_size(
    handle: *const PdfElementHandle,
    error_code: *mut i32,
) -> i32 {
    if handle.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            *error_code = ErrorCode::InternalError as i32;
        }
        return -1;
    }

    let element = &(*handle).0;

    match element.as_ref() {
        PdfElement::Image(_img) => {
            // Placeholder: full implementation would return actual image data size
            *error_code = ErrorCode::Success as i32;
            0
        },
        _ => {
            *error_code = ErrorCode::InvalidStateError as i32;
            -1
        },
    }
}
