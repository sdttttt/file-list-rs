import { Dir } from '@/rust';
import { defineStore } from 'pinia'
import { ref } from 'vue';


export const useDirViewStore = defineStore("dirView", () => {
    const currentDir = ref<Dir>(null);

    function updateCurrentDirView(d: Dir) {
        currentDir.value = d;
    }

    return {
        currentDir,
        updateCurrentDirView,
    }
});
