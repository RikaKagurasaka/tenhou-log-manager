import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.yml";
import jaJP from "./ja-JP.yml";
import enUs from "./en-US.yml";
export const i18n = createI18n({
  legacy: false,
  locale: navigator.language,
  fallbackLocale: "en-US",
  messages: {
    "zh-CN": zhCN,
    "ja-JP": jaJP,
    "en-US": enUs,
  },
});
