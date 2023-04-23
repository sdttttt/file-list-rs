<template>
  <div
    pos-fixed
    z-20
    top-2
    right-2
  >
    <n-button
      quaternary
      circle
      @click="() => store.themeSwitch()"
    >
      <template #icon>
        <n-icon>
          <MoonOutline v-if="store.isDark" />
          <SunnyOutline v-else />
        </n-icon>
      </template>
    </n-button>
  </div>
</template>

<script lang="ts" setup>
import {
    useAppStore
} from "@/store";
import {
    MoonOutline,
    SunnyOutline,
} from "@vicons/ionicons5";

const store = useAppStore();

</script>
