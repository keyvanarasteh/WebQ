class AppState {
    theme = $state<'dark' | 'light' | 'system'>('system');
    animationsEnabled = $state(true);
    sidebarOpen = $state(true);
    isScanning = $state(false);
    activeModule = $state<string | null>(null);

    toggleTheme() {
        if (this.theme === 'dark') this.theme = 'light';
        else if (this.theme === 'light') this.theme = 'system';
        else this.theme = 'dark';
    }

    setScanning(status: boolean, moduleName: string) {
        this.isScanning = status;
        this.activeModule = status ? moduleName : null;
    }

    historicDomains = $state<string[]>([]);

    async fetchScannedDomains() {
        try {
            const { invoke } = await import('@tauri-apps/api/core');
            this.historicDomains = await invoke<string[]>('get_unique_scanned_domains');
        } catch (e) {
            console.error("Failed to fetch historical scanned domains: ", e);
        }
    }
}

export const appState = new AppState();
