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
import {
    useCurrentRecordStore
} from "@/store/modules/current-record";
import {
    storeToRefs
} from "pinia";

const PathSeq = "\\";

export function useTreeView() {
    const currentRecordStore = useCurrentRecordStore();

    const {
        record
    } = storeToRefs(currentRecordStore);

    const treeView = ref<TreeOptionExt[]>([]);

    watch(record, ({
        root
    }) => {
        console.log("记录更新");
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
    });

    return {
        treeView,
    };
}

export function useFinder() {
    const currentRecordStore = useCurrentRecordStore();

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

        if (!currentRecordStore.record) {
            window.$message.warning("当前没有记录, 请先解析文件，或者从解析历史中恢复。");
            return;
        }

        switch (findType) {
        case "dir": {
            const dirsResult = await dbFindDir(currentRecordStore.record.name, keyword);
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
            const filesResult = await dbFindFile(currentRecordStore.record.name, keyword);
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

export function handleLoadDirFunc() {
    const currentRecordStore = useCurrentRecordStore();

    return async (op: TreeOptionExt): Promise<void> => {
        if (!op.meta) {
            op.meta = unwrap(await dbSelect(currentRecordStore.record.name, op.key as string));
        }
        op.children = terserSelectDirToTreeNodes(op.meta as Dir);
    };
}

export function treeNodePropsFunc() {
    const currentRecordStore = useCurrentRecordStore();

    return ({
        option
    }: { option: TreeOptionExt }) => {
        return {
            onClick: async () => {
                if (!option.meta) {
                    option.meta = unwrap(
                        await dbSelect(currentRecordStore.record.name, option.key as string)
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
