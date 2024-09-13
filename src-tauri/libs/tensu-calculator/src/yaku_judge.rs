use std::collections::HashMap;
use std::ops::Not;
use std::sync::{LazyLock, Mutex};

use pai::Yaku::*;
use pai::{
    is_chiitoi, is_kokushi, split_into_melds, EnumIs, Furo, FuroType, IVecPai, Mentsu, NormalMeld, Pai, ReducedFuroType, Yaku,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct YakuJudgeFlags {
    pub is_menzen: bool,
    pub is_riichi: bool,
    pub is_dbl_riichi: bool,
    pub is_tsumo: bool,
    pub is_ippatsu: bool,
    /// chankan or rinshan
    pub is_kan: bool,
    /// haitei or houtei
    pub is_last: bool,
    /// tenhou, chihou, renhou
    pub is_first: bool,
}
#[derive(Debug, Clone, Default)]
pub struct YakuJudgeArgs {
    pub flags: YakuJudgeFlags,
    pub dora_marker: Vec<Pai>,
    pub uradora_marker: Vec<Pai>,
    pub machi: Pai,
    pub bakaze: Pai,
    pub jikaze: Pai,
    pub pai: Vec<Pai>,
    pub furo: Vec<Furo>,
}
#[derive(Debug, Clone, Copy, EnumIs)]
pub enum MentsuDetail {
    Toitsu,
    Shuntsu,
    Anko,
    Minko,
    Ankan,
    Minkan,
}

impl MentsuDetail {
    pub fn is_kotsu(&self) -> bool {
        self.is_anko() || self.is_minko() || self.is_ankan() || self.is_minkan()
    }

    pub fn is_kantsu(&self) -> bool {
        self.is_ankan() || self.is_minkan()
    }

    pub fn is_concealed(&self) -> bool {
        self.is_anko() || self.is_ankan()
    }
}
struct MeldDetail {
    pub mentsu: Vec<(MentsuDetail, Pai)>,
    pub is_bad_shape: bool,
}

impl YakuJudgeArgs {
    fn split_melds_machi_normal(&self, melds: Vec<NormalMeld>) -> Vec<MeldDetail> {
        let machi = self.machi;
        let mut rs = vec![];
        for NormalMeld { mentsu, head } in melds {
            for idx in 0..=mentsu.len() {
                if idx == mentsu.len() {
                    if machi == head {
                        rs.push(MeldDetail {
                            mentsu: mentsu
                                .iter()
                                .cloned()
                                .map(|m| match m {
                                    Mentsu::Shuntsu(p) => (MentsuDetail::Shuntsu, p),
                                    Mentsu::Kotsu(p) => (MentsuDetail::Anko, p),
                                })
                                .chain(std::iter::once((MentsuDetail::Toitsu, head)))
                                .collect(),
                            is_bad_shape: true,
                        });
                    }
                } else {
                    let target = mentsu[idx];
                    if target.include(machi) {
                        rs.push(MeldDetail {
                            mentsu: mentsu
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(i, m)| match m {
                                    Mentsu::Shuntsu(p) => (MentsuDetail::Shuntsu, p),
                                    Mentsu::Kotsu(p) => (
                                        if i == idx && self.flags.is_tsumo.not() {
                                            MentsuDetail::Minko
                                        } else {
                                            MentsuDetail::Anko
                                        },
                                        p,
                                    ),
                                })
                                .chain(std::iter::once((MentsuDetail::Toitsu, head)))
                                .collect(),
                            is_bad_shape: if target.is_shuntsu()
                                && (target.get_pai().get_number() == 1 && machi.get_number() == 3
                                    || target.get_pai().get_number() == 7
                                        && machi.get_number() == 7
                                    || machi.get_number() - 1 == target.get_pai().get_number())
                            {
                                true
                            } else {
                                false
                            },
                        });
                    }
                }
            }
        }
        for f in self.furo.iter().map(|f| f.reduced()) {
            for m in rs.iter_mut() {
                m.mentsu.push(match f.furo_type {
                    ReducedFuroType::Unknown => unreachable!(),
                    ReducedFuroType::Chi => (MentsuDetail::Shuntsu, f.minimum),
                    ReducedFuroType::Pon => (MentsuDetail::Minko, f.minimum),
                    ReducedFuroType::MinKan => (MentsuDetail::Minkan, f.minimum),
                    ReducedFuroType::Ankan => (MentsuDetail::Ankan, f.minimum),
                });
            }
        }

        rs
    }

    fn judge_fu_normal(&self, meld_detail: &MeldDetail) -> (u16, bool) {
        let menzen_ron = if self.flags.is_menzen && self.flags.is_tsumo.not() {
            10
        } else {
            0
        };
        let tsumo = if self.flags.is_tsumo { 2 } else { 0 };
        let bad_shape = if meld_detail.is_bad_shape { 2 } else { 0 };
        let mut fu = 20u16 + menzen_ron + tsumo + bad_shape;
        for (m, p) in meld_detail.mentsu.iter() {
            let mut kotsu_fu = 0;
            match m {
                MentsuDetail::Toitsu => {
                    if *p == self.bakaze {
                        fu += 2;
                    }
                    if *p == self.jikaze {
                        fu += 2;
                    }
                    if p.is_dragon() {
                        fu += 2;
                    }
                }
                MentsuDetail::Shuntsu => {}
                MentsuDetail::Anko => kotsu_fu = 4,
                MentsuDetail::Minko => kotsu_fu = 2,
                MentsuDetail::Ankan => kotsu_fu = 16,
                MentsuDetail::Minkan => kotsu_fu = 8,
            }
            if p.is_terminal() {
                kotsu_fu *= 2
            }
            fu += kotsu_fu;
        }
        let is_pinfu = (fu - menzen_ron - tsumo == 20) && self.flags.is_menzen;
        if is_pinfu {
            fu -= tsumo
        }
        if self.flags.is_menzen.not() && fu == 20 {
            fu = 30;
        }
        let fu = fu.div_ceil(10) * 10;
        (fu, is_pinfu)
    }

    /// (han, fu, tensu, yakus)
    pub fn judge_han_fu(&self) -> (u8, u16, u32, Vec<Yaku>) {
        let mut common_yaku_vec = vec![];
        let flags = &self.flags;
        let pais = &self
            .pai
            .iter()
            .cloned()
            .chain(self.furo.iter().flat_map(|f| f.pais()))
            .collect::<Vec<_>>();

        // Special: 1-han: Akadora
        pais.iter()
            .filter(|p| p.is_aka())
            .for_each(|_| common_yaku_vec.push(Akadora));
        let pais = pais.iter().map(|p| p.remove_aka()).collect::<Vec<_>>();

        // random yaku
        if flags.is_tsumo {
            if flags.is_menzen {
                // Yakuman: Tenhou
                if flags.is_first && self.jikaze.is_z_1() {
                    common_yaku_vec.push(Tenhou);
                }
                // Yakuman: Chihou
                if flags.is_first && self.jikaze.is_z_1().not() {
                    common_yaku_vec.push(Chihou);
                }
                // 1-han: Menzen Tsumo
                common_yaku_vec.push(MenzeTsumo);
            }
            // 1-han: Rinshan Kaihou
            if flags.is_kan {
                common_yaku_vec.push(RinshaKaihou);
            }
            // 1-han: Haitei Raoyue
            if flags.is_last {
                common_yaku_vec.push(HaiteRaoyue);
            }
        } else {
            // 1-han: Chan Kan
            if flags.is_kan {
                common_yaku_vec.push(Chankan);
            }
            // 1-han: Houtei Raoyui
            if flags.is_last {
                common_yaku_vec.push(HouteRaoyui);
            }
        }
        // riichi
        // 1-han: Riichi
        if flags.is_riichi && flags.is_dbl_riichi.not() {
            common_yaku_vec.push(Riichi);
        }
        // 1-han: Double Riichi
        if flags.is_dbl_riichi {
            common_yaku_vec.push(DaburRiichi);
        }
        // 1-han: Ippatsu
        if flags.is_ippatsu {
            common_yaku_vec.push(Ippatsu);
        }
        // dora
        // 1-han: Dora
        (0..self
            .dora_marker
            .iter()
            .map(|marker| marker.get_dora_next())
            .map(|dora| pais.iter().filter(|&p| p.partial_eq(&dora)).count())
            .sum())
            .for_each(|_| common_yaku_vec.push(Dora));
        if flags.is_riichi {
            // 1-han: Uradora
            (0..self
                .uradora_marker
                .iter()
                .map(|marker| marker.get_dora_next())
                .map(|dora| pais.iter().filter(|&p| p.partial_eq(&dora)).count())
                .sum())
                .for_each(|_| common_yaku_vec.push(Uradora));
        }

        // all pai
        // 1-han: Tanyao
        if pais.iter().all(|p| p.is_terminal().not()) {
            common_yaku_vec.push(Tanyao);
        }
        if pais.iter().all(|p| p.is_terminal() && p.is_number()) {
            // Yakuman: Chinroutou
            common_yaku_vec.push(Chinroutou);
            // 2-han: Honroutou
        } else if pais.iter().all(|p| p.is_terminal()) {
            common_yaku_vec.push(Honroutou);
        }
        // Yakuman: Tsuuiisou
        if pais.iter().all(|p| p.is_number().not()) {
            common_yaku_vec.push(Tsuuiisou);
        }
        let first_pai = pais.first().unwrap();
        if pais.iter().all(|p| p.get_suit().eq(&first_pai.get_suit())) {
            let pai_map = pais.to_pai_map();
            if pai_map.iter().all(
                |(p, &cnt)| {
                    if p.is_terminal() {
                        cnt >= 3
                    } else {
                        cnt >= 1
                    }
                },
            ) {
                // Yakuman: Chuure Pouto
                if pai_map.get(&self.machi) % 2 == 0 {
                    common_yaku_vec.push(ChuurePout9Wait);
                } else {
                    common_yaku_vec.push(ChuurePouto);
                }
            } else {
                // 6-han: Chinitsu
                common_yaku_vec.push(Chinitsu);
            }
            // 3-han: Honitsu
        } else if pais
            .iter()
            .all(|p| p.get_suit().eq(&first_pai.get_suit()) || p.is_number().not())
        {
            common_yaku_vec.push(Honitsu);
        }
        // Yakuman: Ryuuiisou
        if pais
            .iter()
            .all(|p| [Pai::S2, Pai::S3, Pai::S4, Pai::S6, Pai::S8, Pai::Z6].contains(p))
        {
            common_yaku_vec.push(Ryuuiisou);
        }
        let mut possible_fus = vec![];
        let mut possible_yakus = vec![];
        if is_kokushi(&pais) {
            possible_fus.push(25);
            // Yakuman: Kokush Musou
            if pais.iter().filter(|&p| p.eq(&self.machi)).count() == 2 {
                possible_yakus.push(vec![KokushMuso13Wait]);
            } else {
                possible_yakus.push(vec![KokushMusou]);
            }
        } else {
            if is_chiitoi(&pais) {
                possible_fus.push(25);
                // 2-han: Chiitoitsu
                possible_yakus.push(vec![Chiitoitsu]);
            }
            for md in self
                .split_melds_machi_normal(split_into_melds(
                    self.pai.iter().map(|p| p.remove_aka()).collect::<Vec<_>>(),
                ))
                .iter()
            {
                let mut this_yakus = vec![];
                let (possible_fu, is_pinfu) = self.judge_fu_normal(md);
                // 1-han: Pinfu
                if is_pinfu {
                    this_yakus.push(Pinfu);
                }
                possible_fus.push(possible_fu);
                if md
                    .mentsu
                    .iter()
                    .any(|(m, p)| m.is_kotsu() && p.eq(&self.jikaze))
                {
                    if self.jikaze.is_z_1() {
                        this_yakus.push(JikazeTon);
                    }
                    if self.jikaze.is_z_2() {
                        this_yakus.push(JikazeNan);
                    }
                    if self.jikaze.is_z_3() {
                        this_yakus.push(JikazeXia);
                    }
                    if self.jikaze.is_z_4() {
                        this_yakus.push(JikazeXia)
                    }
                }
                if md
                    .mentsu
                    .iter()
                    .any(|(m, p)| m.is_kotsu() && p.eq(&self.bakaze))
                {
                    if self.bakaze.is_z_1() {
                        this_yakus.push(BakazeTon);
                    }
                    if self.bakaze.is_z_2() {
                        this_yakus.push(BakazeNan);
                    }
                    if self.bakaze.is_z_3() {
                        this_yakus.push(BakazeXia);
                    }
                    if self.bakaze.is_z_4() {
                        this_yakus.push(BakazePei);
                    }
                }
                md.mentsu.iter().for_each(|(m, p)| {
                    if m.is_kotsu().not() {
                        return;
                    }
                    match p {
                        Pai::Z5 => {
                            this_yakus.push(Haku);
                        }
                        Pai::Z6 => {
                            this_yakus.push(Hatsu);
                        }
                        Pai::Z7 => {
                            this_yakus.push(Chun);
                        }
                        _ => {}
                    }
                });

                let mut shuntsu_map = [[0u8; 10]; 4];
                let mut kotsu_map = [[0u8; 10]; 4];
                let _toitsu = md.mentsu.iter().find(|(m, _)| m.is_toitsu()).unwrap().1;
                for (m, p) in md.mentsu.iter() {
                    match m {
                        MentsuDetail::Toitsu => {}
                        MentsuDetail::Shuntsu => {
                            shuntsu_map[p.get_suit().to_idx() as usize][p.get_number() as usize] +=
                                1;
                        }
                        MentsuDetail::Anko
                        | MentsuDetail::Minko
                        | MentsuDetail::Ankan
                        | MentsuDetail::Minkan => {
                            kotsu_map[p.get_suit().to_idx() as usize][p.get_number() as usize] += 1;
                        }
                    }
                }
                // shuntsu
                if flags.is_menzen {
                    match shuntsu_map.iter().flatten().map(|&w| w / 2u8).sum() {
                        // 3-han: Ryanpeikou
                        2 => {
                            this_yakus.push(Ryanpeikou);
                        }
                        // 1-han: Iipeiko
                        1 => {
                            this_yakus.push(Iipeiko);
                        }
                        _ => {}
                    }
                }
                for i in 1..=9 {
                    if shuntsu_map[0][i] > 0 && shuntsu_map[1][i] > 0 && shuntsu_map[2][i] > 0 {
                        // 2-han: Sanshoku Doujun
                        this_yakus.push(SanshokDoujun);
                    }
                }
                for j in 0..3 {
                    if shuntsu_map[j][1] > 0 && shuntsu_map[j][4] > 0 && shuntsu_map[j][7] > 0 {
                        // 2-han: Ikkituukan
                        this_yakus.push(Ittsu);
                    }
                }
                if md.mentsu.iter().all(|(m, p)| {
                    ((m.is_kotsu() || m.is_toitsu()) && p.is_terminal()
                        || m.is_shuntsu()
                            && p.is_number()
                            && (p.get_number() == 1 || p.get_number() == 7))
                        && md.mentsu.iter().any(|(m, _)| m.is_shuntsu())
                }) {
                    if pais.iter().all(|p| p.is_number()) {
                        // 3-han: Junchan
                        this_yakus.push(Junchan);
                    } else {
                        // 2-han: Chanta
                        this_yakus.push(Chanta);
                    }
                }

                //kotsu
                match md.mentsu.iter().filter(|(m, _)| m.is_concealed()).count() {
                    // Yakuman: Suuankou
                    4 => {
                        if md.is_bad_shape {
                            this_yakus.push(SuuankoTanki);
                        } else {
                            this_yakus.push(Suuankou);
                        }
                    }
                    // 2-han: Sanankou
                    3 => {
                        this_yakus.push(Sanankou);
                    }
                    _ => {}
                }

                match md.mentsu.iter().filter(|(m, _)| m.is_kantsu()).count() {
                    // Yakuman: Suukantsu
                    4 => this_yakus.push(Suukantsu),
                    // 2-han: Sankantsu
                    3 => this_yakus.push(Sankantsu),
                    _ => {}
                }

                for i in 1..=9 {
                    if kotsu_map[0][i] > 0 && kotsu_map[1][i] > 0 && kotsu_map[2][i] > 0 {
                        // 2-han: Sanshoku Doukou
                        this_yakus.push(SanshokDoukou);
                    }
                }

                if md.mentsu.iter().filter(|(m, _)| m.is_kotsu()).count() == 4 {
                    // 2-han: Toitoi
                    this_yakus.push(Toitoi);
                }

                // sangen suushi
                if md.mentsu.iter().filter(|(_, p)| p.is_dragon()).count() == 3 {
                    if md
                        .mentsu
                        .iter()
                        .filter(|(m, p)| p.is_dragon() && m.is_kotsu())
                        .count()
                        == 3
                    {
                        // Yakuman: Daisangen
                        this_yakus.push(Daisangen);
                    } else {
                        // 2-han: Shousangen
                        this_yakus.push(Shousangen);
                    }
                }
                if md.mentsu.iter().filter(|(_, p)| p.is_wind()).count() == 4 {
                    if md
                        .mentsu
                        .iter()
                        .filter(|(m, p)| p.is_wind() && m.is_kotsu())
                        .count()
                        == 4
                    {
                        // Yakuman: Daisuushi
                        this_yakus.push(Daisuushi);
                    } else {
                        // Yakumann: Shousuushi
                        this_yakus.push(Shousuushi);
                    }
                }

                possible_yakus.push(this_yakus);
            }
        }
        let mut rs = Vec::with_capacity(possible_fus.len());
        for (possible, fu) in possible_yakus.iter().zip(possible_fus) {
            let mut yakus = common_yaku_vec
                .iter()
                .chain(possible.iter())
                .cloned()
                .collect::<Vec<_>>();
            if yakus.iter().any(|y| y.is_yakuman()) {
                yakus = yakus.into_iter().filter(|y| y.is_yakuman()).collect();
            }
            let han = yakus
                .iter()
                .map(|y| {
                    let h = y.get_han();
                    if flags.is_menzen.not() && y.kura() {
                        h - 1
                    } else {
                        h
                    }
                })
                .sum();
            let han_to_calc = if yakus.iter().all(|y| y.is_yakuman().not()) && han > 13 {
                13
            } else {
                han
            };
            let tensu = tensu_total(han_to_calc, fu, self.jikaze.is_z_1(), flags.is_tsumo);
            rs.push((han, fu, tensu, yakus));
        }
        rs.iter()
            .max_by(|a, b| {
                a.2.cmp(&b.2).then(a.0.cmp(&b.0)).then(
                    a.3.iter()
                        .filter(|y| y.is_dora_type().not())
                        .count()
                        .cmp(&b.3.iter().filter(|y| y.is_dora_type().not()).count()),
                )
            })
            .unwrap()
            .clone()
    }
}

