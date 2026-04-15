<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Bug, ExternalLink } from 'lucide-svelte';

    let { vulns = [] }: { vulns: Array<{id: string, source: string, vuln_type: string, description: string, severity: {level: string, score: number}}> } = $props();

    function getSeverityStyles(level: string) {
        if (level === 'Critical') return 'bg-red-500/20 text-red-400 border-red-500/30';
        if (level === 'High') return 'bg-orange-500/20 text-orange-400 border-orange-500/30';
        if (level === 'Medium') return 'bg-yellow-500/20 text-yellow-500 border-yellow-500/30';
        if (level === 'Low') return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30';
        return 'bg-surface/50 text-muted border-base';
    }

    function getSeverityLabel(level: string) {
        if (level === 'Critical') return m.sec_nmap_severity_critical();
        if (level === 'High') return m.sec_nmap_severity_high();
        if (level === 'Medium') return m.sec_nmap_severity_medium();
        if (level === 'Low') return m.sec_nmap_severity_low();
        return m.sec_nmap_severity_unknown();
    }
</script>

<div class="border border-subtle bg-glass rounded-xl overflow-hidden backdrop-blur-xl shadow-2xl">
    <div class="overflow-x-auto">
        <table class="w-full text-left text-sm text-primary-text">
            <thead class="bg-glass border-b border-glass text-muted font-medium whitespace-nowrap">
                <tr>
                    <th class="px-6 py-4">{m.sec_nmap_col_cve_id()}</th>
                    <th class="px-6 py-4">{m.sec_nmap_col_severity()}</th>
                    <th class="px-6 py-4 w-full">{m.sec_nmap_col_description()}</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-subtle">
                {#each vulns as v (v.id + v.description.slice(0, 10))}
                    <tr class="hover:bg-glass-hover transition-colors group">
                        <td class="px-6 py-4 align-top">
                            <div class="flex flex-col gap-2">
                                <span class="font-mono font-medium text-primary-text group-hover:text-red-400 transition-colors drop-shadow-[0_0_8px_rgba(248,113,113,0.3)]">{v.id}</span>
                                <span class="text-[10px] uppercase font-bold tracking-wider text-muted">{v.source}</span>
                            </div>
                        </td>
                        <td class="px-6 py-4 align-top">
                            <div class="flex flex-col gap-2">
                                <span class={`px-2.5 py-1 text-xs font-medium rounded-full border w-fit ${getSeverityStyles(v.severity.level)}`}>
                                    {getSeverityLabel(v.severity.level)}
                                </span>
                                {#if v.severity.score > 0}
                                    <span class="text-xs font-mono text-muted pl-1">CVSS: {v.severity.score}</span>
                                {/if}
                            </div>
                        </td>
                        <td class="px-6 py-4 align-top text-muted text-sm leading-relaxed max-w-xl">
                            <div class="mb-2">
                                <span class="px-2 py-0.5 text-[9px] uppercase tracking-wider border border-subtle text-primary-text rounded bg-surface/50">TYPE: {v.vuln_type}</span>
                            </div>
                            {v.description}
                            {#if v.id !== 'N/A' && v.source === 'NVD'}
                                <div class="mt-2 text-xs">
                                    <a href={`https://nvd.nist.gov/vuln/detail/${v.id}`} target="_blank" rel="noreferrer" class="inline-flex items-center gap-1 text-teal-400 hover:text-teal-300 transition-colors">
                                        Exploit Reference <ExternalLink size={12} />
                                    </a>
                                </div>
                            {/if}
                        </td>
                    </tr>
                {/each}
                {#if vulns.length === 0}
                    <tr>
                        <td colspan="3" class="px-6 py-12 text-center text-muted">
                            <div class="flex flex-col items-center justify-center gap-2">
                                <Bug size={32} class="opacity-20" />
                                No zero-day vulnerabilities detected on exposed versions.
                            </div>
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>
