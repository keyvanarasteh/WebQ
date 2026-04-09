import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { Document, Packer, Paragraph, TextRun, HeadingLevel, Table, TableRow, TableCell } from 'docx';

/**
 * Prompts the user with a native save dialog and writes the report to a DOCX file.
 */
export async function exportToDocx(domain: string, reportData: Record<string, unknown>): Promise<boolean> {
    try {
        const filePath = await save({
            title: 'Export DOCX Report',
            defaultPath: `webq_report_${domain.replace(/[^a-z0-9]/gi, '_').toLowerCase()}_${new Date().toISOString().slice(0,10)}.docx`,
            filters: [{ name: 'Word Document', extensions: ['docx'] }]
        });
        
        if (filePath) {
            const children = [];
            
            // Document Title
            children.push(new Paragraph({
                text: "WebQ Cybersecurity Report",
                heading: HeadingLevel.TITLE,
            }));
            
            children.push(new Paragraph({
                children: [
                    new TextRun({ text: `Target Domain: `, bold: true }),
                    new TextRun({ text: domain }),
                ]
            }));
            
            children.push(new Paragraph({
                children: [
                    new TextRun({ text: `Generated on: `, bold: true }),
                    new TextRun({ text: new Date().toUTCString(), color: "666666" }),
                ]
            }));
            
            children.push(new Paragraph({ text: "" })); // spacer
            
            for (const [moduleName, moduleData] of Object.entries(reportData)) {
                children.push(new Paragraph({
                    text: moduleName,
                    heading: HeadingLevel.HEADING_1,
                }));
                
                if (Array.isArray(moduleData)) {
                    if (moduleData.length === 0) {
                        children.push(new Paragraph({ text: "No data found.", color: "666666" }));
                    } else if (typeof moduleData[0] === 'object' && moduleData[0] !== null) {
                        const keys = Object.keys(moduleData[0]);
                        
                        const rows = [];
                        
                        // Header row
                        rows.push(new TableRow({
                            children: keys.map(k => new TableCell({ 
                                children: [new Paragraph({ children: [new TextRun({ text: k, bold: true })] })] 
                            }))
                        }));
                        
                        // Body rows
                        moduleData.forEach(item => {
                            rows.push(new TableRow({
                                children: keys.map(k => {
                                    let val = item[k];
                                    if (typeof val === 'object' && val !== null) {
                                        val = Array.isArray(val) ? `[Array of ${val.length}]` : '[Object]';
                                    }
                                    return new TableCell({ children: [new Paragraph({ text: String(val) })] });
                                })
                            }));
                        });
                        
                        children.push(new Table({ rows }));
                    } else {
                        moduleData.forEach((item: unknown) => {
                            children.push(new Paragraph({
                                text: `• ${String(item)}`,
                                bullet: { level: 0 }
                            }));
                        });
                    }
                } else {
                    const jsonString = JSON.stringify(moduleData, null, 2);
                    jsonString.split('\n').forEach(line => {
                        children.push(new Paragraph({ text: line }));
                    });
                }
                
                children.push(new Paragraph({ text: "" })); // spacer
            }
            
            const doc = new Document({
                sections: [{
                    children: children
                }]
            });
            
            const buf = await Packer.toBuffer(doc);
            await writeFile(filePath, new Uint8Array(buf));
            
            return true;
        }
        return false;
    } catch (err) {
        console.error('Failed to export DOCX:', err);
        throw err;
    }
}
