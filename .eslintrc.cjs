process.env.ESLINT_TSCONFIG = "tsconfig.json";

module.exports = {
    root   : true,
    extends: ["eslint:recommended", "plugin:vue/vue3-recommended"],

    parser       : "vue-eslint-parser",
    // 优先级低于parse的语法解析配置
    parserOptions: {
        // 指定ESlint的解析器
        parser      : "@typescript-eslint/parser",
        // 允许使用ES语法
        ecmaVersion : 2020,
        // 允许使用import
        sourceType  : "module",
        // 允许解析JSX
        ecmaFeatures: {
            jsx: true,
        },
    },

    rules: {
        "no-undef"      : "off",
        "no-unused-vars": "off",

        // 建议
        "multiline-comment-style": ["error", "starred-block"],
        "no-else-return"         : "error",
        "no-extra-boolean-cast"  : "error",
        "no-extra-label"         : "error",
        "no-extra-semi"          : "error",
        "no-floating-decimal"    : "error",
        "no-implicit-coercion"   : "error",
        "no-lonely-if"           : "error",
        "no-undef-init"          : "error",
        "no-unused-labels"       : "error",
        "prefer-arrow-callback"  : "error",
        "prefer-const"           : "error",
        "prefer-template"        : "error",
        curly                    : "error",
        yoda                     : ["error", "always"],
        "dot-notation"           : "error",

        // 风格
        "arrow-parens"                    : ["error", "as-needed"],
        indent                            : ["error", 4],
        "object-curly-newline"            : ["error", "always"],
        "nonblock-statement-body-position": ["error", "any"],
        "no-trailing-spaces"              : "error",
        "max-statements-per-line"         : [
            "error",
            {
                max: 2,
            },
        ],
        // 花括号单独一行，可读性更好
        "brace-style": [
            "error",
            "1tbs",
            {
                allowSingleLine: true,
            },
        ],

        "key-spacing": [
            "error",
            {
                align: "colon",
            },
        ],
        "no-multi-spaces"        : "error",
        semi                     : "error",
        "semi-spacing"           : "error",
        "space-before-blocks"    : "error",
        "switch-colon-spacing"   : "error",
        quotes                   : ["error", "double"],
        "object-property-newline": "error",
        "linebreak-style"        : ["error", "windows"],
        "no-var"                 : "error",

        // vue
        "vue/valid-template-root"       : "off",
        "vue/no-multiple-template-root" : "off",
        "no-mixed-spaces-and-tabs"      : "error",
        "vue/multi-word-component-names": [
            "error",
            {
                ignores: ["index", "401", "404"],
            },
        ],
    },
};
