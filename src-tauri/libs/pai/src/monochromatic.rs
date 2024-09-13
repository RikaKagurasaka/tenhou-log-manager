use std::ops::{Add, Sub};
use crate::Pai;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MonoChromatic {
    bits: u32,
}

impl From<u32> for MonoChromatic {
    fn from(bits: u32) -> Self {
        MonoChromatic { bits }
    }
}

impl Sub for MonoChromatic {
    type Output = MonoChromatic;

    fn sub(self, rhs: MonoChromatic) -> Self::Output {
        MonoChromatic { bits: self.bits - rhs.bits }
    }
}

impl Add for MonoChromatic {
    type Output = MonoChromatic;

    fn add(self, rhs: MonoChromatic) -> Self::Output {
        MonoChromatic { bits: self.bits + rhs.bits }
    }
}


impl MonoChromatic {
    #[inline]
    pub fn get_pai_num(&self, idx: u8) -> u8 {
        ((self.bits & 0b111u32 << (idx - 1) * 3) >> (idx - 1) * 3) as u8
    }

    #[inline]
    pub fn set_pai_num(&mut self, idx: u8, num: u8) {
        let bit = 0b111u32 << (idx - 1) * 3;
        self.bits = (self.bits & !bit) | ((num as u32) << (idx - 1) * 3);
    }

    #[inline]
    pub fn inc_pai_num(&mut self, idx: u8) {
        self.bits += 0b1u32 << (idx - 1) * 3;
    }

    #[inline]
    pub fn dec_pai_num(&mut self, idx: u8) {
        self.bits -= 0b1u32 << (idx - 1) * 3;
    }

    pub fn checked_add(&self, other: MonoChromatic) -> Option<MonoChromatic> {
        let mut rs = MonoChromatic::from(0);
        for i in 1..=9 {
            let num = self.get_pai_num(i) + other.get_pai_num(i);
            if num > 4 {
                return None;
            }
            rs.set_pai_num(i, num);
        }
        Some(rs)
    }

    #[inline]
    pub fn size(&self) -> u8 {
        (1..=9).map(|i| self.get_pai_num(i)).sum()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bits & ((1 << (9 * 3)) - 1) == 0
    }

    pub fn to_compact(&self) -> u32 {
        (1..=9).rev().map(|i| self.get_pai_num(i) as u32).reduce(|acc, x| acc * 5 + x).unwrap()
    }

    pub fn from_compact(mut compact: u32) -> MonoChromatic {
        let mut rs = MonoChromatic::from(0);
        for i in 1..=9 {
            rs.set_pai_num(i, (compact % 5) as u8);
            compact /= 5;
        }
        rs
    }

    pub fn find_all_shuntsu(&self) -> [u32; 4] {
        let mut shuntsu_list = [0u32; 4];
        let mut size = 0usize;
        for i in 1..=7 {
            if self.get_pai_num(i) > 0 && self.get_pai_num(i + 1) > 0 && self.get_pai_num(i + 2) > 0 {
                let mut shuntsu = MonoChromatic::from(0);
                shuntsu.inc_pai_num(i);
                shuntsu.inc_pai_num(i + 1);
                shuntsu.inc_pai_num(i + 2);
                shuntsu_list[size] = shuntsu.bits;
                size += 1;
            }
        }
        shuntsu_list
    }

    pub fn find_all_kotsu(&self) -> [u32; 4] {
        let mut kotsu_list = [0u32; 4];
        let mut size = 0usize;
        for i in 1..=9 {
            if self.get_pai_num(i) >= 3 {
                let mut kotsu = MonoChromatic::from(0);
                kotsu.set_pai_num(i, 3);
                kotsu_list[size] = kotsu.bits;
                size += 1;
            }
        }
        kotsu_list
    }

    pub fn find_all_toitsu(&self) -> [u32; 7] {
        let mut toitsu_list = [0u32; 7];
        let mut size = 0usize;
        for i in 1..=9 {
            if self.get_pai_num(i) >= 2 {
                let mut toitsu = MonoChromatic::from(0);
                toitsu.set_pai_num(i, 2);
                toitsu_list[size] = toitsu.bits;
                size += 1;
            }
        }
        toitsu_list
    }

    pub fn gen_all_shuntsu() -> Vec<MonoChromatic> {
        let mut shuntsu_list = Vec::new();
        for i in 1..=7 {
            let mut shuntsu = MonoChromatic::from(0);
            shuntsu.inc_pai_num(i);
            shuntsu.inc_pai_num(i + 1);
            shuntsu.inc_pai_num(i + 2);
            shuntsu_list.push(shuntsu);
        }
        shuntsu_list
    }

    pub fn gen_all_kotsu() -> Vec<MonoChromatic> {
        let mut kotsu_list = Vec::new();
        for i in 1..=9 {
            let mut kotsu = MonoChromatic::from(0);
            kotsu.set_pai_num(i, 3);
            kotsu_list.push(kotsu);
        }
        kotsu_list
    }

    pub fn gen_all_toitsu() -> Vec<MonoChromatic> {
        let mut toitsu_list = Vec::new();
        for i in 1..=9 {
            let mut toitsu = MonoChromatic::from(0);
            toitsu.set_pai_num(i, 2);
            toitsu_list.push(toitsu);
        }
        toitsu_list
    }

    pub fn represent(&self) -> String {
        let mut rs = String::new();
        for i in 1..=9 {
            for _ in 0..self.get_pai_num(i) {
                rs.push_str(&i.to_string());
            }
        }
        rs
    }

    pub fn is_valid(&self) -> bool {
        let rs = self.size() <= 14 && (1..=9).all(|i| self.get_pai_num(i) <= 4);
        rs
    }

    pub fn get_all_variants(&self, result: &mut Vec<MonoChromatic>) {
        result.clear();
        for i in 1..=9 {
            if self.get_pai_num(i) > 0 {
                result.push({
                    let mut v = self.clone();
                    v.dec_pai_num(i);
                    v
                })
            }
            if self.get_pai_num(i) < 4 && self.size() < 14 {
                result.push({
                    let mut v = self.clone();
                    v.inc_pai_num(i);
                    v
                })
            }
        }
    }

    pub fn from_string<'a>(s: impl Into<&'a str>) -> MonoChromatic {
        let mut rs = MonoChromatic::from(0);
        for c in s.into().chars() {
            let i = c.to_digit(10).unwrap() as u8;
            rs.inc_pai_num(i);
        }
        rs
    }

    pub fn from_iter<'a>(pais: impl IntoIterator<Item=&'a Pai>) -> [MonoChromatic; 4] {
        let mut rs = [MonoChromatic::from(0); 4];
        for &pai in pais.into_iter() {
            let suit = pai.get_suit();
            let num = pai.get_number();
            rs[(suit as u8 / 0x10 - 1) as usize].inc_pai_num(num);
        }
        rs
    }
}
impl<'a> FromIterator<&'a Pai> for [MonoChromatic; 4] {
    fn from_iter<T: IntoIterator<Item=&'a Pai>>(iter: T) -> Self {
        let mut rs = [MonoChromatic::from(0); 4];
        for &pai in iter.into_iter() {
            let suit = pai.get_suit();
            let num = pai.get_number();
            rs[(suit as u8 / 0x10 - 1) as usize].inc_pai_num(num);
        }
        rs
    }
} 
