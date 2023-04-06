// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'

export type BackendResponse<T> = {
    raw: T,
    succ: boolean,
    msg: string,
};

export async function greetRust(): Promise<BackendResponse<String>> {
    return await invoke("greet");
}
