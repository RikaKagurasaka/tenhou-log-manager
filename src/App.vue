<template>
  <div class="grid">
    <div class="navbar">
      <ul>
        <li>
          <RouterLink to="/import" :class="{ active: route.path.startsWith('/import') }"><font-awesome-icon :icon="faFileImport" />{{ t("navbar.import") }}</RouterLink>
        </li>
        <li>
          <RouterLink to="/data" :class="{ active: route.path.startsWith('/data') }"><font-awesome-icon :icon="faChartSimple" />{{ t("navbar.data") }}</RouterLink>
        </li>
        <li>
          <RouterLink to="/about" :class="{ active: route.path.startsWith('/about') }"><font-awesome-icon :icon="faInfoCircle" />{{ t("navbar.about") }}</RouterLink>
        </li>
      </ul>
      <div class="flex-1"></div>
      <div class="flex">
        <ThemeChange />
        <LanguageChange />
      </div>
    </div>
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faFileImport, faChartSimple, faInfoCircle } from "@fortawesome/free-solid-svg-icons";
import { useI18n } from "vue-i18n";
import { RouterLink, useRoute } from "vue-router";
import ThemeChange from "./components/ThemeChange.vue";
import LanguageChange from "./components/LanguageChange.vue";
const route = useRoute();
const { t } = useI18n();
</script>

<style scoped>
.grid {
  @apply w-full h-full items-stretch;
  grid-template-columns: 180px 1fr;
  grid-template-rows: 100%;
}
.navbar {
  @apply flex flex-col h-full bg-base-200;
  & > ul {
    @apply menu rounded-box w-full flex-1 overflow-ellipsis whitespace-nowrap;
    & > li {
      @apply w-full text-lg py-1;
      & > a {
        @apply py-4;
      }
    }
  }
}
</style>
