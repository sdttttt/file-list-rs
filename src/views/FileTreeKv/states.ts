import {
    Dir, dbFindDir, dbFindFile, dbSelect, unwrap
} from "@/rust";
import {
    FileTreeFindForm, TreeOptionExt
} from "@/types";
import {
    Ref, computed, h, ref, unref, watch
} from "vue";
import {
    useDirViewStore
} from "@/store";
import {
    Folder, FolderOpenOutline
} from "@vicons/ionicons5";
import {
    NIcon, TreeOption
} from "naive-ui";

const PathSeq = "\\";
let dbKey = "";
export function useTreeView(
    props: Readonly<
        Omit<
            {
                root: string;
                dbKey: string;
            },
            never
        > & {}
    >
) {
    const treeView = ref<TreeOptionExt[]>([]);

    watch([() => props.root, () => props.dbKey], ([root, newDbKey]) => {
        treeView.value = [
            {
                key   : root,
                label : root,
                isLeaf: false,
                prefix: () =>
                    h(NIcon, null, {
                        default: () => h(Folder),
                    }),
            },
        ];
        dbKey = newDbKey;
    });

    return {
        treeView,
        handleLoadDir,
    };
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
        keyword : "",
    });

    //文件搜索结果
    const finderFileResult = ref<string[]>([]);
    // 文件夹渲染树
    const finderDirTree = ref<TreeOptionExt[]>([]);

    function handleOpenFinderForm() {
        showFinderForm.value = true;
    }

    async function handleFind() {
        const {
            keyword, findType
        } = unref(finderForm);
        if (!keyword) {
            return;
        }

        if (!dbKey) {
            window.$message.warning("dbKey不存在, 请先解析文件，或者从解析历史中恢复。");
            return;
        }

        switch (findType) {
        case "dir": {
            const dirsResult = await dbFindDir(dbKey, keyword);
            if (dirsResult.succ) {
                window.$message.info(
                    `找到 ${dirsResult.raw.length} 个符合结果的目录`
                );
                finderDirTree.value = dirsResult.raw.map(t => ({
                    key   : t.n,
                    label : t.n,
                    isLeaf: false,
                }));
                showFinder.value = true;
            }
            break;
        }
        case "file": {
            const filesResult = await dbFindFile(dbKey, keyword);
            if (filesResult.succ) {
                finderFileResult.value = filesResult.raw;
                window.$message.info(
                    `找到 ${finderFileResult.value.length} 个符合结果的文件`
                );
                showFinderFileResult.value = true;
            }
            break;
        }
        }
    }

    return {
        showFinder,
        handleOpenFinderForm,
        finderForm,
        showFinderFileResult,
        handleFind,
        finderFileResult,
        finderDirTree,
        showFinderForm,
    };
}

export async function handleLoadDir(op: TreeOptionExt): Promise<void> {
    if (!op.meta) {
        op.meta = unwrap(await dbSelect(dbKey, op.key as string));
    }
    op.children = terserSelectDirToTreeNodes(op.meta as Dir);
}

export function treeNodeProps({
    option
}: { option: TreeOptionExt }) {
    return {
        onClick: async () => {
            if (!option.meta) {
                option.meta = unwrap(
                    await dbSelect(dbKey, option.key as string)
                );
            }
            const dirViewStroe = useDirViewStore();
            if (option.isLeaf) {
                // 叶子节点说明是文件了，不需要展示
                return;
            }
            dirViewStroe.updateCurrentDirView(option.meta as Dir);
            console.log(dirViewStroe.currentDir);
        },
    };
}

export function updatePrefixWithExpaned(
    _keys: Array<string | number>,
    _option: Array<TreeOption | null>,
    meta: {
        node: TreeOption | null;
        action: "expand" | "collapse" | "filter";
    }
) {
    if (!meta.node) {
        return;
    }
    switch (meta.action) {
    case "expand":
        meta.node.prefix = () =>
            h(NIcon, null, {
                default: () => h(FolderOpenOutline),
            });
        break;
    case "collapse":
        meta.node.prefix = () =>
            h(NIcon, null, {
                default: () => h(Folder),
            });
        break;
    }
}

function terserSelectDirToTreeNodes(dir: Dir): TreeOptionExt[] {
    const dirNodes = dir.d.map(
        (t): TreeOptionExt => ({
            key   : t.n,
            label : t.n.split(PathSeq).reverse()[0], // 获取最后一段路径
            isLeaf: false,
            prefix: () =>
                h(NIcon, null, {
                    default: () => h(Folder),
                }),
        })
    );

    const fileNodes = dir.f.map(
        (t): TreeOptionExt => ({
            key   : dir.n + PathSeq + t.n,
            label : t.n,
            isLeaf: true,
            meta  : t,
        })
    );

    return [...dirNodes, ...fileNodes];
}
