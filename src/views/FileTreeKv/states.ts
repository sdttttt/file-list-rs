import { Dir, dbSelect } from "@/rust";
import { TreeOption, resultDark } from "naive-ui";
import { ref, watch } from "vue";

export function useTreeView(props: Readonly<Omit<{
    root: string;
}, never> & {}>) {

    const treeView = ref<Dir | null>(null);
    function handleLoadDir(op: TreeOption) {

    }

    watch(() => props.root, async v => {
        const result = await dbSelect(v, v);
        if (result.succ) {
            treeView.value = result.raw
        }
    })
}
