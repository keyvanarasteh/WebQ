import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import jsPDF from 'jspdf';
import autoTable from 'jspdf-autotable';

/**
 * Prompts the user with a native save dialog and writes the report to a PDF file.
 */
export async function exportToPdf(domain: string, reportData: Record<string, unknown>): Promise<boolean> {
    try {
        const filePath = await save({
            title: 'Export PDF Report',
            defaultPath: `webq_report_${domain.replace(/[^a-z0-9]/gi, '_').toLowerCase()}_${new Date().toISOString().slice(0,10)}.pdf`,
            filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
        });
        
        if (filePath) {
            const doc = new jsPDF();
            
            // Document Title
            doc.setFontSize(22);
            doc.text("WebQ Cybersecurity Report", 14, 22);
            
            doc.setFontSize(14);
            doc.text(`Target Domain: ${domain}`, 14, 32);
            
            doc.setFontSize(10);
            doc.setTextColor(100);
            doc.text(`Generated on: ${new Date().toUTCString()}`, 14, 40);
            
            let currentY = 50;
            
            for (const [moduleName, moduleData] of Object.entries(reportData)) {
                // Add a new page if we are too low
                if (currentY > 250) {
                    doc.addPage();
                    currentY = 20;
                }
                
                doc.setFontSize(16);
                doc.setTextColor(15, 23, 42); // slate 900
                doc.text(moduleName, 14, currentY);
                currentY += 8;
                
                if (Array.isArray(moduleData)) {
                    if (moduleData.length === 0) {
                        doc.setFontSize(12);
                        doc.setTextColor(100);
                        doc.text("No data found.", 14, currentY);
                        currentY += 10;
                    } else if (typeof moduleData[0] === 'object' && moduleData[0] !== null) {
                        const keys = Object.keys(moduleData[0]);
                        const body = moduleData.map(item => keys.map(k => {
                            const val = item[k];
                            // Simple stringification logic for nested objects
                            if (typeof val === 'object' && val !== null) {
                                return Array.isArray(val) ? `[Array of ${val.length}]` : '[Object]';
                            }
                            return String(val);
                        }));
                        
                        autoTable(doc, {
                            startY: currentY,
                            head: [keys],
                            body: body,
                            theme: 'grid',
                            headStyles: { fillColor: [9, 9, 11] }, // obsidian
                            styles: { fontSize: 8, overflow: 'linebreak', cellPadding: 2 },
                        });
                        
                        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                        // @ts-ignore - plugin augments jsPDF runtime instance
                        currentY = doc.lastAutoTable.finalY + 15;
                    } else {
                        doc.setFontSize(10);
                        doc.setTextColor(71, 85, 105); // slate 600
                        moduleData.forEach((item: unknown) => {
                            if (currentY > 280) {
                                doc.addPage();
                                currentY = 20;
                            }
                            doc.text(`• ${String(item)}`, 14, currentY);
                            currentY += 6;
                        });
                        currentY += 10;
                    }
                } else {
                    doc.setFontSize(10);
                    doc.setTextColor(71, 85, 105);
                    const jsonString = JSON.stringify(moduleData, null, 2);
                    const lines = doc.splitTextToSize(jsonString, 180);
                    
                    if (currentY + (lines.length * 4) > 280) {
                        doc.addPage();
                        currentY = 20;
                    }
                    
                    doc.text(lines, 14, currentY);
                    currentY += (lines.length * 4) + 10;
                }
            }
            
            const pdfData = doc.output('arraybuffer');
            await writeFile(filePath, new Uint8Array(pdfData));
            
            return true;
        }
        return false;
    } catch (err) {
        console.error('Failed to export PDF:', err);
        throw err;
    }
}
