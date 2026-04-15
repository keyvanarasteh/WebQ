<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Server, Activity, CheckCircle2, ShieldAlert, XCircle, Search } from 'lucide-svelte';

    // The data schema matches ApiScanResult mapping
    let { 
        endpoints = [], 
        endpointsTested = 0, 
        totalPathsProbed = 0 
    }: {
        endpoints: Array<{url: string, status_code: number, api_type: string}>,
        endpointsTested: number,
        totalPathsProbed: number
    } = $props();

    function getStatusColor(status: number) {
        if (status >= 200 && status < 300) return 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20';
        if (status >= 300 && status < 400) return 'text-blue-400 bg-blue-500/10 border-blue-500/20';
        if (status >= 400 && status < 500) return 'text-yellow-400 bg-yellow-500/10 border-yellow-500/20';
        return 'text-red-400 bg-red-500/10 border-red-500/20';
    }

    // Derived coverage metric
    let coveragePercent = $derived(
        totalPathsProbed > 0 ? Math.round((endpointsTested / totalPathsProbed) * 100) : 0
    );
</script>

<div class="mt-6 border border-subtle bg-glass rounded-xl overflow-hidden backdrop-blur-xl shadow-2xl">
    <!-- Header with Metrics -->
    <div class="px-5 py-4 border-b border-subtle bg-glass flex flex-wrap gap-4 items-center justify-between">
        <div class="flex items-center gap-3">
            <Server size={18} class="text-indigo-400" />
            <h3 class="text-sm font-medium text-primary-text tracking-wide uppercase">API Ecosystem Directory</h3>
        </div>
        
        <div class="flex items-center gap-6">
            <div class="flex flex-col items-end">
                <span class="text-[10px] uppercase tracking-widest text-muted font-bold">Paths Probed</span>
                <span class="text-sm font-mono text-primary-text">{totalPathsProbed.toLocaleString()}</span>
            </div>
            <div class="flex flex-col items-end">
                <span class="text-[10px] uppercase tracking-widest text-muted font-bold">Endpoints Tested</span>
                <span class="text-sm font-mono text-indigo-400">{endpointsTested.toLocaleString()}</span>
            </div>
            <div class="h-8 w-px bg-subtle mx-1"></div>
            <div class="flex flex-col items-end">
                <span class="text-[10px] uppercase tracking-widest text-muted font-bold">Attack Coverage</span>
                <div class="flex items-center gap-2">
                    <div class="w-16 h-1.5 bg-surface/50 rounded-full overflow-hidden">
                        <div class="h-full bg-indigo-500 shadow-[0_0_8px_rgba(99,102,241,0.6)]" style="width: {coveragePercent}%"></div>
                    </div>
                    <span class="text-xs font-mono text-primary-text">{coveragePercent}%</span>
                </div>
            </div>
        </div>
    </div>

    <!-- Data Grid -->
    <div class="overflow-x-auto max-h-[400px] custom-scrollbar">
        <table class="w-full text-left text-sm text-primary-text">
            <thead class="bg-glass border-b border-subtle text-muted font-medium sticky top-0 z-10 hidden sm:table-header-group">
                <tr>
                    <th class="px-6 py-3 text-[11px] uppercase tracking-wider">Path / URL</th>
                    <th class="px-6 py-3 text-[11px] uppercase tracking-wider">Architecture</th>
                    <th class="px-6 py-3 text-[11px] uppercase tracking-wider text-right">HTTP Status</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-subtle">
                {#each endpoints as endpoint}
                    <tr class="hover:bg-glass-hover transition-colors group">
                        <td class="px-6 py-3 max-w-[300px]">
                            <div class="flex items-center gap-3">
                                <div class="w-2 h-2 rounded-full {endpoint.status_code < 400 ? 'bg-emerald-500' : 'bg-rose-500'}"></div>
                                <span class="font-mono text-primary-text truncate group-hover:text-indigo-300 transition-colors" title={endpoint.url}>
                                    {endpoint.url}
                                </span>
                            </div>
                        </td>
                        <td class="px-6 py-3">
                            <span class="px-2 py-0.5 text-[10px] uppercase tracking-wider font-bold rounded-md bg-glass border border-subtle text-muted">
                                {endpoint.api_type}
                            </span>
                        </td>
                        <td class="px-6 py-3 text-right">
                            <span class={`font-mono text-xs px-2 py-0.5 rounded border ${getStatusColor(endpoint.status_code)}`}>
                                {endpoint.status_code}
                            </span>
                        </td>
                    </tr>
                {/each}
                
                {#if endpoints.length === 0}
                    <tr>
                        <td colspan="3" class="px-6 py-12">
                            <div class="flex flex-col items-center justify-center text-muted">
                                <Search size={24} class="mb-3 opacity-50" />
                                <span class="text-sm font-mono tracking-widest uppercase">No API Endpoints Discovered</span>
                            </div>
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 6px; height: 6px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
</style>
