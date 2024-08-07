use strum_macros::{EnumIs, EnumIter, FromRepr};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(FromRepr, EnumIs, EnumIter)]
pub enum Yaku {
    /// 門前清自摸和 0
    MenzeTsumo,
    /// 立直 1
    Riichi,
    /// 一発 2
    Ippatsu,
    /// 槍槓 3
    Chankan,
    /// 嶺上開花 4
    RinshaKaihou,
    /// 海底摸月 5
    HaiteRaoyue,
    /// 河底撈魚 6
    HouteRaoyui,
    /// 平和 7
    Pinfu,
    /// 断幺九 8
    Tanyao,
    /// 一盃口 9
    Iipeiko,
    /// 自風 東 10
    JikazeTon,
    /// 自風 南 11
    JikazeNan,
    /// 自風 西 12
    JikazeXia,
    /// 自風 北 13
    JikazePei,
    /// 場風 東 14
    BakazeTon,
    /// 場風 南 15
    BakazeNan,
    /// 場風 西 16
    BakazeXia,
    /// 場風 北 17
    BakazePei,
    /// 役牌 白 18
    Haku,
    /// 役牌 發 19
    Hatsu,
    /// 役牌 中 20
    Chun,
    /// 両立直 21
    DaburRiichi,
    /// 七対子 22
    Chiitoitsu,
    /// 混全帯幺九 23
    Chanta,
    /// 一気通貫 24
    Ittsu,
    /// 三色同順 25
    SanshokDoujun,
    /// 三色同刻 26
    SanshokDoukou,
    /// 三槓子 27
    Sankantsu,
    /// 対々和 28
    Toitoi,
    /// 三暗刻 29
    Sanankou,
    /// 小三元 30
    Shousangen,
    /// 混老頭 31
    Honroutou,
    /// 二盃口 32
    Ryanpeikou,
    /// 純全帯幺九 33
    Junchan,
    /// 混一色 34
    Honitsu,
    /// 清一色 35
    Chinitsu,
    /// 人和 36
    Renhou,
    /// 天和 37
    Tenhou,
    /// 地和 38
    Chihou,
    /// 大三元 39
    Daisangen,
    /// 四暗刻 40
    Suuankou,
    /// 四暗刻単騎 41
    SuuankoTanki,
    /// 字一色 42
    Tsuuiisou,
    /// 緑一色 43
    Ryuuiisou,
    /// 清老頭 44
    Chinroutou,
    /// 九蓮宝燈 45
    ChuurePouto,
    /// 純正九蓮宝燈 46
    ChuurePout9Wait,
    /// 国士無双 47
    KokushMusou,
    /// 国士無双１３面 48
    KokushMuso13Wait,
    /// 大四喜 49
    Daisuushi,
    /// 小四喜 50
    Shousuushi,
    /// 四槓子 51
    Suukantsu,
    /// ドラ 52
    Dora,
    /// 裏ドラ 53
    Uradora,
    /// 赤ドラ 54
    Akadora,
}

impl Yaku {
    pub fn get_han(&self) -> u8 {
        match self {
            Yaku::MenzeTsumo | Yaku::Riichi | Yaku::Ippatsu | Yaku::Chankan | Yaku::RinshaKaihou | Yaku::HaiteRaoyue | Yaku::HouteRaoyui | Yaku::Pinfu | Yaku::Tanyao | Yaku::Iipeiko | Yaku::JikazeTon | Yaku::JikazeNan | Yaku::JikazeXia | Yaku::JikazePei | Yaku::BakazeTon | Yaku::BakazeNan | Yaku::BakazeXia | Yaku::BakazePei | Yaku::Haku | Yaku::Hatsu | Yaku::Chun => 1,
            Yaku::DaburRiichi | Yaku::Chiitoitsu | Yaku::Chanta | Yaku::Ittsu | Yaku::SanshokDoujun | Yaku::SanshokDoukou | Yaku::Sankantsu | Yaku::Toitoi | Yaku::Sanankou | Yaku::Shousangen | Yaku::Honroutou => { 2 }
            Yaku::Ryanpeikou | Yaku::Junchan | Yaku::Honitsu => { 3 }
            Yaku::Chinitsu => { 6 }
            Yaku::Renhou | Yaku::Tenhou | Yaku::Chihou | Yaku::Daisangen | Yaku::Suuankou | Yaku::SuuankoTanki | Yaku::Tsuuiisou | Yaku::Ryuuiisou | Yaku::Chinroutou | Yaku::ChuurePouto | Yaku::ChuurePout9Wait | Yaku::KokushMusou | Yaku::KokushMuso13Wait | Yaku::Daisuushi | Yaku::Shousuushi | Yaku::Suukantsu => { 13 }
            Yaku::Dora | Yaku::Uradora | Yaku::Akadora => { 1 }
        }
    }

