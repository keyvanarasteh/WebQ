<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { invoke } from '@tauri-apps/api/core';
    import { ShieldQuestion, Loader2, AlertCircle, History } from 'lucide-svelte';
    import BypassGrid from '$lib/components/assessment/cloudflare-bypass/BypassGrid.svelte';
    import BypassGuide from '$lib/components/assessment/cloudflare-bypass/BypassGuide.svelte';
    import { slide } from 'svelte/transition';

    let targetDomain = $state('');
    let isScanning = $state(false);
    let scanResult = $state<{
        domain: string;
        cloudflare_protected: boolean;
        found_ips: Array<{ip: string, source: string, confidence: string, description?: string, status?: string}>;
        scan_time_ms: number;
    } | null>(null);
    let errorMessage = $state<string | null>(null);

    async function runScan() {
        if (!targetDomain.trim()) {
            errorMessage = "Domain is required.";
            return;
        }

        isScanning = true;
        errorMessage = null;
        scanResult = null;

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
        <h1 class="text-3xl font-light text-white tracking-tight flex items-center gap-3">
            <ShieldQuestion class="text-teal-400" size={32} />
            {m.sec_cfbypass_title()}
        </h1>
        <p class="text-zinc-400 mt-2 text-lg">{m.sec_cfbypass_desc()}</p>
    </div>

    <div class="bg-black/40 border border-white/5 rounded-2xl p-6 shadow-2xl backdrop-blur-xl">
        <div class="grid grid-cols-1 gap-6">
            <div class="space-y-4">
                <div class="space-y-2">
                    <label for="domain" class="text-sm font-medium text-zinc-300">Target Domain</label>
                    <input
                        id="domain"
                        type="text"
                        bind:value={targetDomain}
                        placeholder="e.g. example.com"
                        class="w-full bg-black/50 border border-white/10 rounded-xl px-4 py-3 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-teal-500/50 transition-all font-mono text-sm"
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
                        class="bg-white text-black hover:bg-zinc-200 disabled:bg-white/10 disabled:text-white/30 disabled:cursor-not-allowed px-6 py-3 rounded-xl font-medium transition-all flex items-center gap-2 shadow-[0_0_20px_rgba(255,255,255,0.1)] hover:shadow-[0_0_25px_rgba(255,255,255,0.2)]"
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

    <BypassGuide />

    {#if scanResult}
        <div transition:slide class="pt-4">
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-xl font-medium text-white flex items-center gap-2">
                    Evaluation Results
                </h3>
                <div class="text-sm text-zinc-400">
                    {m.sec_cfbypass_stats({ count: scanResult.found_ips.length })}
                    <span class="mx-2 opacity-30">|</span>
                    Scan time: {scanResult.scan_time_ms}ms
                </div>
            </div>

            <BypassGrid ips={scanResult.found_ips} />
        </div>
    {/if}

</div>
