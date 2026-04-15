<script lang="ts">
	import { fade, slide } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import * as m from '$lib/paraglide/messages';
	import type { ScanProgressEvent } from '$lib/types/intelligence';
	import { formatRelativeTime } from '$lib/utils/time';

	// Components
	import SubdomainGrid from '$lib/components/recon/subdomain-discovery/SubdomainGrid.svelte';
	import SubdomainGuide from '$lib/components/recon/guides/SubdomainGuide.svelte';
    import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';
	import { reportStore } from '$lib/stores/ReportStore.svelte';

	// Icons
	import { Network, Search, LayoutGrid, GitBranch, ShieldAlert, TimerReset, Filter, ActivitySquare, Globe, ShieldCheck, HelpCircle } from 'lucide-svelte';

	// ── State ─────────────────────────────────────────────────────────────────
	
	let targetDomain = $state('');
	let isScanning = $state(false);
	let errorMsg = $state<string | null>(null);
	
	// Defined data structure matching Rust
	type SubdomainDetail = {
		host: string;
		status: number | null;
		resolution_error: string | null;
	};

	type ResultPayload = {
		domain: string;
		subdomains: SubdomainDetail[];
		total_found: number;
		filtered_count: number;
		response_time_ms: number;
	};

	let scanResult = $state<ResultPayload | null>(null);
	let showGuide = $state(false);

	interface HistoricalScanHydration<T> {
		started_at: string;
		duration_ms: number;
		raw_json_blob: T;
	}

	let localHydration = $state<HistoricalScanHydration<ResultPayload> | null>(null);
	let scanLogs = $state<ScanProgressEvent[]>([]);
	let scanProgress = $state(0);
	let unlistenProgress: UnlistenFn | null = null;

	// ── Handlers ──────────────────────────────────────────────────────────────
	
	async function checkLocalHistory() {
		if (!targetDomain || targetDomain.length < 3) return;
		try {
			const history = await invoke<HistoricalScanHydration<ResultPayload> | null>('get_latest_domain_intel', {
				domain: targetDomain.trim(),
				scanModule: 'SubdomainDiscovery'
			});
			
			if (history) {
				localHydration = history;
				scanResult = history.raw_json_blob;
				errorMsg = null;
			} else {
				localHydration = null;
				scanResult = null;
			}
		} catch (e) {
			console.error("Failed history check", e);
		}
	}
	
	async function handleScan(e: Event) {
		e.preventDefault();
		if (!targetDomain || isScanning) return;
		
		isScanning = true;
		errorMsg = null;
		localHydration = null;
		scanLogs = [];
		scanProgress = 0;
		
		unlistenProgress = await listen<ScanProgressEvent>('scan-progress', (event) => {
			scanLogs.push(event.payload);
			scanProgress = event.payload.percentage;
		});
		
		try {
			// Trigger Rust Tauri command mapped to web-analyzer's subfinder
			const result: ResultPayload = await invoke('scan_subdomains', { 
				domain: targetDomain 
			});
			scanResult = result;
			scanProgress = 100;
			reportStore.addResult(targetDomain, "Subdomain Discovery", scanResult);
		} catch (err) {
			errorMsg = err as string;
			// Reset
			scanResult = null;
		} finally {
			if (unlistenProgress) {
				unlistenProgress();
				unlistenProgress = null;
			}
			isScanning = false;
		}
	}
</script>

