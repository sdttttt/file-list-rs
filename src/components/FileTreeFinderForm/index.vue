<template>
 <div pt-4>
    <n-form
      ref="formRef"
      :label-width="80"
      :model="data"
    >
    
    <n-form-item label="本地文件">
        <n-input v-model:value="data.keyword"></n-input>
    </n-form-item>

      <n-form-item label="解析命令">
        <n-select
         v-model:value="data.findType"
          :options="typeOptions"
        />
      </n-form-item>
    </n-form>
  </div>

</template>
<script lang="ts" setup>
import { FileTreeFindForm } from '@/types';
import { reactive, watch } from 'vue';


defineOptions({
    name: "FileTreeFinderForm",
});

const props = defineProps<{
    data: FileTreeFindForm
}>();

// 解除响应式，防止改变父组件
const data = reactive({
    ...props.data
});

const emit = defineEmits<{
    (e: "update:data", v: FileTreeFindForm): void;
}>();

const typeOptions = [
    {
        label: "文件夹",
        value:"dir",
    },
    {
        label: "文件",
        value:"file",
    },
];


watch(data, v => emit("update:data", v));
</script>
