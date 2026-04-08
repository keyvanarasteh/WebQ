export async function exportToJSON(data: unknown, rootFilename: string) {
    const jsonStr = JSON.stringify(data, null, 2);
    const blob = new Blob([jsonStr], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement("a");
    a.href = url;
    a.download = `${rootFilename}_${new Date().toISOString()}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

export async function exportToCSV(data: unknown[], filename: string) {
    // Phase 2: CSV matrix parsing logic will be injected here
    console.warn("CSV Export skeleton triggered.");
}

export async function exportToPDF(elementId: string, filename: string) {
    // Phase 2: html2canvas or native print logic will be injected here
    console.warn("PDF Export skeleton triggered.");
}
