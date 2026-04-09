<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Bot, Ban, CheckCircle2, ShieldAlert, Navigation } from 'lucide-svelte';

    let { bots = {} }: { bots: Record<string, string> } = $props();

    // Derived to array to easily iterate and sort
    let botEntries = $derived(
        Object.entries(bots).sort((a, b) => a[0].localeCompare(b[0]))
    );

    function getBotConfig(status: string) {
        if (status.includes("Blocked")) {
            return {
                icon: Ban,
                textColor: "text-red-400",
                bgColor: "bg-red-500/10",
                borderColor: "border-red-500/20",
                indicator: "bg-red-500"
            };
        }
        if (status.includes("Allowed")) {
            return {
                icon: CheckCircle2,
                textColor: "text-emerald-400",
                bgColor: "bg-emerald-500/10",
                borderColor: "border-emerald-500/20",
                indicator: "bg-emerald-500"
            };
        }
        return {
            icon: ShieldAlert,
            textColor: "text-yellow-400",
            bgColor: "bg-yellow-500/10",
            borderColor: "border-yellow-500/20",
            indicator: "bg-yellow-500"
        };
    }
</script>

<div class="border border-white/5 bg-black/40 rounded-xl overflow-hidden backdrop-blur-xl h-full flex flex-col">
    <div class="bg-white/5 px-4 py-3 border-b border-white/5 flex items-center gap-2">
        <Navigation size={16} class="text-muted" />
        <h3 class="text-sm font-medium text-primary-text tracking-wide uppercase">{m.sec_geo_crawler_title()}</h3>
    </div>
    
    <div class="p-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 overflow-y-auto custom-scrollbar flex-1">
        {#each botEntries as [botName, status]}
            {@const config = getBotConfig(status)}
            {@const Icon = config.icon}
            <div class="border {config.borderColor} {config.bgColor} rounded-xl p-4 relative overflow-hidden group transition-all">
                <div class="flex items-center gap-3 mb-3">
                    <div class="p-2 rounded-lg bg-black/40 border border-white/5">
                        <Icon size={18} class={config.textColor} />
                    </div>
                    <span class="text-sm font-medium text-primary-text">{botName}</span>
                </div>
                
                <div class="flex items-center gap-2">
                    <div class="w-1.5 h-1.5 rounded-full {config.indicator} {status.includes('Blocked') ? '' : 'animate-pulse'}"></div>
                    <span class={`text-[10px] font-mono tracking-wider uppercase ${config.textColor}`}>
                        {status}
                    </span>
                </div>
            </div>
        {/each}

        {#if botEntries.length === 0}
            <div class="col-span-full h-32 flex items-center justify-center text-muted text-sm font-mono">
                [ Waiting for Scan Results ]
            </div>
        {/if}
    </div>
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 6px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
</style>
