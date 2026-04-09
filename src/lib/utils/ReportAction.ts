import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

export async function generateMarkdownReport(domain: string, data: Record<string, any>) {
    let md = `# WebQ Security Report\n\n`;
    md += `**Target:** ${domain}\n`;
    md += `**Date:** ${new Date().toUTCString()}\n\n`;
    md += `---\n\n`;

    for (const [moduleName, result] of Object.entries(data)) {
        md += `## ${moduleName}\n\n`;
        md += "```json\n";
        md += JSON.stringify(result, null, 2);
        md += "\n```\n\n";
    }

    try {
        const filePath = await save({
            filters: [{ name: 'Markdown', extensions: ['md'] }],
            defaultPath: `${domain.replace(/[^a-z0-9]/gi, '_').toLowerCase()}_webq_report.md`
        });
        
        if (filePath) {
            await writeTextFile(filePath, md);
            return true;
        }
    } catch (e) {
        console.error("Failed to save report:", e);
        throw e;
    }
    return false;
}
