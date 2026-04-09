class ReportStore {
    scans = $state<Record<string, Record<string, any>>>({});

    addResult(domain: string, moduleName: string, data: any) {
        if (!this.scans[domain]) {
            this.scans[domain] = {};
        }
        // Deep copy to prevent proxy binding issues when component unmounts
        this.scans[domain][moduleName] = JSON.parse(JSON.stringify(data));
    }

    getReportForDomain(domain: string): Record<string, any> {
        return this.scans[domain] || {};
    }

    getAvailableDomains(): string[] {
        return Object.keys(this.scans);
    }
}

export const reportStore = new ReportStore();
