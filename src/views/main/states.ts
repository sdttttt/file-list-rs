import { ref, onMounted } from "vue"
import { FileSelectForm, ParseMode } from "@/types";
import { Dir, parseFileListByPathRust, unwrap } from "@/rust";
import { useMessage } from "naive-ui";

export function useFileSelector() {
    const showFileSelection = ref(false);

    const fileSelectionForm = ref<FileSelectForm>({
        path: "",
        mode: ParseMode.DirS
    });

    const fileTree = ref<Dir | undefined>(undefined);

    function handleOpenFileSelector() {
        showFileSelection.value = true;
    }

    async function handleParseFileByPath(): Promise<boolean> {
        if (!fileSelectionForm.value.path) {
            window.$message?.warning("空路径");
            return false;
        }

        window.$message.loading("如果是超过50MB文件，请耐心等待，中间可能出现无响应。")
        var start = new Date().getTime()
        const result = unwrap(await parseFileListByPathRust(fileSelectionForm.value.path));
        if (result) {
            var end = new Date().getTime()
            window.$message?.success(`解析完成: ${end - start}ms`)
            console.log(result);
            fileTree.value = result;
            return true;
        }
        return false;
    }

    return { showFileSelection, handleOpenFileSelector, fileSelectionForm, handleParseFileByPath, fileTree };
}
