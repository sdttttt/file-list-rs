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
        :columns="columns"
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
    useDirViewStore,
    useCurrentRecordStore
} from "@/store";
import {
    storeToRefs
} from "pinia";
import dirSColumns from "./dir-s-columns";
import lsAlhrColumns from "./ls-alhr-columns";
import {
    DataItem
} from "./types";
import {
    useElementSize, useWindowSize
} from "@vueuse/core";
import {
    ParseMode
} from "@/types";

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
    currentDir,
} = storeToRefs(useDirViewStore());

const {
    systemPat,
    record
} = storeToRefs(useCurrentRecordStore());

const columns = computed(() => {
    switch (record.value?.command) {
    case ParseMode.DirS: {
        return dirSColumns;
    }
    case ParseMode.LsALHR: {
        return lsAlhrColumns;
    }
    default: {
        return dirSColumns;
    }
    }
});

const dirData = computed((): DataItem[] => {
    if (!currentDir.value) {
        return [];
    }
    const dirs = currentDir.value.d.map((t): DataItem => ({
        path: t.n.split(systemPat.value).reverse()[0],
    }));
    const files = currentDir.value.f.map((t): DataItem => ({
        path: t.n,
        size: t.s,
        time: t.t,

        chmod: t.c,
        user : t.u,
        group: t.g,
    }));

    return [...dirs, ...files];
});
</script>