static TENSU_TABLE: LazyLock<Mutex<HashMap<(u8, u16), u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Calculate the basic score of a hand.
pub fn calc_tensu(han: u8, fu: u16) -> u32 {
    let mut guard = TENSU_TABLE.try_lock().unwrap();
    if let Some(&tensu) = guard.get(&(han, fu)) {
        tensu
    } else {
        let is_mangan = han >= 5 || (han >= 4 && fu >= 40) || (han >= 3 && fu >= 70);
        let basic: u32 = if is_mangan {
            match han {
                ..=5 => 2000,
                6..=7 => 3000,
                8..=10 => 4000,
                11..=12 => 6000,
                13..=u8::MAX => 8000 * (han as u32 / 13),
            }
        } else {
            fu as u32 * 2u32.pow((2 + han).min(7) as u32)
        };
        guard.insert((han, fu), basic);
        basic
    }
}
fn tensu_ceil(tensu: u32) -> u32 {
    tensu.div_ceil(100) * 100
}

fn tensu_total(han: u8, fu: u16, oya: bool, is_tsumo: bool) -> u32 {
    let tensu = calc_tensu(han, fu);
    if oya {
        if is_tsumo {
            3 * tensu_ceil(tensu * 2)
        } else {
            tensu_ceil(tensu * 6)
        }
    } else {
        if is_tsumo {
            2 * tensu_ceil(tensu) + tensu_ceil(tensu * 2)
        } else {
            tensu_ceil(tensu * 4)
        }
    }
}

