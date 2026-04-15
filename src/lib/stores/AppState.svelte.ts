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
}

export const appState = new AppState();
