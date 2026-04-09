<script lang="ts">
	import { FolderGit2, Globe, FileTerminal, ChevronRight, ChevronDown } from 'lucide-svelte';
	import { slide, fade } from 'svelte/transition';
	
	let { data, rootDomain }: { data: string[], rootDomain: string } = $props();

	type Node = {
		name: string;
		full: string;
		children: Record<string, Node>;
		isLeaf: boolean;
	};

	let tree = $derived.by(() => {
		const root: Node = { name: rootDomain, full: rootDomain, children: {}, isLeaf: true };
		
		for (const subdomain of data) {
			if (subdomain === rootDomain) continue;
			
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
						isLeaf: true
					};
				}
				current.isLeaf = false;
				current = current.children[part] as Node;
			}
		}
		return root;
	});
</script>

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

<div class="w-full bg-background rounded-xl border border-base shadow-xl p-4 font-fira text-secondary-text overflow-x-auto">
	{#if data.length === 0}
		<div class="flex items-center justify-center py-10 text-sm text-muted">
			No tree data to parse.
		</div>
	{:else}
		<div class="flex flex-col">
			<div class="flex items-center gap-2 py-2 px-3 border-b border-base/80 mb-3 bg-surface rounded-t-lg">
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
