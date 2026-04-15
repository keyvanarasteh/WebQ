<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { invoke } from '@tauri-apps/api/core';
    import { ShieldQuestion, Loader2, AlertCircle, History, HelpCircle } from 'lucide-svelte';
    import BypassGrid from '$lib/components/assessment/cloudflare-bypass/BypassGrid.svelte';
    import BypassGuide from '$lib/components/assessment/cloudflare-bypass/BypassGuide.svelte';
    import { slide } from 'svelte/transition';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';
    import type { ScanProgressEvent } from '$lib/types/intelligence';

    let targetDomain = $state('');
    let isScanning = $state(false);
    let scanResult = $state<{
        domain: string;
        cloudflare_protected: boolean;
        found_ips: Array<{ip: string, source: string, confidence: string, description?: string, status?: string}>;
        scan_time_ms: number;
    } | null>(null);
    let errorMessage = $state<string | null>(null);
    let showGuide = $state(false);

    let logs = $state<ScanProgressEvent[]>([]);
    let progressPercent = $state(0);

    $effect(() => {
        let unlistenP: UnlistenFn | null = null;
        listen<ScanProgressEvent>('scan-progress', (event) => {
            logs.push(event.payload);
            progressPercent = event.payload.percentage;
        }).then(u => unlistenP = u);
        return () => { if (unlistenP) unlistenP(); };
    });

    async function runScan() {
        if (!targetDomain.trim()) {
            errorMessage = "Domain is required.";
            return;
        }

        isScanning = true;
        errorMessage = null;
        scanResult = null;
        logs = [];
        progressPercent = 0;

        try {
            scanResult = await invoke('scan_cloudflare_bypass', {
                domain: targetDomain.trim()
            });
        } catch (err: unknown) {
            errorMessage = err instanceof Error ? err.message : String(err);
        } finally {
            isScanning = false;
        }
    }
</script>

<div class="max-w-6xl mx-auto space-y-6">
    <div class="mb-8">
        <h1 class="text-3xl font-light text-primary-text tracking-tight flex items-center gap-3">
            <ShieldQuestion class="text-teal-400" size={32} />
            {m.sec_cfbypass_title()}
        </h1>
        <p class="text-muted mt-2 text-lg">{m.sec_cfbypass_desc()}</p>
        <button
            onclick={() => showGuide = true}
            class="mt-3 flex items-center gap-2 px-4 py-2 bg-teal-500/10 hover:bg-teal-500/20 text-teal-400 text-sm font-medium rounded-lg border border-teal-500/20 transition-colors"
        >
            <HelpCircle size={16} />
            SecOps Guide
        </button>
    </div>

    <div class="bg-glass border border-subtle rounded-2xl p-6 shadow-2xl backdrop-blur-xl">
        <div class="grid grid-cols-1 gap-6">
            <div class="space-y-4">
                <div class="space-y-2">
                    <label for="domain" class="text-sm font-medium text-primary-text">Target Domain</label>
                    <input
                        id="domain"
                        type="text"
                        bind:value={targetDomain}
                        placeholder="e.g. example.com"
                        class="w-full bg-glass border border-glass rounded-xl px-4 py-3 text-primary-text placeholder-muted focus:outline-none focus:ring-2 focus:ring-teal-500/50 transition-all font-mono text-sm"
                        disabled={isScanning}
                        onkeydown={(e) => e.key === 'Enter' && runScan()}
                    />
                </div>
                <!-- Warning or Error -->
                {#if errorMessage}
                    <div transition:slide class="bg-red-500/10 border border-red-500/20 rounded-xl p-4 flex gap-3 text-red-400">
                        <AlertCircle size={20} class="shrink-0 mt-0.5" />
                        <div class="text-sm">{errorMessage}</div>
                    </div>
                {/if}

                <div class="flex justify-end pt-2">
                    <button
                        onclick={runScan}
                        disabled={isScanning || !targetDomain}
                        class="bg-surface text-on-accent hover:bg-surface-hover disabled:bg-glass-hover disabled:text-primary-text/30 disabled:cursor-not-allowed px-6 py-3 rounded-xl font-medium transition-all flex items-center gap-2 shadow-[0_0_20px_rgba(255,255,255,0.1)] hover:shadow-[0_0_25px_rgba(255,255,255,0.2)]"
                    >
                        {#if isScanning}
                            <Loader2 size={18} class="animate-spin" />
                            Scanning History...
                        {:else}
                            <History size={18} />
                            Bypass Cloudflare
                        {/if}
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- Active terminal stream overlay -->
    {#if isScanning}
        <div class="mt-6">
            <ScanTerminal {logs} {progressPercent} />
        </div>
    {/if}

    <BypassGuide bind:isOpen={showGuide} />

    {#if scanResult}
        <div transition:slide class="pt-4">
            <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-4 gap-2">
                <h3 class="text-xl font-medium text-primary-text flex items-center gap-2">
                    Evaluation Results
                </h3>
                <div class="text-sm text-muted flex items-center flex-wrap gap-2">
                    {#if scanResult.cloudflare_protected}
                        <span class="text-xs uppercase font-bold tracking-wider text-rose-400 bg-rose-500/10 border border-rose-500/20 px-2 py-0.5 rounded">Behind Cloudflare</span>
                    {:else}
                        <span class="text-xs uppercase font-bold tracking-wider text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 rounded">No WAF Detected</span>
                    {/if}
                    <span class="px-1 text-subtle">|</span>
                    {m.sec_cfbypass_stats({ count: scanResult.found_ips.length })}
                    <span class="px-1 text-subtle">|</span>
                    Scan time: {scanResult.scan_time_ms}ms
                </div>
            </div>

            <BypassGrid ips={scanResult.found_ips} />
        </div>
    {/if}

</div>
