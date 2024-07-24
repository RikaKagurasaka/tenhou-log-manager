<template>
  <div class="p-8">
    <p class="flex items-baseline">
      <img :src="logo" class="w-32 h-32" />
      <span class="text-6xl font-bold">{{ t("window.title") }}</span>
    </p>
    <div class="divider my-4"></div>
    <div class="info-lists" style="grid-template-columns: min-content 1fr">
      <label>Author </label>
      <span> Rika </span>
      <label>Version</label>
      <span>{{ version }}</span>
      <label>License</label>
      <span>MIT</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { useI18n } from "vue-i18n";
import logo from "@/assets/logo.png";
import { getVersion } from "@tauri-apps/api/app";
import { useAsyncState } from "@vueuse/core";
const { state: version } = useAsyncState(getVersion(), "...");
const { t } = useI18n();
</script>

<style scoped>
.info-lists {
  @apply grid gap-2 text-lg;
  & > * {
    @apply p-2;
  }
  & > label{
    @apply font-bold;
  }
}
</style>
