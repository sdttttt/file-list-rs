// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
import { useMessage } from 'naive-ui'

const message = useMessage()

export type BackendResponse<T> = {
    raw: T,
    succ: boolean,
    msg: string,
};

export type Dir = {
    n: string, // path | name
    f: File[], // files
    d: Dir[], // directories
    s: String, // size
};

export type File = {
    n: string, // name
    s: String, // size
    t: String // time
};

export async function greetRust(): Promise<BackendResponse<String>> {
    return await invoke("greet");
}


export async function parseFileListByPathRust(path: string): Promise<BackendResponse<Dir>> {
    return await invoke("mem_parse", { path });
}

export function unwrap<T>(result: BackendResponse<T>): T | undefined {
    if (result.succ) {
        return result.raw
    } else {
        message.error(result.msg)
    }
}
