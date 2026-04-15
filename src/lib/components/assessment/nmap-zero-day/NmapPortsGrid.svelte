<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { ServerCog, TerminalSquare, HelpCircle } from 'lucide-svelte';
    import NmapPortsGridGuide from './NmapPortsGridGuide.svelte';

    let { ports = [] }: { ports: Array<{port: number, state: string, service: string, version: string, product?: string, cpe?: string[]}> } = $props();

    let guideOpen = $state(false);
</script>

<NmapPortsGridGuide bind:isOpen={guideOpen} />
<div class="border border-subtle bg-glass rounded-xl overflow-hidden backdrop-blur-xl shadow-2xl">
    <div class="px-4 py-3 border-b border-subtle flex items-center justify-between bg-glass">
        <div class="flex items-center gap-2">
            <ServerCog size={16} class="text-red-400" />
            <span class="text-sm font-medium text-primary-text tracking-wide uppercase">Open Ports & Services</span>
        </div>
        <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-red-400 hover:bg-red-500/10 border border-transparent hover:border-red-500/20 transition-all" title="How this works">
            <HelpCircle class="size-4" />
        </button>
    </div>
    <div class="overflow-x-auto">
        <table class="w-full text-left text-sm text-primary-text">
            <thead class="bg-glass border-b border-glass text-muted font-medium">
                <tr>
                    <th class="px-6 py-4">{m.sec_nmap_col_port()}</th>
                    <th class="px-6 py-4">State</th>
                    <th class="px-6 py-4">{m.sec_nmap_col_service()}</th>
                    <th class="px-6 py-4">{m.sec_nmap_col_version()} & CPE</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-subtle">
                {#each ports as p (p.port)}
                    <tr class="hover:bg-glass-hover transition-colors group">
                        <td class="px-6 py-4">
                            <div class="flex items-center gap-3">
                                <div class="w-8 h-8 rounded-lg bg-emerald-400/10 flex items-center justify-center text-emerald-400 border border-emerald-400/20">
                                    <ServerCog size={16} />
                                </div>
                                <span class="font-mono font-medium text-emerald-400 drop-shadow-[0_0_8px_rgba(52,211,153,0.3)]">{p.port}</span>
                            </div>
                        </td>
                        <td class="px-6 py-4">
                            <span class={`px-2 py-0.5 text-[10px] uppercase font-bold tracking-wider rounded ${p.state === 'open' ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : 'bg-yellow-500/20 text-yellow-500 border border-yellow-500/30'}`}>
                                {p.state}
                            </span>
                        </td>
                        <td class="px-6 py-4">
                            <span class="px-2.5 py-1 text-xs font-mono rounded-md bg-glass text-primary-text border border-glass">
                                {p.service || 'unknown'}
                            </span>
                        </td>
                        <td class="px-6 py-4">
                            <div class="flex flex-col gap-2">
                                <div class="flex items-center gap-2 text-muted">
                                    <TerminalSquare size={14} />
                                    <span>{p.product || 'N/A'} {p.version}</span>
                                </div>
                                {#if p.cpe && p.cpe.length > 0}
                                    <div class="flex flex-wrap gap-1">
                                        {#each p.cpe as cp}
                                            <span class="text-[9px] font-mono text-indigo-300 bg-indigo-500/10 px-1.5 py-[1px] rounded border border-indigo-500/20">
                                                {cp}
                                            </span>
                                        {/each}
                                    </div>
                                {/if}
                            </div>
                        </td>
                    </tr>
                {/each}
                {#if ports.length === 0}
                    <tr>
                        <td colspan="4" class="px-6 py-12 text-center text-muted">
                            No open ports discovered. The server might be blocking ping or heavily firewalled.
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>
