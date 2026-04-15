<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { CheckCircle2, XCircle, HelpCircle, ShieldAlert } from 'lucide-svelte';

    let { ips = [] }: { ips: Array<{ip: string, source: string, confidence: string, description?: string, status?: string}> } = $props();

    function getConfidenceColor(conf: string) {
        if (conf === 'Very High') return 'text-red-400 bg-red-400/10 border-red-400/20';
        if (conf === 'High') return 'text-orange-400 bg-orange-400/10 border-orange-400/20';
        if (conf === 'Medium') return 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20';
        return 'text-muted bg-surface/50 border-base';
    }
</script>

<div class="mt-6 border border-subtle bg-glass rounded-xl overflow-hidden backdrop-blur-xl shadow-2xl">
    <div class="overflow-x-auto">
        <table class="w-full text-left text-sm text-primary-text">
            <thead class="bg-glass border-b border-glass text-muted font-medium">
                <tr>
                    <th class="px-6 py-4">{m.sec_cfbypass_col_ip()}</th>
                    <th class="px-6 py-4">{m.sec_cfbypass_col_source()}</th>
                    <th class="px-6 py-4">{m.sec_cfbypass_col_confidence()}</th>
                    <th class="px-6 py-4">{m.sec_cfbypass_col_status()}</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-subtle">
                {#each ips as item (item.ip + item.source)}
                    <tr class="hover:bg-glass-hover transition-colors group">
                        <td class="px-6 py-4">
                            <div class="flex flex-col gap-1.5">
                                <div class="flex items-center gap-3">
                                    <div class="w-8 h-8 rounded-lg bg-red-400/10 flex items-center justify-center text-red-400 border border-red-400/20">
                                        <ShieldAlert size={16} />
                                    </div>
                                    <span class="font-mono text-primary-text group-hover:text-red-400 transition-colors drop-shadow-[0_0_8px_rgba(248,113,113,0.3)]">{item.ip}</span>
                                </div>
                                {#if item.description}
                                    <div class="text-[11px] text-muted pl-11">
                                        {item.description}
                                    </div>
                                {/if}
                            </div>
                        </td>
                        <td class="px-6 py-4">
                            <span class="px-2.5 py-1 text-xs font-mono rounded-md bg-glass text-primary-text border border-glass">
                                {item.source}
                            </span>
                        </td>
                        <td class="px-6 py-4">
                            <span class={`px-2.5 py-1 text-xs font-medium rounded-full border ${getConfidenceColor(item.confidence)}`}>
                                {item.confidence}
                            </span>
                        </td>
                        <td class="px-6 py-4">
                            {#if item.status === 'active'}
                                <div class="flex items-center gap-2 text-emerald-400">
                                    <CheckCircle2 size={16} class="drop-shadow-[0_0_8px_rgba(52,211,153,0.5)]" />
                                    <span class="font-medium">{m.sec_cfbypass_status_active()}</span>
                                </div>
                            {:else if item.status === 'inactive'}
                                <div class="flex items-center gap-2 text-rose-400">
                                    <XCircle size={16} />
                                    <span class="font-medium">{m.sec_cfbypass_status_inactive()}</span>
                                </div>
                            {:else}
                                <div class="flex items-center gap-2 text-muted">
                                    <HelpCircle size={16} />
                                    <span class="font-medium">{m.sec_cfbypass_status_unverified()}</span>
                                </div>
                            {/if}
                        </td>
                    </tr>
                {/each}
                {#if ips.length === 0}
                    <tr>
                        <td colspan="4" class="px-6 py-12 text-center text-muted">
                            No unproxied origin IPs detected.
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>
