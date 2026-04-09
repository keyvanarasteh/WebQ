<script module>
	export interface CrawlLog {
		id: number;
		timestamp: string;
		status: 'info' | 'warn' | 'success' | 'error';
		message: string;
	}
</script>

<script lang="ts">
	import { TerminalSquare, ShieldAlert } from 'lucide-svelte';
	import { fade } from 'svelte/transition';

	let { logs = [] }: { logs: CrawlLog[] } = $props();

	let containerNode: HTMLElement | null = $state(null);

	$effect(() => {
		// Auto scroll to bottom when logs update
		if (logs.length > 0 && containerNode) {
			containerNode.scrollTop = containerNode.scrollHeight;
		}
	});

	function getStatusColor(status: string) {
		switch (status) {
			case 'info': return 'text-cyan-400';
			case 'warn': return 'text-amber-400';
			case 'success': return 'text-emerald-400';
			case 'error': return 'text-rose-400';
			default: return 'text-slate-400';
		}
	}
</script>

<div class="bg-[#09090b] border border-[#27272a] rounded-lg overflow-hidden flex flex-col w-full shadow-inner shadow-black/80 font-mono" style="min-height: 250px;">
	<div class="bg-[#18181b] border-b border-[#27272a] p-2.5 flex items-center justify-between shadow-sm z-10 px-4 shrink-0">
		<div class="flex items-center gap-2">
			<TerminalSquare size={16} class="text-emerald-500" />
			<span class="text-xs text-white font-bold tracking-widest uppercase">Harvesting Console</span>
		</div>
		<div class="flex items-center gap-4 text-[10px] uppercase font-bold text-slate-500">
			<span class="flex items-center gap-1.5"><span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span> Active Session</span>
		</div>
	</div>

	<div 
		bind:this={containerNode}
		class="p-4 overflow-y-auto space-y-2 text-[11px] leading-relaxed flex-grow scroll-smooth"
		style="max-height: 400px;"
	>
		{#each logs as log (log.id)}
			<div class="flex items-start gap-3 group" in:fade={{duration:150}}>
				<span class="text-slate-600 shrink-0">[{log.timestamp}]</span>
				<span class="shrink-0 font-bold uppercase w-14 {getStatusColor(log.status)}">{log.status}</span>
				<span class="text-slate-300 break-words group-hover:text-white transition-colors">{log.message}</span>
			</div>
		{:else}
			<div class="h-full flex items-center justify-center text-slate-600 py-10 opacity-50 text-[10px] tracking-widest uppercase">
				Waiting for crawler initialization...
			</div>
		{/each}
	</div>
</div>
