<template>
  <n-layout
    position="absolute"
    has-sider
  >
    <n-layout-sider
      content-style="padding: 24px;"
      collapse-mode="width"
      :collapsed-width="300"
      :width="800"
      default-collapsed
      :native-scrollbar="false"
      show-trigger="arrow-circle"
      bordered
    >
      <n-space vertical>
        <n-grid
          x-gap="6"
          :cols="2"
        >
          <n-gi>
            <n-button
              type="primary"
              strong
              style="width: 100%"
              @click="handleOpenFileSelector"
            >
              解析文件
            </n-button>
          </n-gi>

          <n-gi>
            <n-button
              strong
              style="width: 100%"
              @click="handleOpenHistory"
            >
              历史记录
            </n-button>
          </n-gi>
        </n-grid>

        <FileTreeKv />
      </n-space>
    </n-layout-sider>

    <n-layout content-style="padding: 0rem;">
      <DirView />
    </n-layout>
  </n-layout>

  <n-modal
    v-model:show="showFileSelection"
    preset="dialog"
    title="文件解析"
    positive-text="解析"
    negative-text="算了"
    @positive-click="handleParseFileByPath"
  >
    <file-selector-form v-model:data="fileSelectionForm" />
  </n-modal>

  <n-modal
    v-model:show="showHistoryList"
    preset="card"
    title="解析记录"
    negative-text="关闭"
    style="width: 800px;"
    @negative-click="() => (showHistoryList = false)"
  >
    <ParseRecords
      :data="historyList"
      @selected="handleRecover"
      @remove="handleRemoveHistory"
    />
  </n-modal>
</template>

<script lang="ts" setup>
import FileSelectorForm from "@/components/FileSelectorForm/index.vue";
import FileTreeKv from "@/views/FileTreeKv/index.vue";
import DirView from "@/views/DirView/index.vue";
import ParseRecords from "@/components/ParseRecords/index.vue";
import {
    useMainView
} from "./states";
defineOptions({
    name: "Main",
});

const {
    useFileSelector,
    useHistory,
} = useMainView();

const {
    showFileSelection,
    handleOpenFileSelector,
    fileSelectionForm,
    handleParseFileByPath,
} = useFileSelector();

const {
    showHistoryList,
    historyList,
    handleOpenHistory,
    handleRemoveHistory,
    handleRecover,
} = useHistory();
</script>
