<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, ListChecks, Eye, ShieldCheck, ChevronRight, ChevronLeft, Target } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Bulk Validation", icon: ListChecks },
        { id: 1, label: "2. Diagnostic Layers", icon: Eye },
        { id: 2, label: "3. Threat Intel", icon: Target }
    ];

    function nextTab() {
        if (activeTab < tabs.length - 1) activeTab++;
    }
    function prevTab() {
        if (activeTab > 0) activeTab--;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div 
        class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div 
            class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30 shadow-inner">
                            <ListChecks size={28} class="text-accent" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.recon_validator_title ? m.recon_validator_title() : 'Bulk Validation & Live Recon'}</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Attack Surface Management through endpoint verification.</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 relative">
                {#if activeTab === 0}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Initial Scoping</h3>
                        <p class="text-sm text-muted mb-8 max-w-3xl leading-relaxed">
                            Bulk validation acts as the initial broad stroke of <strong class="text-primary-text">Attack Surface Management (ASM)</strong>. When dealing with thousands of potential subdomains, confirming what is alive and actively resolving over HTTP/HTTPS is crucial for scoping engagements and reducing noise.
                        </p>

                        <div class="bg-linear-to-r from-blue-900/20 to-transparent border-l-4 border-blue-500 p-6 rounded-r-xl shadow-lg">
                            <p class="text-blue-300 font-semibold mb-2">Why It Matters</p>
                            <p class="text-sm text-muted leading-relaxed">
                                You cannot brute force vulnerabilities on an endpoint that is internally inaccessible or dead. Validating live hosts ensures you direct deep-scanning efforts (like port mapping or fuzzing) only at responding targets, drastically reducing assessment times.
                            </p>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-cyan-300">Diagnostic Layers</h3>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Active probing operates hierarchically across several protocols.</p>
                        
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <div class="bg-sunken border border-base p-6 rounded-xl flex flex-col items-center text-center shadow-lg hover:border-purple-500/30 transition-colors">
                                <div class="p-4 bg-purple-500/10 border border-purple-500/20 text-purple-400 rounded-2xl mb-4 shadow-inner"><Eye class="size-6" /></div>
                                <h4 class="font-bold text-primary-text text-base mb-3">DNS Resolution</h4>
                                <p class="text-xs text-muted leading-relaxed">Detects if the domain resolves to an IP. A missing DNS record but present subdomain in a bruteforce list might indicate a potential Subdomain Takeover vector.</p>
                            </div>

                            <div class="bg-sunken border border-base p-6 rounded-xl flex flex-col items-center text-center shadow-lg hover:border-blue-500/30 transition-colors">
                                <div class="p-4 bg-blue-500/10 border border-blue-500/20 text-blue-400 rounded-2xl mb-4 shadow-inner"><ListChecks class="size-6" /></div>
                                <h4 class="font-bold text-primary-text text-base mb-3">Port 80 (HTTP)</h4>
                                <p class="text-xs text-muted leading-relaxed">Unencrypted web traffic. Modern sites should immediately 301 redirect this to HTTPS. A 200 OK on Port 80 without redirection is an instant security audit flag.</p>
                            </div>

                            <div class="bg-sunken border border-base p-6 rounded-xl flex flex-col items-center text-center shadow-lg hover:border-green-500/30 transition-colors">
                                <div class="p-4 bg-green-500/10 border border-green-500/20 text-green-400 rounded-2xl mb-4 shadow-inner"><ShieldCheck class="size-6" /></div>
                                <h4 class="font-bold text-primary-text text-base mb-3">Port 443 (HTTPS)</h4>
                                <p class="text-xs text-muted leading-relaxed">Secure web traffic. Successful connection indicates an active service and SSL certificate deployment, meaning the endpoint is a live target for deep scanning.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Automated Discovery Funnel</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl leading-relaxed">Understanding the funneling phase of a penetration test or bounty hunting session.</p>

                        <div class="bg-glass border border-emerald-500/20 rounded-xl p-8 shadow-inner relative overflow-hidden">
                            <div class="absolute right-0 top-0 w-48 h-48 bg-emerald-500/5 rounded-bl-full pointer-events-none"></div>
                            
                            <div class="flex flex-col gap-6 relative z-10">
                                <div class="flex items-center gap-4">
                                    <div class="bg-surface text-primary-text font-mono text-xs px-3 py-1 rounded border border-base">10,000+</div>
                                    <p class="text-sm text-primary-text"><strong>Subdomain Scraping:</strong> Tools like Amass and Subfinder pull raw, noisy permutations from public sources.</p>
                                </div>
                                <div class="w-0.5 h-6 bg-emerald-500/30 ml-8"></div>
                                
                                <div class="flex items-center gap-4">
                                    <div class="bg-cyan-900 border border-cyan-500 text-cyan-300 font-mono text-xs px-3 py-1 rounded shadow-[0_0_10px_rgba(6,182,212,0.3)]">Bulk Validation</div>
                                    <p class="text-sm text-primary-text"><strong>HTTPX / Validator:</strong> Pings all domains on P80/443 async to filter out dead space.</p>
                                </div>
                                <div class="w-0.5 h-6 bg-emerald-500/30 ml-8"></div>
                                
                                <div class="flex items-center gap-4">
                                    <div class="bg-emerald-900 border border-emerald-500 text-emerald-400 font-bold font-mono text-xs px-3 py-1 rounded shadow-[0_0_15px_rgba(16,185,129,0.5)]">~300 Live</div>
                                    <p class="text-sm text-emerald-100"><strong>Deep Target Scope:</strong> SecOps tooling focuses solely on those 300 live targets to preserve time, bandwidth, and OPSEC in tools like Nuclei or ZAP.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-muted hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-glass rounded mx-1 text-muted border border-glass font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button 
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle hover:border-glass transition-all focus:ring-2 focus:ring-glass outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}
                    
                    {#if activeTab < tabs.length - 1}
                        <button 
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-cyan-500/50 outline-none"
                        >
                            Continue <ChevronRight size={16} />
                        </button>
                    {:else}
                        <button 
                            onclick={close}
                            class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(16,185,129,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-emerald-500/50 outline-none"
                        >
                            Complete
                        </button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    :global(.custom-scrollbar::-webkit-scrollbar) {
        width: 8px;
        height: 8px;
    }
    :global(.custom-scrollbar::-webkit-scrollbar-track) {
        background: transparent;
    }
    :global(.custom-scrollbar::-webkit-scrollbar-thumb) {
        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    :global(.custom-scrollbar::-webkit-scrollbar-thumb:hover) {
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
