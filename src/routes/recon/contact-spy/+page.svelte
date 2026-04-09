<script lang="ts">
	import { appState } from '$lib/stores/AppState.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import * as m from '$lib/paraglide/messages';

	// Components
	import ContactMasonry from '$lib/components/recon/contact-spy/ContactMasonry.svelte';
	import CrawlingConsole, { type CrawlLog } from '$lib/components/recon/contact-spy/CrawlingConsole.svelte';
	import ContactSpyGuide from '$lib/components/recon/guides/ContactSpyGuide.svelte';
	import { reportStore } from '$lib/stores/ReportStore.svelte';

	// Icons
	import { Users, Search, ActivitySquare, Fingerprint, HelpCircle } from 'lucide-svelte';

	// ── State ─────────────────────────────────────────────────────────────────
	
	let targetDomain = $state('');
	let showGuide = $state(false);

	// Strongly typed according to backend response
	let scanResult = $state<any>(null); // ContactSpyResult
	let logs = $state<CrawlLog[]>([]);

	let logIdCounters = 0;
	let unlistenCrawlStatus: (() => void) | null = null;

	$effect(() => {
		async function setupListener() {
			unlistenCrawlStatus = await listen<{ url: string, status: string }>('crawl_status', (event) => {
				const now = new Date();
				const payload = event.payload;
				logs.push({
					id: logIdCounters++,
					timestamp: now.toISOString().slice(11, 23),
					status: 'info',
					message: `Scraping: ${payload.url}`
				});
				if (logs.length > 500) logs.shift(); // Keep bounded
			});
		}
		setupListener();

		return () => {
			if (unlistenCrawlStatus) unlistenCrawlStatus();
		};
	});

	// ── Logic ─────────────────────────────────────────────────────────────────

	async function performScan() {
		if (!targetDomain) return;
		appState.setScanning(true, 'CONTACT SPY');
		scanResult = null;
		logs = [];
		showGuide = false; // Collapse guide when starting scan to reveal full card width
		
		try {
			logs.push({
				id: logIdCounters++,
				timestamp: new Date().toISOString().slice(11, 23),
				status: 'success',
				message: `Started BFS Spider task on ${targetDomain} with MAX_DEPTH 2...`
			});
			// Limit to 25 pages by default to keep responsiveness high
			scanResult = await invoke('scan_contacts', { domain: targetDomain, maxPages: 25 });
			reportStore.addResult(targetDomain, "Contact Spy", scanResult);
			logs.push({
				id: logIdCounters++,
				timestamp: new Date().toISOString().slice(11, 23),
				status: 'success',
				message: `Successfully mapped surface contacts.`
			});
		} catch (e: unknown) {
			console.error("Scan Failed", e);
			const msg = e instanceof Error ? e.message : String(e);
			logs.push({
				id: logIdCounters++,
				timestamp: new Date().toISOString().slice(11, 23),
				status: 'error',
				message: `Crawl aborted: ${msg}`
			});
		} finally {
			appState.setScanning(false, '');
		}
	}
</script>

<div class="flex h-full flex-col gap-6 p-6 overflow-y-auto">
	<div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-base pb-6">
		<div class="flex items-center gap-4">
			<div class="h-10 w-10 shrink-0 rounded-lg bg-emerald-500/20 flex items-center justify-center border border-emerald-500/30">
				<Fingerprint class="text-emerald-400 size-5" />
			</div>
			<div>
				<div class="flex items-center gap-3">
					<h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.nav_contact_spy()}</h1>
					<button 
						class="text-muted hover:text-emerald-400 transition-colors"
						onclick={() => showGuide = true}
						aria-label="View Educational Guide"
					>
						<HelpCircle class="w-5 h-5" />
					</button>
				</div>
				<p class="text-muted mt-2">Comprehensive extraction of organizational surface contacts via OSINT harvesting.</p>
			</div>
		</div>
	</div>

	<!-- Top Section: Input & Stats -->
	<div class="grid gap-6 lg:grid-cols-12">
		<!-- Left: Input Control -->
		<div class="lg:col-span-8 bg-elevated border border-base rounded-lg p-6 flex flex-col justify-center">
			<h2 class="text-lg font-bold text-primary-text mb-4 tracking-widest uppercase flex items-center gap-2">
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
						class="w-full bg-background border border-base rounded-lg py-3 pl-10 pr-4 text-primary-text focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-all font-mono text-sm"
						onkeydown={(e) => e.key === 'Enter' && performScan()}
						disabled={appState.isScanning}
					/>
				</div>
				<button 
					onclick={performScan}
					disabled={appState.isScanning}
					class="px-8 py-3 rounded-lg bg-emerald-500 text-on-accent font-bold uppercase tracking-wider hover:bg-emerald-400 focus:ring-2 focus:ring-emerald-500/50 focus:outline-none disabled:opacity-50 transition-all shrink-0 flex items-center gap-2"
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
		<div class="lg:col-span-4 bg-elevated border border-base rounded-lg p-6 flex flex-col items-center justify-center text-center">
			<Users size={32} class="text-emerald-500/50 mb-3" />
			<h3 class="text-primary-text font-bold tracking-widest uppercase text-sm mb-1">{m.nav_contact_spy()} Mode Active</h3>
			<p class="text-xs text-muted font-fira">OSINT techniques applied.</p>
		</div>
	</div>

	<!-- Results Area -->
	<div class="grow flex flex-col gap-6">
		{#if appState.isScanning || logs.length > 0}
			<CrawlingConsole logs={logs} />
		{/if}
		<ContactMasonry results={scanResult} isLoading={appState.isScanning} />
	</div>
</div>

<!-- Slide-over Educational Guide -->
<ContactSpyGuide bind:isOpen={showGuide} />
