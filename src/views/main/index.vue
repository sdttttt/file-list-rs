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
        <n-button
          type="primary"
          style="width: 100%"
          @click="handleOpenFileSelector"
        >
          解析文件
        </n-button>
        <FileTreeMem
          v-if="backendMode == ParseBackend.Mem"
          :data="fileTree"
        />
        <FileTreeKv
          v-if="backendMode == ParseBackend.Sled"
          :root="root"
          :db-key="dbKey"
        />
      </n-space>
    </n-layout-sider>
    <n-layout
      content-style="padding: 0rem;"
    >
      <DirView />
    </n-layout>
  </n-layout>

  <n-modal
    v-model:show="showFileSelection"
    preset="dialog"
    title="文件选择"
    positive-text="解析"
    negative-text="算了"
    @positive-click="handleParseFileByPath"
  >
    <file-selector-form v-model:data="fileSelectionForm" />
  </n-modal>
</template>

<script lang="ts" setup>
import FileSelectorForm from "@/components/FileSelectorForm/index.vue";
import FileTreeMem from "@/views/FileTreeMem/index.vue";
import FileTreeKv from "@/views/FileTreeKv/index.vue";
import DirView from "@/views/DirView/index.vue";
import {
    useFileSelector
} from "./states";
import {
    ParseBackend
} from "@/types";
defineOptions({
    name: "Main",
});

const {
    showFileSelection,
    handleOpenFileSelector,
    fileSelectionForm,
    fileTree,
    handleParseFileByPath,
    backendMode,
    root,
    dbKey,
} = useFileSelector();
</script>
