use serde::{Serialize, Serializer};
use pai::yaku::Yaku;

#[derive(Debug, Serialize, Default, Clone)]
pub struct Counter {
    /// 场数
    pub matches: u32,
    /// 一位次数
    pub rank1: u32,
    /// 二位次数
    pub rank2: u32,
    /// 三位次数
    pub rank3: u32,
    /// 四位次数
    pub rank4: u32,
    /// 被飞次数
    pub tobi: u32,

    /// 局数
    pub rounds: u32,

    /// 和了次数
    pub wins: u32,
    /// 自摸次数
    pub win_tsumo: u32,
    /// 荣和次数
    pub win_ron: u32,
    /// 和了净点数
    pub win_score: i64,
    /// 和了总点数
    pub win_total_score: i64,
    /// 和了总巡目
    pub win_total_junme: u32,
    /// 和了时立直次数
    pub win_riichi: u32,
    /// 和了时默听次数
    pub win_dama: u32,
    /// 和了时副露次数
    pub win_furo: u32,
    /// 和了时庄家次数
    pub win_oya: u32,
    /// 和了时子家次数
    pub win_ko: u32,

    /// 放铳次数
    pub loses: u32,
    /// 放铳净点数
    pub lose_score: i64,
    /// 放铳总点数
    pub lose_total_score: i64,
    /// 放铳总巡目
    pub lose_total_junme: u32,
    /// 放铳时立直次数
    pub lose_riichi: u32,
    /// 放铳时门前次数
    pub lose_menzen: u32,
    /// 放铳时副露次数
    pub lose_furo: u32,
    /// 子家放铳亲家次数
    pub lose_ko_to_oya: u32,
    /// 子家放铳子家次数
    pub lose_ko_to_ko: u32,
    /// 亲家放铳子家次数
    pub lose_oya_to_ko: u32,
    /// 放铳给立直者次数
    pub lose_to_riichi: u32,
    /// 放铳给默听者次数
    pub lose_to_dama: u32,
    /// 放铳给副露者次数
    pub lose_to_furo: u32,

    /// 被自摸次数
    pub be_tsumo: u32,
    /// 被自摸总点数
    pub be_tsumo_total_score: i64,
    /// 庄家被自摸次数
    pub be_tsumo_oya: u32,
    /// 庄家被自摸总点数
    pub be_tsumo_oya_total_score: i64,
    /// 庄家被炸庄次数
    pub be_tsumo_oya_mangan: u32,
    /// 庄家被炸庄总点数
    pub be_tsumo_oya_mangan_total_score: i64,
    /// 被自摸总巡目
    pub be_tsumo_total_junme: u32,
    /// 被自摸时立直次数
    pub be_tsumo_riichi: u32,
    /// 被自摸时门前次数
    pub be_tsumo_menzen: u32,
    /// 被自摸时副露次数
    pub be_tsumo_furo: u32,
    /// 子家被亲家自摸次数
    pub be_tsumo_ko_to_oya: u32,
    /// 子家被子家自摸次数
    pub be_tsumo_ko_to_ko: u32,
    /// 亲家被子家自摸次数
    pub be_tsumo_oya_to_ko: u32,

    /// 横移动次数
    pub no_change: u32,

    /// 流局次数
    pub draw: u32,
    /// 流局总点数
    pub draw_total_score: i64,
    /// 流局听牌次数
    pub draw_tenpai: u32,

    /// 立直次数
    pub riichi: u32,
    /// 两立直次数
    pub riichi_double: u32,
    /// 先制立直次数
    pub riichi_first: u32,
    /// 追立直次数
    pub riichi_follow: u32,
    /// 被追立直次数
    pub riichi_followed: u32,
    /// 立直总巡目
    pub riichi_total_junme: u32,
    /// 立直和了次数
    pub riichi_win: u32,
    /// 立直放铳次数
    pub riichi_lose: u32,
    /// 立直被自摸次数
    pub riichi_be_tsumo: u32,
    /// 立直流局次数
    pub riichi_draw: u32,
    /// 立直和了点数
    pub riichi_win_score: u64,
    /// 立直放铳点数
    pub riichi_lose_score: i64,
    /// 立直总收支
    pub riichi_total_score: i64,
    /// 一发自摸次数
    pub riichi_ippatsu_tsumo: u32,

    /// 总收支
    pub total_score: i64,

    /// 副露次数
    pub total_furo: u32,

    /// 各役种出现次数
    pub yakus: YakuCounter,

    /// 总计R值
    pub tot_rate: f32,
}

#[derive(Debug, Clone)]
pub struct YakuCounter {
    pub yakus: [u32; 55],
}

impl YakuCounter {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, u32> {
        self.yakus.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.yakus.iter()
    }

    pub fn get(&self, idx: Yaku) -> u32 {
        self.yakus[idx as usize]
    }

    pub fn get_mut(&mut self, idx: Yaku) -> &mut u32 {
        &mut self.yakus[idx as usize]
    }
}

impl Default for YakuCounter {
    fn default() -> Self {
        Self {
            yakus: [0; 55],
        }
    }
}

impl Serialize for YakuCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.yakus.serialize(serializer)
    }
}