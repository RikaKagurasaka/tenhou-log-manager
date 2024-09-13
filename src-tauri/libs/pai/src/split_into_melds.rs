use crate::{BTreeCountMap, IVecPai, Pai};
use std::fmt::{Debug, Display, Formatter};
use strum_macros::{EnumIs, EnumTryAs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIs, EnumTryAs)]
pub enum Mentsu {
    Shuntsu(Pai),
    Kotsu(Pai),
}

impl Display for Mentsu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mentsu::Shuntsu(pai) => write!(f, "{}", pai),
            Mentsu::Kotsu(pai) => write!(f, "{}", pai),
        }
    }
}

impl Mentsu {
    pub fn get_pai(&self) -> Pai {
        match self {
            Mentsu::Shuntsu(p) => *p,
            Mentsu::Kotsu(p) => *p,
        }
    }
    pub fn include(&self, pai: Pai) -> bool {
        if pai.is_unknown() || self.get_pai().is_unknown() {
            return false;
        }
        match *self {
            Mentsu::Shuntsu(p) => p.eq(&pai) || (p + 1).eq(&pai) || (p + 2).eq(&pai),
            Mentsu::Kotsu(p) => p.eq(&pai),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct NormalMeld {
    pub mentsu: Vec<Mentsu>,
    pub head: Pai,
}

pub fn is_chiitoi(pais: &Vec<Pai>) -> bool {
    let pai_map = pais.to_pai_map();
    if pais.len() == 14 && pai_map.iter().all(|(_, &cnt)| cnt.eq(&2)) {
        return true;
    }
    false
}

pub fn is_kokushi(pais: &Vec<Pai>) -> bool {
    let pai_map = pais.to_pai_map();
    if pais.len() == 14
        && pai_map.iter().all(|(pai, _)| pai.is_terminal())
        && pai_map.key_len() == 13
    {
        return true;
    }
    false
}

pub fn split_into_melds(pais: Vec<Pai>) -> Vec<NormalMeld> {
    let mut rs = vec![];
    let pai_map = pais.to_pai_map();

    fn dfs(pai_map: &mut BTreeCountMap<Pai>) -> Option<Vec<Vec<Mentsu>>> {
        let first_pai = if let Some(&pai) = pai_map.first_key() {
            pai
        } else {
            return Some(vec![vec![]]);
        };
        let kotsu_available = pai_map.get(&first_pai).ge(&3);
        let shuntsu_available = first_pai.is_number()
            && pai_map.contains_key(&(first_pai + 1))
            && pai_map.contains_key(&(first_pai + 2));
        let mut rs = vec![];
        if kotsu_available {
            pai_map.remove_n(&first_pai, 3);
            if let Some(inner) = dfs(pai_map) {
                for mut i in inner {
                    let mut meld = vec![Mentsu::Kotsu(first_pai)];
                    meld.append(&mut i);
                    rs.push(meld);
                }
            }
            pai_map.insert_n(first_pai, 3);
        }
        if shuntsu_available {
            pai_map.remove_one(&first_pai);
            pai_map.remove_one(&(first_pai + 1));
            pai_map.remove_one(&(first_pai + 2));
            if let Some(inner) = dfs(pai_map) {
                for mut i in inner {
                    let mut meld = vec![Mentsu::Shuntsu(first_pai)];
                    meld.append(&mut i);
                    rs.push(meld);
                }
            }
            pai_map.insert_one(first_pai);
            pai_map.insert_one(first_pai + 1);
            pai_map.insert_one(first_pai + 2);
        }
        if rs.is_empty() {
            return None;
        }
        Some(rs)
    }

    for (&pai, &cnt) in pai_map.iter() {
        if cnt.ge(&2) {
            let mut cloned = pai_map.clone();
            cloned.remove_n(&pai, 2);
            if let Some(inner) = dfs(&mut cloned) {
                for i in inner {
                    rs.push(NormalMeld {
                        mentsu: i,
                        head: pai,
                    });
                }
            }
        }
    }

    rs
}

#[test]
fn split_into_melds_test() {
    let tehai = Vec::<Pai>::from_string("1234445666789m3m");
    split_into_melds(tehai);
    let tehai = Vec::<Pai>::from_string("11123456789999m");
    split_into_melds(tehai);
    let tehai = Vec::<Pai>::from_string("11122233378999m");
    split_into_melds(tehai);
    let tehai = Vec::<Pai>::from_string("123456m22z");
    split_into_melds(tehai);
    let tehai = Vec::<Pai>::from_string("123m123p123z12355z");
    split_into_melds(tehai);
}
