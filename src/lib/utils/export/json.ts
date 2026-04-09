import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

/**
 * Prompts the user with a native save dialog and writes the report to a JSON file.
 */
export async function exportToJson(domain: string, reportData: Record<string, unknown>): Promise<boolean> {
    try {
        const filePath = await save({
            title: 'Export JSON Report',
            defaultPath: `webq_report_${domain.replace(/[^a-z0-9]/gi, '_').toLowerCase()}_${new Date().toISOString().slice(0,10)}.json`,
            filters: [{ name: 'JSON', extensions: ['json'] }]
        });
        
        if (filePath) {
            await writeTextFile(filePath, JSON.stringify(reportData, null, 2));
            return true;
        }
        return false;
    } catch (err) {
        console.error('Failed to export JSON:', err);
        throw err;
    }
}
