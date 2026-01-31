use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    SP,
    PC,
}

impl Register {
    /// Parse register from instruction encoding (0-7)
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            0 => Some(Self::R0),
            1 => Some(Self::R1),
            2 => Some(Self::R2),
            3 => Some(Self::R3),
            4 => Some(Self::R4),
            5 => Some(Self::R5),
            6 => Some(Self::SP),
            7 => Some(Self::PC),
            _ => None,
        }
    }

    /// Get the instruction encoding for this register (0-7)
    pub fn to_code(self) -> u16 {
        match self {
            Self::R0 => 0,
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::SP => 6,
            Self::PC => 7,
        }
    }

    /// Get register name as used in assembly
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R0 => "R0",
            Self::R1 => "R1",
            Self::R2 => "R2",
            Self::R3 => "R3",
            Self::R4 => "R4",
            Self::R5 => "R5",
            Self::SP => "SP",
            Self::PC => "PC",
        }
    }

    /// Parse register from assembly text (case-insensitive)
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "R0" | "%0" => Some(Self::R0),
            "R1" | "%1" => Some(Self::R1),
            "R2" | "%2" => Some(Self::R2),
            "R3" | "%3" => Some(Self::R3),
            "R4" | "%4" => Some(Self::R4),
            "R5" | "%5" => Some(Self::R5),
            "SP" | "R6" | "%6" => Some(Self::SP),
            "PC" | "R7" | "%7" => Some(Self::PC),
            _ => None,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<u16> for Register {
    fn from(code: u16) -> Self {
        Self::from_code(code).expect("invalid register code")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_code_conversion() {
        assert_eq!(Register::R0.to_code(), 0);
        assert_eq!(Register::PC.to_code(), 7);
        assert_eq!(Register::from_code(0), Some(Register::R0));
        assert_eq!(Register::from_code(7), Some(Register::PC));
        assert_eq!(Register::from_code(8), None);
    }

    #[test]
    fn test_register_parse() {
        assert_eq!(Register::parse("R0"), Some(Register::R0));
        assert_eq!(Register::parse("r0"), Some(Register::R0));
        assert_eq!(Register::parse("SP"), Some(Register::SP));
        assert_eq!(Register::parse("sp"), Some(Register::SP));
        assert_eq!(Register::parse("PC"), Some(Register::PC));
        assert_eq!(Register::parse("%5"), Some(Register::R5));
        assert_eq!(Register::parse("invalid"), None);
    }
}
