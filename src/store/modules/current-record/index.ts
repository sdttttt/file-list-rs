import {
    Dir, HistoryRecordItem
} from "@/rust";
import {
    defineStore
} from "pinia";
import {
    ref
} from "vue";

export const useCurrentRecordStore = defineStore("current-record", () => {
    const record = ref<HistoryRecordItem>(null);

    const dbKey = computed(() => record.value.dbKey);
    function updateCurrentRecord(t: HistoryRecordItem) {
        record.value = t;
    }

    return {
        record,
        updateCurrentRecord,
        dbKey,
    };
});
