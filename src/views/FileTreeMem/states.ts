import {
    Dir
} from "@/rust";
import {
    TreeOptionExt
} from "@/types";
import {
    TreeNodeProps, TreeOption
} from "naive-ui/es/tree/src/interface";
import {
    Ref, onMounted, ref, watch
} from "vue";

const KeySep = "$R$";
const FileEnd = "F";
export function useTreeView(
    props: Readonly<
        Omit<
            {
                data?: Dir;
            },
            never
        > & {}
    >
) {
    const treeView = ref<TreeOptionExt | null>(null);
    const realTree = ref<Dir | null>(null);

    watch(realTree, v => {
        treeView.value = terserRootDirToTreeNode(v);
    });

    watch(
        () => props.data,
        v => {
            realTree.value = v;
        }
    );

    async function handleLoadDir(node: TreeOption): Promise<void> {
        console.log(`extends: ${node.key}`);
        node.children = terserSelectDirChildrenToTreeNodes(
            realTree.value,
            node.key as string
        );
        return;
    }

    return {
        treeView,
        handleLoadDir,
    };
}

function terserRootDirToTreeNode(root: Dir): TreeOptionExt {
    const key = KeySep;
    const label = root.n;
    const isLeaf = false;

    const childrenDir = [];
    for (let i = 0; i < root.d.length; i++) {
        const subKey = key + i;
        const subLabel = root.d[i].n;
        const subIsLeaf = false;

        childrenDir.push({
            key   : subKey,
            label : subLabel,
            isLeaf: subIsLeaf,
        });
    }

    const childrenFile = [];
    for (let i = 0; i < root.f.length; i++) {
        const subKey = key + i + FileEnd;
        const subLabel = root.f[i].n;
        const subIsLeaf = true;

        childrenFile.push({
            key   : subKey,
            label : subLabel,
            isLeaf: subIsLeaf,
        });
    }

    return {
        key,
        label,
        isLeaf,
        children: [...childrenDir, ...childrenFile],
        meta    : root,
    };
}

function terserSelectDirChildrenToTreeNodes(
    realRootTree: Dir,
    key: string
): TreeOptionExt[] {
    const indexVec = key.split(KeySep).filter(t => 0 != t.trim().length);
    let targetDir = realRootTree;
    for (const index of indexVec) {
        targetDir = targetDir.d[Number(index)];
    }

    const childrenDir = [];
    for (let i = 0; i < targetDir.d.length; i++) {
        const subKey = key + KeySep + i;
        const subLabel = targetDir.d[i].n;
        const subIsLeaf = false;

        childrenDir.push({
            key   : subKey,
            label : subLabel,
            isLeaf: subIsLeaf,
        });
    }

    const childrenFile = [];
    for (let i = 0; i < targetDir.f.length; i++) {
        const subKey = key + KeySep + i + FileEnd;
        const subLabel = targetDir.f[i].n;
        const subIsLeaf = true;

        childrenFile.push({
            key   : subKey,
            label : subLabel,
            isLeaf: subIsLeaf,
        });
    }

    return [...childrenDir, ...childrenFile];
}
