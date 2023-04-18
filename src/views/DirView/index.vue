<template>
  <n-layout>
    <n-layout-header ref="headerEl">
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
        :max-height="tableHeight"
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
    computed, h, ref, unref, watch
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
import {
    useElementSize, useWindowSize
} from "@vueuse/core";

const headerEl = ref(null);
const {
    height: headerHeight
} = useElementSize(headerEl);
const {
    height: windowHeight
} = useWindowSize();

const tableHeight = computed(
    () => windowHeight.value - headerHeight.value - 40
);

watch(tableHeight, v => {
    console.log(`headerHeight: ${headerHeight.value}`);
    console.log(`windowHeight:${windowHeight.value}`);
    console.log(`tableHeight:${v}`);
});

type DataItem = { path: string; size?: string; time?: string };

const columns: DataTableColumns<DataItem> = [
    {
        title : "名称",
        key   : "path",
        sorter: "default",
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
        filterOptions: [
            {
                label: "文件夹",
                value: "dir"
            },
            {
                label: "文件",
                value: "file"
            }
        ],
        filter: (value, row) => {
            if ("dir" === value) {
                return !row.size && !row.time;
            }
            if ("file" === value) {
                return !(!row.size && !row.time);
            }

            return true;
        }
    },
    {
        title : "大小",
        key   : "size",
        sorter: (row1, row2) => {
            row1.size = row1.size || "";
            row2.size = row2.size || "";
            const size1 = Number(row1.size?.split("").filter(t => /\d/g.test(t)).join(""));
            const size2 = Number(row2.size?.split("").filter(t => /\d/g.test(t)).join(""));
            if (!size1) {
                return -1;
            }

            return size1 - size2;
        },
    },
    {
        title : "修改时间",
        key   : "time",
        sorter: (row1, row2) => {
            row1.time = row1.time || "";
            row2.time = row2.time || "";
            if ("" === row1.time) {
                return -1;
            }

            const time1 = new Date(row1.time);
            const time2 = new Date(row2.time);

            return time1.valueOf() - time2.valueOf();
        },
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
