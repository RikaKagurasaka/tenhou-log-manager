#[repr(u8)]
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

impl From<Yaku> for u8 {
    fn from(value: Yaku) -> Self {
        value as u8
    }
}
