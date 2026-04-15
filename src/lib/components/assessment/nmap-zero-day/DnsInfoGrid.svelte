<script lang="ts">
    import { Network } from 'lucide-svelte';

    let { dnsInfo }: { dnsInfo: { ipv4?: string, ipv6?: string } } = $props();
</script>

{#if dnsInfo.ipv4 || dnsInfo.ipv6}
    <div class="flex flex-col gap-3">
        <h2 class="text-lg font-medium text-primary-text flex items-center gap-2">
            <Network size={18} class="text-rose-400" />
            DNS Resolution
        </h2>
        <div class="border border-subtle bg-glass rounded-xl overflow-hidden backdrop-blur-xl shadow-2xl p-4">
            <div class="space-y-3">
                {#if dnsInfo.ipv4}
                    <div class="flex items-center justify-between">
                        <span class="text-xs font-semibold text-muted uppercase tracking-wider">A Record (IPv4)</span>
                        <span class="text-sm font-mono text-cyan-300 bg-cyan-500/10 px-2 py-0.5 rounded border border-cyan-500/20">{dnsInfo.ipv4}</span>
                    </div>
                {/if}
                {#if dnsInfo.ipv6}
                    <div class="flex items-center justify-between border-t border-subtle pt-3">
                        <span class="text-xs font-semibold text-muted uppercase tracking-wider">AAAA Record (IPv6)</span>
                        <span class="text-sm font-mono text-cyan-300 bg-cyan-500/10 px-2 py-0.5 rounded border border-cyan-500/20">{dnsInfo.ipv6}</span>
                    </div>
                {/if}
            </div>
        </div>
    </div>
{/if}
