<template>
    <div pt-4>
    <n-form
      ref="formRef"
      :label-width="80"
      :model="data"
    >
    
    <n-form-item label="本地文件">
        <div w-100 flex flex-row  justify-between>
        <n-input v-model:value="data.path" placeholder="路径(注意文件必须是UTF-8格式，否则无法解析)" disabled>
        </n-input>
        <n-button ml-2 @click="handleOpenFileSelector">选择</n-button>
        </div>
    </n-form-item>

      <n-form-item label="解析命令">
        <n-select
         v-model:value="data.command"
          :options="typeOptions"
        />
      </n-form-item>

      <n-form-item label="解析后端">
        <n-select
         v-model:value="data.backend"
          :options="backendOptions"
        />
      </n-form-item>
    </n-form>
  </div>
</template>

<script lang="ts" setup>
import {  reactive, watch } from "vue";
import { FileSelectForm, ParseBackend, ParseMode } from "@/types";
import { UploadFileInfo } from "naive-ui"
import { open } from '@tauri-apps/api/dialog';

defineOptions({
    name: "FileSelectorForm",
});

const props = defineProps<{
    data: FileSelectForm;
}>();

// 解除响应式，防止改变父组件
const data = reactive({
    ...props.data,
});

const emit = defineEmits<{
    (e: "update:data", v: FileSelectForm): void;
}>();

const typeOptions = [
    {
        label: "dir /s *.*",
        value: ParseMode.DirS,
    },
];

const backendOptions = [
    {
        label: "内存模式 (消耗内存)",
        value: ParseBackend.Mem,
    },

    {
        label: "Sled模式 (消耗硬盘)",
        value: ParseBackend.Sled,
    }
];

async function handleOpenFileSelector()
{
   const selected = await open({
        multiple: false,
    });

    data.path = <string>selected;
}

watch(data, v => emit("update:data", v));

</script>
