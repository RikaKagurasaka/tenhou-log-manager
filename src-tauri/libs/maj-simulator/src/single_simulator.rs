use std::cell::{RefCell, RefMut};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use strum::IntoEnumIterator;
use pai::Pai;
use shanten_calculator::ShantenCalculator;
use crate::utils::{PaiMap, PaiSet};

pub struct SingleSimulator {
    shanten_calculator: ShantenCalculator,
    excellent_yuukouhai: RefCell<HashMap<BTreeSet<Pai>, PaiSet>>,
    good_yuukouhai: RefCell<HashMap<BTreeSet<Pai>, PaiSet>>,
}

impl SingleSimulator {
    const SHANTEN_CALCULATOR_PATH: &'static str = "shanten_calculator.bin";
    pub fn build() -> Self {
        let shanten_calculator = ShantenCalculator::try_load(Self::SHANTEN_CALCULATOR_PATH).unwrap_or({
            let shanten_calculator = ShantenCalculator::build();
            shanten_calculator.save(Self::SHANTEN_CALCULATOR_PATH);
            shanten_calculator
        });
        SingleSimulator {
            shanten_calculator,
            excellent_yuukouhai: Default::default(),
            good_yuukouhai: Default::default(),
        }
    }
}

