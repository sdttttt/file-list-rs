<template>
  <n-data-table
    :columns="columns"
    :data="props.data"
  />
</template>

<script lang="ts" setup>
import {
    h
} from "vue";
import {
    HistoryRecordItem
} from "@/rust";
import {
    DataTableColumns, NButton
} from "naive-ui";

defineOptions({
    name: "ParseRecords",
});

const props = withDefaults(
    defineProps<{
        data: HistoryRecordItem[];
    }>(),
    {
        data: () => [],
    }
);

const emit = defineEmits<{
    (e: "selected", t: HistoryRecordItem): void;
    (e: "remove", t: HistoryRecordItem): void;
}>();

const columns: DataTableColumns<HistoryRecordItem> = [
    {
        title: "别名",
        key  : "name",
    },
    {
        title: "解析命令",
        key  : "command",
    },
    {
        title: "根路径",
        key  : "root",
    },
    {
        title: "Action",
        key  : "actions",
        render(row) {
            return h(
                "span",
                {
                    style: "display: flex; justify-content: space-around;",
                },
                [
                    h(
                        NButton,
                        {
                            strong  : true,
                            tertiary: true,
                            onClick : () => {
                                emit("selected", row);
                            },
                        },
                        {
                            default: () => "恢复",
                        }
                    ),

                    h(
                        NButton,
                        {
                            strong  : true,
                            tertiary: true,
                            type    : "error",
                            onClick : () => {
                                emit("remove", row);
                            },
                        },
                        {
                            default: () => "删除",
                        }
                    ),
                ]
            );
        },
    },
];
</script>
