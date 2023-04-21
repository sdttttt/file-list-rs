<template>
  <n-space vertical>
    <n-grid
      x-gap="6"
      :cols="2"
    >
      <n-gi>
        <n-button
          v-if="showFinder"
          strong
          secondary
          type="warning"
          style="width: 100%"
          @click="() => (showFinder = false)"
        >
          关闭查询树
        </n-button>
        <n-button
          v-else
          type="info"
          strong
          secondary
          style="width: 100%"
          @click="handleOpenFinderForm"
        >
          查找
        </n-button>
      </n-gi>
    </n-grid>

    <n-tree
      :data="showFinder ? finderDirTree : treeView"
      :node-props="treeNodeProps"
      block-line
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
    preset="card"
    title="符合结果的文件路径"
    style="width: 800px"
  >
    <n-space vertical>
      <n-input
        v-model:value="finderFileFilterKeyword"
        style="width: 300px"
        show-count
        placeholder="过滤"
        clearable
      />
      <n-scrollbar style="max-height: 600px">
        <n-table striped>
          <thead>
            <tr>
              <th>路径</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="t in finderFileList"
              :key="t"
            >
              <td>{{ t }}</td>
            </tr>
          </tbody>
        </n-table>
      </n-scrollbar>
    </n-space>
  </n-modal>
</template>

<script lang="ts" setup>
import {
    handleLoadDirFunc,
    treeNodePropsFunc,
    updatePrefixWithExpaned,
    useFinder,
    useTreeView,
} from "./states";
import FileTreeFinderForm from "@/components/FileTreeFinderForm/index.vue";

const {
    treeView
} = useTreeView();

const handleLoadDir = handleLoadDirFunc();
const treeNodeProps = treeNodePropsFunc();
const {
    showFinder,
    handleOpenFinderForm,
    finderForm,
    showFinderFileResult,
    finderFileList,
    handleFind,
    finderFileFilterKeyword,
    finderDirTree,
    showFinderForm,
} = useFinder();
</script>
