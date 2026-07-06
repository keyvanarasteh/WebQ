<script lang="ts">
    import { X, FileJson, FileText, FileBadge, FileArchive } from 'lucide-svelte';
    import { reportStore } from '$lib/stores/ReportStore.svelte';
    import { appState } from '$lib/stores/AppState.svelte';
    import { exportToJson, exportToMarkdown, exportToPdf, exportToDocx } from '$lib/utils/export';
    import { invoke } from '@tauri-apps/api/core';
    import { toast } from 'svelte-sonner';

    let { open = $bindable(false) } : { open: boolean } = $props();

    type ScanRow = {
        id: string;
        target_domain: string;
        scan_module: string;
        status: string;
        started_at: string;
    };

    let domains = $state<string[]>([]);
    let selectedDomain = $state('');
    let isLoadingDomains = $state(false);
    let reportCache = $state<Record<string, Record<string, unknown>>>({});

    $effect(() => {
        if (open) {
            void refreshDomains();
        }
    });

    $effect(() => {
        if (domains.length > 0 && (!selectedDomain || !domains.includes(selectedDomain))) {
            selectedDomain = domains[0] ?? '';
        } else if (domains.length === 0) {
            selectedDomain = '';
        }
    });

    async function refreshDomains() {
        isLoadingDomains = true;

        try {
            const dbDomains = await invoke<string[]>('get_unique_scanned_domains');
            appState.historicDomains = dbDomains;
            domains = uniqueDomains([...dbDomains, ...reportStore.getAvailableDomains()]);
        } catch (e) {
            console.error('Failed to load report domains', e);
            domains = reportStore.getAvailableDomains();
        } finally {
            isLoadingDomains = false;
        }
    }

    function uniqueDomains(values: string[]) {
        return values.filter((domain, index, all) => domain && all.indexOf(domain) === index);
    }

    async function getReportData(domain: string): Promise<Record<string, unknown>> {
        const cached = reportCache[domain];
        if (cached && Object.keys(cached).length > 0) {
            return cached;
        }

        const data: Record<string, unknown> = { ...reportStore.getReportForDomain(domain) };

        try {
            const scans = await invoke<ScanRow[]>('get_scans_paginated', {
                limit: 100,
                offset: 0,
                filterDomain: domain,
                filterModule: null,
                filterStatus: 'Completed',
                dateFrom: null,
                dateTo: null,
                sortBy: 'started_at',
                sortDir: 'desc'
            });

            for (const scan of scans) {
                if (data[scan.scan_module]) continue;
                data[scan.scan_module] = await invoke<unknown>('get_scan_blob_details', { scanId: scan.id });
            }
        } catch (e) {
            console.error('Failed to hydrate report data', e);
        }

        reportCache = { ...reportCache, [domain]: data };
        return data;
    }

    async function handleExport(format: 'json' | 'md' | 'pdf' | 'docx') {
        if (!selectedDomain) {
            toast.error("Please select a domain to export.");
            return;
        }

        const data = await getReportData(selectedDomain);
        if (Object.keys(data).length === 0) {
            toast.error("No data available for the selected domain.");
            return;
        }

        try {
            let success = false;
            toast.info(`Generating ${format.toUpperCase()} report...`);
            
            open = false; // Close modal

            if (format === 'json') success = await exportToJson(selectedDomain, data);
            else if (format === 'md') success = await exportToMarkdown(selectedDomain, data);
            else if (format === 'pdf') success = await exportToPdf(selectedDomain, data);
            else if (format === 'docx') success = await exportToDocx(selectedDomain, data);

            if (success) {
                toast.success(`Report exported successfully!`);
            }
        } catch (e) {
            toast.error(`Failed to export ${format.toUpperCase()} report`);
        }
    }
</script>

{#if open}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-overlay backdrop-blur-sm">
        <div class="bg-background border border-base rounded-xl shadow-2xl w-full max-w-md overflow-hidden relative">
            
            <div class="border-b border-base p-4 flex items-center justify-between bg-surface/50">
                <h2 class="font-bold text-lg text-primary-text flex items-center gap-2">
                    <FileBadge class="size-5 text-cyan-500" />
                    Export Security Report
                </h2>
                <button class="text-muted hover:text-rose-400 transition-colors" onclick={() => open = false}>
                    <X class="size-5" />
                </button>
            </div>

            <div class="p-6 space-y-6">
                <div>
                    <label class="block text-xs font-bold text-muted mb-2 tracking-widest uppercase" for="domain-select">Select Target Domain</label>
                    <select id="domain-select" bind:value={selectedDomain} class="w-full bg-surface border border-base rounded-lg p-3 text-sm text-primary-text outline-none focus:border-cyan-500/50 focus:ring-1 focus:ring-cyan-500/50 transition-all appearance-none cursor-pointer">
                        <option value="" disabled>Select a domain...</option>
                        {#each domains as domain (domain)}
                            <option value={domain}>{domain}</option>
                        {/each}
                    </select>
                    {#if isLoadingDomains}
                        <p class="text-[10px] text-cyan-500 mt-2 font-mono">Loading scan history...</p>
                    {:else if domains.length === 0}
                        <p class="text-[10px] text-amber-500 mt-2 font-mono">No scan data available. Run a scan first.</p>
                    {/if}
                </div>

                <div>
                    <label class="block text-xs font-bold text-muted mb-2 tracking-widest uppercase" for="export-format">Export Format</label>
                    <div class="grid grid-cols-4 gap-3" id="export-format">
                        <button onclick={() => handleExport('json')} disabled={!selectedDomain} class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border border-base bg-surface/30 hover:bg-surface hover:border-emerald-500/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed group">
                            <FileJson class="size-6 text-emerald-400 group-hover:scale-110 transition-transform" />
                            <span class="text-xs font-bold text-primary-text tracking-widest">JSON</span>
                        </button>
                        
                        <button onclick={() => handleExport('md')} disabled={!selectedDomain} class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border border-base bg-surface/30 hover:bg-surface hover:border-blue-500/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed group">
                            <FileText class="size-6 text-blue-400 group-hover:scale-110 transition-transform" />
                            <span class="text-xs font-bold text-primary-text tracking-widest">MD</span>
                        </button>
                        
                        <button onclick={() => handleExport('pdf')} disabled={!selectedDomain} class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border border-base bg-surface/30 hover:bg-surface hover:border-rose-500/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed group">
                            <FileBadge class="size-6 text-rose-400 group-hover:scale-110 transition-transform" />
                            <span class="text-xs font-bold text-primary-text tracking-widest">PDF</span>
                        </button>

                        <button onclick={() => handleExport('docx')} disabled={!selectedDomain} class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border border-base bg-surface/30 hover:bg-surface hover:border-violet-500/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed group">
                            <FileArchive class="size-6 text-violet-400 group-hover:scale-110 transition-transform" />
                            <span class="text-xs font-bold text-primary-text tracking-widest">DOCX</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}
