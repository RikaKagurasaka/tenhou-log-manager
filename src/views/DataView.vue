<template>
  <UserIdConfig v-if="!userId" />
  <template v-else>
    <DataComponent v-bind="counterStore.counter"  />
  </template>
</template>

<script setup lang="ts">
import { useLocalStorage } from "@vueuse/core";
import UserIdConfig from "../components/UserIdConfig.vue";
import { useCounterStore } from "../stores/counter";
import { onMounted, watch } from "vue";
import DataComponent from "../components/DataComponent.vue";
const userId = useLocalStorage("userId", "");
const counterStore = useCounterStore();
watch(counterStore, ({ counter }) => {
  console.log(counter);
});
</script>

<style scoped></style>
