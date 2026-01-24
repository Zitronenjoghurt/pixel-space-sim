use std::ops::Deref;

#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct RGBA([u8; 4]);

impl RGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub const fn normal_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        )
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn a(&self) -> u8 {
        self.0[3]
    }

    pub const fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    pub const fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    pub const fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }

    pub const fn yellow() -> Self {
        Self::new(255, 255, 0, 255)
    }

    pub const fn magenta() -> Self {
        Self::new(255, 0, 255, 255)
    }

    pub const fn cyan() -> Self {
        Self::new(0, 255, 255, 255)
    }

    pub const fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub const fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }
}

impl Deref for RGBA {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for RGBA {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<u32> for RGBA {
    fn from(value: u32) -> Self {
        Self(value.to_ne_bytes())
    }
}

impl From<RGBA> for u32 {
    fn from(value: RGBA) -> Self {
        u32::from_ne_bytes(value.0)
    }
}
