use std::ops::{Add, Not, Sub};
use strum_macros::{Display, EnumIs, EnumIter, FromRepr};
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(FromRepr, EnumIter, EnumIs)]
pub enum Pai {
    #[default]
    Unknown = 0,
    M1 = 0x11,
    M2 = 0x12,
    M3 = 0x13,
    M4 = 0x14,
    M5 = 0x15,
    M0 = 0x95,
    M6 = 0x16,
    M7 = 0x17,
    M8 = 0x18,
    M9 = 0x19,
    P1 = 0x21,
    P2 = 0x22,
    P3 = 0x23,
    P4 = 0x24,
    P5 = 0x25,
    P0 = 0xA5,
    P6 = 0x26,
    P7 = 0x27,
    P8 = 0x28,
    P9 = 0x29,
    S1 = 0x31,
    S2 = 0x32,
    S3 = 0x33,
    S4 = 0x34,
    S5 = 0x35,
    S0 = 0xB5,
    S6 = 0x36,
    S7 = 0x37,
    S8 = 0x38,
    S9 = 0x39,
    Z1 = 0x41,
    Z2 = 0x49,
    Z3 = 0x51,
    Z4 = 0x59,
    Z5 = 0x61,
    Z6 = 0x69,
    Z7 = 0x71,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum Suit {
    Unknown = 0,
    M = 0x10,
    P = 0x20,
    S = 0x30,
    Z = 0x40,
}

impl Suit {
    pub fn from_char(c: char) -> Self {
        match c {
            'm' => Suit::M,
            'p' => Suit::P,
            's' => Suit::S,
            'z' => Suit::Z,
            _ => Suit::Unknown,
        }
    }


    pub fn from_idx(idx: u8) -> Self {
        match idx {
            0 => Suit::M,
            1 => Suit::P,
            2 => Suit::S,
            3 => Suit::Z,
            _ => Suit::Unknown,
        }
    }

    pub fn to_idx(&self) -> u8 {
        match self {
            Suit::M => 0,
            Suit::P => 1,
            Suit::S => 2,
            Suit::Z => 3,
            _ => 4,
        }
    }
}
impl Pai {
    #[inline]
    pub fn get_suit(&self) -> Suit {
        match (*self as u8) & 0x70 {
            0x10 => Suit::M,
            0x20 => Suit::P,
            0x30 => Suit::S,
            0x40..=0x70 => Suit::Z,
            _ => Suit::Unknown,
        }
    }

    #[inline]
    pub fn get_number(&self) -> u8 {
        if self.is_number() {
            (*self as u8) & 0x0f
        } else {
            (*self as u8 - 0x41) / 0x08 + 1
        }
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        (*self as u8) & 0x40 == 0
    }
    #[inline]
    pub fn is_terminal(&self) -> bool {
        (*self as u8) & 0x07 == 0x01
    }
    #[inline]
    pub fn is_aka(&self) -> bool {
        (*self as u8) & 0x80 == 0x80
    }

    pub fn remove_aka(&self) -> Self {
        if self.is_aka() {
            Self::from_repr((*self as u8) - 0x80).unwrap_or(Pai::Unknown)
        } else {
            *self
        }
    }

    #[inline]
    pub fn is_wind(&self) -> bool {
        self.is_number().not() && (*self as u8) & 0x20 == 0
    }

    #[inline]
    pub fn is_dragon(&self) -> bool {
        self.is_number().not() && (*self as u8) & 0x20 == 0x20
    }


    pub fn from_char(c: char, suit: Suit) -> Self {
        let c = c as u8 - '0' as u8;
        Self::from_repr(match c {
            1..=9 => {
                match suit {
                    Suit::Unknown => 0,
                    Suit::Z => Suit::Z as u8 + (c - 1) * 0x08 + 1,
                    _ => suit as u8 + c,
                }
            }
            0 => {
                match suit {
                    Suit::Unknown | Suit::Z => 0,
                    _ => suit as u8 + 0x05 + 0x80
                }
            }
            _ => 0,
        }).unwrap_or(Pai::Unknown)
    }

    pub fn get_dora_next(&self) -> Self {
        match self.get_suit() {
            Suit::Unknown => { Pai::Unknown }
            Suit::M | Suit::P | Suit::S => {
                if self.get_number() == 9 {
                    Self::from_repr((*self as u8) - 8).unwrap_or(Pai::Unknown)
                } else {
                    Self::from_repr((*self as u8) + 1).unwrap_or(Pai::Unknown)
                }
            }
            Suit::Z => {
                match self.get_number() {
                    1 => Pai::Z2,
                    2 => Pai::Z3,
                    3 => Pai::Z4,
                    4 => Pai::Z1,
                    5 => Pai::Z6,
                    6 => Pai::Z7,
                    7 => Pai::Z5,
                    _ => Pai::Unknown,
                }
            }
        }
    }

    pub fn partial_eq(&self, other: &Pai) -> bool {
        if self.is_unknown() || other.is_unknown() {
            return false;
        }
        self.eq(other) || self.get_suit().eq(&other.get_suit()) && self.get_number().eq(&other.get_number())
    }
}

impl Add<u8> for Pai {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::from_repr(self as u8 + rhs).unwrap_or(Pai::Unknown)
    }
}

impl Sub<u8> for Pai {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self::from_repr(self as u8 - rhs).unwrap_or(Pai::Unknown)
    }
}

impl Sub<Pai> for Pai {
    type Output = u8;

    fn sub(self, rhs: Self) -> Self::Output {
        (self as u8).abs_diff(rhs as u8)
    }
}