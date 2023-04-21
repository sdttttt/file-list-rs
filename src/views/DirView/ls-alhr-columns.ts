import {
    Folder
} from "@vicons/ionicons5";
import {
    NIcon
} from "naive-ui";
import {
    DataTableColumns
} from "naive-ui";
import {
    DataItem
} from "./types";

const columns: DataTableColumns<DataItem> = [
    {
        title : "名称",
        key   : "path",
        sorter: "default",
        render: row => {
            if (!row.size && !row.time) {
                return h(
                    "span",
                    {
                        style: "display: inline-flex; align-items:center;",
                    },
                    [
                        h(
                            NIcon,
                            {
                                style: "margin-right: 7px; ",
                            },
                            {
                                default: () => h(Folder),
                            }
                        ),
                        h("div", null, row.path),
                    ]
                );
            }

            return row.path;
        },
        filterOptions: [
            {
                label: "文件夹",
                value: "dir"
            },
            {
                label: "文件",
                value: "file"
            }
        ],
    },
    {
        title : "大小",
        key   : "size",
        sorter: "default",
    },
    {
        title: "权限",
        key  : "chmod",
    },
    {
        title: "所属用户",
        key  : "user",
    },
    {
        title: "所属组",
        key  : "group",
    },
    {
        title : "修改时间",
        key   : "time",
        sorter: "default",
    },
];

export default columns;
