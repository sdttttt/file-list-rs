<template>
  <NProdiver>
    <div class="container">
      <Main />
    </div>
  </NProdiver>
</template>

<script setup lang="ts">
import Main from "@/views/main/index.vue";
import NProdiver from "./NProdiver.vue";
</script>

<style scoped>
.main-view {
    display: flex;
    flex-direction: row;
}

.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
