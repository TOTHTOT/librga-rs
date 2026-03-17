//! Rectangle types for RGA operations

/// Rectangle definition for RGA operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    /// X coordinate of upper-left corner
    pub x: i32,
    /// Y coordinate of upper-left corner
    pub y: i32,
    /// Width of rectangle
    pub width: i32,
    /// Height of rectangle
    pub height: i32,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create a rectangle at origin with given size
    pub fn at_origin(width: i32, height: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// Check if rectangle is empty (zero area)
    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    /// Get rectangle area
    pub fn area(&self) -> i32 {
        if self.is_empty() {
            0
        } else {
            self.width * self.height
        }
    }
}

impl From<Rect> for crate::im_rect {
    fn from(r: Rect) -> Self {
        crate::im_rect {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}

impl From<crate::im_rect> for Rect {
    fn from(r: crate::im_rect) -> Self {
        Self {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}
