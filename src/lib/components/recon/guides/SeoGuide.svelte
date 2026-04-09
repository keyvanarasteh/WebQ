<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import { 
		X, 
		Search, 
		ShieldAlert, 
		Code2, 
		BookOpen,
		Globe,
        AlertOctagon
	} from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages.js';
	import { onMount, onDestroy } from 'svelte';

	let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

	let activeTab = $state<'intro' | 'threats' | 'osint'>('intro');

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && isOpen) {
			isOpen = false;
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeydown);
	});

	onDestroy(() => {
		if (typeof window !== 'undefined') {
			window.removeEventListener('keydown', handleKeydown);
		}
	});

	// Reset tab when reopened
	$effect(() => {
		if (isOpen) {
			activeTab = 'intro';
		}
	});

	const tabs = [
		{ id: 'intro', label: 'Introduction', icon: BookOpen },
		{ id: 'threats', label: 'Threat Vectors', icon: ShieldAlert },
		{ id: 'osint', label: 'OSINT & Graph', icon: Globe }
	] as const;
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6"
		role="dialog"
		aria-modal="true"
		aria-labelledby="seo-guide-title"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 bg-black/80 backdrop-blur-sm cursor-default w-full h-full border-none p-0 focus:outline-none"
			onclick={() => (isOpen = false)}
			aria-label="Close"
			transition:fade={{ duration: 200 }}
		></button>

		<div
			class="relative w-full max-w-4xl bg-qix-obsidian border border-[#27272a] rounded-xl shadow-2xl overflow-hidden flex flex-col md:flex-row shadow-cyan-900/20"
			transition:fly={{ y: 20, duration: 400, easing: backOut }}
		>
			<!-- Close Button -->
			<button
				onclick={() => (isOpen = false)}
				class="absolute top-4 right-4 p-2 text-gray-400 hover:text-white bg-[#121214] hover:bg-[#27272a] rounded-full transition-colors z-10 focus:outline-none focus:ring-2 focus:ring-cyan-500/50"
				aria-label="Close"
			>
				<X class="size-4" />
			</button>

			<!-- Sidebar Navigation -->
			<div class="w-full md:w-64 bg-[#121214] border-r border-[#27272a] shrink-0 flex flex-col">
				<div class="p-6 border-b border-[#27272a]">
					<div class="flex items-center gap-3">
						<div class="p-2 bg-cyan-500/10 rounded-lg">
							<Search class="size-5 text-cyan-400" />
						</div>
						<div>
							<h2 id="seo-guide-title" class="font-bold text-white text-sm">SEO & Metadata</h2>
							<div class="text-[10px] text-cyan-500/70 uppercase tracking-widest font-mono mt-1">Reconnaissance</div>
						</div>
					</div>
				</div>

				<div class="p-4 flex-1 flex flex-row md:flex-col gap-2 overflow-x-auto md:overflow-visible">
					{#each tabs as tab (tab.id)}
						<button
							onclick={() => (activeTab = tab.id)}
							class="flex items-center gap-3 px-4 py-3 rounded-lg text-sm transition-all duration-200 text-left whitespace-nowrap focus:outline-none focus:ring-2 focus:ring-cyan-500/50
                            {activeTab === tab.id
								? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20'
								: 'text-gray-400 hover:bg-[#27272a] hover:text-gray-200 border border-transparent'}"
						>
							<tab.icon class="size-4 {activeTab === tab.id ? 'text-cyan-400' : 'text-gray-500'}" />
							<span class="font-medium">{tab.label}</span>
						</button>
					{/each}
				</div>
                
                <div class="p-6 border-t border-[#27272a] hidden md:block">
					<div class="bg-qix-obsidian rounded p-3 border border-[#27272a]">
						<div class="text-[10px] font-mono text-gray-500 uppercase tracking-wider mb-1">Module Type</div>
						<div class="text-xs text-gray-300 font-medium">OSINT Footprinting</div>
					</div>
				</div>
			</div>

			<!-- Content Area -->
			<div class="flex-1 overflow-y-auto custom-scrollbar p-6 md:p-8 max-h-[80vh] md:max-h-[600px] bg-qix-obsidian">
				<!-- INTRO TAB -->
				{#if activeTab === 'intro'}
					<div in:fade={{ duration: 200, delay: 100 }}>
						<h3 class="text-xl font-bold text-white mb-4">Search Engine & Metadata Recon</h3>
						<p class="text-sm leading-relaxed text-gray-400 mb-6">
							While typically the domain of marketing teams, <strong class="text-white">SEO (Search Engine Optimization) and Metadata</strong> attributes hold massive implications for OSINT (Open-Source Intelligence) and footprinting. Attackers extract software versions, authors, indexing directives, and social network integrations from meta tags.
						</p>

						<div class="grid grid-cols-1 gap-4">
                            <div class="p-5 bg-[#121214] border border-[#27272a] rounded-xl flex items-start gap-4">
								<div class="p-2 bg-purple-500/10 rounded border border-purple-500/20 shrink-0">
									<Globe class="size-5 text-purple-400" />
								</div>
								<div>
									<h4 class="text-sm font-bold text-gray-200 mb-1">Open-Source Intelligence</h4>
									<p class="text-xs text-gray-500 leading-relaxed">
										Metadata tags often map out an organization's social structure, connected corporate entities, and marketing service providers without actively initiating a scan.
									</p>
								</div>
							</div>

							<div class="p-5 bg-[#121214] border border-[#27272a] rounded-xl flex items-start gap-4">
								<div class="p-2 bg-red-500/10 rounded border border-red-500/20 shrink-0">
									<AlertOctagon class="size-5 text-red-400" />
								</div>
								<div>
									<h4 class="text-sm font-bold text-gray-200 mb-1">Information Disclosure</h4>
									<p class="text-xs text-gray-500 leading-relaxed">
										Developer shortcuts like leaving exact software versions or raw API routes in tags provide a massive shortcut for an attacker selecting an exploit payload.
									</p>
								</div>
							</div>
						</div>
					</div>
				{/if}

				<!-- THREATS TAB -->
				{#if activeTab === 'threats'}
					<div in:fade={{ duration: 200, delay: 100 }}>
						<h3 class="text-xl font-bold text-white mb-2 flex items-center gap-2">
                            <ShieldAlert class="size-5 text-cyan-400" /> Metadata Threat Vectors
                        </h3>
						<p class="text-sm text-gray-400 mb-6">
							Common meta tags that silently leak backend information.
						</p>

						<div class="space-y-6">
                            <div class="bg-[#121214] border border-[#27272a] rounded-xl p-5">
                                <h4 class="text-sm font-bold text-cyan-400 mb-2">1. The "Generator" Tag Disclosure</h4>
                                <p class="text-xs text-gray-400 mb-3">CMS platforms (WordPress, Joomla, Jekyll) often inject a generator tag indicating the exact software and version making the site.</p>
                                <div class="bg-qix-obsidian border border-[#27272a] rounded overflow-hidden">
                                    <div class="bg-[#18181b] px-3 py-1.5 border-b border-[#27272a] text-[10px] text-gray-500 font-mono flex items-center justify-between">
                                        HTML
                                    </div>
                                    <div class="p-3 overflow-x-auto custom-scrollbar">
                                        <pre class="text-xs font-mono"><span class="text-pink-400">&lt;meta</span> <span class="text-green-300">name</span>=<span class="text-yellow-300">"generator"</span> <span class="text-green-300">content</span>=<span class="text-yellow-300">"WordPress 5.8.1"</span> <span class="text-pink-400">/&gt;</span></pre>
                                    </div>
                                </div>
                                <div class="mt-4 p-3 bg-red-500/5 border border-red-500/20 rounded-lg flex gap-3">
                                    <AlertOctagon class="size-4 text-red-400 shrink-0 mt-0.5" />
                                    <div>
                                        <div class="text-[10px] font-bold text-red-500 uppercase tracking-wider mb-1">SecOps Reality</div>
                                        <div class="text-xs text-red-200/70">Directly maps the version to known CVE exploits. Once the version is known, exploitation scripts can be chosen precisely.</div>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="bg-[#121214] border border-[#27272a] rounded-xl p-5">
                                <h4 class="text-sm font-bold text-cyan-400 mb-2">2. Robots.txt & Indexing Directives</h4>
                                <p class="text-xs text-gray-400 mb-3"><code class="text-white bg-[#27272a] px-1 py-0.5 rounded">noindex, nofollow</code> directives often exist to hide staging sites, admin portals, or internal API documentation.</p>
                                <div class="bg-qix-obsidian border border-[#27272a] rounded overflow-hidden">
                                    <div class="bg-[#18181b] px-3 py-1.5 border-b border-[#27272a] text-[10px] text-gray-500 font-mono flex items-center justify-between">
                                        HTML
                                    </div>
                                    <div class="p-3 overflow-x-auto custom-scrollbar">
                                        <pre class="text-xs font-mono"><span class="text-pink-400">&lt;meta</span> <span class="text-green-300">name</span>=<span class="text-yellow-300">"robots"</span> <span class="text-green-300">content</span>=<span class="text-yellow-300">"noindex, nofollow"</span> <span class="text-pink-400">/&gt;</span></pre>
                                    </div>
                                </div>
                                <div class="mt-4 p-3 bg-green-500/5 border border-green-500/20 rounded-lg flex gap-3">
                                    <ShieldAlert class="size-4 text-green-400 shrink-0 mt-0.5" />
                                    <div>
                                        <div class="text-[10px] font-bold text-green-500 uppercase tracking-wider mb-1">Secure Dev Duty</div>
                                        <div class="text-xs text-green-200/70">Use these tags to ensure internal admin dashboards don't accidentally leak into Google Dorks, but realize they do not prevent direct enumeration. They are not an access control mechanism.</div>
                                    </div>
                                </div>
                            </div>
						</div>
					</div>
				{/if}

				<!-- OSINT TAB -->
				{#if activeTab === 'osint'}
					<div in:fade={{ duration: 200, delay: 100 }}>
						<h3 class="text-xl font-bold text-white mb-2 flex items-center gap-2">
							<Globe class="size-5 text-cyan-400" /> OSINT & Open Graph Metrics
						</h3>
						<p class="text-sm text-gray-400 mb-6">
							Open Graph (<code class="text-cyan-400 bg-cyan-950/30 px-1 py-0.5 rounded">og:title</code>, <code class="text-cyan-400 bg-cyan-950/30 px-1 py-0.5 rounded">og:image</code>) and Twitter Cards provide structured data for link unfurling.
						</p>

						<div class="grid grid-cols-1 gap-6">
                            <div class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden p-1">
                                <div class="border border-[#27272a] border-dashed rounded-lg p-6 bg-qix-obsidian flex flex-col items-center justify-center text-center">
                                    <div class="p-4 bg-[#121214] rounded-full border border-[#27272a] mb-4">
                                        <Search class="size-8 text-cyan-500/50" />
                                    </div>
                                    <h4 class="text-base font-bold text-gray-200 mb-2">Corporate Identity Mapping</h4>
                                    <p class="text-xs text-gray-500 max-w-sm">
                                        Extracts linked social accounts, publisher identities, and authors. Heavily utilized in spear-phishing campaigns to identify target personnel and corporate branding assets.
                                    </p>
                                </div>
                            </div>
                            
                            <div class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden p-1">
                                <div class="border border-[#27272a] border-dashed rounded-lg p-6 bg-qix-obsidian flex flex-col items-center justify-center text-center">
                                    <div class="p-4 bg-[#121214] rounded-full border border-[#27272a] mb-4">
                                        <Code2 class="size-8 text-yellow-500/50" />
                                    </div>
                                    <h4 class="text-base font-bold text-gray-200 mb-2">Sitemap Reconnaissance</h4>
                                    <p class="text-xs text-gray-500 max-w-sm">
                                        Aggregating <code>sitemap.xml</code> entries provides a highly structured directory map of all content the owner wants indexed, often revealing internal tools that share the index route.
                                    </p>
                                </div>
                            </div>
						</div>
					</div>
				{/if}

			</div>
		</div>
	</div>
{/if}

<style>
	/* Use a global modifier pattern for the custom scrollbar to avoid Svelte unused-selector warnings */
	:global(.custom-scrollbar::-webkit-scrollbar) {
		width: 6px;
		height: 6px;
	}
	:global(.custom-scrollbar::-webkit-scrollbar-track) {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 4px;
	}
	:global(.custom-scrollbar::-webkit-scrollbar-thumb) {
		background: rgba(39, 39, 42, 0.8);
		border-radius: 4px;
	}
	:global(.custom-scrollbar::-webkit-scrollbar-thumb:hover) {
		background: rgba(63, 63, 70, 0.8);
	}
</style>
