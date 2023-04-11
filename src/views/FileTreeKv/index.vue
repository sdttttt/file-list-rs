<template>
  <n-space vertical>
    <n-button
      v-if="showFinder"
      type="warning"
      size="small"
      @click="() => (showFinder = false)"
    >
      关闭查询树
    </n-button>
    <n-button
      v-else
      size="small"
      type="info"
      @click="handleOpenFinderForm"
    >
      查找
    </n-button>

    <n-tree
      :data="showFinder ? finderDirTree : treeView"
      :node-props="treeNodeProps"
      @load="handleLoadDir"
      @update:expanded-keys="updatePrefixWithExpaned"
    />
  </n-space>
  <n-modal
    v-model:show="showFinderForm"
    preset="dialog"
    title="查找"
    positive-text="确认"
    negative-text="算了"
    @positive-click="handleFind"
  >
    <FileTreeFinderForm v-model:data="finderForm" />
  </n-modal>

  <n-modal
    v-model:show="showFinderFileResult"
    preset="dialog"
    title="符合结果的文件路径"
  >
    <n-list>
      <n-list-item
        v-for="t in finderFileResult"
        :key="t"
      >
        {{ t }}
      </n-list-item>
    </n-list>
  </n-modal>
</template>

<script lang="ts" setup>
import {
    useTreeView,
    useFinder,
    handleLoadDir,
    treeNodeProps,
    updatePrefixWithExpaned,
} from "./states";
import FileTreeFinderForm from "@/components/FileTreeFinderForm/index.vue";
const props = defineProps<{
    root: string;
    dbKey: string;
}>();

const {
    treeView
} = useTreeView(props);
const {
    showFinder,
    handleOpenFinderForm,
    finderForm,
    showFinderFileResult,
    handleFind,
    finderFileResult,
    finderDirTree,
    showFinderForm,
} = useFinder();
</script>
