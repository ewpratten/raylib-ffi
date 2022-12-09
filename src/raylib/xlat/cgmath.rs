//! Conversions between the `cgmath` crate and raylib's math types.

impl Into<crate::raylib::ffi::Vector2> for cgmath::Vector2<f32> {
    fn into(self) -> crate::raylib::ffi::Vector2 {
        crate::raylib::ffi::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<crate::raylib::ffi::Vector2> for cgmath::Vector2<f32> {
    fn from(vec: crate::raylib::ffi::Vector2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
        }
    }
}

impl Into<crate::raylib::ffi::Vector3> for cgmath::Vector3<f32> {
    fn into(self) -> crate::raylib::ffi::Vector3 {
        crate::raylib::ffi::Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl From<crate::raylib::ffi::Vector3> for cgmath::Vector3<f32> {
    fn from(vec: crate::raylib::ffi::Vector3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl Into<crate::raylib::ffi::Vector4> for cgmath::Vector4<f32> {
    fn into(self) -> crate::raylib::ffi::Vector4 {
        crate::raylib::ffi::Vector4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl From<crate::raylib::ffi::Vector4> for cgmath::Vector4<f32> {
    fn from(vec: crate::raylib::ffi::Vector4) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: vec.w,
        }
    }
}

impl Into<crate::raylib::ffi::Quaternion> for cgmath::Quaternion<f32> {
    fn into(self) -> crate::raylib::ffi::Quaternion {
        crate::raylib::ffi::Quaternion {
            x: self.v.x,
            y: self.v.y,
            z: self.v.z,
            w: self.s,
        }
    }
}

impl From<crate::raylib::ffi::Quaternion> for cgmath::Quaternion<f32> {
    fn from(quat: crate::raylib::ffi::Quaternion) -> Self {
        Self {
            v: cgmath::Vector3::new(quat.x, quat.y, quat.z),
            s: quat.w,
        }
    }
}

impl Into<crate::raylib::ffi::Matrix> for cgmath::Matrix4<f32> {
    fn into(self) -> crate::raylib::ffi::Matrix {
        crate::raylib::ffi::Matrix {
            m0: self.x.x,
            m1: self.x.y,
            m2: self.x.z,
            m3: self.x.w,
            m4: self.y.x,
            m5: self.y.y,
            m6: self.y.z,
            m7: self.y.w,
            m8: self.z.x,
            m9: self.z.y,
            m10: self.z.z,
            m11: self.z.w,
            m12: self.w.x,
            m13: self.w.y,
            m14: self.w.z,
            m15: self.w.w,
        }
    }
}