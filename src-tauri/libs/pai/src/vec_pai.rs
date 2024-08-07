use std::collections::HashMap;
use std::fmt::Display;
use crate::{BTreeCountMap, MonoChromatic};
use crate::pai::{Pai, Suit};

pub trait IVecPai {
    fn from_string(str: impl Into<String>) -> Self;
    fn represent(&self) -> String;
    fn to_pai_map(&self) -> BTreeCountMap<Pai>;
}

impl IVecPai for Vec<Pai> {
    fn from_string(str: impl Into<String>) -> Self {
        let mut buf = vec![];
        let mut rs = vec![];
        for c in str.into().chars() {
            match c.to_ascii_lowercase() {
                '0'..='9' => buf.push(c),
                'm' | 'p' | 's' | 'z' => {
                    let suit = Suit::from_char(c);
                    while let Some(num) = buf.pop() {
                        rs.push(Pai::from_char(num, suit));
                    }
                }
                _ => {}
            }
        }
        rs
    }

    fn represent(&self) -> String {
        let suits = MonoChromatic::from_pai_vec(self);
        let mut s = String::new();
        for i in 0..4 {
            s += &suits[i].represent();
            if s.chars().last().unwrap_or('a').is_digit(10) {
                s += Suit::from_idx(i as u8).to_string().to_ascii_lowercase().as_str();
            }
        }
        s
    }

    fn to_pai_map(&self) -> BTreeCountMap<Pai> {
        let mut map = BTreeCountMap::from_iter(self.iter().cloned());
        map.remove_all(&Pai::Unknown);
        map
    }
}

impl Display for Pai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rs = String::new();
        rs += &self.get_number().to_string();
        rs += &match self.get_suit() {
            Suit::M => "m",
            Suit::P => "p",
            Suit::S => "s",
            Suit::Z => "z",
            _ => "",
        };
        write!(f, "{}", rs)
    }
}


