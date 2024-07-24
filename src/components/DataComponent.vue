<template>
  <div class="m-auto h-full flex flex-col" style="max-width: 90rem">
    <p class="text-4xl font-bold m-4">
      <span class="relative">
        <span class="absolute left-0 w-full h-4 bottom-0 bg-info opacity-30 -z-10"></span>
        <span class="px-1">{{ userId }}</span>
      </span>
      <button class="btn btn-ghost text-sm" @click="userId = ``">
        <FontAwesomeIcon :icon="faEdit" />
      </button>
    </p>
    <div class="grid grid-cols-1 p-4 gap-4 overflow-auto flex-1" v-if="calculated">
      <div class="card" v-for="(o, key) in calculated">
        <div>
          {{ t(`data.${key}._title`) }}
        </div>
        <div class="grid-cols-4">
          <div v-for="(v, k) in o" :key="k">
            <template v-for="(vv, kk) in v">
              <span>{{ t(`data.${key}.${kk}`) }}</span>
              <span>{{ vv }}</span>
            </template>
          </div>
        </div>
      </div>
    </div>
    <div v-else class="flex-1 flex items-center justify-center max-w-full" style="width: 90em">
      <span class="loading loading-lg"></span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAsyncState, useLocalStorage } from "@vueuse/core";
import { useI18n } from "vue-i18n";
import { computedCounters, Counter } from "../stores/counter";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faEdit } from "@fortawesome/free-solid-svg-icons";
import { invoke } from "@tauri-apps/api/core";
const { t } = useI18n();
const userId = useLocalStorage("userId", "");
let { state: calculated } = useAsyncState(
  invoke<Counter>("parse_logs", { id: userId.value }).then((c) => computedCounters(c)),
  null
);
</script>

<style scoped>
.card {
  @apply bg-base-200 w-full shadow-xl p-2;
  & > :nth-child(1) {
    @apply card-title m-4 text-2xl;
  }
  & > :nth-child(2) {
    @apply grid text-base font-normal items-start;
    & > div {
      @apply grid grid-cols-2;
      & > * {
        @apply mx-1 my-1;
      }
      & > :nth-child(odd) {
        @apply text-left font-bold ml-8;
      }
      & > :nth-child(even) {
        @apply text-right mr-8;
      }
    }
  }
}
</style>
