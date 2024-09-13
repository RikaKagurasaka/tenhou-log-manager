use std::collections::{BTreeMap, BTreeSet, HashSet};
use lazy_static::lazy_static;
use pai::Pai;
use shanten_calculator::ShantenCalculator;

pub type PaiMap = BTreeMap<Pai, u8>;
pub type PaiSet = BTreeSet<Pai>;

/// If return true, the target MAY a yuukouhai,
/// otherwise, the target MUST NOT be a yuukouhai.
pub fn may_be_yuukouhai(hand: &PaiMap, target: Pai) -> bool {
    hand.iter().any(|(&pai, &num)| num > 0 && (target as u8).abs_diff(pai as u8) <= 2)
}

pub enum YuukouhaiType {
    None,
    /// シャンテンを進める牌
    Excellent,
    /// 受け入れが広くなるような牌
    Good,
    /// 引いたら手の中に残すような牌
    Dual,
}

lazy_static! {
    static ref SHANTEN_CALCULATOR: ShantenCalculator = ShantenCalculator::build();
}

