use std::ops::Deref;

#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct RGBA([u8; 4]);

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
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

    pub fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }

    pub fn yellow() -> Self {
        Self::new(255, 255, 0, 255)
    }

    pub fn magenta() -> Self {
        Self::new(255, 0, 255, 255)
    }

    pub fn cyan() -> Self {
        Self::new(0, 255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub fn white() -> Self {
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
