use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use tenhou_parser::maj_event::{MajEvent, NakiType};

use crate::counter::Counter;
use crate::yaku::Yaku;

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub dan: u8,
    pub rate: f32,
    pub tehai: Vec<u8>,
    pub furo: Vec<MajEvent>,
    pub junme: u8,
    pub discards: Vec<u8>,
    pub score: i32,
    pub reached: bool,
    pub id: String,
}

pub struct Game {
    pub kyoku: u8,
    pub honba: u8,
    pub kyotaku: u8,
    pub oya: u8,
    pub dora_marker: [Option<u8>; 5],
    pub players: [Player; 4],
    counters: [Option<Rc<RefCell<Counter>>>; 4],
    pub registered_counters: HashMap<String, Rc<RefCell<Counter>>>,
}

impl Game {
    pub fn create_counters(ids: Vec<impl ToString>) -> HashMap<String, Rc<RefCell<Counter>>> {
        ids.into_iter()
            .map(|x| (x.to_string(), Rc::new(RefCell::new(Counter::default()))))
            .collect()
    }

    pub fn new(registered_counters: &HashMap<String, Rc<RefCell<Counter>>>) -> Self {
        Game {
            kyoku: 0,
            honba: 0,
            kyotaku: 0,
            oya: 0,
            dora_marker: [None; 5],
            players: core::array::from_fn(|_| Player::default()),
            counters: core::array::from_fn(|_| None),
            registered_counters: registered_counters.clone(),
        }
    }

    pub fn get_player(&self, player: u8) -> &Player {
        &self.players[player as usize]
    }

    pub fn get_player_mut(&mut self, player: u8) -> &mut Player {
        &mut self.players[player as usize]
    }

    pub fn on_event(&mut self, e: MajEvent) {
        match e {
            MajEvent::UN { dan, rate, ref id } => {
                for i in 0..4 {
                    let player = self.get_player_mut(i as u8);
                    player.dan = dan[i];
                    player.rate = rate[i];
                    player.id = id[i].clone();
                    if self.registered_counters.contains_key(&id[i]) {
                        self.counters[i] = Some(self.registered_counters[&id[i]].clone());
                    } else {
                        self.counters[i] = None;
                    }
                }
            }
            MajEvent::Init {
                dora_marker,
                honba,
                kyoku,
                kyotaku,
                oya,
                scores,
                tehais,
            } => {
                self.kyoku = kyoku;
                self.honba = honba;
                self.kyotaku = kyotaku;
                self.oya = oya;
                for i in 0..4 {
                    let player = self.get_player_mut(i as u8);
                    player.tehai = tehais[i].to_vec();
                    player.score = scores[i];
                    player.reached = false;
                    player.furo.clear();
                    player.junme = 0;
                    player.discards.clear();
                }
                self.dora_marker = [None; 5];
                self.dora_marker[0] = Some(dora_marker);
            }
            MajEvent::Ryuukyoku {
                honba,
                kyotaku,
                is_special,
                after_scores,
                ..
            } => {
                if is_special {
                    return;
                }
                self.honba = honba;
                self.kyotaku = kyotaku;
                for i in 0..4 {
                    self.get_player_mut(i as u8).score = after_scores[i];
                }
            }
            MajEvent::Dora { dora_marker } => {
                *self.dora_marker.iter_mut().find(|x| x.is_none()).unwrap() = Some(dora_marker);
            }
            MajEvent::ReachAccepted {
                actor,
                after_scores,
            } => {
                self.get_player_mut(actor).reached = true;
                self.get_player_mut(actor).score = after_scores[actor as usize];
            }
            MajEvent::Dahai { actor, pai } => {
                self.get_player_mut(actor).junme += 1;
                self.get_player_mut(actor).discards.push(pai);
            }
            MajEvent::Naki {
                actor,
                ref consumed,
                pai,
                target,
                r#type,
            } => {
                self.get_player_mut(actor).tehai = self
                    .get_player(actor)
                    .tehai
                    .iter()
                    .filter(|&x| !consumed.contains(x))
                    .cloned()
                    .collect();
                self.get_player_mut(actor).furo.push(MajEvent::Naki {
                    actor,
                    consumed: consumed.clone(),
                    pai,
                    target,
                    r#type,
                });
            }
            MajEvent::Agari { after_scores, .. } => {
                for i in 0..4 {
                    self.get_player_mut(i as u8).score = after_scores[i];
                }
            }
            _ => {}
        };
        let is_owari = match e {
            MajEvent::Agari { owari: true, .. } => true,
            MajEvent::Ryuukyoku { owari: true, .. } => true,
            _ => false,
        };
        self.update_counter(e);
        if is_owari {
            let mut scores = self
                .players
                .iter()
                .enumerate()
                .map(|(i, x)| (i, x.score))
                .collect::<Vec<(usize, i32)>>();
            scores.sort_by(|a, b| b.1.cmp(&a.1));
            let mean_rate = (self.players.iter().map(|x| x.rate).sum::<f32>() / 4.0).max(1500.0);
            for (rank, &(i, score)) in scores.iter().enumerate() {
                let counter = self.counters[i].clone();
                if let Some(counter) = counter {
                    let mut counter = counter.borrow_mut();
                    match rank {
                        0 => {
                            counter.rank1 += 1;
                            counter.tot_rate += 30. + mean_rate / 40.;
                        }
                        1 => {
                            counter.rank2 += 1;
                            counter.tot_rate += 10. + mean_rate / 40.;
                        }
                        2 => {
                            counter.rank3 += 1;
                            counter.tot_rate += -10. + mean_rate / 40.;
                        }
                        3 => {
                            counter.rank4 += 1;
                            counter.tot_rate += -30. + mean_rate / 40.;
                        }
                        _ => {}
                    }
                    if score < 0 {
                        counter.tobi += 1;
                    }
                }
            }
        }
    }

