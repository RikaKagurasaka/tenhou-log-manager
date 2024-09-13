use bincode::{Decode, Encode};
use std::collections::HashMap;

use pai::monochromatic::MonoChromatic;
use pai::pai::Pai;
use pai::vec_pai::IVecPai;

use crate::ut_map::UtMap;

#[derive(Encode, Decode, Clone)]
pub struct ShantenCalculator {
    suhai: UtMap,
    jihai: UtMap,
}
pub mod error {
    use std::fmt::{Debug, Display, Formatter};

    pub enum ShantenCalculatorLoadError {
        IoError(std::io::Error),
        BincodeError(bincode::error::DecodeError),
    }

    impl From<std::io::Error> for ShantenCalculatorLoadError {
        fn from(e: std::io::Error) -> Self {
            ShantenCalculatorLoadError::IoError(e)
        }
    }

    impl From<bincode::error::DecodeError> for ShantenCalculatorLoadError {
        fn from(e: bincode::error::DecodeError) -> Self {
            ShantenCalculatorLoadError::BincodeError(e)
        }
    }

    impl Debug for ShantenCalculatorLoadError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ShantenCalculatorLoadError::IoError(e) => write!(f, "IoError: {}", e),
                ShantenCalculatorLoadError::BincodeError(e) => write!(f, "BincodeError: {}", e),
            }
        }
    }

    impl Display for ShantenCalculatorLoadError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ShantenCalculatorLoadError::IoError(e) => write!(f, "IoError: {}", e),
                ShantenCalculatorLoadError::BincodeError(e) => write!(f, "BincodeError: {}", e),
            }
        }
    }

    impl std::error::Error for ShantenCalculatorLoadError {}
}

impl ShantenCalculator {
    pub fn build() -> Self {
        let mut suhai = UtMap::default();
        let mut jihai = UtMap::default();
        suhai.fill_agari_forms(false);
        jihai.fill_agari_forms(true);
        suhai.calc_all();
        jihai.calc_all();
        ShantenCalculator { suhai, jihai }
    }

    pub fn save(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        bincode::encode_into_std_write(&self, &mut file, bincode::config::standard()).unwrap();
    }

    pub fn try_load(path: &str) -> Result<Self, error::ShantenCalculatorLoadError> {
        let mut file = std::fs::File::open(path)?;
        Ok(bincode::decode_from_std_read(
            &mut file,
            bincode::config::standard(),
        )?)
    }

    pub fn calc_distance<'a,I: IntoIterator<Item = &'a Pai>>(&self, pais: &I) -> u8 {
        let mut rs = u32::MAX;
        let mentsu_cnt =
            ((pais.into_iter().map(|x| (x.size()) as f32).sum::<f32>() - 2.0) / 3.0).ceil() as u8;
        let comb = sum_combinations(mentsu_cnt);
        for m in comb.iter() {
            for toitsu_idx in 0..4 {
                let mut val = 0u32;
                for suit in 0..4 {
                    let ut = if suit == 3 { &self.jihai } else { &self.suhai };
                    val += ut.get(&pais[suit], m[suit], toitsu_idx == suit) as u32;
                }
                rs = rs.min(val);
            }
        }
        rs as u8
    }

    pub fn calc_shanten(&self, pais: &(impl IntoIterator<Item = Pai> + ExactSizeIterator)) -> i8 {
        let pai_len_mod3 = (pais.len() % 3) as i8;
        let distance = self.calc_distance(pais);
        let val = (distance as f32) / 2.;
        if pai_len_mod3 == 2 {
            val.floor() as i8 - 1
        } else {
            val.ceil() as i8 - 1
        }
    }

    pub fn calc_distance_chitoi(&self, pais: &impl IntoIterator<Item = Pai>) -> u8 {
        let pai_cnt_map = pais.to_pai_map();
        let mut cnt = [0; 5];
        for (&_pai, &c) in pai_cnt_map.iter() {
            cnt[c as usize] += 1;
        }
        let tot_pai_types = pai_cnt_map.key_len() as u8;
        let to_discard_over_type = if tot_pai_types > 7 {
            tot_pai_types - 7
        } else {
            0
        };
        let to_discard_over_cnt = cnt[4] * 2 + cnt[3];
        let to_draw_type = if tot_pai_types < 7 {
            (7 - tot_pai_types) * 2
        } else {
            0
        };
        let to_draw_cnt = cnt[1] - to_discard_over_type;
        to_discard_over_type + to_discard_over_cnt + to_draw_type + to_draw_cnt
    }

    pub fn calc_shanten_chitoi(&self, pais: &impl IntoIterator<Item = Pai>) -> i8 {
        let pais_len = pais.len();
        let distance = self.calc_distance_chitoi(pais);
        let val = (distance as f32) / 2.;
        if pais_len == 14 {
            val.floor() as i8 - 1
        } else {
            val.ceil() as i8 - 1
        }
    }

