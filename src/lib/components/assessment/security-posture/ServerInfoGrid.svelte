<script lang="ts">
    import { ServerCrash, AlertCircle } from 'lucide-svelte';

    let { serverInfo }: {
        serverInfo: {
            server_headers: Record<string, string>,
            information_disclosure: string[],
            disclosure_count: number,
            security_level: string
        }
    } = $props();

    function getLevelColor(lvl: string) {
        if (lvl === 'Critical' || lvl === 'Poor') return 'text-red-400 border-red-400/20 bg-red-400/10';
        if (lvl === 'Medium' || lvl === 'Fair') return 'text-yellow-400 border-yellow-400/20 bg-yellow-400/10';
        return 'text-emerald-400 border-emerald-400/20 bg-emerald-400/10';
    }
</script>

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6">
    <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
            <ServerCrash class="w-5 h-5 text-indigo-400" />
            <h3 class="text-lg font-medium text-primary-text">Server Imprint</h3>
        </div>
        <div class={`px-2.5 py-1 text-xs uppercase font-bold rounded-md border ${getLevelColor(serverInfo.security_level)}`}>
            {serverInfo.security_level} Posture
        </div>
    </div>

    <!-- Found Headers -->
    {#if Object.keys(serverInfo.server_headers).length > 0}
        <div class="space-y-2 mb-6">
            {#each Object.entries(serverInfo.server_headers) as [sigKey, sigVal]}
                <div class="flex items-center justify-between p-2.5 bg-surface/50 border border-subtle rounded-lg">
                    <span class="text-xs font-semibold text-muted uppercase tracking-wider">{sigKey}</span>
                    <span class="text-xs font-mono text-indigo-300 bg-indigo-500/10 px-2 py-0.5 rounded border border-indigo-500/20">
                        {sigVal}
                    </span>
                </div>
            {/each}
        </div>
    {:else}
        <div class="p-3 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-sm mb-6 rounded-lg text-center">
            No server signatures leaked. Good cloaking.
        </div>
    {/if}

    <!-- Disclosure Vulnerabilities -->
    {#if serverInfo.disclosure_count > 0}
        <div>
            <h4 class="text-xs font-semibold text-muted uppercase tracking-widest mb-3 flex items-center gap-2">
                <AlertCircle size={12} class="text-red-400" /> Data Disclosure Traps
            </h4>
            <div class="space-y-2">
                {#each serverInfo.information_disclosure as disc}
                    <div class="p-2.5 text-xs text-red-200/80 bg-red-500/5 border border-red-500/10 rounded font-mono leading-relaxed">
                        > {disc}
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>
