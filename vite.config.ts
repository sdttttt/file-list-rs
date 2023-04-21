import vue from "@vitejs/plugin-vue";
import VueMacros from "unplugin-vue-macros/vite";
import path from "node:path";
import AutoImport from "unplugin-auto-import/vite";

import {
    defineConfig
} from "vite";
import Components from "unplugin-vue-components/vite";
import {
    NaiveUiResolver
} from "unplugin-vue-components/resolvers";
import UnoCSS from "unocss/vite";

/**
 * * 项目根路径
 * @descrition 结尾不带/
 */
export function getRootPath() {
    return path.resolve(process.cwd());
}

/**
 * * 项目src路径
 * @param srcName src目录名称(默认: "src")
 * @descrition 结尾不带斜杠
 */
export function getSrcPath(srcName = "src") {
    return path.resolve(getRootPath(), srcName);
}

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        VueMacros({
            plugins: {
                vue: vue(),
                // vueJsx: VueJsx(), // if needed
            },
        }),

        AutoImport({
            imports: [
                "vue",
                "vue/macros",
                "@vueuse/core",
                {
                    "naive-ui": [
                        "useDialog",
                        "useMessage",
                        "useNotification",
                        "useLoadingBar",
                    ],
                },
            ],
            dts: "src/auto-imports.d.ts",
        }),

        Components({
            resolvers     : [NaiveUiResolver()],
            extensions    : ["vue", "tsx"],
            dirs          : ["src/components", "src/views/*/components"],
            allowOverrides: false,
            dts           : "src/components.d.ts",
        }),

        UnoCSS(),
    ],

    /*
     * Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
     * prevent vite from obscuring rust errors
     */
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server     : {
        port      : 1420,
        strictPort: true,
    },
    resolve: {
        alias: {
            "@": getSrcPath(),
        },
    },

    /*
     * to make use of `TAURI_DEBUG` and other env variables
     * https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
     */
    envPrefix: ["VITE_", "TAURI_"],
    build    : {
        // Tauri supports es2021
        target:
            "windows" == process.env.TAURI_PLATFORM ? "chrome105" : "safari13",
        // don't minify for debug builds
        minify               : !process.env.TAURI_DEBUG ? "esbuild" : false,
        // produce sourcemaps for debug builds
        sourcemap            : Boolean(process.env.TAURI_DEBUG),
        reportCompressedSize : true,
        chunkSizeWarningLimit: 1024,
    },
}));
