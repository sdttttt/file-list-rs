<template>
  <n-layout>
    <n-layout-header>
      <n-card>
        <template #header>
          {{ currentDir?.n || "未选择文件夹" }}
        </template>
        大小: {{ currentDir?.s || "无" }}
      </n-card>
    </n-layout-header>
    <n-layout-content>
      <n-data-table
        :columns="columns"
        size="small"
        :data="dirData"
        :bordered="false"
      />
    </n-layout-content>
  </n-layout>
</template>

<script lang="ts" setup>
import {
    useDirViewStore
} from "@/store";
import {
    storeToRefs
} from "pinia";
import {
    computed, h
} from "vue";
import {
    DataTableColumns
} from "naive-ui";
import {
    Folder, FolderOpenOutline
} from "@vicons/ionicons5";
import {
    NIcon, TreeOption
} from "naive-ui";

type DataItem = { path: string; size?: string; time?: string };

const columns: DataTableColumns<DataItem> = [
    {
        title : "名称",
        key   : "path",
        render: row => {
            if (!row.size && !row.time) {
                return h(
                    "span",
                    {
                        style: "display: inline-flex; align-items:center;",
                    },
                    [
                        h(
                            NIcon,
                            {
                                style: "margin-right: 7px; ",
                            },
                            {
                                default: () => h(Folder),
                            }
                        ),
                        h("div", null, row.path),
                    ]
                );
            }

            return row.path;
        },
    },
    {
        title: "大小",
        key  : "size",
    },
    {
        title: "修改时间",
        key  : "time",
    },
];

const {
    currentDir
} = storeToRefs(useDirViewStore());

const dirData = computed((): DataItem[] => {
    if (!currentDir.value) {
        return [];
    }
    const dirs = currentDir.value.d.map(t => ({
        path: t.n.split("\\").reverse()[0],
    }));
    const files = currentDir.value.f.map(t => ({
        path: t.n,
        size: t.s,
        time: t.t,
    }));

    return [...dirs, ...files];
});
</script>
