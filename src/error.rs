//! Error types for RGA operations

use thiserror::Error;

/// Result type alias for RGA operations
pub type RgaResult<T> = Result<T, RgaError>;

/// Error types for RGA operations
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum RgaError {
    /// Operation not supported
    #[error("operation not supported")]
    NotSupported,

    /// Out of memory
    #[error("out of memory")]
    OutOfMemory,

    /// Invalid parameter
    #[error("invalid parameter")]
    InvalidParam,

    /// Illegal parameter
    #[error("illegal parameter")]
    IllegalParam,

    /// Version mismatch
    #[error("version error")]
    VersionError,

    /// No session available
    #[error("no session")]
    NoSession,

    /// General failure
    #[error("operation failed")]
    Failed,

    /// Unknown error with status code
    #[error("unknown error: {0}")]
    Unknown(i32),
}

impl RgaError {
    /// Create error from IM_STATUS code
    pub fn from_status(status: crate::IM_STATUS) -> Self {
        use crate::ffi::*;
        match status {
            IM_STATUS_IM_STATUS_NOT_SUPPORTED => Self::NotSupported,
            IM_STATUS_IM_STATUS_OUT_OF_MEMORY => Self::OutOfMemory,
            IM_STATUS_IM_STATUS_INVALID_PARAM => Self::InvalidParam,
            IM_STATUS_IM_STATUS_ILLEGAL_PARAM => Self::IllegalParam,
            IM_STATUS_IM_STATUS_ERROR_VERSION => Self::VersionError,
            IM_STATUS_IM_STATUS_NO_SESSION => Self::NoSession,
            IM_STATUS_IM_STATUS_FAILED => Self::Failed,
            _ => Self::Unknown(status),
        }
    }

    /// Get the underlying status code
    pub fn to_status(&self) -> crate::IM_STATUS {
        use crate::ffi::*;
        match self {
            Self::NotSupported => IM_STATUS_IM_STATUS_NOT_SUPPORTED,
            Self::OutOfMemory => IM_STATUS_IM_STATUS_OUT_OF_MEMORY,
            Self::InvalidParam => IM_STATUS_IM_STATUS_INVALID_PARAM,
            Self::IllegalParam => IM_STATUS_IM_STATUS_ILLEGAL_PARAM,
            Self::VersionError => IM_STATUS_IM_STATUS_ERROR_VERSION,
            Self::NoSession => IM_STATUS_IM_STATUS_NO_SESSION,
            Self::Failed => IM_STATUS_IM_STATUS_FAILED,
            Self::Unknown(code) => *code,
        }
    }
}
