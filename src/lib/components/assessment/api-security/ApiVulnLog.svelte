<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { AlertTriangle, Terminal, Code2, Globe } from 'lucide-svelte';

    let { vulnerabilities = [] }: { 
        vulnerabilities: Array<{
            vuln_type: string, 
            subtype: string, 
            endpoint: string, 
            parameter: string, 
            payload: string, 
            severity: string, 
            confidence: string, 
            evidence: string
        }> 
    } = $props();

    function getSeverityColor(sev: string) {
        if (sev === 'CRITICAL') return 'text-red-400 bg-red-400/10 border-red-400/20';
        if (sev === 'HIGH') return 'text-orange-400 bg-orange-400/10 border-orange-400/20';
        if (sev === 'MEDIUM') return 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20';
        return 'text-emerald-400 bg-emerald-400/10 border-emerald-400/20';
    }
</script>

<div class="border border-white/5 bg-black/40 rounded-xl overflow-hidden backdrop-blur-xl flex flex-col max-h-[600px]">
    <div class="bg-white/5 px-4 py-3 border-b border-white/5 flex items-center gap-2">
        <Terminal size={16} class="text-zinc-400" />
        <h3 class="text-sm font-medium text-zinc-300 tracking-wide uppercase">{m.sec_api_vuln_log()}</h3>
        <div class="ml-auto flex items-center gap-2">
            <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-red-500/10 border border-red-500/20">
                <div class="w-1.5 h-1.5 rounded-full bg-red-400 animate-pulse"></div>
                <span class="text-[10px] font-mono text-red-400">LIVE FEED</span>
            </div>
        </div>
    </div>
    
    <div class="overflow-y-auto flex-1 p-0 custom-scrollbar">
        {#if vulnerabilities.length === 0}
            <div class="h-32 flex items-center justify-center text-zinc-500 text-sm font-mono border-b border-white/5">
                [ No Vulnerabilities Injected into Matrix Log ]
            </div>
        {/if}

        <div class="divide-y divide-white/5">
            {#each vulnerabilities as v, i (v.endpoint + v.payload + i)}
                <div class="p-4 hover:bg-white/[0.02] transition-colors group">
                    <div class="flex flex-wrap items-start justify-between gap-4 mb-2">
                        <div class="flex items-center gap-2">
                            <span class={`px-2 py-0.5 text-[10px] font-bold tracking-wider rounded border ${getSeverityColor(v.severity)}`}>
                                {v.severity}
                            </span>
                            <span class="text-sm font-medium text-white">{v.vuln_type}</span>
                            <span class="text-xs text-zinc-500 hidden sm:inline">- {v.subtype}</span>
                        </div>
                        <span class="text-xs font-mono text-zinc-500 px-2 py-0.5 bg-black/40 rounded border border-white/5">
                            CONFIDENCE: {v.confidence}
                        </span>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-3">
                        <div class="bg-black/40 rounded-lg p-3 border border-white/5 relative overflow-hidden">
                            <div class="flex items-center gap-2 mb-2 text-zinc-500">
                                <Globe size={14} />
                                <span class="text-xs font-medium uppercase tracking-wider">{m.sec_api_vuln_endpoint()}</span>
                            </div>
                            <div class="text-xs font-mono text-teal-400 break-all leading-relaxed">
                                {v.endpoint}
                            </div>
                            {#if v.parameter}
                                <div class="mt-2 text-xs font-mono">
                                    <span class="text-zinc-500">Param: </span><span class="text-orange-400">?{v.parameter}=</span>
                                </div>
                            {/if}
                        </div>

                        <div class="bg-red-500/5 rounded-lg p-3 border border-red-500/10 relative overflow-hidden">
                            <div class="flex items-center gap-2 mb-2 text-red-500/50">
                                <Code2 size={14} />
                                <span class="text-xs font-medium uppercase tracking-wider">{m.sec_api_vuln_payload()}</span>
                            </div>
                            <div class="text-xs font-mono text-red-400 break-all leading-relaxed">
                                {v.payload}
                            </div>
                            <div class="mt-2 text-[10px] font-mono text-zinc-500 leading-snug border-t border-red-500/10 pt-2">
                                <span class="text-zinc-600">EVIDENCE: </span> {v.evidence}
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 6px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.2);
    }
</style>
