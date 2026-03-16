//! Batch job processing for RGA

use crate::{
    error::{RgaError, RgaResult},
    ffi::*,
};

/// Context-based RGA operations (alternative to global state)
pub struct Context {
    id: im_ctx_id_t,
}

impl Context {
    /// Create a new RGA context
    pub fn new() -> RgaResult<Self> {
        let id = unsafe { imbegin(0) };
        if id == 0 {
            return Err(RgaError::NoSession);
        }
        Ok(Self { id })
    }

    /// Get context ID
    pub fn id(&self) -> im_ctx_id_t {
        self.id
    }

    /// Cancel the context
    pub fn cancel(&self) -> RgaResult<()> {
        let status = unsafe { imcancel(self.id) };

        if status == IM_STATUS_IM_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(RgaError::from_status(status))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let _ = self.cancel();
    }
}
