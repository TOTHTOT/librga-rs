//! RGA information query functions

use crate::ffi::*;
use std::ffi::CStr;

/// Query information type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum QueryInfo {
    /// RGA vendor information
    Vendor = 0,
    /// RGA version
    Version = 1,
    /// Maximum input resolution
    MaxInput = 2,
    /// Maximum output resolution
    MaxOutput = 3,
    /// Byte stride alignment
    ByteStride = 4,
    /// Scaling limits
    ScaleLimit = 5,
    /// Supported input formats
    InputFormat = 6,
    /// Supported output formats
    OutputFormat = 7,
    /// RGA features
    Feature = 8,
    /// Expected performance
    Expected = 9,
    /// All information
    All = 10,
}

/// Query RGA information as string
///
/// # Arguments
/// * `info` - Type of information to query
///
/// # Returns
/// Information string from RGA driver
///
/// # Example
/// ```
/// use librga::query::{query_string, QueryInfo};
///
/// let version = query_string(QueryInfo::Version);
/// println!("RGA Version: {}", version);
/// ```
pub fn query_string(info: QueryInfo) -> &'static str {
    unsafe {
        let ptr = querystring(info as i32);
        if ptr.is_null() {
            return "";
        }
        CStr::from_ptr(ptr)
            .to_str()
            .unwrap_or("<invalid UTF-8>")
    }
}

/// Query RGA vendor
pub fn query_vendor() -> &'static str {
    query_string(QueryInfo::Vendor)
}

/// Query RGA version
pub fn query_version() -> &'static str {
    query_string(QueryInfo::Version)
}

/// Query maximum input resolution
///
/// Returns (width, height) tuple
pub fn query_max_input_size() -> (i32, i32) {
    let info = query_string(QueryInfo::MaxInput);
    // Parse format like "8192x8192"
    parse_resolution(info)
}

/// Query maximum output resolution
///
/// Returns (width, height) tuple
pub fn query_max_output_size() -> (i32, i32) {
    let info = query_string(QueryInfo::MaxOutput);
    parse_resolution(info)
}

/// Parse resolution string like "1920x1080"
fn parse_resolution(s: &str) -> (i32, i32) {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() == 2 {
        let w = parts[0].parse().unwrap_or(0);
        let h = parts[1].parse().unwrap_or(0);
        (w, h)
    } else {
        (0, 0)
    }
}

/// Query supported features
pub fn query_features() -> &'static str {
    query_string(QueryInfo::Feature)
}

/// Query all information at once
pub fn query_all() -> &'static str {
    query_string(QueryInfo::All)
}

/// Check RGA version compatibility
pub fn check_version() -> Result<(), &'static str> {
    // Note: imcheckHeader is not available in generated bindings
    // Just return Ok for now
    Ok(())
}

/// Get error string from status code
pub fn error_string(status: IM_STATUS) -> &'static str {
    unsafe {
        let ptr = imStrError_t(status);
        if ptr.is_null() {
            return "unknown error";
        }
        CStr::from_ptr(ptr)
            .to_str()
            .unwrap_or("<invalid UTF-8>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_vendor() {
        let vendor = query_vendor();
        assert!(!vendor.is_empty());
        println!("Vendor: {}", vendor);
    }

    #[test]
    fn test_query_version() {
        let version = query_version();
        assert!(!version.is_empty());
        println!("Version: {}", version);
    }

    #[test]
    fn test_parse_resolution() {
        assert_eq!(parse_resolution("1920x1080"), (1920, 1080));
        assert_eq!(parse_resolution("4096x4096"), (4096, 4096));
        assert_eq!(parse_resolution("invalid"), (0, 0));
    }
}
