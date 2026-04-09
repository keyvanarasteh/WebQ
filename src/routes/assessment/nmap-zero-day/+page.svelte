<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { Play, Loader2, Server, Bug, Crosshair } from 'lucide-svelte';
    import * as m from '$lib/paraglide/messages';
    
    import NmapPortsGrid from '$lib/components/assessment/nmap-zero-day/NmapPortsGrid.svelte';
    import NmapVulnGrid from '$lib/components/assessment/nmap-zero-day/NmapVulnGrid.svelte';
    import NmapGuide from '$lib/components/assessment/nmap-zero-day/NmapGuide.svelte';
    import { HelpCircle } from 'lucide-svelte';

    let domain = $state('');
    let loading = $state(false);
    let error = $state<string | null>(null);
    let result = $state<any>(null);
    let showGuide = $state(false);

    async function analyzeDomain() {
        if (!domain.trim()) {
            error = "Please enter a valid domain (e.g. hackthissite.org)";
            return;
        }

        loading = true;
        error = null;
        result = null;

        try {
            const response = await invoke('scan_nmap_zero_day', { domain });
            console.log("Nmap Zero Day result:", response);
            result = response;
        } catch (e: any) {
            console.error("Nmap Zero Day error:", e);
            error = e?.toString() || 'Failed to execute Nmap scan';
        } finally {
            loading = false;
        }
    }
</script>

<div class="h-full flex flex-col p-6 overflow-y-auto">
    <!-- Header Area -->
    <div class="flex items-center gap-4 mb-8">
        <div class="w-12 h-12 bg-rose-500/10 rounded-xl flex items-center justify-center border border-rose-500/20 shadow-[0_0_15px_rgba(244,63,94,0.15)]">
            <Crosshair class="text-rose-400" size={24} />
        </div>
        <div>
            <h1 class="text-2xl font-bold text-primary-text tracking-tight">{m.sec_nmap_title()}</h1>
            <p class="text-muted text-sm mt-1 max-w-2xl">
                {m.sec_nmap_desc()}
            </p>
        </div>
        <button
            onclick={() => showGuide = true}
            class="ml-auto flex items-center gap-2 px-4 py-2 bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 text-sm font-medium rounded-lg border border-rose-500/20 transition-colors"
        >
            <HelpCircle size={16} />
            SecOps Guide
        </button>
    </div>

    <!-- Input Form Area -->
    <div class="flex gap-4 mb-8">
        <div class="relative flex-1">
            <input
                type="text"
                bind:value={domain}
                onkeydown={(e) => e.key === 'Enter' && analyzeDomain()}
                placeholder="example.com or 192.168.1.1"
                class="w-full bg-glass border border-glass rounded-xl px-5 py-3.5 text-primary-text placeholder:text-muted focus:outline-none focus:ring-2 focus:ring-rose-500/50 focus:border-rose-500/50 transition-all font-mono"
                disabled={loading}
            />
            <div class="absolute inset-y-0 right-4 flex items-center pointer-events-none">
                <div class="w-2 h-2 rounded-full bg-rose-500/50 shadow-[0_0_8px_rgba(244,63,94,0.8)] animate-pulse"></div>
            </div>
        </div>

        <button
            onclick={analyzeDomain}
            disabled={loading || !domain.trim()}
            class="bg-gradient-to-r from-rose-600 to-rose-500 hover:from-rose-500 hover:to-rose-400 text-primary-text px-8 py-3.5 rounded-xl font-medium flex items-center gap-2 transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-[0_0_15px_rgba(244,63,94,0.2)]"
        >
            {#if loading}
                <Loader2 size={18} class="animate-spin" />
                <span>Scanning...</span>
            {:else}
                <Play size={18} />
                <span>Scan Target</span>
            {/if}
        </button>
    </div>

    <!-- Error State -->
    {#if error}
        <div class="bg-red-500/10 border border-red-500/20 text-red-400 p-4 rounded-xl mb-8 flex items-start gap-3 backdrop-blur-sm">
            <div class="mt-0.5">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
            </div>
            <div>
                <h4 class="font-medium text-red-300">Analysis Error</h4>
                <p class="text-sm mt-1">{error}</p>
            </div>
        </div>
    {/if}

    <!-- Result Layout -->
    {#if result}
        <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 items-start">
            
            <div class="flex flex-col gap-6">
                <!-- IP Meta Data -->
                <div class="flex items-center gap-3 bg-glass border border-subtle rounded-xl p-4">
                    <span class="text-muted text-sm">{m.sec_nmap_ip_resolved()}:</span>
                    <span class="font-mono text-primary-text tracking-widest">{result.ip}</span>
                </div>

                <!-- Ports Configuration -->
                <div class="flex flex-col gap-3">
                    <h2 class="text-lg font-medium text-primary-text flex items-center gap-2">
                        <Server size={18} class="text-rose-400" />
                        {m.sec_nmap_open_ports()}
                        <span class="bg-rose-500/20 text-rose-400 text-xs py-0.5 px-2 rounded-full border border-rose-500/30">
                            {result.open_ports.length} Found
                        </span>
                    </h2>
                    <NmapPortsGrid ports={result.open_ports} />
                </div>
            </div>

            <!-- Vulnerability Grid -->
            <div class="flex flex-col gap-3">
                <h2 class="text-lg font-medium text-primary-text flex items-center gap-2">
                    <Bug size={18} class="text-rose-400" />
                    {m.sec_nmap_vulnerabilities()}
                    {#if result.vulnerabilities.length > 0}
                        <span class="bg-red-500/20 text-red-400 text-xs py-0.5 px-2 rounded-full border border-red-500/30">
                            {result.vulnerabilities.length} Detected
                        </span>
                    {/if}
                </h2>
                <NmapVulnGrid vulns={result.vulnerabilities} />
            </div>
        </div>
    {/if}

    <NmapGuide bind:isOpen={showGuide} />
</div>