    pub fn calc_distance_gokushi(&self, pais: &impl IntoIterator<Item = Pai>) -> u8 {
        let old_len = pais.len();
        let pais = pais
            .iter()
            .filter(|&x| x.is_terminal())
            .cloned()
            .collect::<Vec<_>>();
        let to_discard_non_yao9 = old_len - pais.len();
        let pai_cnt_map = pais.to_pai_map();
        let types = pai_cnt_map.key_len();
        let has_toitsu = pais.len() > types;
        let to_discard_over_cnt = if has_toitsu {
            (pais.len() - 1) - types
        } else {
            0
        };
        let to_draw_cnt = if !has_toitsu { 1 } else { 0 };
        let to_draw_type = 13 - types;
        (to_discard_non_yao9 + to_discard_over_cnt + to_draw_cnt + to_draw_type) as u8
    }

    pub fn calc_shanten_gokushi(&self, pais: &impl IntoIterator<Item = Pai>) -> i8 {
        let pais_len = pais.len();
        let distance = self.calc_distance_gokushi(pais);
        let val = (distance as f32) / 2.;
        if pais_len == 14 {
            val.floor() as i8 - 1
        } else {
            val.ceil() as i8 - 1
        }
    }

    pub fn calc_distance_all(&self, pais: &impl IntoIterator<Item = Pai>) -> u8 {
        self.calc_distance(pais)
            .min(self.calc_distance_chitoi(pais))
            .min(self.calc_distance_gokushi(pais))
    }

    pub fn calc_shanten_all(&self, pais: &impl IntoIterator<Item = Pai>) -> i8 {
        self.calc_shanten(pais)
            .min(self.calc_shanten_chitoi(pais))
            .min(self.calc_shanten_gokushi(pais))
    }
}
fn sum_combinations(sum: u8) -> Vec<[u8; 4]> {
    let mut combinations = Vec::new();

    for a in 0..=sum {
        for b in 0..=(sum - a) {
            for c in 0..=(sum - a - b) {
                let d = sum - a - b - c;
                combinations.push([a, b, c, d]);
            }
        }
    }

    combinations
}

#[test]
fn build_and_save() {
    let calc = ShantenCalculator::build();
    calc.save("shanten_calculator.bin");
}

#[test]
fn load_and_calc() {
    let calc = ShantenCalculator::try_load("shanten_calculator.bin").unwrap();
    let pais = Vec::<Pai>::from_string("1m");
    println!("{:?}", pais);
    let pais = Vec::<Pai>::from_string("123m456p369s");
    println!("{:?}", pais);
    let pais = Vec::<Pai>::from_string("123m456p3s69s135z77z");
    println!("{:?}", pais);
    assert_eq!(
        calc.calc_shanten(&Vec::<Pai>::from_string("39m3p26s16z6m")),
        4
    );
    assert_eq!(
        calc.calc_shanten(&Vec::<Pai>::from_string("288m29p126s46z2m")),
        3
    );
    assert_eq!(
        calc.calc_shanten(&Vec::<Pai>::from_string("3689m58p2479s135z1s")),
        5
    );
    assert_eq!(
        calc.calc_shanten(&Vec::<Pai>::from_string("129m12579p106s14z8p")),
        3
    );
    assert_eq!(
        calc.calc_shanten(&Vec::<Pai>::from_string("23444456777998m")),
        0
    );
    assert_eq!(
        calc.calc_shanten(&Vec::<Pai>::from_string("23444456677899m")),
        -1
    );
}

#[test]
fn chitoi() {
    let calc = ShantenCalculator::try_load("shanten_calculator.bin").unwrap();
    assert_eq!(
        calc.calc_shanten_chitoi(&Vec::<Pai>::from_string("1133m")),
        4
    );
    assert_eq!(
        calc.calc_shanten_chitoi(&Vec::<Pai>::from_string("133488m669p4488s3z")),
        1
    );
    assert_eq!(
        calc.calc_shanten_chitoi(&Vec::<Pai>::from_string("13388m669p4488s3z")),
        1
    );
    assert_eq!(
        calc.calc_shanten_chitoi(&Vec::<Pai>::from_string("1111m2222p3333s24z")),
        5
    );
}

#[test]
fn gokushi() {
    let calc = ShantenCalculator::try_load("shanten_calculator.bin").unwrap();
    assert_eq!(
        calc.calc_shanten_gokushi(&Vec::<Pai>::from_string("19m19p19s1234567z")),
        0
    );
    assert_eq!(
        calc.calc_shanten_gokushi(&Vec::<Pai>::from_string("1129m89p19s1234567z")),
        1
    );
    assert_eq!(
        calc.calc_shanten_gokushi(&Vec::<Pai>::from_string("123m112233p12399s")),
        8
    );
}
