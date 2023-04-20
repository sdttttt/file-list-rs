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

export function useMainView() {
    // Mem解析的组件参数
    const fileTree = ref<Dir | undefined>(undefined);
    // Sled解析的组件参数
    const root = ref("");
    const dbKey = ref("");

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
            case ParseBackend.Mem: {
                const memResult = unwrap(
                    await memParse(fileSelectionForm.value.path)
                );
                if (memResult) {
                    const end = new Date().getTime();
                    window.$message?.success(`解析完成: ${end - start}ms`);
                    console.log(memResult);
                    fileTree.value = memResult;
                    return true;
                }
                break;
            }
            case ParseBackend.Sled: {

                const sledResult = unwrap(
                    await kvParse(
                        name,
                        command,
                        path)
                );
                if (sledResult) {
                    const end = new Date().getTime();
                    window.$message?.success(`解析完成: ${end - start}ms`);
                    console.log(sledResult);
                    root.value = sledResult.root;
                    dbKey.value = sledResult.dbKey;
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
            backendMode,
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

        async function handleRecover(kv: HistoryRecordItem) {
            dbKey.value = kv.dbKey;
            root.value = kv.root;
            showHistoryList.value = false;
        }

        async function handleRemoveHistory({
            dbKey
        }: HistoryRecordItem) {
            const result = await removeRecord(dbKey);
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

        fileTree,
        root,
        dbKey,
    };
}