#[test]
fn test_yaku_judge() {
    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: vec![Pai::M4],
            uradora_marker: vec![],
            machi: Pai::S9,
            bakaze: Pai::Z1,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("111222333p78999s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (6, 20, 18000, vec![MenzeTsumo, Pinfu, Iipeiko, Junchan])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: vec![Pai::Z2],
            uradora_marker: vec![],
            machi: Pai::S9,
            bakaze: Pai::Z1,
            jikaze: Pai::Z4,
            pai: IVecPai::from_string("444666p66677889s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (1, 30, 1100, vec![MenzeTsumo])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: vec![Pai::S6],
            uradora_marker: vec![],
            machi: Pai::P4,
            bakaze: Pai::Z2,
            jikaze: Pai::Z3,
            pai: IVecPai::from_string("333m344445666p22s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (3, 50, 6400, vec![Tanyao, Sanankou])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: true,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("2p8m"),
            uradora_marker: IVecPai::from_string("7m1s"),
            machi: Pai::Z2,
            bakaze: Pai::Z1,
            jikaze: Pai::Z3,
            pai: IVecPai::from_string("345m789p22244z"),
            furo: vec![Furo {
                furo_type: FuroType::AnKan,
                target: Pai::M1,
                consumed: [Pai::M1, Pai::M1, Pai::M1]
            }],
        }
        .judge_han_fu(),
        (2, 70, 4700, vec![MenzeTsumo, Riichi])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: false,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("852p"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::Z7,
            bakaze: Pai::Z2,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("123678s77z"),
            furo: vec![
                Furo {
                    furo_type: FuroType::AnKan,
                    target: Pai::M9,
                    consumed: [Pai::M9, Pai::M9, Pai::M9]
                },
                Furo {
                    furo_type: FuroType::Chakan,
                    target: Pai::Z5,
                    consumed: [Pai::Z5, Pai::Z5, Pai::Z5]
                },
            ],
        }
        .judge_han_fu(),
        (1, 80, 3900, vec![Haku])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: true,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("6p"),
            uradora_marker: IVecPai::from_string("0s"),
            machi: Pai::M4,
            bakaze: Pai::Z2,
            jikaze: Pai::Z2,
            pai: IVecPai::from_string("111406m456p22277s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (3, 40, 5200, vec![Akadora, MenzeTsumo, Riichi])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: true,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("9s"),
            uradora_marker: IVecPai::from_string("4p"),
            machi: Pai::M6,
            bakaze: Pai::Z1,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("456888m222p123s11z"),
            furo: vec![],
        }
        .judge_han_fu(),
        (3, 40, 7800, vec![MenzeTsumo, Riichi, Dora])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("7s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::M4,
            bakaze: Pai::Z1,
            jikaze: Pai::Z2,
            pai: IVecPai::from_string("234789m22567p123s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (2, 20, 1500, vec![MenzeTsumo, Pinfu])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: false,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("48s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::S3,
            bakaze: Pai::Z1,
            jikaze: Pai::Z2,
            pai: IVecPai::from_string("33366s"),
            furo: vec![
                Furo {
                    furo_type: FuroType::Chakan,
                    target: Pai::Z6,
                    consumed: [Pai::Z6, Pai::Z6, Pai::Z6]
                },
                Furo {
                    furo_type: FuroType::Pon,
                    target: Pai::S4,
                    consumed: [Pai::S4, Pai::S4, Pai::Unknown]
                },
                Furo {
                    furo_type: FuroType::Chi,
                    target: Pai::S2,
                    consumed: [Pai::S1, Pai::S3, Pai::Unknown]
                },
            ],
        }
        .judge_han_fu(),
        (3, 40, 5200, vec![Honitsu, Hatsu])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: false,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("78m3s2p"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::M2,
            bakaze: Pai::Z2,
            jikaze: Pai::Z4,
            pai: IVecPai::from_string("22345m"),
            furo: vec![
                Furo {
                    furo_type: FuroType::AnKan,
                    target: Pai::P1,
                    consumed: [Pai::P1, Pai::P1, Pai::P1]
                },
                Furo {
                    furo_type: FuroType::AnKan,
                    target: Pai::P9,
                    consumed: [Pai::P9, Pai::P9, Pai::P9]
                },
                Furo {
                    furo_type: FuroType::Chakan,
                    target: Pai::Z3,
                    consumed: [Pai::Z3, Pai::Z3, Pai::Z3]
                },
            ],
        }
        .judge_han_fu(),
        (2, 110, 7100, vec![Sankantsu])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("3s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::P7,
            bakaze: Pai::Z2,
            jikaze: Pai::Z4,
            pai: IVecPai::from_string("33m0577p99s223344z"),
            furo: vec![],
        }
        .judge_han_fu(),
        (4, 25, 6400, vec![Akadora, MenzeTsumo, Chiitoitsu])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: false,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: true,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("8s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::M3,
            bakaze: Pai::Z1,
            jikaze: Pai::Z4,
            pai: IVecPai::from_string("234m234p23455s"),
            furo: vec![Furo {
                furo_type: FuroType::Chi,
                target: Pai::P6,
                consumed: [Pai::P0, Pai::P7, Pai::Unknown],
            }],
        }
        .judge_han_fu(),
        (3, 30, 4000, vec![Akadora, Tanyao, SanshokDoujun])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("8s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::M1,
            bakaze: Pai::Z2,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("19m119p19s1234567z"),
            furo: vec![],
        }
        .judge_han_fu(),
        (13, 25, 48000, vec![KokushMusou])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("8s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::P1,
            bakaze: Pai::Z2,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("19m119p19s1234567z"),
            furo: vec![],
        }
        .judge_han_fu(),
        (13, 25, 48000, vec![KokushMuso13Wait])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("8s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::S1,
            bakaze: Pai::Z2,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("11123406778999s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (13, 40, 48000, vec![ChuurePouto])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: true,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("8s"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::S7,
            bakaze: Pai::Z2,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("11123406778999s"),
            furo: vec![],
        }
        .judge_han_fu(),
        (13, 40, 48000, vec![ChuurePout9Wait])
    );

    assert_eq!(
        YakuJudgeArgs {
            flags: YakuJudgeFlags {
                is_menzen: false,
                is_riichi: false,
                is_dbl_riichi: false,
                is_tsumo: false,
                is_ippatsu: false,
                is_kan: false,
                is_last: false,
                is_first: false,
            },
            dora_marker: IVecPai::from_string("1z"),
            uradora_marker: IVecPai::from_string(""),
            machi: Pai::Z7,
            bakaze: Pai::Z2,
            jikaze: Pai::Z1,
            pai: IVecPai::from_string("22277z"),
            furo: vec![
                Furo {
                    furo_type: FuroType::Chakan,
                    target: Pai::P1,
                    consumed: [Pai::P1, Pai::P1, Pai::P1,]
                },
                Furo {
                    furo_type: FuroType::Pon,
                    target: Pai::Z5,
                    consumed: [Pai::Z5, Pai::Z5, Pai::Unknown,]
                },
                Furo {
                    furo_type: FuroType::AnKan,
                    target: Pai::Z6,
                    consumed: [Pai::Z6, Pai::Z6, Pai::Z6,]
                },
            ],
        }
        .judge_han_fu(),
        (
            12,
            90,
            36000,
            vec![Dora, Dora, Dora, Honroutou, BakazeNan, Haku, Hatsu, Toitoi, Shousangen]
        )
    );
}
