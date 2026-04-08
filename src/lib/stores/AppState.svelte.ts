class AppState {
    theme = $state<'dark' | 'light'>('dark');
    sidebarOpen = $state(true);
    isScanning = $state(false);
    activeModule = $state<string | null>(null);

    toggleTheme() {
        this.theme = this.theme === 'dark' ? 'light' : 'dark';
    }

    setScanning(status: boolean, moduleName: string) {
        this.isScanning = status;
        this.activeModule = status ? moduleName : null;
    }
}

export const appState = new AppState();
