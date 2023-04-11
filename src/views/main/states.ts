import {
    ref
} from "vue";
import {
    FileSelectForm, ParseBackend, ParseMode
} from "@/types";
import {
    Dir, kvParse, memParse, unwrap
} from "@/rust";

export function useFileSelector() {
    const showFileSelection = ref(false);

    const fileSelectionForm = ref<FileSelectForm>({
        path   : "",
        command: ParseMode.DirS,
        backend: ParseBackend.Sled,
    });

    const backendMode = ref<ParseBackend>(ParseBackend.Mem);

    // Mem解析的组件参数
    const fileTree = ref<Dir | undefined>(undefined);
    // Sled解析的组件参数
    const root = ref("");
    const dbKey = ref("");

    function handleOpenFileSelector() {
        showFileSelection.value = true;
    }

    async function handleParseFileByPath(): Promise<boolean> {
        if (!fileSelectionForm.value.path) {
            window.$message?.warning("空路径");
            return false;
        }
        backendMode.value = fileSelectionForm.value.backend;

        window.$message.loading(
            "如果是超过50MB文件，请耐心等待，中间可能出现无响应。"
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
                await kvParse(fileSelectionForm.value.path)
            );
            if (sledResult) {
                const end = new Date().getTime();
                window.$message?.success(`解析完成: ${end - start}ms`);
                console.log(sledResult);
                root.value = sledResult.rootPath;
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
        fileTree,
        backendMode,
        root,
        dbKey,
    };
}
