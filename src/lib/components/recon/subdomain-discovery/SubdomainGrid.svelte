<script lang="ts">
	import { ExternalLink, Copy, Target, HelpCircle, Network } from 'lucide-svelte';
	import { fade } from 'svelte/transition';
	import * as m from '$lib/paraglide/messages';
	import SubdomainGridGuide from './SubdomainGridGuide.svelte';
	
	let { data, isPending = false }: { data: string[] | null, isPending?: boolean } = $props();
	let guideOpen = $state(false);
</script>

<SubdomainGridGuide bind:isOpen={guideOpen} />

<div class="overflow-x-auto w-full rounded-lg border border-base/50 relative">
	<div class="absolute top-3 right-3 z-10">
		<button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-indigo-500/10 border border-transparent hover:border-indigo-500/20 transition-all" title="View Guide">
			<HelpCircle class="size-4" />
		</button>
	</div>

	{#if isPending || !data}
		<div class="p-8 flex flex-col items-center justify-center gap-3 text-center min-h-[300px] bg-background">
			<span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
			<Network class="size-10 text-muted/30" />
			<p class="text-sm text-muted">{m.recon_subdomain_pending()}</p>
		</div>
	{:else}
		<table class="w-full text-left text-sm text-secondary-text">
			<thead class="bg-background text-xs uppercase text-muted font-fira border-b border-base/50">
				<tr>
					<th scope="col" class="px-4 py-3 border-r border-base/50 w-8">#</th>
					<th scope="col" class="px-6 py-3 border-r border-base/50">Host</th>
					<th scope="col" class="px-6 py-3 border-r border-base/50 w-32 text-center">Status</th>
					<th scope="col" class="px-4 py-3 w-40 text-right pr-12">Actions</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-700/50 font-mono bg-surface">
				{#each data as item, i (item)}
					<tr class="hover:bg-indigo-500/10 transition-colors group" in:fade={{ delay: Math.min(i * 5, 500), duration: 200 }}>
						<td class="px-4 py-3 whitespace-nowrap text-muted border-r border-base/50 text-xs text-center">
							{i + 1}
						</td>
						<td class="px-6 py-3 whitespace-nowrap font-medium text-indigo-300 group-hover:text-indigo-200 transition-colors">
							{item}
						</td>
						<td class="px-6 py-3 whitespace-nowrap border-r border-base/50 text-center">
							<span class="inline-flex items-center gap-1.5 rounded bg-background px-2 py-1.5 text-xs text-muted border border-base/50 group-hover:border-indigo-500/50 transition-colors">
								<Target size={12} class="text-indigo-500" />
								Discovered
							</span>
						</td>
						<td class="px-4 py-3 flex items-center justify-end gap-2 pr-12">
							<button 
								class="text-muted hover:text-indigo-400 hover:bg-indigo-500/10 rounded p-1.5 transition-colors"
								onclick={() => navigator.clipboard.writeText(item)}
								title="Copy to clipboard"
							>
								<Copy size={16} />
							</button>
							<a 
								href={`http://${item}`}
								target="_blank"
								rel="noopener noreferrer"
								class="text-muted hover:text-emerald-400 hover:bg-emerald-500/10 rounded p-1.5 transition-colors"
								title="Open in browser"
							>
								<ExternalLink size={16} />
							</a>
						</td>
					</tr>
				{:else}
					<tr>
						<td colspan="4" class="px-6 py-12 text-center text-muted font-fira">
							No subdomains to display.
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>
