<template>
    <div v-if="!treeView">
        空文件树
    </div>

    <n-tree
      v-if="treeView"
      :data="[treeView]"
      @load="handleLoadDir"
      style="max-height: 1000px"
    />
</template>

<script lang="ts" setup>
import {Dir} from "@/rust"
import { useTreeView } from "./states";
const props = defineProps<{
    data?: Dir
}>();

const { treeView, handleLoadDir } = useTreeView(props);

</script>
