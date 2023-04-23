import {
    defineStore
} from "pinia";

export const useAppStore = defineStore("app", () => {

    const isDark = useDark();
    const themeSwitch = useToggle(isDark);

    return {
        isDark,
        themeSwitch,
    };
});
