<script lang="ts">
    import type { ScanProgressEvent } from '$lib/types/intelligence';

    let { logs = [], progressPercent = 0 }: { logs: ScanProgressEvent[]; progressPercent: number } = $props();

    let terminalContainer: HTMLElement;

    // Auto-scroll on new logs
    $effect(() => {
        if (logs.length && terminalContainer) {
            terminalContainer.scrollTop = terminalContainer.scrollHeight;
        }
    });
</script>

<div class="relative flex flex-col w-full h-[450px] rounded-xl border border-white/10 bg-black/80 backdrop-blur-xl shadow-2xl overflow-hidden font-mono">
    <!-- Header -->
    <div class="flex flex-row items-center justify-between px-4 py-3 border-b border-white/10 bg-white/5">
        <div class="flex items-center gap-3">
            <div class="flex gap-1.5">
                <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
            </div>
            <span class="text-xs font-semibold tracking-wider text-neutral-400 uppercase">Q-Analyzer Neural Interface</span>
        </div>
        <div class="text-xs font-bold text-teal-400 font-mono">
            {Math.round(progressPercent)}%
        </div>
    </div>

    <!-- Progress Bar -->
    <div class="w-full h-1 bg-neutral-900">
        <div 
            class="h-full bg-teal-500 shadow-[0_0_10px_rgba(20,184,166,0.8)] transition-all duration-300 ease-out" 
            style="width: {progressPercent}%"
        ></div>
    </div>

    <!-- Terminal Body -->
    <div bind:this={terminalContainer} class="flex-1 p-5 overflow-y-auto space-y-2 text-sm custom-scrollbar">
        {#each logs as log, i (i)}
            <div class="flex items-start gap-4 animate-fade-in group">
                <span class="font-bold shrink-0 {log.status === 'Error' ? 'text-red-500' : 'text-neutral-500'}">
                    [{log.status === 'Info' ? '*' : log.status === 'Success' ? '+' : '!'}]
                </span>
                <span class="text-fuchsia-400/80 shrink-0 w-28 uppercase text-xs mt-0.5 tracking-wider font-semibold">
                    {log.module}
                </span>
                <span class="leading-relaxed {log.status === 'Success' ? 'text-teal-400' : log.status === 'Error' ? 'text-red-400' : 'text-neutral-300 group-hover:text-white transition-colors duration-200'}">
                    {log.message}
                </span>
            </div>
        {/each}
        {#if progressPercent < 100}
            <div class="flex items-center gap-3 text-teal-500/70 mt-4 animate-pulse pt-2">
                <span class="w-2 h-4 bg-teal-500 inline-block"></span>
                <span class="text-xs tracking-widest font-semibold">AWAITING DATALINK...</span>
            </div>
        {/if}
    </div>
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.2);
    }
    .animate-fade-in {
        animation: fadeIn 0.3s ease-out forwards;
    }
    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(4px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>
