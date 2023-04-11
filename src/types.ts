import { Dir, File } from "./rust";
import { TreeOption } from "naive-ui";

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

export interface TreeOptionExt extends TreeOption {
    meta?: Dir | File,
};

export type FileTreeFindForm = {
    keyword: string,
    findType: "file" | "dir",
};
