<template>
  <div pt-4>
    <n-form
      ref="formRef"
      :label-width="80"
      :model="data"
      :rules="rules"
    >
      <n-form-item
        label="别名"
        path="name"
      >
        <n-input
          v-model:value="data.name"
          placeholder="本次解析别名, 唯一"
        />
      </n-form-item>

      <n-form-item label="本地文件">
        <div
          w-100
          flex
          flex-row
          justify-between
        >
          <n-input
            v-model:value="data.path"
            placeholder="注意文件必须是UTF-8格式"
            disabled
          />
          <n-button
            ml-2
            @click="handleOpenFileSelector"
          >
            选择
          </n-button>
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
import {
    reactive, watch
} from "vue";
import {
    FileSelectForm, ParseBackend, ParseMode
} from "@/types";
import {
    SelectOption
} from "naive-ui";
import {
    open
} from "@tauri-apps/api/dialog";

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

const typeOptions: SelectOption[] = [
    {
        label: ParseMode.DirS,
        value: ParseMode.DirS,
    },

    {
        label: ParseMode.LsALHR,
        value: ParseMode.LsALHR,
    },
];

const backendOptions = [
    {
        label   : "内存模式 (消耗内存)",
        value   : ParseBackend.Mem,
        disabled: true,
    },

    {
        label: "Sled模式 (消耗硬盘)",
        value: ParseBackend.Sled,
    },
];

const rules = {
    name: {
        required: true,
        message : "必须填写一个别名",
        trigger : "blur"
    },
};

async function handleOpenFileSelector() {
    const selected = await open({
        multiple: false,
    });
    data.path = selected as string;
}

watch(data, v => emit("update:data", v));
</script>
