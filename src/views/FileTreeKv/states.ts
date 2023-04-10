import { Dir, dbSelect, unwrap, dbFindDir, dbFindFile } from "@/rust";
import { FileTreeFindForm, TreeNode } from "@/types";
import { TreeOption } from "naive-ui";
import { Ref, ref, unref, watch, computed } from "vue";

const PathSeq = "\\";
let dbKey = "";
export function useTreeView(props: Readonly<Omit<{
    root: string;
    dbKey: string,
}, never> & {}>) {
    const treeView = ref<TreeNode[]>([]);

    watch([() => props.root, () => props.dbKey], ([root, newDbKey]) => {
        treeView.value = [
            {
                key: root,
                label: root,
                isLeaf: false,
            }
        ];
        dbKey = newDbKey;
    })

    return {
        treeView,
        handleLoadDir
    }
}

export function useFinder() {
    // 显示搜索树
    const showFinder = ref(false);
    // 显示搜索输入表单
    const showFinderForm = ref(false);
    // 显示文件查询结果
    const showFinderFileResult = ref(false);
    // 搜索表单
    const finderForm = ref<FileTreeFindForm>({
        findType: "dir",
        keyword: "",
    });

    //文件搜索结果
    const finderFileResult = ref<string[]>([]);
    // 文件夹渲染树
    const finderDirTree = ref<TreeNode[]>([]);

    function handleOpenFinderForm() {
        showFinderForm.value = true;
    }

    async function handleFind() {
        const { keyword, findType } = unref(finderForm);
        if (!keyword) {
            return;
        }

        if (!dbKey) {
            window.$message.warning("dbKey不存在。")
        }

        switch (findType) {
            case "dir":
                const dirsResult = await dbFindDir(dbKey, keyword);
                if (dirsResult.succ) {
                    window.$message.info(`找到 ${dirsResult.raw.length} 个符合结果的目录`)
                    finderDirTree.value = dirsResult.raw.map(t => ({
                        key: t.n,
                        label: t.n,
                        isLeaf: false,
                    }));
                    showFinder.value = true;
                }
                break;
            case "file":
                const filesResult = await dbFindFile(dbKey, keyword);
                if (filesResult.succ) {
                    finderFileResult.value = filesResult.raw;
                    window.$message.info(`找到 ${finderFileResult.value.length} 个符合结果的文件`)
                    showFinderFileResult.value = true;
                }
                break;
        }
    }

    return {
        showFinder, handleOpenFinderForm,
        finderForm, showFinderFileResult,
        handleFind, finderFileResult,
        finderDirTree,
        showFinderForm
    }
}

export async function handleLoadDir(op: TreeOption): Promise<void> {
    const dir = unwrap(await dbSelect(dbKey, op.key as string));
    op.children = terserSelectDirToTreeNodes(dir);
}


function terserSelectDirToTreeNodes(dir: Dir): TreeNode[] {

    const dirNodes = dir.d.map((t): TreeNode => ({
        key: t.n,
        label: t.n.split(PathSeq).reverse()[0], // 获取最后一段路径
        isLeaf: false,
    }));

    const fileNodes = dir.f.map((t): TreeNode => ({
        key: dir.n + PathSeq + t.n,
        label: t.n,
        isLeaf: true
    }));



    return [...dirNodes, ...fileNodes];
}
