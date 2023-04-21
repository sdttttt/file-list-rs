import {
    Dir, HistoryRecordItem
} from "@/rust";
import {
    ParseMode, SystemPat
} from "@/types";
import {
    defineStore
} from "pinia";
import {
    ref
} from "vue";

export const useCurrentRecordStore = defineStore("current-record", () => {
    const record = ref<HistoryRecordItem>();

    const dbKey = computed(() => record.value.dbKey);

    // 文件系统的间隔符
    const systemPat = computed(() => {
        switch (record.value.command) {
        case ParseMode.DirS: {
            return SystemPat.Windows;
        }
        case ParseMode.LsALHR: {
            return SystemPat.Unix;
        }
        }
    });

    function updateCurrentRecord(t: HistoryRecordItem) {
        record.value = t;
    }

    return {
        record,
        updateCurrentRecord,
        dbKey,
        systemPat,
    };
});