<div class="flex h-full flex-col gap-6 p-6 overflow-y-auto">
	<div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-base pb-6">
		<div class="flex items-center gap-4">
			<div class="h-10 w-10 shrink-0 rounded-lg bg-indigo-500/20 flex items-center justify-center border border-indigo-500/30">
				<Network class="text-indigo-400 size-5" />
			</div>
			<div>
				<h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.nav_subdomain_discovery()}</h1>
				<p class="text-muted mt-2">Passive infrastructure surface discovery utilizing Open Source Intelligence.</p>
			</div>
			<button
				onclick={() => showGuide = true}
				class="p-2 ml-2 transition-colors border rounded-lg bg-surface border-base text-muted hover:text-primary-text"
				title="View SecOps Guide"
			>
				<HelpCircle class="w-4 h-4" />
			</button>
		</div>
	</div>

	<!-- Top Section: Interactive Guide & Input -->
	<div class="grid gap-6 lg:grid-cols-12">
		<div class="lg:col-span-8 flex flex-col gap-4">
			
			<!-- Input Form -->
			<div class="rounded-xl border border-base/50 bg-surface p-6 shadow-xl backdrop-blur-sm">
				<form onsubmit={handleScan} class="flex flex-col gap-4 sm:flex-row sm:items-end">
					<div class="flex-1 space-y-2">
						<label for="domain-input" class="text-sm font-semibold text-secondary-text font-fira">
							Target Domain
						</label>
						<div class="relative">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<Globe size={18} class="text-muted" />
							</div>
							<!-- svelte-ignore a11y_autofocus -->
							<input
								id="domain-input"
								type="text"
								list="historic-domains"
								bind:value={targetDomain}
								disabled={isScanning}
								onchange={() => checkLocalHistory()}
								placeholder="e.g. example.com"
								class="block w-full rounded-lg border border-base bg-surface p-3 pl-10 text-sm text-primary-text placeholder-muted shadow-inner focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 disabled:opacity-50"
								autofocus
							/>
						</div>
					</div>
					
					<button
						type="submit"
						disabled={isScanning || !targetDomain}
						class="flex h-11 items-center justify-center gap-2 rounded-lg bg-indigo-600 px-6 font-semibold text-primary-text shadow-lg shadow-indigo-600/20 transition-all hover:bg-indigo-500 hover:shadow-indigo-500/30 disabled:pointer-events-none disabled:opacity-50"
					>
						{#if isScanning}
							<div class="h-4 w-4 animate-spin rounded-full border-2 border-slate-300 border-t-white"></div>
							<span class="font-fira">Scanning...</span>
						{:else}
							<Search size={18} />
							<span class="font-fira">Discover</span>
						{/if}
					</button>
				</form>

				{#if errorMsg}
					<div class="mt-4 flex items-start gap-3 rounded-lg border border-rose-500/20 bg-rose-500/10 p-4 text-rose-400" transition:slide>
						<ShieldAlert size={20} class="shrink-0 mt-0.5" />
						<div class="flex flex-col">
							<span class="text-sm font-bold">Execution Failed</span>
							<span class="text-xs text-rose-300/80">{errorMsg}</span>
						</div>
					</div>
				{/if}
			</div>

			<!-- Stats Row -->
			{#if localHydration && !isScanning}
				<div class="flex items-center gap-2 justify-end -mt-2 animate-fade-in pr-2 mb-2">
					<span class="inline-flex items-center gap-1.5 px-3 py-1 bg-cyan-500/10 text-cyan-400 rounded-full text-[10px] font-mono border border-cyan-500/20 uppercase tracking-widest shrink-0">
						Restored Scan from {formatRelativeTime(localHydration.started_at)} ({localHydration.duration_ms}ms)
					</span>
				</div>
			{/if}

			<div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
				<div class="flex flex-col items-center justify-center rounded-xl border border-base/50 bg-surface p-4 shadow-sm relative overflow-hidden group">
					<div class="absolute inset-0 bg-linear-to-t from-sky-500/5 to-transparent"></div>
					<ActivitySquare size={20} class="text-sky-400 mb-2 relative z-10" />
					<span class="text-xs text-muted font-fira relative z-10">{m.recon_subdomain_total_found()}</span>
					<span class="text-2xl font-black text-secondary-text mt-1 relative z-10">
						{scanResult ? scanResult.total_found : '--'}
					</span>
				</div>
				
				<div class="flex flex-col items-center justify-center rounded-xl border border-base/50 bg-surface p-4 shadow-sm relative overflow-hidden group">
					<div class="absolute inset-0 bg-linear-to-t from-emerald-500/5 to-transparent"></div>
					<ShieldCheck size={20} class="text-emerald-400 mb-2 relative z-10" />
					<span class="text-xs text-muted font-fira relative z-10">Resolved</span>
					<span class="text-2xl font-black text-secondary-text mt-1 relative z-10">
						{scanResult ? scanResult.subdomains.length : '--'}
					</span>
				</div>

				<div class="flex flex-col items-center justify-center rounded-xl border border-base/50 bg-surface p-4 shadow-sm relative overflow-hidden group">
					<div class="absolute inset-0 bg-linear-to-t from-rose-500/5 to-transparent"></div>
					<Filter size={20} class="text-rose-400 mb-2 relative z-10" />
					<span class="text-xs text-muted font-fira relative z-10">{m.recon_subdomain_filtered()}</span>
					<span class="text-2xl font-black text-secondary-text mt-1 relative z-10">
						{scanResult ? scanResult.filtered_count : '--'}
					</span>
				</div>

				<div class="flex flex-col items-center justify-center rounded-xl border border-base/50 bg-surface p-4 shadow-sm relative overflow-hidden group">
					<div class="absolute inset-0 bg-linear-to-t from-amber-500/5 to-transparent"></div>
					<TimerReset size={20} class="text-amber-400 mb-2 relative z-10" />
					<span class="text-xs text-muted font-fira relative z-10">{m.recon_subdomain_response_time()}</span>
					<span class="text-2xl font-black text-secondary-text mt-1 relative z-10">
						{scanResult ? `${scanResult.response_time_ms}ms` : '--'}
					</span>
				</div>
			</div>
		</div>

		{#if isScanning || scanLogs.length > 0}
			<div class="mt-4 animate-fade-in lg:col-span-8">
				<ScanTerminal logs={scanLogs} progressPercent={scanProgress} />
			</div>
		{/if}

		<SubdomainGuide bind:isOpen={showGuide} />
	</div>

	<!-- Results Layout -->
	<div class="flex flex-col flex-1 rounded-xl border border-base/50 bg-surface overflow-hidden shadow-xl" in:fade>
		<!-- Control Bar -->
		<div class="flex items-center justify-between border-b border-base/50 bg-surface px-4 py-3">
			<h3 class="text-sm font-bold text-secondary-text font-fira flex items-center gap-2">
				<Network size={16} class="text-indigo-400" />
				{m.recon_subdomain_board_title()}
			</h3>
		</div>

		<!-- Content Area -->
		<div class="flex-1 p-4 overflow-y-auto min-h-[350px]">
			<div in:fade>
				<SubdomainGrid data={scanResult?.subdomains ?? null} isPending={!scanResult} />
			</div>
		</div>
	</div>
</div>
