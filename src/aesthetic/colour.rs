use serde::Serialize;

pub enum Colour {
    RGB(u8, u8, u8),
    Hex(u32),
}

impl Default for Colour {
    fn default() -> Self {
        Self::RGB(128, 128, 128)
    }
}

impl Colour {
    fn to_hex_string(&self) -> String {
        match self {
            Self::RGB(r, g, b) => format!("#{:2x}{:2x}{:2x}", r, g, b),
            Self::Hex(h) => format!("#{:16x}", h),
        }
    }
}

impl Serialize for Colour {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_hex_string().serialize(serializer)
    }
}
