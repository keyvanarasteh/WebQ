<script lang="ts">
	import { ExternalLink, Copy, Target } from 'lucide-svelte';
	import { fade } from 'svelte/transition';
	
	let { data }: { data: string[] } = $props();
</script>

<div class="overflow-x-auto w-full rounded-lg border border-slate-700/50">
	<table class="w-full text-left text-sm text-slate-300">
		<thead class="bg-qix-obsidian text-xs uppercase text-slate-400 font-fira border-b border-slate-700/50">
			<tr>
				<th scope="col" class="px-4 py-3 border-r border-slate-700/50 w-8">#</th>
				<th scope="col" class="px-6 py-3 border-r border-slate-700/50">Host</th>
				<th scope="col" class="px-6 py-3 border-r border-slate-700/50 w-32 text-center">Status</th>
				<th scope="col" class="px-4 py-3 w-40 text-right">Actions</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-slate-700/50 font-mono bg-slate-900/50">
			{#each data as item, i (item)}
				<tr class="hover:bg-indigo-500/10 transition-colors group" in:fade={{ delay: Math.min(i * 5, 500), duration: 200 }}>
					<td class="px-4 py-3 whitespace-nowrap text-slate-500 border-r border-slate-700/50 text-xs">
						{i + 1}
					</td>
					<td class="px-6 py-3 whitespace-nowrap font-medium text-indigo-300 group-hover:text-indigo-200 transition-colors">
						{item}
					</td>
					<td class="px-6 py-3 whitespace-nowrap border-r border-slate-700/50 text-center">
						<span class="inline-flex items-center gap-1.5 rounded bg-qix-obsidian px-2 py-1.5 text-xs text-slate-400 border border-slate-700/50 group-hover:border-indigo-500/50 transition-colors">
							<Target size={12} class="text-indigo-500" />
							Discovered
						</span>
					</td>
					<td class="px-4 py-3 flex items-center justify-end gap-2">
						<!-- action buttons -->
						<button 
							class="text-slate-500 hover:text-indigo-400 hover:bg-indigo-500/10 rounded p-1.5 transition-colors"
							onclick={() => navigator.clipboard.writeText(item)}
							title="Copy to clipboard"
						>
							<Copy size={16} />
						</button>
						<a 
							href={`http://${item}`}
							target="_blank"
							rel="noopener noreferrer"
							class="text-slate-500 hover:text-emerald-400 hover:bg-emerald-500/10 rounded p-1.5 transition-colors"
							title="Open in browser"
						>
							<ExternalLink size={16} />
						</a>
					</td>
				</tr>
			{:else}
				<tr>
					<td colspan="4" class="px-6 py-12 text-center text-slate-500 font-fira">
						No subdomains to display.
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
