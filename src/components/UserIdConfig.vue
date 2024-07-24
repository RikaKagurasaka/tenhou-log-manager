<template>
  <div class="w-full h-full flex flex-col items-center justify-center max-w-5xl m-auto">
    <p class="text-5xl font-bold m-4 mb-16">{{ t("userIdSet.prompt") }}</p>
    <input class="input input-primary flex text-6xl h-32 w-full text-center" type="text" v-model="useridInput" spellcheck="false" autocomplete="off" @keypress.enter="confirmUserId" />
    <button class="btn btn-primary mt-8 btn-block" @click="confirmUserId">确定</button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useLocalStorage } from "@vueuse/core";
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
const userId = useLocalStorage("userId", "");
const useridInput = ref("");
const guessed = ref(false);
const { t } = useI18n();

onMounted(async () => {
  let rs = await invoke<string>("guess_user_id", {});
  if (rs) {
    useridInput.value = rs;
    guessed.value = true;
  }
});

function confirmUserId() {
  userId.value = useridInput.value;
}
</script>

<style scoped></style>
