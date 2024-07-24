<template>
  <div class="p-8">
    <p class="text-lg">{{ t("importView.prompt") }}</p>
    <textarea class="textarea textarea-bordered w-full mt-4 p-2 py-4 bg-base-200 drop-shadow-sm h-1/3" v-model="textContent"></textarea>
    <button class="btn btn-primary mt-4 btn-block" :disabled="matchedCounts == 0 || downloading" @click="downloadLogs">
      <template v-if="!downloading">
        {{ t("importView.btnText", { plural: matchedCounts, count: matchedCounts }) }}
      </template>
      <span v-else class="loading loading-spinner"></span>
    </button>
    <div class="divider"></div>
    <p class="text-2xl font-bold">{{ t("importView.autoImportTitle") }}</p>
    <p class="text-lg">
      {{ t("importView.autoImportDesc") }}
    </p>
    <button class="btn btn-info mt-4 btn-block" @click="scanLocalLogs">
      {{ t("importView.autoImportBtnText") }}
    </button>
    <p class="text-sm text-info">
      {{ t("importView.autoImportHint") }}
    </p>
  </div>

  <dialog id="importModal" class="modal">
    <div class="modal-box">
      <p class="py-4" v-if="!downlodedResults[1]">
        <p>{{ t("importView.importModalText.success",{count:downlodedResults[0][0]}) }}</p>
        <p>{{ t("importView.importModalText.skip",{count:downlodedResults[0][1]}) }}</p>
        <p>{{ t("importView.importModalText.fail",{count:downlodedResults[0][2]}) }}</p>
      </p>
      <p class="py-4" v-else>
        {{ t("importView.importModalError", { msg: downlodedResults[1] }) }}
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

const reg = /\d{10}gm-\d{4}-\d{4}-[0-9a-f]{8}/g;
const textContent = ref("");
const downloading = ref(false);
const downlodedResults = ref([[0, 0, 0], ""]);
const { t } = useI18n();

const matchedCounts = computed(() => {
  return [...textContent.value.matchAll(reg)].length;
});

async function scanLocalLogs() {
  textContent.value = await invoke<string>("scan_local_logs", {});
}

async function downloadLogs() {
  downloading.value = true;
  downlodedResults.value = [[0, 0, 0], ""];
  try {
    downlodedResults.value = await invoke("download_logs", { ids: [...textContent.value.matchAll(reg)].join("\n") });
  } finally {
    downloading.value = false;
    // @ts-ignore
    importModal.showModal();
  }
}
</script>

<style scoped>
p {
  @apply py-2;
}
</style>
