// When using the Tauri API npm package:
import {
    invoke
} from "@tauri-apps/api/tauri";

export type BackendResponse<T> = {
    raw: T;
    succ: boolean;
    msg: string;
};

export type KvParseResponseRaw = {
    dbKey: string;
    rootPath: string;
};

export type RootAndDbKey = {
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
};

export async function greetRust(): Promise<BackendResponse<String>> {
    return await invoke("greet");
}

// 内存解析
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
    path: string
): Promise<BackendResponse<KvParseResponseRaw>> {
    return await invoke("kv_parse", {
        path,
    });
}

/*
 * 数据查询，用于sled解析。
 * sled 解析完成后，整个数据会保存在硬盘上，通过Rust的后端来访问这些数据
 */
export async function dbSelect(
    dbKey: string,
    path: string
): Promise<BackendResponse<Dir>> {
    return await invoke("db_select", {
        dbKey,
        path,
    });
}

// 查询文件夹，用于sled解析。
export async function dbFindDir(
    dbKey: string,
    regExp: string
): Promise<BackendResponse<Dir[]>> {
    return await invoke("db_find_dir", {
        dbKey,
        regExp,
    });
}

// 查询文件，用于sled解析。
export async function dbFindFile(
    dbKey: string,
    regExp: string
): Promise<BackendResponse<string[]>> {
    return await invoke("db_find_file", {
        dbKey,
        regExp,
    });
}

export async function parseRecords(): Promise<BackendResponse<RootAndDbKey[]>> {
    return await invoke("parse_records");
}

export async function removeRecord(
    dbKey: string
): Promise<BackendResponse<void>> {
    return await invoke("remove_record", {
        dbKey,
    });
}

// 解包上面的事件函数
export function unwrap<T>(result: BackendResponse<T>): T | undefined {
    if (result.succ) {
        return result.raw;
    }
    window.$message?.error(result.msg);
}
