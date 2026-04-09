<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    import { Sun, Moon, Menu, FileDown } from 'lucide-svelte';
    import ReportExporterModal from '$lib/components/reports/ReportExporterModal.svelte';

    let showExportModal = $state(false);
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

        <button class="text-xs font-bold text-muted hover:text-accent p-2 rounded-md hover:bg-surface transition-all border border-base" onclick={() => {
            const currentLang = localStorage.getItem('webq-lang') || 'en';
            const nextLang = currentLang === 'en' ? 'tr' : 'en';
            localStorage.setItem('webq-lang', nextLang);
            window.location.reload();
        }}>
            {typeof localStorage !== 'undefined' && localStorage.getItem('webq-lang') === 'tr' ? 'TR' : 'EN'}
        </button>

        <button class="text-muted hover:text-accent p-2 rounded-md hover:bg-surface transition-all" onclick={() => appState.toggleTheme()}>
            {#if appState.theme === 'dark'}
                <Sun class="size-5" />
            {:else}
                <Moon class="size-5" />
            {/if}
        </button>
    </div>
</header>

<ReportExporterModal bind:open={showExportModal} />
