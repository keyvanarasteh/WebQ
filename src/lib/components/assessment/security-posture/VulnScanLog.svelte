<script lang="ts">
    import { Crosshair, ShieldAlert } from 'lucide-svelte';

    let { vulnScan }: {
        vulnScan: {
            vulnerabilities_found: number,
            vulnerabilities: Array<{vuln_type: string, severity: string, description: string}>,
            risk_level: string
        }
    } = $props();

</script>

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6 relative overflow-hidden group">
    <div class="absolute -left-10 -bottom-10 opacity-[0.03] group-hover:opacity-10 transition-opacity pointer-events-none">
        <Crosshair size={180} />
    </div>

    <div class="flex items-center justify-between mb-6 relative">
        <div class="flex items-center gap-3">
            <ShieldAlert class="w-5 h-5 text-rose-500" />
            <h3 class="text-lg font-medium text-primary-text">Static Vulnerabilities</h3>
        </div>
        {#if vulnScan.vulnerabilities_found > 0}
            <div class="px-2.5 py-1 text-xs uppercase font-bold rounded-md bg-rose-500/20 border border-rose-500/30 text-rose-400">
                {vulnScan.vulnerabilities_found} Critical Weaknesses
            </div>
        {:else}
            <div class="px-2.5 py-1 text-xs uppercase font-bold rounded-md bg-emerald-500/10 border border-emerald-500/20 text-emerald-400">
                Clean
            </div>
        {/if}
    </div>

    {#if vulnScan.vulnerabilities_found > 0}
        <div class="space-y-3 relative">
            {#each vulnScan.vulnerabilities as v}
                <div class="p-3 bg-rose-500/5 border border-rose-500/10 rounded-lg hover:border-rose-500/30 transition-colors">
                    <div class="flex items-center justify-between mb-2">
                        <span class="text-sm font-semibold text-rose-300">{v.vuln_type}</span>
                        <span class="text-[10px] uppercase tracking-wider font-bold text-rose-400/80 bg-rose-500/10 px-2 py-0.5 rounded">
                            {v.severity}
                        </span>
                    </div>
                    <p class="text-xs text-muted leading-relaxed">
                        {v.description}
                    </p>
                </div>
            {/each}
        </div>
    {:else}
        <div class="text-center py-8">
            <div class="inline-flex items-center justify-center p-3 rounded-full bg-emerald-500/10 border border-emerald-500/20 mb-3 text-emerald-400 relative">
                <Crosshair size={24} />
                <div class="absolute inset-0 border border-emerald-400/30 rounded-full animate-ping opacity-20"></div>
            </div>
            <p class="text-sm text-emerald-400/80">No immediate structural vulnerabilities scanned passively.</p>
        </div>
    {/if}
</div>
