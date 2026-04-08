<script lang="ts">
	import { FolderGit2, Globe, CheckCircle2 } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	let { data, rootDomain }: { data: string[]; rootDomain: string } = $props();

	// ── Tree Data Structure ─────────────────────────────────────────────────────────

	type TreeNode = {
		part: string;
		fullPath: string;
		children: Record<string, TreeNode>;
		isLeaf: boolean;
	};

	// Pro Svelte 5 pattern: Transform flat list into nested tree inside $derived.by()
	// This prevents reactivity loops by decoupling the algorithm from the template.
	let tree = $derived.by(() => {
		const root: TreeNode = {
			part: rootDomain || 'ROOT',
			fullPath: rootDomain,
			children: {},
			isLeaf: false
		};

		if (!data || data.length === 0) return root;

		for (const domain of data) {
			const parts = domain.split('.').reverse();
			let current = root;
			let pathAcc = '';

			for (let i = 0; i < parts.length; i++) {
				const part = parts[i] || 'unknown';
				pathAcc = pathAcc === '' ? part : `${part}.${pathAcc}`;
				
				if (!current.children[part]) {
					current.children[part] = {
						part,
						fullPath: pathAcc,
						children: {},
						isLeaf: false
					};
				}
				current = current.children[part];
				
				// If this is the last part, mark as leaf
				if (i === parts.length - 1) {
					current.isLeaf = true;
				}
			}
		}

		return root;
	});
</script>

<!-- Recursive Snippet for the tree rendering -->
{#snippet treeNode(node: TreeNode, depth: number)}
	{@const hasChildren = Object.keys(node.children).length > 0}
	
	<div class="relative flex flex-col font-mono text-sm group" style={`margin-left: ${depth === 0 ? 0 : 2}rem`}>
		
		<!-- Visual connection lines -->
		{#if depth > 0}
			<div class="absolute -left-[1.3rem] top-4 h-[1px] w-4 border-t border-dashed border-slate-600"></div>
			<div class="absolute -left-[1.3rem] top-[-10px] bottom-0 w-[1px] border-l border-dashed border-slate-600 group-last:bottom-auto group-last:h-[26px]"></div>
		{/if}

		<div class="flex items-center gap-2 py-1.5 z-10 w-fit">
			<div class={`flex h-6 w-6 shrink-0 items-center justify-center rounded-md border ${
				hasChildren ? 'border-sky-500/30 bg-sky-500/10 text-sky-400' : 'border-emerald-500/30 bg-emerald-500/10 text-emerald-400'
			}`}>
				{#if hasChildren}
					<FolderGit2 size={12} />
				{:else}
					<Globe size={12} />
				{/if}
			</div>

			<span class={`font-semibold ${hasChildren ? 'text-slate-300' : 'text-slate-400'}`}>
				{node.part}
			</span>
			
			{#if node.isLeaf}
				<CheckCircle2 size={14} class="text-emerald-500/50" />
			{/if}
			
			{#if depth > 0 && node.fullPath}
				<span class="ml-2 rounded-full border border-slate-700/50 bg-slate-800/50 px-2 py-0.5 text-[10px] text-slate-500 opacity-0 group-hover:opacity-100 transition-opacity">
					{node.fullPath}
				</span>
			{/if}
		</div>

		{#if hasChildren}
			<!-- To comply with Svelte 5 rules, we must iterate with (key) -->
			<!-- Using Object.entries ensures stable rendering -->
			<div class="flex flex-col relative w-full pt-1">
				{#each Object.entries(node.children) as [key, childNode] (key)}
					{@render treeNode(childNode, depth + 1)}
				{/each}
			</div>
		{/if}
	</div>
{/snippet}

<!-- Main Container -->
<div class="rounded-xl border border-slate-700/50 bg-slate-900/50 p-6 min-h-[400px] overflow-x-auto">
	{#if data.length === 0}
		<div class="flex h-full w-full items-center justify-center text-slate-500 font-fira text-sm">
			{m.recon_subdomain_empty()}
		</div>
	{:else}
		<div class="flex flex-col relative">
			{@render treeNode(tree, 0)}
		</div>
	{/if}
</div>
