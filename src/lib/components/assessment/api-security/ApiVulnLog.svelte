<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { AlertTriangle, Terminal, Code2, Globe, HelpCircle } from 'lucide-svelte';
    import ApiVulnLogGuide from './ApiVulnLogGuide.svelte';

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

    let guideOpen = $state(false);

    function getSeverityColor(sev: string) {
        if (sev === 'CRITICAL') return 'text-red-400 bg-red-400/10 border-red-400/20';
        if (sev === 'HIGH') return 'text-orange-400 bg-orange-400/10 border-orange-400/20';
        if (sev === 'MEDIUM') return 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20';
        return 'text-emerald-400 bg-emerald-400/10 border-emerald-400/20';
    }
</script>

<ApiVulnLogGuide bind:isOpen={guideOpen} />
<div class="border border-subtle bg-glass rounded-xl overflow-hidden backdrop-blur-xl flex flex-col max-h-[600px]">
    <div class="bg-glass px-4 py-3 border-b border-subtle flex items-center gap-2">
        <Terminal size={16} class="text-muted" />
        <h3 class="text-sm font-medium text-primary-text tracking-wide uppercase">{m.sec_api_vuln_log()}</h3>
        <div class="ml-auto flex items-center gap-2">
            <button
                onclick={() => guideOpen = true}
                class="p-1.5 rounded-lg text-muted hover:text-rose-400 hover:bg-rose-500/10 border border-transparent hover:border-rose-500/20 transition-all"
                title="How this works"
            >
                <HelpCircle class="size-4" />
            </button>
            <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-red-500/10 border border-red-500/20">
                <div class="w-1.5 h-1.5 rounded-full bg-red-400 animate-pulse"></div>
                <span class="text-[10px] font-mono text-red-400">LIVE FEED</span>
            </div>
        </div>
    </div>
    
    <div class="overflow-y-auto flex-1 p-0 custom-scrollbar">
        {#if vulnerabilities.length === 0}
            <div class="h-32 flex items-center justify-center text-muted text-sm font-mono border-b border-subtle">
                [ No Vulnerabilities Injected into Matrix Log ]
            </div>
        {/if}

        <div class="divide-y divide-subtle">
            {#each vulnerabilities as v, i (v.endpoint + v.payload + i)}
                <div class="p-4 hover:bg-glass-hover transition-colors group">
                    <div class="flex flex-wrap items-start justify-between gap-4 mb-2">
                        <div class="flex items-center gap-2">
                            <span class={`px-2 py-0.5 text-[10px] font-bold tracking-wider rounded border ${getSeverityColor(v.severity)}`}>
                                {v.severity}
                            </span>
                            <span class="text-sm font-medium text-primary-text">{v.vuln_type}</span>
                            <span class="text-xs text-muted hidden sm:inline">- {v.subtype}</span>
                        </div>
                        <span class="text-xs font-mono text-muted px-2 py-0.5 bg-glass rounded border border-subtle">
                            CONFIDENCE: {v.confidence}
                        </span>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-3">
                        <div class="bg-glass rounded-lg p-3 border border-subtle relative overflow-hidden">
                            <div class="flex items-center gap-2 mb-2 text-muted">
                                <Globe size={14} />
                                <span class="text-xs font-medium uppercase tracking-wider">{m.sec_api_vuln_endpoint()}</span>
                            </div>
                            <div class="text-xs font-mono text-teal-400 break-all leading-relaxed">
                                {v.endpoint}
                            </div>
                            {#if v.parameter}
                                <div class="mt-2 text-xs font-mono">
                                    <span class="text-muted">Param: </span><span class="text-orange-400">?{v.parameter}=</span>
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
                            <div class="mt-2 text-[10px] font-mono text-muted leading-snug border-t border-red-500/10 pt-2">
                                <span class="text-muted">EVIDENCE: </span> {v.evidence}
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
