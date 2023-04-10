export enum ParseMode {
    DirS = "0"
}

export enum ParseBackend {
    Mem = "0",
    Sled = "1",
}

export type FileSelectForm = {
    path: string;
    command: ParseMode,
    backend: ParseBackend,
}

export type TreeNode = {
    key: string,
    label: string,
    isLeaf: boolean,
    children?: TreeNode[]
};

export type FileTreeFindForm = {
    keyword: string,
    findType: "file" | "dir",
};
