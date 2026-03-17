//! RGA buffer management

use std::ffi::c_void;
use std::ptr::null_mut;

use crate::{
    error::{RgaError, RgaResult},
    ffi::*,
    format::PixelFormat,
};

/// Handle to an imported RGA buffer
#[derive(Debug)]
pub struct RgaBuffer {
    handle: rga_buffer_handle_t,
    width: i32,
    height: i32,
    format: PixelFormat,
    wstride: i32,
    hstride: i32,
    // Keep track of whether this buffer owns the handle
    owns_handle: bool,
}

impl RgaBuffer {
    /// Import buffer from file descriptor
    ///
    /// # Safety
    /// The file descriptor must be valid and remain open for the lifetime of this buffer.
    pub unsafe fn from_fd_unchecked(
        fd: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> RgaResult<Self> {
        let param = im_handle_param_t {
            width: width as u32,
            height: height as u32,
            format: format as u32,
        };
        let handle = importbuffer_fd(fd, &param as *const _ as *mut _);
        if handle == 0 {
            return Err(RgaError::InvalidParam);
        }

        Ok(Self {
            handle,
            width,
            height,
            format,
            wstride: width,
            hstride: height,
            owns_handle: true,
        })
    }

    /// Import buffer from file descriptor with specific stride
    ///
    /// # Safety
    /// The file descriptor must be valid and remain open for the lifetime of this buffer.
    pub unsafe fn from_fd_with_stride_unchecked(
        fd: i32,
        width: i32,
        height: i32,
        wstride: i32,
        hstride: i32,
        format: PixelFormat,
    ) -> RgaResult<Self> {
        let param = im_handle_param_t {
            width: width as u32,
            height: height as u32,
            format: format as u32,
        };
        let handle = importbuffer_fd(fd, &param as *const _ as *mut _);
        if handle == 0 {
            return Err(RgaError::InvalidParam);
        }

        Ok(Self {
            handle,
            width,
            height,
            format,
            wstride,
            hstride,
            owns_handle: true,
        })
    }

    /// Import buffer from virtual address
    ///
    /// # Safety
    /// The virtual address must be valid and remain allocated for the lifetime of this buffer.
    pub unsafe fn from_virtual_addr_unchecked(
        addr: *mut c_void,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> RgaResult<Self> {
        let param = im_handle_param_t {
            width: width as u32,
            height: height as u32,
            format: format as u32,
        };
        let handle = importbuffer_virtualaddr(addr, &param as *const _ as *mut _);
        if handle == 0 {
            return Err(RgaError::InvalidParam);
        }

        Ok(Self {
            handle,
            width,
            height,
            format,
            wstride: width,
            hstride: height,
            owns_handle: true,
        })
    }

    /// Import buffer from physical address
    ///
    /// # Safety
    /// The physical address must be valid.
    pub unsafe fn from_physical_addr_unchecked(
        addr: u64,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> RgaResult<Self> {
        let param = im_handle_param_t {
            width: width as u32,
            height: height as u32,
            format: format as u32,
        };
        let handle = importbuffer_physicaladdr(addr, &param as *const _ as *mut _);
        if handle == 0 {
            return Err(RgaError::InvalidParam);
        }

        Ok(Self {
            handle,
            width,
            height,
            format,
            wstride: width,
            hstride: height,
            owns_handle: true,
        })
    }

    /// Create buffer from existing handle (doesn't take ownership)
    pub fn from_handle(
        handle: rga_buffer_handle_t,
        width: i32,
        height: i32,
        format: PixelFormat,
    ) -> Self {
        Self {
            handle,
            width,
            height,
            format,
            wstride: width,
            hstride: height,
            owns_handle: false,
        }
    }

    /// Create buffer from existing handle with stride
    pub fn from_handle_with_stride(
        handle: rga_buffer_handle_t,
        width: i32,
        height: i32,
        wstride: i32,
        hstride: i32,
        format: PixelFormat,
    ) -> Self {
        Self {
            handle,
            width,
            height,
            format,
            wstride,
            hstride,
            owns_handle: false,
        }
    }

    /// Get the buffer handle
    pub fn handle(&self) -> rga_buffer_handle_t {
        self.handle
    }

    /// Get buffer width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get buffer height
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Get buffer format
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Get horizontal stride
    pub fn wstride(&self) -> i32 {
        self.wstride
    }

    /// Get vertical stride
    pub fn hstride(&self) -> i32 {
        self.hstride
    }

    /// Set horizontal stride
    pub fn set_wstride(&mut self, wstride: i32) {
        self.wstride = wstride;
    }

    /// Set vertical stride
    pub fn set_hstride(&mut self, hstride: i32) {
        self.hstride = hstride;
    }

    /// Wrap this buffer into an rga_buffer_t for FFI calls
    pub fn wrap(&self) -> rga_buffer_t {
        rga_buffer_t {
            vir_addr: null_mut(),
            phy_addr: null_mut(),
            fd: -1,
            width: self.width,
            height: self.height,
            wstride: self.wstride,
            hstride: self.hstride,
            format: self.format as i32,
            color_space_mode: 0, // IM_COLOR_SPACE_DEFAULT
            __bindgen_anon_1: unsafe { std::mem::zeroed() },
            rd_mode: 0,
            color: 0,
            colorkey_range: im_colorkey_range { max: 0, min: 0 },
            nn: im_nn_t {
                scale_r: 0,
                scale_g: 0,
                scale_b: 0,
                offset_r: 0,
                offset_g: 0,
                offset_b: 0,
            },
            rop_code: 0,
            handle: self.handle,
        }
    }

    /// Wrap with color space mode
    pub fn wrap_with_color_space(
        &self,
        color_space: crate::ops::color::ColorSpaceMode,
    ) -> rga_buffer_t {
        let mut buf = self.wrap();
        buf.color_space_mode = color_space as i32;
        buf
    }

    /// Calculate buffer size in bytes
    pub fn size_bytes(&self) -> usize {
        (self.wstride * self.hstride * self.format.bytes_per_pixel()) as usize
    }

    /// Take ownership of the handle (prevents release on drop)
    pub fn take_handle(&mut self) -> rga_buffer_handle_t {
        self.owns_handle = false;
        self.handle
    }

    /// Release the buffer handle explicitly
    pub fn release(&mut self) -> RgaResult<()> {
        if self.owns_handle && self.handle != 0 {
            let status = unsafe { releasebuffer_handle(self.handle) };
            self.handle = 0;
            self.owns_handle = false;
            use crate::ffi::*;
            if status == IM_STATUS_IM_STATUS_SUCCESS {
                Ok(())
            } else {
                Err(RgaError::from_status(status))
            }
        } else {
            Ok(())
        }
    }
}

impl Clone for RgaBuffer {
    fn clone(&self) -> Self {
        // Cloned buffer doesn't own the handle
        Self {
            handle: self.handle,
            width: self.width,
            height: self.height,
            format: self.format,
            wstride: self.wstride,
            hstride: self.hstride,
            owns_handle: false,
        }
    }
}

impl Drop for RgaBuffer {
    fn drop(&mut self) {
        if self.owns_handle && self.handle != 0 {
            unsafe {
                let _ = releasebuffer_handle(self.handle);
            }
        }
    }
}

/// Builder for creating RGA buffers with custom parameters
#[derive(Debug)]
pub struct RgaBufferBuilder {
    width: i32,
    height: i32,
    format: PixelFormat,
    wstride: Option<i32>,
    hstride: Option<i32>,
    color_space: i32,
    global_alpha: i32,
}

impl RgaBufferBuilder {
    /// Create a new buffer builder
    pub fn new(width: i32, height: i32, format: PixelFormat) -> Self {
        Self {
            width,
            height,
            format,
            wstride: None,
            hstride: None,
            color_space: 0, // IM_COLOR_SPACE_DEFAULT
            global_alpha: 0xff,
        }
    }

    /// Set horizontal stride
    pub fn wstride(mut self, wstride: i32) -> Self {
        self.wstride = Some(wstride);
        self
    }

    /// Set vertical stride
    pub fn hstride(mut self, hstride: i32) -> Self {
        self.hstride = Some(hstride);
        self
    }

    /// Set color space mode
    pub fn color_space(mut self, color_space: i32) -> Self {
        self.color_space = color_space;
        self
    }

    /// Set global alpha value
    pub fn global_alpha(mut self, alpha: i32) -> Self {
        self.global_alpha = alpha;
        self
    }

    /// Build from file descriptor
    ///
    /// # Safety
    /// The file descriptor must be valid.
    pub unsafe fn build_from_fd(self, fd: i32) -> RgaResult<RgaBuffer> {
        let param = im_handle_param_t {
            width: self.width as u32,
            height: self.height as u32,
            format: self.format as u32,
        };
        let handle = importbuffer_fd(fd, &param as *const _ as *mut _);
        if handle == 0 {
            return Err(RgaError::InvalidParam);
        }

        Ok(RgaBuffer {
            handle,
            width: self.width,
            height: self.height,
            format: self.format,
            wstride: self.wstride.unwrap_or(self.width),
            hstride: self.hstride.unwrap_or(self.height),
            owns_handle: true,
        })
    }

    /// Build from virtual address
    ///
    /// # Safety
    /// The virtual address must be valid and remain allocated.
    pub unsafe fn build_from_virtual_addr(self, addr: *mut c_void) -> RgaResult<RgaBuffer> {
        let param = im_handle_param_t {
            width: self.width as u32,
            height: self.height as u32,
            format: self.format as u32,
        };
        let handle = importbuffer_virtualaddr(addr, &param as *const _ as *mut _);
        if handle == 0 {
            return Err(RgaError::InvalidParam);
        }

        Ok(RgaBuffer {
            handle,
            width: self.width,
            height: self.height,
            format: self.format,
            wstride: self.wstride.unwrap_or(self.width),
            hstride: self.hstride.unwrap_or(self.height),
            owns_handle: true,
        })
    }
}
