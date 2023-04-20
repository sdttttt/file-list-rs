<template>
  <NProdiver>
    <div
      flex-1
      flex-row
    >
      <Main />
    </div>
  </NProdiver>
</template>

<script setup lang="ts">
import Main from "@/views/main/index.vue";
import NProdiver from "./NProdiver.vue";
</script>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
