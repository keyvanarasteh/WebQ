<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { FileLock2, AlertCircle, CheckCircle2 } from 'lucide-svelte';

    let { headersAnalysis }: {
        headersAnalysis: {
            score: number,
            missing_critical: string[],
            missing_high: string[],
            headers: Record<string, { present: boolean, value: string, importance: string, security_level: string }>
        }
    } = $props();

    // Svelte 5 derived filter for non-missing headers
    let presentHeaders = $derived(
        Object.entries(headersAnalysis.headers)
            .filter(([_, data]) => data.present)
    );
</script>

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6">
    <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
            <FileLock2 class="w-5 h-5 text-accent" />
            <h3 class="text-lg font-medium text-primary-text">{m.sec_posture_headers()}</h3>
        </div>
        <div class="flex gap-2">
            {#if headersAnalysis.missing_critical.length > 0}
                <span class="px-2.5 py-1 text-[10px] uppercase font-bold rounded-full bg-red-500/20 text-red-300">
                    {headersAnalysis.missing_critical.length} Critical Missing
                </span>
            {/if}
            {#if headersAnalysis.missing_high.length > 0}
                <span class="px-2.5 py-1 text-[10px] uppercase font-bold rounded-full bg-yellow-500/20 text-yellow-300">
                    {headersAnalysis.missing_high.length} High Missing
                </span>
            {/if}
        </div>
    </div>

    {#if headersAnalysis.missing_critical.length > 0 || headersAnalysis.missing_high.length > 0}
        <div class="mb-6 space-y-2">
            <h4 class="text-xs font-semibold text-muted uppercase tracking-widest mb-3">Missing Headers</h4>
            {#each headersAnalysis.missing_critical as missing}
                <div class="flex items-center gap-2 p-2.5 rounded text-sm bg-red-500/10 border border-red-500/20 text-red-300 font-mono">
                    <AlertCircle class="w-4 h-4 shrink-0" />
                    {missing}
                </div>
            {/each}
            {#each headersAnalysis.missing_high as missing}
                <div class="flex items-center gap-2 p-2.5 rounded text-sm bg-yellow-500/10 border border-yellow-500/20 text-yellow-300 font-mono">
                    <AlertCircle class="w-4 h-4 shrink-0" />
                    {missing}
                </div>
            {/each}
        </div>
    {/if}

    {#if presentHeaders.length > 0}
        <div>
            <h4 class="text-xs font-semibold text-muted uppercase tracking-widest mb-3">Active Headers</h4>
            <div class="space-y-1">
                {#each presentHeaders as [key, value] (key)}
                    <div class="flex flex-col gap-2 py-3 border-b border-subtle last:border-0 group">
                        <div class="flex justify-between items-center">
                            <div class="flex items-center gap-2">
                                <CheckCircle2 class="w-3.5 h-3.5 text-emerald-400" />
                                <span class="text-xs font-mono font-medium text-primary-text">{key}</span>
                            </div>
                            <span class="text-xs font-mono text-cyan-300/80 truncate max-w-[150px] sm:max-w-xs group-hover:text-cyan-300 transition-colors cursor-help" title={value.value}>
                                {value.value}
                            </span>
                        </div>
                        <div class="flex justify-end gap-2 pr-2">
                            <span class="px-2 py-[2px] rounded text-[9px] uppercase tracking-wider font-bold border border-surface bg-surface/50 text-muted">
                                {value.importance}
                            </span>
                            <span class={`px-2 py-[2px] rounded text-[9px] uppercase tracking-wider font-bold border ${value.security_level === 'High' ? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-400' : 'border-indigo-500/20 bg-indigo-500/10 text-indigo-400'}`}>
                                {value.security_level} Posture
                            </span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>
