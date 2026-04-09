<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import { 
		X, 
		Shield, 
		Lock,
		AlertTriangle, 
		CheckCircle,
		XCircle,
		BookOpen
	} from 'lucide-svelte';
	import { onMount, onDestroy } from 'svelte';

	let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

	let activeTab = $state<'intro' | 'headers' | 'reality'>('intro');

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
		{ id: 'headers', label: 'Core Headers', icon: Lock },
		{ id: 'reality', label: 'SecOps Reality', icon: AlertTriangle }
	] as const;
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6"
		role="dialog"
		aria-modal="true"
		aria-labelledby="security-headers-guide-title"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 bg-black/80 backdrop-blur-sm cursor-default w-full h-full border-none p-0 focus:outline-none"
			onclick={() => (isOpen = false)}
			aria-label="Close"
			transition:fade={{ duration: 200 }}
		></button>

		<div
			class="relative w-full max-w-4xl bg-background border border-base rounded-xl shadow-2xl overflow-hidden flex flex-col md:flex-row shadow-emerald-900/20"
			transition:fly={{ y: 20, duration: 400, easing: backOut }}
		>
			<!-- Close Button -->
			<button
				onclick={() => (isOpen = false)}
				class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-[#121214] hover:bg-surface rounded-full transition-colors z-10 focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
				aria-label="Close"
			>
				<X class="size-4" />
			</button>

			<!-- Sidebar Navigation -->
			<div class="w-full md:w-64 bg-[#121214] border-r border-base shrink-0 flex flex-col">
				<div class="p-6 border-b border-base">
					<div class="flex items-center gap-3">
						<div class="p-2 bg-emerald-500/10 rounded-lg">
							<Shield class="size-5 text-emerald-400" />
						</div>
						<div>
							<h2 id="security-headers-guide-title" class="font-bold text-primary-text text-sm">Security Headers</h2>
							<div class="text-[10px] text-emerald-500/70 uppercase tracking-widest font-mono mt-1">Vulnerability</div>
						</div>
					</div>
				</div>

				<div class="p-4 flex-1 flex flex-row md:flex-col gap-2 overflow-x-auto md:overflow-visible">
					{#each tabs as tab (tab.id)}
						<button
							onclick={() => (activeTab = tab.id)}
							class="flex items-center gap-3 px-4 py-3 rounded-lg text-sm transition-all duration-200 text-left whitespace-nowrap focus:outline-none focus:ring-2 focus:ring-emerald-500/50
                            {activeTab === tab.id
								? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20'
								: 'text-muted hover:bg-surface hover:text-primary-text border border-transparent'}"
						>
							<tab.icon class="size-4 {activeTab === tab.id ? 'text-emerald-400' : 'text-muted'}" />
							<span class="font-medium">{tab.label}</span>
						</button>
					{/each}
				</div>
                
                <div class="p-6 border-t border-base hidden md:block">
					<div class="bg-background rounded p-3 border border-base">
						<div class="text-[10px] font-mono text-muted uppercase tracking-wider mb-1">Module Standard</div>
						<div class="text-xs text-primary-text font-medium">OWASP Top 10</div>
					</div>
				</div>
			</div>

			<!-- Content Area -->
			<div class="flex-1 overflow-y-auto custom-scrollbar p-6 md:p-8 max-h-[80vh] md:max-h-[600px] bg-background">
				<!-- INTRO TAB -->
				{#if activeTab === 'intro'}
					<div in:fade={{ duration: 200, delay: 100 }}>
						<h3 class="text-xl font-bold text-primary-text mb-4">HTTP Security Headers</h3>
						<p class="text-sm leading-relaxed text-muted mb-6">
							<strong class="text-primary-text">Security Headers</strong> are HTTP response headers that restrict modern browsers from running into easily preventable vulnerabilities like Cross-Site Scripting (XSS), Clickjacking, or packet sniffing. Finding missing security headers is step one of any fundamental penetration test or bug bounty.
						</p>

						<div class="grid grid-cols-1 gap-4">
                            <div class="p-5 bg-emerald-500/5 border border-emerald-500/20 rounded-xl flex items-start gap-4">
								<div class="p-2 bg-emerald-500/10 rounded border border-emerald-500/20 shrink-0">
									<CheckCircle class="size-5 text-emerald-400" />
								</div>
								<div>
									<h4 class="text-sm font-bold text-emerald-300 mb-1">Zero-Cost Security</h4>
									<p class="text-xs text-emerald-200/70 leading-relaxed">
										Implementing headers requires no code changes to your application logic, only a single block of server or proxy configuration. They provide immediate, massive returns on securing client states.
									</p>
								</div>
							</div>
						</div>
					</div>
				{/if}

				<!-- HEADERS TAB -->
				{#if activeTab === 'headers'}
					<div in:fade={{ duration: 200, delay: 100 }}>
						<h3 class="text-xl font-bold text-primary-text mb-2 flex items-center gap-2">
                            <Lock class="size-5 text-emerald-400" /> Core Headers Breakdown
                        </h3>
						<p class="text-sm text-muted mb-6">
							Essential headers that must be present in every HTTP response.
						</p>

						<div class="space-y-4">
                            <!-- Strict-Transport-Security -->
                            <div class="bg-[#121214] border border-base p-4 rounded-xl">
                                <h4 class="font-bold text-green-400 text-sm mb-2 font-mono flex items-center gap-2">
                                    Strict-Transport-Security (HSTS)
                                    <span class="px-2 py-0.5 rounded text-[9px] bg-green-500/20 text-green-400 border border-green-500/30 uppercase tracking-widest font-sans">Critical</span>
                                </h4>
                                <p class="text-xs text-muted mb-3">Forces the browser to always connect via HTTPS, preventing man-in-the-middle SSL downgrade attacks.</p>
                                <div class="bg-background p-3 rounded-lg border border-base overflow-x-auto custom-scrollbar">
                                    <code class="text-xs font-mono text-primary-text whitespace-nowrap">Strict-Transport-Security: max-age=31536000; includeSubDomains; preload</code>
                                </div>
                            </div>
                  
                            <!-- Content-Security-Policy -->
                            <div class="bg-[#121214] border border-base p-4 rounded-xl">
                                <h4 class="font-bold text-blue-400 text-sm mb-2 font-mono flex items-center gap-2">
                                    Content-Security-Policy (CSP)
                                    <span class="px-2 py-0.5 rounded text-[9px] bg-blue-500/20 text-blue-400 border border-blue-500/30 uppercase tracking-widest font-sans">Advanced</span>
                                </h4>
                                <p class="text-xs text-muted mb-3">The ultimate defense against Cross-Site Scripting (XSS). It defines a strict whitelist of locations where the browser is allowed to load scripts, images, and styles from.</p>
                                <div class="bg-background p-3 rounded-lg border border-base overflow-x-auto custom-scrollbar">
                                    <code class="text-xs font-mono text-primary-text whitespace-nowrap">Content-Security-Policy: default-src 'self'; script-src 'self' https://apis.google.com</code>
                                </div>
                            </div>
                  
                            <!-- X-Frame-Options -->
                            <div class="bg-[#121214] border border-base p-4 rounded-xl">
                                <h4 class="font-bold text-orange-400 text-sm mb-2 font-mono flex items-center gap-2">
                                    X-Frame-Options
                                    <span class="px-2 py-0.5 rounded text-[9px] bg-orange-500/20 text-orange-400 border border-orange-500/30 uppercase tracking-widest font-sans">Critical</span>
                                </h4>
                                <p class="text-xs text-muted mb-3">Prevents Clickjacking attacks by denying browsers from rendering the page inside an invisible <code>&lt;iframe&gt;</code> on an attacker's website.</p>
                                <div class="bg-background p-3 rounded-lg border border-base overflow-x-auto custom-scrollbar">
                                    <code class="text-xs font-mono text-orange-300 whitespace-nowrap">X-Frame-Options: DENY</code>
                                </div>
                            </div>
						</div>
					</div>
				{/if}

				<!-- REALITY TAB -->
				{#if activeTab === 'reality'}
					<div in:fade={{ duration: 200, delay: 100 }}>
						<h3 class="text-xl font-bold text-primary-text mb-2 flex items-center gap-2">
							<AlertTriangle class="size-5 text-emerald-400" /> Dev vs. SecOps Reality
						</h3>
						<p class="text-sm text-muted mb-6">
							The gap between theoretical ratings and practical danger.
						</p>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="p-5 bg-red-500/5 border border-red-500/20 rounded-xl flex flex-col gap-3">
                                <h5 class="text-xs font-bold text-red-500 uppercase tracking-widest flex items-center gap-2">
                                    <XCircle class="size-4" /> SecOps Reality
                                </h5>
                                <p class="text-xs text-muted leading-relaxed">
                                    Missing headers are usually marked as 'Low' or 'Informational' severity in automated scanners. However, chaining a missing CSP with a minor Stored XSS vulnerability immediately escalates the threat to <strong>'Critical' Account Takeover</strong>.
                                </p>
                            </div>

                            <div class="p-5 bg-green-500/5 border border-green-500/20 rounded-xl flex flex-col gap-3">
                                <h5 class="text-xs font-bold text-green-500 uppercase tracking-widest flex items-center gap-2">
                                    <CheckCircle class="size-4" /> Secure Dev Duty
                                </h5>
                                <p class="text-xs text-muted leading-relaxed">
                                    Implementation is extremely easy but widely forgotten. In SvelteKit frontend servers or reverse proxies like NGINX, a standard block of hardcoded default headers solves 99% of configuration audits instantly.
                                </p>
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
