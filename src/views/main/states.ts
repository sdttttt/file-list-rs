import {
    ref
} from "vue";
import {
    FileSelectForm, ParseBackend, ParseMode
} from "@/types";
import {
    Dir,
    HistoryRecordItem,
    kvParse,
    memParse,
    parseRecords,
    removeRecord,
    unwrap,
} from "@/rust";
import {
    useCurrentRecordStore
} from "@/store/modules/current-record";

export function useMainView() {

    const currentRecordStore = useCurrentRecordStore();

    function useFileSelector() {
        const showFileSelection = ref(false);

        const fileSelectionForm = ref<FileSelectForm>({
            name   : "",
            path   : "",
            command: ParseMode.DirS,
            backend: ParseBackend.Sled,
        });

        const backendMode = ref<ParseBackend>(ParseBackend.Sled);

        function handleOpenFileSelector() {
            showFileSelection.value = true;
        }

        async function handleParseFileByPath(): Promise<boolean> {
            const {
                name = "",
                command,
                path = ""
            } = unref(fileSelectionForm);

            if ("" === name.trim()) {
                window.$message?.warning("必须填写一个别名");
                return false;
            }

            if ("" === path.trim()) {
                window.$message?.warning("空路径");
                return false;
            }

            backendMode.value = fileSelectionForm.value.backend;

            window.$message.loading(
                "如果是超过100MB的文件, 请确保硬盘空间足够, 然后耐心等待.."
            );

            const start = new Date().getTime();
            switch (backendMode.value) {
            case ParseBackend.Sled: {
                const resultRecord = unwrap(
                    await kvParse(
                        name,
                        command,
                        path)
                );

                if (resultRecord) {
                    const end = new Date().getTime();
                    window.$message?.success(`解析完成: ${end - start}ms`);
                    console.log(resultRecord);
                    currentRecordStore.updateCurrentRecord(resultRecord);
                    return true;
                }
                break;
            }
            default:
                window.$message?.error("未知的解析模式");
            }
            return false;
        }

        return {
            showFileSelection,
            handleOpenFileSelector,
            fileSelectionForm,
            handleParseFileByPath,
        };
    }

    function useHistory() {
        const showHistoryList = ref(false);
        const historyList = ref<HistoryRecordItem[]>([]);

        const refreshParserecords = async () => {
            const list = unwrap(await parseRecords());
            console.log(list);
            historyList.value = list;
        };

        async function handleOpenHistory() {
            await refreshParserecords();
            showHistoryList.value = true;
        }

        async function handleRecover(t: HistoryRecordItem) {
            currentRecordStore.updateCurrentRecord(t);
            showHistoryList.value = false;
        }

        async function handleRemoveHistory({
            name
        }: HistoryRecordItem) {
            const result = await removeRecord(name);
            if (!result.succ) {
                window.$message.error(result.msg);
                return;
            }
            await refreshParserecords();
            window.$message.success("已删除");
        }

        return {
            showHistoryList,
            historyList,

            handleOpenHistory,
            handleRemoveHistory,
            handleRecover,
        };
    }

    return {
        useFileSelector,
        useHistory,

    };
}
