// When using the Tauri API npm package:
import {
    invoke
} from "@tauri-apps/api/tauri";
import {
    ParseMode
} from "./types";

export enum CommandMode {
    DirS = ""
}

export type BackendResponse<T> = {
    raw: T;
    succ: boolean;
    msg: string;
};

export type HistoryRecordItem = {
    name: string,
    command: ParseMode,
    root: string;
    dbKey: string;
};

/**
 * 内存解析模式和Sled解析模式返回的Dir结构会不一样。
 * 第一是在n (path)字段上:
 * 内存解析模式返回的root是一个绝对路径，之后所有的子目录的n字段都是在root之上的相对路径
 * Sled解析模式返回的root同样是一个绝对路径，但是所有的子目录的n字段也都是绝对路径
 */
export type Dir = {
    n: string; // path | name
    f: File[]; // files
    d: Dir[]; // directories
    s: string; // size
};

export type File = {
    n: string; // name
    s: string; // size
    t: string; // time

    c?: string; // chmod only unix
};

export async function greetRust(): Promise<BackendResponse<String>> {
    return await invoke("greet");
}

/**
 * 内存解析
 * @deprecated
 */
export async function memParse(path: string): Promise<BackendResponse<Dir>> {
    return await invoke("mem_parse", {
        path,
    });
}

/*
 * sled解析
 * 返回一个Root目录的路径
 * 这个很重要，要好好保存，除了路径查询，这个还是整个db的键
 */
export async function kvParse(
    name: String,
    command: String,
    path: string
): Promise<BackendResponse<HistoryRecordItem>> {
    return await invoke("kv_parse", {
        name,
        command,
        path,
    });
}

/*
 * 数据查询，用于sled解析。
 * sled 解析完成后，整个数据会保存在硬盘上，通过Rust的后端来访问这些数据
 */
export async function dbSelect(
    name: string,
    path: string
): Promise<BackendResponse<Dir>> {
    return await invoke("db_select", {
        name,
        path,
    });
}

// 查询文件夹，用于sled解析。
export async function dbFindDir(
    name: string,
    regExp: string
): Promise<BackendResponse<Dir[]>> {
    return await invoke("db_find_dir", {
        name,
        regExp,
    });
}

// 查询文件，用于sled解析。
export async function dbFindFile(
    name: string,
    regExp: string
): Promise<BackendResponse<string[]>> {
    return await invoke("db_find_file", {
        name,
        regExp,
    });
}

export async function parseRecords(): Promise<BackendResponse<HistoryRecordItem[]>> {
    return await invoke("parse_records");
}

export async function removeRecord(
    name: string
): Promise<BackendResponse<void>> {
    return await invoke("remove_record", {
        name,
    });
}

// 解包上面的事件函数
export function unwrap<T>(result: BackendResponse<T>): T | undefined {
    if (result.succ) {
        return result.raw;
    }
    window.$message?.error(result.msg);
}
