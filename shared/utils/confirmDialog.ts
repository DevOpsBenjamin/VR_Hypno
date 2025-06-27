import { confirm as tauriConfirm } from '@tauri-apps/api/dialog';

export async function confirmDialog(confirmTxt: string, title: string): Promise<boolean> {
    return await tauriConfirm(confirmTxt, { title: title });
}