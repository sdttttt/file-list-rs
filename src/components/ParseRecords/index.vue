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
    RootAndDbKey
} from "@/rust";
import {
    DataTableColumns, NButton
} from "naive-ui";

defineOptions({
    name: "ParseRecords",
});

const props = withDefaults(
    defineProps<{
        data: RootAndDbKey[];
    }>(),
    {
        data: () => [],
    }
);

const emit = defineEmits<{
    (e: "selected", t: RootAndDbKey): void;
    (e: "remove", t: RootAndDbKey): void;
}>();

const columns: DataTableColumns<RootAndDbKey> = [
    {
        title: "根路径",
        key  : "root",
    },
    {
        title: "DbKey",
        key  : "dbKey",
    },
    {
        title: "Action",
        key  : "actions",
        render(row) {
            return h("span", {
                style: "display: flex; justify-content: space-around;"
            }, [
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
            ]);
        },
    },
];
</script>
