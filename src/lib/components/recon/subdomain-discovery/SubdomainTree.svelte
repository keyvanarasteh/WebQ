<script lang="ts">
	import { FolderGit2, Globe, FileTerminal, ChevronRight, ChevronDown, HelpCircle, Network } from 'lucide-svelte';
	import { slide, fade } from 'svelte/transition';
	import * as m from '$lib/paraglide/messages';
	import SubdomainTreeGuide from './SubdomainTreeGuide.svelte';
	
	type SubdomainDetail = {
		host: string;
		status: number | null;
		resolution_error: string | null;
	};
	
	let { data, rootDomain, isPending = false }: { data: SubdomainDetail[] | null, rootDomain: string, isPending?: boolean } = $props();
	let guideOpen = $state(false);

	type Node = {
		name: string;
		full: string;
		children: Record<string, Node>;
		isLeaf: boolean;
		status?: number | null | undefined;
		error?: string | null | undefined;
	};

	let tree = $derived.by(() => {
		const root: Node = { name: rootDomain, full: rootDomain, children: {}, isLeaf: true };
		if (!data) return root;
		
		for (const detail of data) {
			const subdomain = typeof detail === 'string' ? detail : (detail?.host || '');
			if (!subdomain || subdomain === rootDomain) continue;
			
			let prefix = subdomain;
			if (subdomain.endsWith(`.${rootDomain}`)) {
				prefix = subdomain.slice(0, -(rootDomain.length + 1));
			}
			
			const parts = prefix.split('.').reverse();
			let current = root;
			
			let builtPrefix = "";
			for (let i = 0; i < parts.length; i++) {
				const part = parts[i];
				if (!part) continue;
				if (!current.children[part]) {
					builtPrefix = builtPrefix ? `${part}.${builtPrefix}` : part;
					const fqdn = `${builtPrefix}.${rootDomain}`;
					current.children[part] = {
						name: part,
						full: fqdn,
						children: {},
						isLeaf: true,
						status: i === parts.length - 1 ? (detail as any).status : undefined,
						error: i === parts.length - 1 ? (detail as any).resolution_error : undefined,
					};
				} else if (i === parts.length - 1) {
					current.children[part].status = (detail as any).status;
					current.children[part].error = (detail as any).resolution_error;
				}
				current.isLeaf = false;
				current = current.children[part] as Node;
			}
		}
		return root;
	});
</script>

<SubdomainTreeGuide bind:isOpen={guideOpen} />

{#snippet treeNode(node: Node, depth: number)}
	<div class="flex flex-col">
		<div 
			class="flex items-center gap-2 py-1.5 px-2 hover:bg-surface-hover/50 rounded-lg transition-colors group cursor-pointer"
			style="margin-left: {depth * 1.5}rem"
		>
			{#if Object.keys(node.children).length > 0}
				<FolderGit2 size={16} class="text-indigo-400 group-hover:text-indigo-300" />
			{:else}
				<Globe size={14} class="text-muted group-hover:text-emerald-400" />
			{/if}
			
			<span class="font-mono text-sm text-secondary-text group-hover:text-primary-text transition-colors">
				{node.name}
			</span>
			
			{#if node.status === 200}
				<span class="ml-2 inline-block size-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]" title="Live (200)"></span>
			{:else if node.status === 403}
				<span class="ml-2 inline-block size-2 rounded-full bg-amber-500 shadow-[0_0_8px_rgba(245,158,11,0.5)]" title="Forbidden (403)"></span>
			{:else if node.status === 404}
				<span class="ml-2 inline-block size-2 rounded-full bg-slate-500" title="Not Found (404)"></span>
			{:else if node.status}
				<span class="ml-2 inline-block size-2 rounded-full bg-indigo-500" title="Active ({node.status})"></span>
			{:else if node.isLeaf}
				<span class="ml-2 inline-block size-2 rounded-full bg-rose-500 shadow-[0_0_8px_rgba(239,68,68,0.5)]" title={node.error || "Offline"}></span>
			{/if}

			{#if !Object.keys(node.children).length}
				<a 
					href={`http://${node.full}`} 
					target="_blank" 
					rel="noopener noreferrer" 
					class="text-[10px] text-tertiary-text ml-2 hidden group-hover:inline-block font-mono hover:text-emerald-400 transition-colors"
					title="Open {node.full}"
				>
					({node.full})
				</a>
			{/if}
		</div>
		
		{#if Object.keys(node.children).length > 0}
			<div class="relative flex flex-col border-l border-base/50 ml-2 pt-1">
				{#each Object.values(node.children).sort((a,b) => a.name.localeCompare(b.name)) as child}
					{@render treeNode(child, depth + 1)}
				{/each}
			</div>
		{/if}
	</div>
{/snippet}

<div class="w-full bg-background rounded-xl border border-base shadow-xl p-4 font-fira text-secondary-text overflow-x-auto relative min-h-[300px]">
	<div class="absolute top-3 right-3 z-10">
		<button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title="View Guide">
			<HelpCircle class="size-4" />
		</button>
	</div>

	{#if isPending || !data}
		<div class="flex h-full flex-col items-center justify-center gap-3 text-center min-h-[250px] mt-4">
			<span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
			<FolderGit2 class="size-10 text-muted/30" />
			<p class="text-sm text-muted">{m.recon_subdomain_pending()}</p>
		</div>
	{:else if data.length === 0}
		<div class="flex items-center justify-center py-10 text-sm text-muted min-h-[250px]">
			No tree data to parse.
		</div>
	{:else}
		<div class="flex flex-col mt-2">
			<div class="flex items-center gap-2 py-2 px-3 border-b border-base/80 mb-3 bg-surface rounded-t-lg w-max min-w-full">
				<Globe size={18} class="text-emerald-500" />
				<span class="font-bold text-emerald-400 tracking-wide text-lg">{tree.name}</span>
			</div>
			
			<div class="flex flex-col pl-1 space-y-1">
				{#each Object.values(tree.children).sort((a,b) => a.name.localeCompare(b.name)) as child}
					{@render treeNode(child, 0)}
				{/each}
			</div>
		</div>
	{/if}
</div>
