<template>
  <UserIdConfig v-if="!userId" />
  <template v-else>
    <DataComponent />
  </template>
</template>

<script setup lang="ts">
import { useLocalStorage } from "@vueuse/core";
import UserIdConfig from "../components/UserIdConfig.vue";
import DataComponent from "../components/DataComponent.vue";
const userId = useLocalStorage("userId", "");
</script>

<style scoped></style>
