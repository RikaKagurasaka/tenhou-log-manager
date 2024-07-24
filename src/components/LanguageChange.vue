<template>
  <div class="dropdown dropdown-top">
    <div tabindex="0" role="button" class="btn m-1">{{ locale2letters(locale) }}</div>
    <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] w-full shadow p-1">
      <template v-for="l in availableLocales" :key="l">
        <li v-if="l !== locale">
          <a @click="locale = l">{{ locale2letters(l) }}</a>
        </li>
      </template>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { window } from "@tauri-apps/api";
import { useLocalStorage } from "@vueuse/core";
import { onMounted, Ref, toValue, watch } from "vue";
import { useI18n } from "vue-i18n";

const { t, locale, availableLocales } = useI18n();
const localeLs = useLocalStorage("locale", locale);
locale.value = localeLs.value;
onMounted(() => {
  document.documentElement.lang = locale.value;
  window.Window.getCurrent().setTitle(t("window.title"));
});
watch(locale, (value) => {
  document.documentElement.lang = value;
  window.Window.getCurrent().setTitle(t("window.title"));
});
const locale2letters = (locale: string | Ref<string>) => toValue(locale).substring(0, 2).toUpperCase();
</script>

<style scoped></style>
