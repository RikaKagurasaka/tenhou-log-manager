import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.yml";
import jaJP from "./ja-JP.yml";
export const i18n = createI18n({
  legacy: false,
  locale: "zh-CN",
  messages: {
    "zh-CN": zhCN,
    "ja-JP": jaJP,
  },
});
