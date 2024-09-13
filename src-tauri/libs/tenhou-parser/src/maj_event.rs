use crate::utils::{GetAttribute, IntoActor, IntoNumVec};
use quick_xml::events::BytesStart;
use std::convert::TryInto;
use strum_macros::{EnumIs, EnumTryAs};
use urlencoding::decode;

type Pai = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NakiType {
    Chii,
    Pon,
    Kan,
    Ankan,
    Kakan,
    Daiminkan,
    #[default]
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GoType {
    // &  1
    pub is_pvp: bool,
    // & 16
    pub is_sanma: bool,
    // & 128
    pub is_up: bool,
    // & 32
    pub is_sp_or_phonix: bool,
    // & 8
    pub is_south: bool,
    // & 4
    pub is_furo: bool,
    // & 2
    pub is_not_aka: bool,
    // & 64
    pub is_fast: bool,

    pub is_room: bool,
}

impl GoType {
    pub fn from(num: u32) -> Self {
        Self {
            is_pvp: num & 1 != 0,
            is_sanma: num & 16 != 0,
            is_up: num & 128 != 0,
            is_sp_or_phonix: num & 32 != 0,
            is_south: num & 8 != 0,
            is_furo: num & 4 != 0,
            is_not_aka: num & 2 != 0,
            is_fast: num & 64 != 0,
            is_room: false,
        }
    }

    pub fn applicable(&self) -> bool {
        !self.is_sanma && self.is_pvp && !self.is_room
    }
}

#[derive(Debug, Clone, Default)]
#[derive(EnumIs,EnumTryAs)]
pub enum MajEvent {
    #[default]
    Unknown,
    Go {
        r#type: GoType,
    },
    UN {
        id: [String; 4],
        dan: [u8; 4],
        rate: [f32; 4],
    },
    Init {
        dora_marker: Pai,
        honba: u8,
        kyoku: u8,
        kyotaku: u8,
        oya: u8,
        scores: [i32; 4],
        tehais: [[Pai; 13]; 4],
    },
    Ryuukyoku {
        honba: u8,
        kyotaku: u8,
        is_special: bool,
        after_scores: [i32; 4],
        diff_scores: [i32; 4],
        tenpai: [bool; 4],
        owari: bool,
    },
    Dora {
        dora_marker: Pai,
    },
    ReachRequest {
        actor: u8,
    },
    ReachAccepted {
        actor: u8,
        after_scores: [i32; 4],
    },
    Dahai {
        actor: u8,
        pai: Pai,
    },
    Tsumo {
        actor: u8,
        pai: Pai,
    },
    Naki {
        actor: u8,
        consumed: Vec<Pai>,
        pai: Option<Pai>,
        target: Option<u8>,
        r#type: NakiType,
    },
    Agari {
        honba: u8,
        kyotaku: u8,
        hai: Vec<Pai>,
        naki: Option<Vec<MajEvent>>,
        machi: Pai,
        han: u8,
        hu: u8,
        score: i32,
        yaku: Vec<u8>,
        dora_marker: Vec<Pai>,
        ura_marker: Option<Vec<Pai>>,
        actor: u8,
        fromwho: u8,
        paowho: Option<u8>,
        after_scores: [i32; 4],
        diff_scores: [i32; 4],
        owari: bool,
    },
}

pub trait ToMajEvent {
    fn to_maj_event(&self) -> Option<MajEvent>;
}

impl<'a> ToMajEvent for &BytesStart<'a> {
    fn to_maj_event(&self) -> Option<MajEvent> {
        let e = self;
        match String::from_utf8(e.name().as_ref().to_vec())
            .unwrap()
            .as_str()
        {
            "SHUFFLE" | "TAIKYOKU" | "BYE" => None,
            "GO" => {
                let r#type: u32 = e.get_attribute("type").unwrap().parse().unwrap();
                let is_room = e
                    .get_attribute("lobby")
                    .unwrap_or("0".to_string())
                    .parse()
                    .unwrap_or(0)
                    != 0;
                let mut r#type = GoType::from(r#type);
                r#type.is_room = is_room;
                Some(MajEvent::Go { r#type })
            }
            "UN" => {
                if e.get_attribute("dan").is_none() || e.get_attribute("rate").is_none() {
                    return None;
                }
                let dan = e
                    .get_attribute("dan")
                    .unwrap()
                    .into_num_vec()
                    .as_slice()
                    .try_into();
                let rate = e
                    .get_attribute("rate")
                    .unwrap()
                    .into_num_vec()
                    .as_slice()
                    .try_into();
                if let (Ok(dan), Ok(rate)) = (dan, rate) {
                    let id = [
                        decode(e.get_attribute("n0").unwrap().as_str())
                            .unwrap()
                            .to_string(),
                        decode(e.get_attribute("n1").unwrap().as_str())
                            .unwrap()
                            .to_string(),
                        decode(e.get_attribute("n2").unwrap().as_str())
                            .unwrap()
                            .to_string(),
                        decode(e.get_attribute("n3").unwrap().as_str())
                            .unwrap()
                            .to_string(),
                    ];
                    Some(MajEvent::UN { dan, rate, id })
                } else {
                    None
                }
            }
            "INIT" => {
                let seed: Vec<u8> = e.get_attribute("seed").unwrap().into_num_vec();
                let now_kyu: u8 = seed[0];
                let _bakaze: &str = if now_kyu < 4 {
                    "E"
                } else if now_kyu < 8 {
                    "S"
                } else {
                    "W"
                };
                let dora_marker: Pai = Pai::from(seed[5]);
                let honba: u8 = seed[1];
                let kyotaku: u8 = seed[2];
                let kyoku: u8 = (now_kyu % 4) + 1;
                let oya: u8 = e.get_attribute("oya").unwrap().parse().unwrap();
                let scores: [i32; 4] = e
                    .get_attribute("ten")
                    .unwrap()
                    .into_num_vec()
                    .iter()
                    .map(|&x: &i32| x * 100)
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let tehais: [[Pai; 13]; 4] = [
                    e.get_attribute("hai0")
                        .unwrap()
                        .into_num_vec()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                    e.get_attribute("hai1")
                        .unwrap()
                        .into_num_vec()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                    e.get_attribute("hai2")
                        .unwrap()
                        .into_num_vec()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                    e.get_attribute("hai3")
                        .unwrap()
                        .into_num_vec()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                ];
                Some(MajEvent::Init {
                    dora_marker,
                    honba,
                    kyoku,
                    kyotaku,
                    oya,
                    scores,
                    tehais,
                })
            }
            t if b"TUVW".contains(&t.as_bytes()[0]) && e.attributes().count() == 0 => {
                let tag = t.chars().next().unwrap();
                let actor: u8 = tag.into_actor();
                let pai_num: u8 = t[1..].to_string().parse().unwrap();
                let pai = Pai::from(pai_num);
                Some(MajEvent::Tsumo { actor, pai })
            }
            t if b"DEFG".contains(&t.as_bytes()[0]) && e.attributes().count() == 0 => {
                let tag = t.chars().next().unwrap();
                let actor: u8 = tag.into_actor();
                let pai_num: u8 = t[1..].to_string().parse().unwrap();
                let pai = Pai::from(pai_num);
                Some(MajEvent::Dahai { actor, pai })
            }
            "RYUUKYOKU" => {
                let ba = e.get_attribute("ba").unwrap().into_num_vec();
                let honba: u8 = ba[0];
                let kyotaku: u8 = ba[1];
                let is_special = e.get_attribute("type").is_some();
                let score_arr = e
                    .get_attribute("sc")
                    .unwrap()
                    .into_num_vec()
                    .iter()
                    .map(|&x: &i32| x * 100)
                    .collect::<Vec<i32>>();
                let before_scores: [i32; 4] = score_arr
                    .iter()
                    .step_by(2)
                    .cloned()
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let diff_scores: [i32; 4] = score_arr
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .cloned()
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let after_scores = before_scores
                    .iter()
                    .zip(diff_scores.iter())
                    .map(|(x, y)| x + y)
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let owari = e.get_attribute("owari").is_some();
                let tenpai: [bool; 4] = if is_special {
                    [false; 4]
                } else {
                    [
                        e.get_attribute("hai0").is_some(),
                        e.get_attribute("hai1").is_some(),
                        e.get_attribute("hai2").is_some(),
                        e.get_attribute("hai3").is_some(),
                    ]
                };
                Some(MajEvent::Ryuukyoku {
                    honba,
                    kyotaku,
                    is_special,
                    after_scores,
                    diff_scores,
                    tenpai,
                    owari,
                })
            }
            "DORA" => {
                let pai_num: u8 = e.get_attribute("hai").unwrap().parse().unwrap();
                let pai = Pai::from(pai_num);
                Some(MajEvent::Dora { dora_marker: pai })
            }
            "REACH" => {
                let actor: u8 = e.get_attribute("who").unwrap().parse().unwrap();
                let typenum: u8 = e.get_attribute("step").unwrap().parse().unwrap();
                if typenum == 1 {
                    Some(MajEvent::ReachRequest { actor })
                } else {
                    let after_scores: [i32; 4] = e
                        .get_attribute("ten")
                        .unwrap()
                        .into_num_vec()
                        .iter()
                        .map(|&x: &i32| x * 100)
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap();
                    Some(MajEvent::ReachAccepted {
                        actor,
                        after_scores,
                    })
                }
            }
            "AGARI" => {
                let ba = e.get_attribute("ba").unwrap().into_num_vec();
                let ten = e.get_attribute("ten").unwrap().into_num_vec();
                let honba = ba[0];
                let kyotaku = ba[1];
                let hu = ten[0] as u8;
                let score = ten[1];
                let yaku = if let Some(yaku) = e.get_attribute("yaku") {
                    yaku.into_num_vec()
                        .chunks(2)
                        .flat_map(|y| {
                            let [nowyaku, val] = [y[0], y[1]];
                            return if nowyaku == 52 || nowyaku == 53 || nowyaku == 54 {
                                (0..val).map(|_| (nowyaku, 1)).collect::<Vec<_>>()
                            } else {
                                vec![(nowyaku, val)]
                            };
                        })
                        .collect()
                } else {
                    e.get_attribute("yakuman")
                        .unwrap()
                        .into_num_vec()
                        .iter()
                        .map(|&x: &u8| (x, 13))
                        .collect::<Vec<_>>()
                };
                let han = yaku.iter().map(|&(_, val)| val).sum();
                let yaku = yaku.iter().map(|&(nowyaku, _)| nowyaku).collect();
                let hai = e.get_attribute("hai").unwrap().into_num_vec();
                let machi_num: u8 = e.get_attribute("machi").unwrap().parse().unwrap();
                let machi = Pai::from(machi_num);
                let actor = e.get_attribute("who").unwrap().parse().unwrap();
                let paowho = e.get_attribute("paoWho").map(|x| x.parse().unwrap());
                let fromwho = e.get_attribute("fromWho").unwrap().parse().unwrap();
                let naki = if let Some(naki_raw_list) = e.get_attribute("m") {
                    Some(
                        naki_raw_list
                            .into_num_vec()
                            .iter()
                            .map(|&naki_raw: &i32| parse_naki(actor, naki_raw as u32))
                            .collect(),
                    )
                } else {
                    None
                };
                let dora_marker = e
                    .get_attribute("doraHai")
                    .unwrap()
                    .into_num_vec()
                    .iter()
                    .map(|&x: &u8| Pai::from(x))
                    .collect();
                let ura_marker = e.get_attribute("doraHaiUra").map(|x| {
                    x.into_num_vec()
                        .iter()
                        .map(|&x: &u8| Pai::from(x))
                        .collect()
                });
                let scores_arr: [i32; 8] = e
                    .get_attribute("sc")
                    .unwrap()
                    .into_num_vec()
                    .iter()
                    .map(|&x: &i32| x * 100)
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let before_scores: [i32; 4] = scores_arr
                    .iter()
                    .step_by(2)
                    .cloned()
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let diff_scores: [i32; 4] = scores_arr
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .cloned()
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let after_scores = before_scores
                    .iter()
                    .zip(diff_scores.iter())
                    .map(|(x, y)| x + y)
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                let owari = e.get_attribute("owari").is_some();
                Some(MajEvent::Agari {
                    honba,
                    kyotaku,
                    hai,
                    naki,
                    machi,
                    han,
                    hu,
                    score,
                    yaku,
                    dora_marker,
                    ura_marker,
                    actor,
                    fromwho,
                    paowho,
                    after_scores,
                    diff_scores,
                    owari,
                })
            }
            "N" => {
                let actor: u8 = e.get_attribute("who").unwrap().parse().unwrap();
                let m = e.get_attribute("m").unwrap().parse().unwrap();
                Some(parse_naki(actor, m))
            }

            e => {
                panic!("Unknown tag: {}", e);
            }
        }
    }
}

fn parse_naki(actor: u8, m: u32) -> MajEvent {
    if m & 4 != 0 {
        //chii
        let tile_detail = [(m >> 3) & 3, (m >> 5) & 3, (m >> 7) & 3];
        let block1 = m >> 10;
        let called = block1 % 3;
        let base = (block1 / 21) * 8 + (block1 / 3) * 4;
        let target = (actor + 3) % 4;
        let consumed_hai = tile_detail[called as usize] + 4 * called + base;
        let hai = Pai::from(consumed_hai as u8);
        let consumed_num = (0..3)
            .filter(|&i| i != called)
            .map(|i| (tile_detail[i as usize] + 4 * i + base) as u8)
            .collect::<Vec<u8>>();
        let consumed = consumed_num.iter().map(|&x: &u8| Pai::from(x)).collect();
        MajEvent::Naki {
            actor,
            consumed,
            pai: Some(hai),
            target: Some(target),
            r#type: NakiType::Chii,
        }
    } else if m & 24 != 0 {
        //pon
        let tile4th = (m >> 5) & 3;
        let target_r = m & 3;
        let block1 = m >> 9;
        let called = block1 % 3;
        let base = 4 * (block1 / 3);
        let target = (actor + target_r as u8) % 4;
        let r#type = if m & 8 != 0 {
            NakiType::Pon
        } else {
            NakiType::Kakan
        };
        let pon_tile = (0..4)
            .filter(|&i| i != tile4th)
            .map(|i| (i + base) as u8)
            .collect::<Vec<u8>>();
        let (consumed_hai, consumed_num) = if r#type == NakiType::Pon {
            let consumed_hai = pon_tile[called as usize];
            let consumed_num = (0..3)
                .filter(|&i| i != called)
                .map(|i| pon_tile[i as usize])
                .collect::<Vec<u8>>();
            (consumed_hai, consumed_num)
        } else {
            let consumed_hai = tile4th + base;
            let consumed_num = pon_tile;
            (consumed_hai as u8, consumed_num)
        };
        let consumed = consumed_num.iter().map(|&x: &u8| Pai::from(x)).collect();
        let hai = Pai::from(consumed_hai);
        MajEvent::Naki {
            actor,
            consumed,
            pai: Some(hai),
            target: Some(target),
            r#type,
        }
    } else {
        //kan
        let target_r = m & 3;
        let target = (actor + target_r as u8) % 4;
        let block1 = m >> 8;
        let called = block1 % 4;
        let base = 4 * (block1 / 4);
        let consumed_num = (0..4)
            .filter(|&i| i != called)
            .map(|i| (i + base) as u8)
            .collect::<Vec<u8>>();
        let consumed_hai = called + base;
        let hai = Pai::from(consumed_hai as u8);
        if target == actor {
            let consumed = (0..4)
                .map(|i| i + base)
                .map(|x| Pai::from(x as u8))
                .collect();
            MajEvent::Naki {
                actor,
                consumed,
                pai: None,
                target: None,
                r#type: NakiType::Ankan,
            }
        } else {
            let consumed = consumed_num.iter().map(|&x: &u8| Pai::from(x)).collect();
            MajEvent::Naki {
                actor,
                consumed,
                pai: Some(hai),
                target: Some(target),
                r#type: NakiType::Daiminkan,
            }
        }
    }
}
