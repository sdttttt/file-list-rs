<template>
  <n-layout>
    <n-layout-header ref="headerEl">
      <n-card>
        <template #header>
          {{ currentDir?.n || "未选择文件夹" }}
        </template>
        <n-space>
          <span>
            大小: {{ currentDir?.s || "无" }}
          </span>
          <span>
            条目: {{ dirData.length || 0 }}
          </span>
        </n-space>
      </n-card>
    </n-layout-header>
    <n-layout-content>
      <n-data-table
        :columns="dirSColumns"
        size="small"
        :data="dirData"
        :bordered="false"
        :max-height="tableHeight"
      />
    </n-layout-content>
  </n-layout>
</template>

<script lang="ts" setup>
import {
    useDirViewStore
} from "@/store";
import {
    storeToRefs
} from "pinia";
import {
    computed, h, ref, unref, watch
} from "vue";
import dirSColumns, {
    DataItem
} from "./dir-s-columns";
import {
    useElementSize, useWindowSize
} from "@vueuse/core";

const headerEl = ref(null);
const {
    height: headerHeight
} = useElementSize(headerEl);
const {
    height: windowHeight
} = useWindowSize();

const tableHeight = computed(
    () => windowHeight.value - headerHeight.value - 40
);

watch(tableHeight, v => {
    console.log(`headerHeight: ${headerHeight.value}`);
    console.log(`windowHeight:${windowHeight.value}`);
    console.log(`tableHeight:${v}`);
});

const {
    currentDir
} = storeToRefs(useDirViewStore());

const dirData = computed((): DataItem[] => {
    if (!currentDir.value) {
        return [];
    }
    const dirs = currentDir.value.d.map(t => ({
        path: t.n.split("\\").reverse()[0],
    }));
    const files = currentDir.value.f.map(t => ({
        path: t.n,
        size: t.s,
        time: t.t,
    }));

    return [...dirs, ...files];
});
</script>
