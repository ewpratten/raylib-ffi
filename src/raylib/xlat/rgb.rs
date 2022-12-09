//! Conversions between the `rgb` crate and the `Color` type.

impl From<crate::raylib::ffi::Color> for rgb::RGBA8 {
    fn from(color: crate::raylib::ffi::Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl Into<crate::raylib::ffi::Color> for rgb::RGBA8 {
    fn into(self) -> crate::raylib::ffi::Color {
        crate::raylib::ffi::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl From<crate::raylib::ffi::Color> for rgb::RGB8 {
    fn from(color: crate::raylib::ffi::Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

impl Into<crate::raylib::ffi::Color> for rgb::RGB8 {
    fn into(self) -> crate::raylib::ffi::Color {
        crate::raylib::ffi::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: 255,
        }
    }
}
