<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import { ExternalLink, ShieldCheck } from 'lucide-svelte';

	let { data }: { data: string[] } = $props();
</script>

{#if data.length === 0}
	<div class="flex flex-col items-center justify-center p-12 text-center border border-slate-700/50 bg-slate-800/10 rounded-xl border-dashed">
		<ShieldCheck size={48} class="text-slate-600 mb-4" />
		<h4 class="text-lg font-bold text-slate-300 font-fira">{m.recon_subdomain_empty()}</h4>
		<p class="text-sm text-slate-500 font-inter max-w-md mt-2">
			{m.recon_subdomain_pending()}
		</p>
	</div>
{:else}
	<div class="overflow-x-auto rounded-xl border border-slate-700/50 bg-slate-900/50 mt-4">
		<table class="w-full text-left text-sm text-slate-400 font-inter">
			<thead class="bg-slate-800/50 text-xs uppercase text-slate-400 border-b border-slate-700/50">
				<tr>
					<th scope="col" class="px-6 py-4 font-fira font-semibold w-16">#</th>
					<th scope="col" class="px-6 py-4 font-fira font-semibold">Subdomain</th>
					<th scope="col" class="px-6 py-4 font-fira font-semibold text-right w-24">Link</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-700/30">
				{#each data as domain, i (domain)}
					<tr class="hover:bg-slate-800/30 transition-colors">
						<td class="px-6 py-4 font-mono text-xs text-slate-500">
							{i + 1}
						</td>
						<td class="px-6 py-4 font-medium text-slate-200">
							{domain}
						</td>
						<td class="px-6 py-4 text-right">
							<a
								href={`http://${domain}`}
								target="_blank"
								rel="noopener noreferrer"
								class="inline-flex items-center gap-2 rounded-md bg-slate-800/50 px-3 py-1.5 text-xs font-semibold text-sky-400 hover:bg-sky-500/20 hover:text-sky-300 transition-colors"
								aria-label={`Open ${domain} in new tab`}
							>
								<ExternalLink size={14} />
							</a>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}
