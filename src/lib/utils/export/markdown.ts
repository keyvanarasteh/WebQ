import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

/**
 * Prompts the user with a native save dialog and writes the report to a Markdown file.
 */
export async function exportToMarkdown(domain: string, reportData: Record<string, unknown>): Promise<boolean> {
    try {
        const filePath = await save({
            title: 'Export Markdown Report',
            defaultPath: `webq_report_${domain.replace(/[^a-z0-9]/gi, '_').toLowerCase()}_${new Date().toISOString().slice(0,10)}.md`,
            filters: [{ name: 'Markdown', extensions: ['md'] }]
        });
        
        if (filePath) {
            const mdContent = generateMarkdown(domain, reportData);
            await writeTextFile(filePath, mdContent);
            return true;
        }
        return false;
    } catch (err) {
        console.error('Failed to export Markdown:', err);
        throw err;
    }
}

function generateMarkdown(domain: string, data: Record<string, unknown>): string {
    let md = `# WebQ Comprehensive Security Report\n`;
    md += `> Target: **${domain}**\n>\n> Date: ${new Date().toUTCString()}\n\n`;
    md += `---\n\n`;
    
    for (const [moduleName, moduleData] of Object.entries(data)) {
        md += `## ${moduleName}\n\n`;
        
        if (Array.isArray(moduleData)) {
            md += `*Discovered **${moduleData.length}** items.*\n\n`;
            if (moduleData.length > 0 && typeof moduleData[0] === 'object' && moduleData[0] !== null) {
                md += generateTable(moduleData);
            } else {
                moduleData.forEach((item: unknown) => { md += `- ${item}\n`; });
            }
        } else if (typeof moduleData === 'object' && moduleData !== null) {
            md += "```json\n" + JSON.stringify(moduleData, null, 2) + "\n```\n";
        } else {
            md += `${moduleData}\n`;
        }
        md += `\n`;
    }
    
    return md;
}

function generateTable(items: Record<string, unknown>[]): string {
    const keys = Object.keys(items[0]);
    let md = `| ${keys.join(' | ')} |\n`;
    md += `| ${keys.map(() => '---').join(' | ')} |\n`;
    for (const item of items) {
        md += `| ${keys.map(k => {
            const val = item[k];
            if (typeof val === 'object' && val !== null) {
                return '[Complex Object]';
            }
            return String(val).replace(/\|/g, '\\|');
        }).join(' | ')} |\n`;
    }
    return md + '\n';
}
