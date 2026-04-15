<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    import { Menu, FileDown } from 'lucide-svelte';
    import ReportExporterModal from '$lib/components/reports/ReportExporterModal.svelte';
    import SegmentedControl from '$lib/components/ui/SegmentedControl.svelte';

    let showExportModal = $state(false);

    // Light/Dark segmented
    const themeOptions = [
        { label: 'Dark', value: 'dark' },
        { label: 'Light', value: 'light' }
    ];

    // Language segmented
    const langOptions = [
        { label: 'EN', value: 'en' },
        { label: 'TR', value: 'tr' }
    ];
    let currentLang = $state(typeof localStorage !== 'undefined' ? (localStorage.getItem('webq-lang') || 'en') : 'en');
    
    $effect(() => {
        if (typeof window !== 'undefined') {
            const savedLang = localStorage.getItem('webq-lang') || 'en';
            if (currentLang !== savedLang && (currentLang === 'en' || currentLang === 'tr')) {
                localStorage.setItem('webq-lang', currentLang);
                window.location.reload();
            }
        }
    });
</script>

<header class="h-16 w-full border-b border-base bg-background/80 backdrop-blur-md flex items-center justify-between px-6 z-10">
    <button class="text-muted hover:text-primary-text transition-colors" onclick={() => appState.sidebarOpen = !appState.sidebarOpen}>
        <Menu class="size-5" />
    </button>

    <div class="flex items-center gap-4">
        {#if appState.isScanning}
            <div class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-accent text-xs font-semibold tracking-wide">
                <span class="relative flex h-2 w-2">
                  <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-cyan-400 opacity-75"></span>
                  <span class="relative inline-flex rounded-full h-2 w-2 bg-cyan-500"></span>
                </span>
                SCAN: {appState.activeModule ? appState.activeModule.toUpperCase() : ''}
            </div>
        {/if}

        <button class="text-muted hover:text-accent p-2 rounded-md hover:bg-surface transition-all border border-base flex items-center gap-1" onclick={() => showExportModal = true} title="Export Security Report">
            <FileDown class="size-4" />
            <span class="text-xs font-bold hidden md:inline">REPORT</span>
        </button>

        <div class="hidden sm:flex scale-90 origin-right items-center gap-2">
            <SegmentedControl options={langOptions} bind:value={currentLang} name="langToggle" />
        </div>
        <div class="hidden sm:block scale-90 origin-right">
            <SegmentedControl options={themeOptions} bind:value={appState.theme} name="themeToggleTop" />
        </div>
    </div>
</header>

<ReportExporterModal bind:open={showExportModal} />
