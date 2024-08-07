import { invoke } from "@tauri-apps/api/core";
import { useAsyncState, useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";

export const useCounterStore = defineStore("counter", () => {
  const userId = useLocalStorage("userId", "");
  const { state } = useAsyncState(async () => {
    if (userId) {
      let rs = await invoke<Counter>("parse_logs", { id: userId.value });
      return computedCounters(rs);
    } else {
      return null;
    }
  }, undefined);
  return state;
});

export interface Counter {
  /// 场数
  matches: number;
  /// 一位次数
  rank1: number;
  /// 二位次数
  rank2: number;
  /// 三位次数
  rank3: number;
  /// 四位次数
  rank4: number;
  /// 被飞次数
  tobi: number;

  /// 局数
  rounds: number;

  /// 和了次数
  wins: number;
  /// 自摸次数
  win_tsumo: number;
  /// 荣和次数
  win_ron: number;
  /// 和了净点数
  win_score: number;
  /// 和了总点数
  win_total_score: number;
  /// 和了总巡目
  win_total_junme: number;
  /// 和了时立直次数
  win_riichi: number;
  /// 和了时默听次数
  win_dama: number;
  /// 和了时副露次数
  win_furo: number;
  /// 和了时庄家次数
  win_oya: number;
  /// 和了时子家次数
  win_ko: number;

  /// 放铳次数
  loses: number;
  /// 放铳净点数
  lose_score: number;
  /// 放铳总点数
  lose_total_score: number;
  /// 放铳总巡目
  lose_total_junme: number;
  /// 放铳时立直次数
  lose_riichi: number;
  /// 放铳时门前次数
  lose_menzen: number;
  /// 放铳时副露次数
  lose_furo: number;
  /// 子家放铳亲家次数
  lose_ko_to_oya: number;
  /// 子家放铳子家次数
  lose_ko_to_ko: number;
  /// 亲家放铳子家次数
  lose_oya_to_ko: number;
  /// 放铳给立直者次数
  lose_to_riichi: number;
  /// 放铳给默听者次数
  lose_to_dama: number;
  /// 放铳给副露者次数
  lose_to_furo: number;

  /// 被自摸次数
  be_tsumo: number;
  /// 被自摸总点数
  be_tsumo_total_score: number;

  /// 庄家被自摸次数
  be_tsumo_oya: number;
  /// 庄家被自摸总点数
  be_tsumo_oya_total_score: number;
  /// 庄家被炸庄次数
  be_tsumo_oya_mangan: number;
  /// 庄家被炸庄总点数
  be_tsumo_oya_mangan_total_score: number;
  /// 被自摸总巡目
  be_tsumo_total_junme: number;
  /// 被自摸时立直次数
  be_tsumo_riichi: number;
  /// 被自摸时门前次数
  be_tsumo_menzen: number;
  /// 被自摸时副露次数
  be_tsumo_furo: number;
  /// 子家被亲家自摸次数
  be_tsumo_ko_to_oya: number;
  /// 子家被子家自摸次数
  be_tsumo_ko_to_ko: number;
  /// 亲家被子家自摸次数
  be_tsumo_oya_to_ko: number;

  /// 横移动次数
  no_change: number;

  /// 流局次数
  draw: number;
  /// 流局总点数
  draw_total_score: number;
  /// 流局听牌次数
  draw_tenpai: number;

  /// 立直次数
  riichi: number;
  /// 两立直次数
  riichi_double: number;
  /// 先制立直次数
  riichi_first: number;
  /// 追立直次数
  riichi_follow: number;
  /// 被追立直次数
  riichi_followed: number;
  /// 立直总巡目
  riichi_total_junme: number;
  /// 立直和了次数
  riichi_win: number;
  /// 立直放铳次数
  riichi_lose: number;
  /// 立直被自摸次数
  riichi_be_tsumo: number;
  /// 立直流局次数
  riichi_draw: number;
  /// 立直和了点数
  riichi_win_score: number;
  /// 立直总收支
  riichi_total_score: number;
  /// 一发自摸次数
  riichi_ippatsu_tsumo: number;

  /// 总收支
  total_score: number;

  /// 副露次数
  total_furo: number;

  /// 各役种出现次数
  yakus: number[];

  /// 总计R值
  tot_rate: number;
}

export function computedCounters(c: Counter) {
  return {
    basic: [
      {
        matches: c.matches,
        rounds: c.rounds,
        stableRate: Math.round((c.tot_rate / c.matches) * 40),
        roundIo: Math.round((c.total_score / c.rounds) * 100) / 100,
        netIo: Math.round((c.win_score + c.lose_score) / c.rounds),
      },
      {
        rank1Rate: percentify(c.rank1 / c.matches),
        rank2Rate: percentify(c.rank2 / c.matches),
        rank3Rate: percentify(c.rank3 / c.matches),
        rank4Rate: percentify(c.rank4 / c.matches),
        flyRate: percentify(c.tobi / c.matches),
      },
      {
        winRate: percentify(c.wins / c.rounds),
        loseRate: percentify(c.loses / c.rounds),
        drawRate: percentify(c.draw / c.rounds),
        noChangeRate: percentify(c.no_change / c.rounds),
        winLossDiff: percentify((c.wins - c.loses) / c.rounds),
      },
      {
        riichiRate: percentify(c.riichi / c.rounds),
        furoRate: percentify(c.total_furo / c.rounds),
        damaronRate: percentify(c.win_dama / c.wins),
        drawTenpaiRate: percentify(c.draw_tenpai / c.draw),
        tsumoRate: percentify(c.win_tsumo / c.wins),
      },
    ],
    winLoss: [
      {
        winRate: percentify(c.wins / c.rounds),
        tsumoRate: percentify(c.win_tsumo / c.wins),
        ronRate: percentify(c.win_ron / c.wins),
        winScore: Math.round(c.win_score / c.wins),
        winJunme: Math.round((c.win_total_junme / c.wins) * 100) / 100,
      },
      {
        winRiichiRate: percentify(c.win_riichi / c.wins),
        winDamaRate: percentify(c.win_dama / c.wins),
        winFuroRate: percentify(c.win_furo / c.wins),
        winOyaRate: percentify(c.win_oya / c.wins),
        winKoRate: percentify(c.win_ko / c.wins),
      },
      {
        loseRate: percentify(c.loses / c.rounds),
        loseScore: Math.round(c.lose_score / c.loses),
        loseJunme: Math.round((c.lose_total_junme / c.loses) * 100) / 100,
        loseRiichiRate: percentify(c.lose_riichi / c.loses),
        loseMenzenRate: percentify(c.lose_menzen / c.loses),
        loseFuroRate: percentify(c.lose_furo / c.loses),
      },
      {
        loseKoToOyaRate: percentify(c.lose_ko_to_oya / c.loses),
        loseKoToKoRate: percentify(c.lose_ko_to_ko / c.loses),
        loseOyaToKoRate: percentify(c.lose_oya_to_ko / c.loses),
        lossToRiichRate: percentify(c.lose_to_riichi / c.loses),
        lossToDamaRate: percentify(c.lose_to_dama / c.loses),
        lossToFuroRate: percentify(c.lose_to_furo / c.loses),
      },
      {
        beTsumoOyaManganRate: percentify(c.be_tsumo_oya_mangan / c.be_tsumo),
      },
      {
        beTsumoOyaManganScore: Math.round(c.be_tsumo_oya_mangan_total_score / c.be_tsumo_oya_mangan),
      },
    ],
    riichi: [
      {
        riichRate: percentify(c.riichi / c.rounds),
        dblRiichiRate: percentify(c.riichi_double / c.riichi),
        riichScore: Math.round(c.riichi_win_score / c.riichi_win),
        riichCost: Math.round((c.riichi_total_score - c.riichi_win_score) / c.riichi_lose),
        riichIo: Math.round(c.riichi_total_score / c.riichi),
      },
      {
        riichWinRate: percentify(c.riichi_win / c.riichi),
        riichLoseRate: percentify(c.riichi_lose / c.riichi),
        riichBeTsumoRate: percentify(c.riichi_be_tsumo / c.riichi),
        riichDrawRate: percentify(c.riichi_draw / c.riichi),
      },
      {
        riichFirstRate: percentify(c.riichi_first / c.riichi),
        riichFollowRate: percentify(c.riichi_follow / c.riichi),
        riichFollowedRate: percentify(c.riichi_followed / c.riichi),
        riichJunme: Math.round((c.riichi_total_junme / c.riichi) * 100) / 100,
      },
      {
        riichiIppatsuRate: percentify(c.yakus[2] / c.riichi_win),
        riichiTsumoRate: percentify(c.yakus[0] / c.riichi_win),
        riichUra: percentify(c.yakus[53] / c.riichi_win),
        riichiIppatsuTsumoRate: percentify(c.riichi_ippatsu_tsumo / c.riichi),
      },
    ],
    yakus: [
      {
        menzeTsumo: percentify(c.yakus[0] / c.wins),
        riichi: percentify(c.yakus[1] / c.wins),
        ippatsu: percentify(c.yakus[2] / c.wins),
        pinfu: percentify(c.yakus[7] / c.wins),
        tanyao: percentify(c.yakus[8] / c.wins),
      },
      {
        jikaze: percentify((c.yakus[10] + c.yakus[11] + c.yakus[12] + c.yakus[13]) / c.wins),
        bakaze: percentify((c.yakus[14] + c.yakus[15] + c.yakus[16] + c.yakus[17]) / c.wins),
        yakuhai: percentify((c.yakus[18] + c.yakus[19] + c.yakus[20]) / c.wins),
        dora: percentify(c.yakus[52] / c.wins),
        uraDora: percentify(c.yakus[53] / c.wins),
        akaDora: percentify(c.yakus[54] / c.wins),
      },
      {
        chiitoi: percentify(c.yakus[22] / c.wins),
        toitoi: percentify(c.yakus[28] / c.wins),
        honitsu: percentify(c.yakus[34] / c.wins),
        chinitsu: percentify(c.yakus[35] / c.wins),
      },
      {
        ippeiko: percentify(c.yakus[9] / c.wins),
        chanta: percentify(c.yakus[23] / c.wins),
        ittsu: percentify(c.yakus[24] / c.wins),
        sanshoku: percentify(c.yakus[25] / c.wins),
        junchan: percentify(c.yakus[33] / c.wins),
      },
    ],
  };
}

function percentify(value: number) {
  return (value * 100).toFixed(2) + "%";
}
