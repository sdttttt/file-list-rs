<template>

    <n-tree
      :data="treeView? [treeView]: []"
      @load="handleLoadDir"
      style="max-height: 1000px"
    />
</template>

<script lang="ts" setup>
import { Dir } from "@/rust"
import { useTreeView } from "./states";
const props = defineProps<{
    data?: Dir
}>();

const { treeView, handleLoadDir } = useTreeView(props);

</script>