    pub fn require_menzen(&self) -> bool {
        match self {
            Yaku::MenzeTsumo => { true }
            Yaku::Riichi => { true }
            Yaku::Ippatsu => { true }
            Yaku::Chankan => { false }
            Yaku::RinshaKaihou => { false }
            Yaku::HaiteRaoyue => { false }
            Yaku::HouteRaoyui => { false }
            Yaku::Pinfu => { true }
            Yaku::Tanyao => { false }
            Yaku::Iipeiko => { true }
            Yaku::JikazeTon => { false }
            Yaku::JikazeNan => { false }
            Yaku::JikazeXia => { false }
            Yaku::JikazePei => { false }
            Yaku::BakazeTon => { false }
            Yaku::BakazeNan => { false }
            Yaku::BakazeXia => { false }
            Yaku::BakazePei => { false }
            Yaku::Haku => { false }
            Yaku::Hatsu => { false }
            Yaku::Chun => { false }
            Yaku::DaburRiichi => { true }
            Yaku::Chiitoitsu => { true }
            Yaku::Chanta => { false }
            Yaku::Ittsu => { false }
            Yaku::SanshokDoujun => { false }
            Yaku::SanshokDoukou => { false }
            Yaku::Sankantsu => { false }
            Yaku::Toitoi => { false }
            Yaku::Sanankou => { false }
            Yaku::Shousangen => { false }
            Yaku::Honroutou => { false }
            Yaku::Ryanpeikou => { true }
            Yaku::Junchan => { false }
            Yaku::Honitsu => { false }
            Yaku::Chinitsu => { false }
            Yaku::Renhou => { true }
            Yaku::Tenhou => { true }
            Yaku::Chihou => { true }
            Yaku::Daisangen => { false }
            Yaku::Suuankou => { true }
            Yaku::SuuankoTanki => { true }
            Yaku::Tsuuiisou => { false }
            Yaku::Ryuuiisou => { false }
            Yaku::Chinroutou => { false }
            Yaku::ChuurePouto => { true }
            Yaku::ChuurePout9Wait => { true }
            Yaku::KokushMusou => { true }
            Yaku::KokushMuso13Wait => { true }
            Yaku::Daisuushi => { false }
            Yaku::Shousuushi => { false }
            Yaku::Suukantsu => { false }
            Yaku::Dora => { false }
            Yaku::Uradora => { true }
            Yaku::Akadora => { false }
        }
    }

    pub fn kura(&self) -> bool {
        match self {
            Yaku::MenzeTsumo => { false }
            Yaku::Riichi => { false }
            Yaku::Ippatsu => { false }
            Yaku::Chankan => { false }
            Yaku::RinshaKaihou => { false }
            Yaku::HaiteRaoyue => { false }
            Yaku::HouteRaoyui => { false }
            Yaku::Pinfu => { false }
            Yaku::Tanyao => { false }
            Yaku::Iipeiko => { false }
            Yaku::JikazeTon => { false }
            Yaku::JikazeNan => { false }
            Yaku::JikazeXia => { false }
            Yaku::JikazePei => { false }
            Yaku::BakazeTon => { false }
            Yaku::BakazeNan => { false }
            Yaku::BakazeXia => { false }
            Yaku::BakazePei => { false }
            Yaku::Haku => { false }
            Yaku::Hatsu => { false }
            Yaku::Chun => { false }
            Yaku::DaburRiichi => { false }
            Yaku::Chiitoitsu => { false }
            Yaku::Chanta => { true }
            Yaku::Ittsu => { true }
            Yaku::SanshokDoujun => { true }
            Yaku::SanshokDoukou => { false }
            Yaku::Sankantsu => { false }
            Yaku::Toitoi => { false }
            Yaku::Sanankou => { false }
            Yaku::Shousangen => { false }
            Yaku::Honroutou => { false }
            Yaku::Ryanpeikou => { false }
            Yaku::Junchan => { true }
            Yaku::Honitsu => { true }
            Yaku::Chinitsu => { true }
            Yaku::Renhou => { false }
            Yaku::Tenhou => { false }
            Yaku::Chihou => { false }
            Yaku::Daisangen => { false }
            Yaku::Suuankou => { false }
            Yaku::SuuankoTanki => { false }
            Yaku::Tsuuiisou => { false }
            Yaku::Ryuuiisou => { false }
            Yaku::Chinroutou => { false }
            Yaku::ChuurePouto => { false }
            Yaku::ChuurePout9Wait => { false }
            Yaku::KokushMusou => { false }
            Yaku::KokushMuso13Wait => { false }
            Yaku::Daisuushi => { false }
            Yaku::Shousuushi => { false }
            Yaku::Suukantsu => { false }
            Yaku::Dora => { false }
            Yaku::Uradora => { false }
            Yaku::Akadora => { false }
        }
    }

    pub fn is_yakuman(&self) -> bool {
        self.get_han() >= 13
    }
    
    pub fn is_dora_type(&self) -> bool {
        match self {
            Yaku::Dora|Yaku::Akadora |Yaku::Uradora => { true }
            _ => { false }
        }
    }
}

impl From<Yaku> for u8 {
    fn from(yaku: Yaku) -> u8 {
        yaku as u8
    }
}