    fn update_counter(&mut self, e: MajEvent) {
        match e {
            MajEvent::UN { .. } => {
                for i in 0..4 {
                    let counter = self.counters[i].as_ref();
                    if let Some(counter) = counter {
                        let mut counter = counter.borrow_mut();
                        counter.matches += 1;
                    }
                }
            }
            MajEvent::Init { .. } => {
                for i in 0..4 {
                    let counter = self.counters[i].as_ref();
                    if let Some(counter) = counter {
                        let mut counter = counter.borrow_mut();
                        counter.rounds += 1;
                    }
                }
            }
            MajEvent::ReachAccepted { actor, .. } => {
                let junme = self.get_player(actor).junme;
                let naki_arr = self
                    .players
                    .iter()
                    .map(|x| x.furo.len())
                    .collect::<Vec<usize>>();
                let reached_lst = self
                    .players
                    .iter()
                    .enumerate()
                    .filter(|(_i, x)| x.reached)
                    .map(|(i, _x)| i as u8)
                    .collect::<Vec<u8>>();
                for i in 0..4 {
                    let counter = self.counters[i as usize].as_ref();
                    if let Some(counter) = counter {
                        let mut counter = counter.borrow_mut();
                        if i == actor {
                            counter.riichi += 1;
                            counter.riichi_double +=
                                if junme <= 0 && naki_arr.iter().all(|&x| x == 0) {
                                    1
                                } else {
                                    0
                                };
                            if reached_lst.iter().any(|&x| x != actor) {
                                counter.riichi_follow += 1;
                            } else {
                                counter.riichi_first += 1;
                            }
                            counter.riichi_total_junme += junme as u32;
                            counter.riichi_total_score += -1000;
                        } else {
                            if reached_lst.iter().any(|&x| x == i) {
                                counter.riichi_followed += 1;
                            }
                        }
                    }
                }
            }
            MajEvent::Ryuukyoku {
                is_special,
                diff_scores,
                tenpai,
                ..
            } => {
                if is_special {
                    return;
                }
                for i in 0..4 {
                    let reached = self.get_player(i as u8).reached;
                    let counter = self.counters[i].as_ref();
                    if let Some(counter) = counter {
                        let mut counter = counter.borrow_mut();
                        counter.draw += 1;
                        counter.draw_total_score += diff_scores[i] as i64;
                        counter.total_score += diff_scores[i] as i64;
                        counter.draw_tenpai += if tenpai[i] { 1 } else { 0 };
                        counter.riichi_draw += if reached { 1 } else { 0 };
                        counter.total_furo += self.get_player(i as u8).furo.len() as u32;
                        if reached {
                            counter.riichi_total_score += diff_scores[i] as i64;
                        }
                    }
                }
            }
            MajEvent::Agari {
                score,
                ref yaku,
                actor,
                fromwho,
                diff_scores,
                ..
            } => {
                let reached_arr = self
                    .players
                    .iter()
                    .map(|x| x.reached)
                    .collect::<Vec<bool>>();
                let is_menzen_arr = self
                    .players
                    .iter()
                    .map(|x| {
                        x.furo.iter().all(|x| match x {
                            MajEvent::Naki {
                                r#type: NakiType::Ankan,
                                ..
                            } => true,
                            _ => false,
                        })
                    })
                    .collect::<Vec<bool>>();
                for i in 0..4 {
                    let junme = self.get_player(i as u8).junme;
                    let counter = self.counters[i].as_ref();
                    if let Some(counter) = counter {
                        let mut counter = counter.borrow_mut();
                        if i as u8 == actor {
                            counter.wins += 1;
                            counter.win_tsumo += if fromwho == actor { 1 } else { 0 };
                            counter.win_ron += if fromwho != actor { 1 } else { 0 };
                            counter.win_total_score += diff_scores[i] as i64;
                            counter.win_total_junme += junme as u32;
                            counter.win_riichi += if reached_arr[i] { 1 } else { 0 };
                            counter.win_dama += if !reached_arr[i] && is_menzen_arr[i] {
                                1
                            } else {
                                0
                            };
                            counter.win_furo += if !is_menzen_arr[i] { 1 } else { 0 };
                            counter.win_oya += if actor == self.oya { 1 } else { 0 };
                            counter.win_ko += if actor != self.oya { 1 } else { 0 };
                            if yaku.contains(&Yaku::Riichi.into())
                                || yaku.contains(&Yaku::DaburRiichi.into())
                            {
                                counter.riichi_win += 1;
                                counter.riichi_win_score += score as u64;
                                counter.riichi_total_score += diff_scores[i] as i64;
                            }
                            counter.yakus.iter_mut().enumerate().for_each(|(i, x)| {
                                if yaku.contains(&(i as u8)) {
                                    *x += 1;
                                }
                            });
                        } else if i as u8 == fromwho {
                            counter.loses += 1;
                            counter.lose_total_score += diff_scores[i] as i64;
                            counter.lose_total_junme += junme as u32;
                            counter.lose_riichi += if reached_arr[i] { 1 } else { 0 };
                            counter.lose_menzen += if is_menzen_arr[i] { 1 } else { 0 };
                            counter.lose_furo += if !is_menzen_arr[i] { 1 } else { 0 };
                            counter.lose_to_riichi +=
                                if reached_arr[actor as usize] { 1 } else { 0 };
                            counter.lose_to_dama +=
                                if !reached_arr[actor as usize] && is_menzen_arr[actor as usize] {
                                    1
                                } else {
                                    0
                                };
                            counter.lose_to_furo +=
                                if !is_menzen_arr[actor as usize] { 1 } else { 0 };
                            counter.lose_ko_to_oya += if actor == self.oya { 1 } else { 0 };
                            counter.lose_ko_to_ko += if actor != self.oya && i as u8 != self.oya {
                                1
                            } else {
                                0
                            };
                            counter.lose_oya_to_ko += if actor != self.oya && i as u8 == self.oya {
                                1
                            } else {
                                0
                            };
                            if reached_arr[i] {
                                counter.riichi_lose += 1;
                                counter.lose_total_score += -1000;
                                counter.riichi_total_score += diff_scores[i] as i64 ;
                            }
                        } else if actor == fromwho {
                            counter.be_tsumo += 1;
                            counter.be_tsumo_total_score += diff_scores[i] as i64;
                            counter.be_tsumo_total_junme += junme as u32;
                            counter.be_tsumo_riichi += if reached_arr[i] { 1 } else { 0 };
                            counter.be_tsumo_menzen += if is_menzen_arr[i] { 1 } else { 0 };
                            counter.be_tsumo_furo += if !is_menzen_arr[i] { 1 } else { 0 };
                            counter.be_tsumo_ko_to_oya += if actor == self.oya { 1 } else { 0 };
                            counter.be_tsumo_ko_to_ko += if actor != self.oya && i as u8 != self.oya
                            {
                                1
                            } else {
                                0
                            };
                            counter.be_tsumo_oya_to_ko +=
                                if actor != self.oya && i as u8 == self.oya {
                                    1
                                } else {
                                    0
                                };
                            if reached_arr[i] {
                                counter.riichi_be_tsumo += 1;
                                counter.be_tsumo_total_score += -1000;
                                counter.riichi_total_score += diff_scores[i] as i64 ;
                            }
                        } else {
                            counter.no_change += 1;
                        }
                        counter.total_furo += self.get_player(i as u8).furo.len() as u32;
                    }
                }
            }
            _ => {}
        }
    }
}
