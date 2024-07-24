<template>
  <label class="swap swap-rotate">
    <!-- this hidden checkbox controls the state -->
    <input type="checkbox" :checked="theme == 'light'" @click="click" />
    <!-- sun icon -->
    <FontAwesomeIcon class="swap-on h-8 w-8" :icon="faSun" />
    <!-- moon icon -->
    <FontAwesomeIcon class="swap-off h-8 w-8" :icon="faMoon" />
  </label>
</template>

<script setup lang="ts">
import { faMoon, faSun } from "@fortawesome/free-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { useLocalStorage } from "@vueuse/core";
import { watch } from "vue";

const theme = useLocalStorage("theme", window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");
document.documentElement.setAttribute("data-theme", theme.value);
watch(theme, (value) => {
  document.documentElement.setAttribute("data-theme", value);
});

function click() {
  theme.value = theme.value == "light" ? "dark" : "light";
}
</script>

<style scoped></style>
