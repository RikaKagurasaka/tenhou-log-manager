use std::collections::HashSet;
use std::hash::RandomState;
use std::ops::Not;

use bincode::{Decode, Encode};

use pai::monochromatic::MonoChromatic;

pub(crate) const UT_UNCALCULATED: u8 = 255;
pub(crate) const UT_INFINITY: u8 = 254;
pub(crate) const UT_CALCULATING: u8 = 253;

#[derive(Encode, Decode, Clone)]
pub struct UtMap {
    num: Vec<u8>,
}
impl Default for UtMap {
    fn default() -> Self {
        let map = vec![UT_UNCALCULATED; 5usize.pow(9) * 10];
        UtMap { num: map }
    }
}


impl UtMap {
    pub fn get(&self, form: &MonoChromatic, mentzu_cnt: u8, including_toitsu: bool) -> u8 {
        self.num[form.to_compact() as usize * 10 + if including_toitsu { 5 } else { 0 } + mentzu_cnt as usize]
    }

    pub fn get_min(&self, form: &MonoChromatic, mentzu_cnt: Option<u8>, including_toitsu: Option<bool>) -> u8 {
        let mut min = UT_INFINITY;
        for i in if let Some(mentzu_cnt) = mentzu_cnt { mentzu_cnt..=mentzu_cnt } else { 0..=4 } {
            for j in if let Some(including_toitsu) = including_toitsu { if including_toitsu { 1..=1 } else { 0..=0 } } else { 0..=1 } {
                min = min.min(self.get(form, i, j == 1));
            }
        }
        min
    }

    pub fn get_mut(&mut self, form: &MonoChromatic, mentzu_cnt: u8, including_toitsu: bool) -> &mut u8 {
        &mut self.num[form.to_compact() as usize * 10 + if including_toitsu { 5 } else { 0 } + mentzu_cnt as usize]
    }

    fn cartesian_product(this: &HashSet<MonoChromatic>, other: &HashSet<MonoChromatic>) -> HashSet<MonoChromatic> {
        let mut rs = HashSet::new();
        for &a in this {
            for &b in other {
                let _a_repr = a.represent();
                let _b_repr = b.represent();
                if let Some(c) = a.checked_add(b) {
                    rs.insert(c);
                }
            }
        }
        rs
    }

    pub fn fill_agari_forms(&mut self, is_jihai: bool) {
        let all_shuntsu: HashSet<_, RandomState> = if is_jihai { HashSet::new() } else { HashSet::from_iter(MonoChromatic::gen_all_shuntsu()) };
        let all_kotsu = HashSet::from_iter(MonoChromatic::gen_all_kotsu());
        let all_toitsu = HashSet::from_iter(MonoChromatic::gen_all_toitsu());
        let all_mentzus = all_shuntsu.union(&all_kotsu).cloned().collect::<HashSet<MonoChromatic>>();
        let size3_forms = all_mentzus.clone();
        let size6_forms = Self::cartesian_product(&size3_forms, &size3_forms);
        let size9_forms = Self::cartesian_product(&size6_forms, &size3_forms);
        let size12_forms = Self::cartesian_product(&size9_forms, &size3_forms);

        let size2_forms = all_toitsu.clone();
        let size5_forms = Self::cartesian_product(&size3_forms, &size2_forms);
        let size8_forms = Self::cartesian_product(&size6_forms, &size2_forms);
        let size11_forms = Self::cartesian_product(&size9_forms, &size2_forms);
        let size14_forms = Self::cartesian_product(&size12_forms, &size2_forms);

        let all_forms = [
            size3_forms, size6_forms, size9_forms, size12_forms,
            size2_forms, size5_forms, size8_forms, size11_forms, size14_forms
        ].iter().cloned().flatten().collect::<HashSet<MonoChromatic>>();

        all_forms.iter().for_each(|f| {
            let size = f.size();
            let mentzu_cnt = (size + 1) / 3;
            let including_toitsu = size % 3 == 2;
            *self.get_mut(f, mentzu_cnt, including_toitsu) = 0;
            for i in 0..=4 {
                for j in 0..=1 {
                    let new_size = i * 3 + 2 * j;
                    *self.get_mut(f, i, j == 1) = new_size.abs_diff(size);
                }
            }
        });


        *self.get_mut(&MonoChromatic::from(0), 0, false) = 0;
    }

    pub fn calc_all(&mut self) {
        let mut modified;
        let mut results = vec![];
        loop {
            modified = false;
            for f in 0..5u32.pow(9) {
                let form = MonoChromatic::from_compact(f);
                if form.is_valid().not() {
                    continue;
                }
                for mentzu_cnt in 0..=4 {
                    for including_toitsu in 0..=1 {
                        results.clear();
                        let cur = self.get(&form, mentzu_cnt, including_toitsu == 1);
                        if cur < UT_INFINITY {
                            if results.is_empty() {
                                form.get_all_variants(&mut results);
                            }
                            for variant in results.iter() {
                                let ref_ = self.get_mut(variant, mentzu_cnt, including_toitsu == 1);
                                if *ref_ > cur + 1 {
                                    *ref_ = cur + 1;
                                    modified = true;
                                }
                            }
                        }
                    }
                }
            }
            if !modified {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ut_map() {
        let mut ut_map = UtMap::default();
        ut_map.fill_agari_forms(false);
    }

    #[test]
    fn test_ut_map_calc() {
        let mut ut_map = UtMap::default();
        ut_map.fill_agari_forms(false);
        let start = chrono::Utc::now();
        ut_map.calc_all();
        let end = chrono::Utc::now();
        println!("Time: {:?}", end - start);
    }
}


