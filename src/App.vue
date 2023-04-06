
<template>
    <div class="container">
    <n-layout position="absolute" has-sider >
      <n-layout-sider
        content-style="padding: 24px;"
        :native-scrollbar="false"
        bordered
      >
        <n-button>打开</n-button>
      </n-layout-sider>
      <n-layout content-style="padding: 2rem;" :native-scrollbar="false">
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
        <n-h2>平山道</n-h2>
      </n-layout>
    </n-layout>
</div>
</template>

<script setup lang="ts">
import {  greetRust  } from "./rust";
import { onMounted } from "vue";

onMounted(async () => {
    const aa = await greetRust();
    console.log(aa);
});

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
