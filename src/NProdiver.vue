<template>
  <n-config-provider wh-full>
    <n-loading-bar-provider>
      <n-dialog-provider>
        <n-notification-provider>
          <n-message-provider>
            <slot />
            <NaiveProviderContent />
          </n-message-provider>
        </n-notification-provider>
      </n-dialog-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<script lang="ts" setup>
import {
    defineComponent, h
} from "vue";
import {
    useDialog,
    useLoadingBar,
    useMessage,
    useNotification,
} from "naive-ui";

import {
    setupDialog, setupMessage
} from "@/utils/common/index";

defineOptions({
    name: "AppProvider",
});

// 挂载naive组件的方法至window, 以便在全局使用
function setupNaiveTools() {
    window.$loadingBar = useLoadingBar();
    window.$notification = useNotification();

    // @ts-ignore
    window.$message = setupMessage(useMessage());
    window.$dialog = setupDialog(useDialog());
}

const NaiveProviderContent = defineComponent({
    setup() {
        setupNaiveTools();
    },
    render() {
        return h("div");
    },
});
</script>
