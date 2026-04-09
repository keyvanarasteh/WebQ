<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { ServerCog, TerminalSquare } from 'lucide-svelte';

    let { ports = [] }: { ports: Array<{port: number, state: string, service: string, version: string, product?: string, cpe?: string[]}> } = $props();
</script>

<div class="border border-white/5 bg-black/40 rounded-xl overflow-hidden backdrop-blur-xl shadow-2xl">
    <div class="overflow-x-auto">
        <table class="w-full text-left text-sm text-primary-text">
            <thead class="bg-white/5 border-b border-white/10 text-muted font-medium">
                <tr>
                    <th class="px-6 py-4">{m.sec_nmap_col_port()}</th>
                    <th class="px-6 py-4">{m.sec_nmap_col_service()}</th>
                    <th class="px-6 py-4">{m.sec_nmap_col_version()}</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-white/5">
                {#each ports as p (p.port)}
                    <tr class="hover:bg-white/[0.02] transition-colors group">
                        <td class="px-6 py-4">
                            <div class="flex items-center gap-3">
                                <div class="w-8 h-8 rounded-lg bg-emerald-400/10 flex items-center justify-center text-emerald-400 border border-emerald-400/20">
                                    <ServerCog size={16} />
                                </div>
                                <span class="font-mono font-medium text-emerald-400 drop-shadow-[0_0_8px_rgba(52,211,153,0.3)]">{p.port}</span>
                            </div>
                        </td>
                        <td class="px-6 py-4">
                            <span class="px-2.5 py-1 text-xs font-mono rounded-md bg-white/5 text-primary-text border border-white/10">
                                {p.service || 'unknown'}
                            </span>
                        </td>
                        <td class="px-6 py-4">
                            <div class="flex items-center gap-2 text-muted">
                                <TerminalSquare size={14} />
                                <span>{p.product || 'N/A'} {p.version}</span>
                            </div>
                        </td>
                    </tr>
                {/each}
                {#if ports.length === 0}
                    <tr>
                        <td colspan="3" class="px-6 py-12 text-center text-muted">
                            No open ports discovered. The server might be blocking ping or heavily firewalled.
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>
