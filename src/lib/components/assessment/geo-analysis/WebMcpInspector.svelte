<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Network, Blocks, CheckCircle2, XCircle, Search } from 'lucide-svelte';

    let { webmcp }: {
        webmcp: {
            found: boolean,
            endpoints: string[],
            html_features: string[]
        }
    } = $props();
</script>

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6">
    <div class="flex items-center justify-between mb-5">
        <div class="flex items-center gap-3">
            <Blocks class={`w-5 h-5 ${webmcp.found ? 'text-indigo-400' : 'text-muted'}`} />
            <h3 class="text-lg font-medium text-primary-text tracking-wide">Model Context Protocol (MCP)</h3>
        </div>
        {#if webmcp.found}
            <div class="px-2.5 py-1 text-[10px] uppercase font-bold tracking-widest rounded-full bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 shadow-[0_0_12px_rgba(99,102,241,0.2)]">
                Active Protocol
            </div>
        {:else}
            <div class="px-2.5 py-1 text-[10px] uppercase font-bold tracking-widest rounded-full bg-surface border border-subtle text-muted">
                Inactive
            </div>
        {/if}
    </div>

    {#if webmcp.found}
        <div class="space-y-5">
            <!-- Discovered MCP Endpoints -->
            {#if webmcp.endpoints.length > 0}
                <div>
                    <h4 class="text-[11px] font-bold text-muted uppercase tracking-widest mb-2 flex items-center gap-2">
                        <Network size={14} class="text-indigo-400" /> Discovered Servers
                    </h4>
                    <div class="space-y-2">
                        {#each webmcp.endpoints as ep}
                            <div class="flex items-center gap-2 p-2.5 bg-glass border border-subtle rounded-lg">
                                <div class="w-1.5 h-1.5 rounded-full bg-indigo-400 animate-pulse"></div>
                                <span class="font-mono text-xs text-primary-text">{ep}</span>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <!-- Html Protocol Features -->
            {#if webmcp.html_features.length > 0}
                <div>
                    <h4 class="text-[11px] font-bold text-muted uppercase tracking-widest mb-2">Advertised Capabilities</h4>
                    <div class="flex flex-wrap gap-2">
                        {#each webmcp.html_features as feature}
                            <span class="px-2.5 py-1 bg-surface/50 border border-subtle font-mono text-[10px] text-muted rounded-md uppercase">
                                {feature}
                            </span>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    {:else}
        <div class="flex flex-col items-center justify-center py-6 text-center">
            <div class="p-3 bg-glass border border-subtle rounded-full mb-3 text-muted">
                <Search size={20} />
            </div>
            <span class="text-sm font-medium text-secondary-text mb-1">No MCP Integration Found</span>
            <p class="text-xs text-muted max-w-xs leading-relaxed">
                The target architecture does not expose standard Model Context Protocol servers or html link relations.
            </p>
        </div>
    {/if}
</div>
