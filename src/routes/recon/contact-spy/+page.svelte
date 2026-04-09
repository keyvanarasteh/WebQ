<script lang="ts">
	import { appState } from '$lib/stores/AppState.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import * as m from '$lib/paraglide/messages';

	// Components
	import ContactMasonry from '$lib/components/recon/contact-spy/ContactMasonry.svelte';
	import ContactSpyGuide from '$lib/components/recon/guides/ContactSpyGuide.svelte';

	// Icons
	import { Users, Search, ActivitySquare, Fingerprint } from 'lucide-svelte';

	// ── State ─────────────────────────────────────────────────────────────────
	
	let targetDomain = $state('');
	let showGuide = $state(false);

	// Strongly typed according to backend response
	let scanResult = $state<any>(null); // ContactSpyResult

	// ── Logic ─────────────────────────────────────────────────────────────────

	async function performScan() {
		if (!targetDomain) return;
		appState.setScanning(true, 'CONTACT SPY');
		scanResult = null;
		showGuide = false; // Collapse guide when starting scan to reveal full card width
		
		try {
			// Limit to 25 pages by default to keep responsiveness high
			scanResult = await invoke('scan_contacts', { domain: targetDomain, maxPages: 25 });
		} catch (e) {
			console.error("Scan Failed", e);
		} finally {
			appState.setScanning(false, '');
		}
	}
</script>

<div class="flex h-full flex-col gap-6 p-6 overflow-y-auto">
	<div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-[#27272a] pb-6">
		<div class="flex items-center gap-4">
			<div class="h-10 w-10 shrink-0 rounded-lg bg-emerald-500/20 flex items-center justify-center border border-emerald-500/30">
				<Fingerprint class="text-emerald-400 size-5" />
			</div>
			<div>
				<h1 class="text-3xl font-black text-white tracking-widest uppercase">{m.nav_contact_spy()}</h1>
				<p class="text-gray-400 mt-2">Comprehensive extraction of organizational surface contacts via OSINT harvesting.</p>
			</div>
		</div>
	</div>

	<!-- Top Section: Input & Stats -->
	<div class="grid gap-6 lg:grid-cols-12">
		<!-- Left: Input Control -->
		<div class="lg:col-span-8 bg-[#18181b] border border-[#27272a] rounded-lg p-6 flex flex-col justify-center">
			<h2 class="text-lg font-bold text-white mb-4 tracking-widest uppercase flex items-center gap-2">
				<Search size={18} class="text-emerald-500" />
				{m.recon_contact_target()}
			</h2>
			<div class="flex items-center gap-2 w-full">
				<div class="relative w-full">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-emerald-500" />
					<input 
						type="text" 
						bind:value={targetDomain} 
						placeholder="Enter target domain..."
						class="w-full bg-[#09090b] border border-[#27272a] rounded-lg py-3 pl-10 pr-4 text-white focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-all font-mono text-sm"
						onkeydown={(e) => e.key === 'Enter' && performScan()}
						disabled={appState.isScanning}
					/>
				</div>
				<button 
					onclick={performScan}
					disabled={appState.isScanning}
					class="px-8 py-3 rounded-lg bg-emerald-500 text-black font-bold uppercase tracking-wider hover:bg-emerald-400 focus:ring-2 focus:ring-emerald-500/50 focus:outline-none disabled:opacity-50 transition-all shrink-0 flex items-center gap-2"
				>
					{#if appState.isScanning}
						<ActivitySquare size={18} class="animate-pulse" />
						{m.recon_subdomain_pending()}
					{:else}
						<Fingerprint size={18} />
						{m.recon_subdomain_scan()}
					{/if}
				</button>
			</div>
		</div>

		<!-- Right: Quick Actions / KPI -->
		<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
		<div class="lg:col-span-4 bg-[#18181b] border border-[#27272a] rounded-lg p-6 flex flex-col items-center justify-center text-center group cursor-pointer hover:border-emerald-500/50 transition-colors" onclick={() => showGuide = true}>
			<Users size={32} class="text-emerald-500/50 group-hover:text-emerald-400 mb-3 transition-colors" />
			<h3 class="text-white font-bold tracking-widest uppercase text-sm mb-1">{m.btn_educational_guide()}</h3>
			<p class="text-xs text-slate-500 font-fira">Click to view OSINT techniques behind Contact Spying.</p>
		</div>
	</div>

	<!-- Results Area -->
	<div class="flex-grow">
		<ContactMasonry results={scanResult} isLoading={appState.isScanning} />
	</div>
</div>

<!-- Slide-over Educational Guide -->
{#if showGuide}
	<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-50 flex justify-end">
		<div class="fixed inset-0 bg-black/60 backdrop-blur-sm" onclick={() => showGuide = false}></div>
		<div class="relative z-50 h-full max-w-sm w-full animate-in slide-in-from-right-full duration-300">
			<ContactSpyGuide onClose={() => showGuide = false} />
		</div>
	</div>
{/if}
