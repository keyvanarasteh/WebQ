<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { FileSearch, Search, Globe, AlertCircle, Loader2, HelpCircle } from 'lucide-svelte';
    import * as m from '$lib/paraglide/messages';
    
    import ScannerMasonry from '$lib/components/recon/advanced-scanner/ScannerMasonry.svelte';
    import AdvancedScannerGuide from '$lib/components/recon/advanced-scanner/AdvancedScannerGuide.svelte';
    import type { ScannerResult } from '$lib/components/recon/advanced-scanner/ScannerMasonry.svelte';

    let targetDomain = $state('');
    let isScanning = $state(false);
    let scanError = $state<string | null>(null);
    let scanResult = $state<ScannerResult | null>(null);
    let isGuideOpen = $state(false);

    async function handleScan() {
        if (!targetDomain) return;
        
        isScanning = true;
        scanError = null;
        scanResult = null;
        
        try {
            const rawResult = await invoke('scan_advanced_content', {
                domain: targetDomain
            });
            scanResult = rawResult as ScannerResult;
        } catch (error) {
            console.error('Advanced Content scan failed:', error);
            scanError = error as string;
        } finally {
            isScanning = false;
        }
    }
</script>

<div class="space-y-6">
    <div class="flex flex-col sm:flex-row sm:items-end justify-between gap-4">
        <div>
            <div class="flex items-center gap-3 mb-2">
                <div class="p-2 border rounded-lg bg-teal-500/10 border-teal-500/20 text-teal-400">
                    <FileSearch class="w-5 h-5" />
                </div>
                <h1 class="text-2xl font-semibold tracking-tight text-primary-text">
                    {m.recon_scanner_title()}
                </h1>
                
                <button
                    onclick={() => isGuideOpen = true}
                    class="p-2 ml-2 transition-colors border rounded-lg bg-surface border-base text-muted hover:text-primary-text"
                    title="View Module Guide"
                >
                    <HelpCircle class="w-4 h-4" />
                </button>
            </div>
            <p class="text-sm text-muted">
                {m.recon_scanner_subtitle()}
            </p>
        </div>
        
        <div class="flex items-center w-full sm:w-auto">
            <div class="relative w-full sm:w-80 group">
                <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                    <Globe class="w-4 h-4 text-muted group-focus-within:text-teal-500 transition-colors" />
                </div>
                <input
                    type="text"
                    bind:value={targetDomain}
                    placeholder="example.com"
                    class="block w-full p-2.5 pl-10 text-sm text-primary-text bg-black border border-base rounded-lg focus:ring-teal-500 focus:border-teal-500 outline-none transition-all placeholder-gray-600"
                    onkeydown={(e) => e.key === 'Enter' && handleScan()}
                    disabled={isScanning}
                />
            </div>
            <button
                onclick={handleScan}
                disabled={isScanning || !targetDomain}
                class="min-w-[120px] flex items-center justify-center gap-2 p-2.5 ml-2 text-sm font-medium text-black bg-teal-500 rounded-lg hover:bg-teal-400 focus:ring-4 focus:outline-none focus:ring-teal-500/50 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
            >
                {#if isScanning}
                    <Loader2 class="w-4 h-4 animate-spin text-black" />
                    <span>Scan</span>
                {:else}
                    <Search class="w-4 h-4" />
                    <span>Analyze</span>
                {/if}
            </button>
        </div>
    </div>

    <!-- Error State -->
    {#if scanError}
        <div class="flex items-start gap-3 p-4 border rounded-xl bg-red-500/5 border-red-500/20 text-red-400">
            <AlertCircle class="w-5 h-5 shrink-0 mt-0.5" />
            <div>
                <h4 class="text-sm font-medium">Scan Engine Error</h4>
                <p class="text-xs text-red-400/80 mt-1">{scanError}</p>
            </div>
        </div>
    {/if}

    <!-- Scanning State Indicator -->
    {#if isScanning}
        <div class="flex flex-col items-center justify-center p-12 py-20 mt-8 border border-dashed rounded-xl border-teal-500/30 bg-teal-500/5">
            <Loader2 class="w-10 h-10 text-teal-500 animate-spin mb-4" />
            <h3 class="font-medium text-primary-text mb-2">{m.recon_scanner_loading()}</h3>
            <p class="text-xs text-muted max-w-md text-center">{m.recon_scanner_info()}</p>
            
            <div class="w-64 h-1.5 mt-8 bg-surface rounded-full overflow-hidden">
                <div class="h-full bg-teal-500 rounded-full w-2/3 animate-pulse"></div>
            </div>
        </div>
    {:else}
        <ScannerMasonry result={scanResult} />
    {/if}

    <AdvancedScannerGuide bind:isOpen={isGuideOpen} />
</div>
