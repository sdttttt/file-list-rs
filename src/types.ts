import {
    Dir, File
} from "./rust";
import {
    TreeOption
} from "naive-ui";

export enum ParseMode {
    DirS = "dir /s *.*",
    LsALHR = "ls -alhR",
}

export enum ParseBackend {
    Mem = "0",
    Sled = "1",
}

export type FileSelectForm = {
    name: string;
    path: string;
    command: ParseMode;
    backend: ParseBackend;
};

export interface TreeOptionExt extends TreeOption {
    meta?: Dir | File;
}

export type FileTreeFindForm = {
    keyword: string;
    findType: "file" | "dir";
};
