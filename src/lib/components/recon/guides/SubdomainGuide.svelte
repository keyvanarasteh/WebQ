<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Network, Search, AlertTriangle, ShieldCheck, ChevronRight, ChevronLeft, ArrowRight } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Overview", icon: Network },
        { id: 1, label: "2. Techniques", icon: Search },
        { id: 2, label: "3. Mitigation", icon: ShieldCheck }
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
        class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6 bg-black/80 backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div 
            class="bg-[#0A0C10] border border-indigo-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(99,102,241,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-blue-950/40 via-indigo-900/10 to-transparent p-6 border-b border-blue-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-blue-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-zinc-400 hover:text-white bg-white/5 hover:bg-blue-500/20 rounded-xl transition-all border border-white/5 hover:border-blue-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-blue-500/20 to-indigo-600/10 flex items-center justify-center border border-blue-500/30 shadow-inner">
                            <Network size={28} class="text-blue-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-white">{m.recon_subdomain_guide_title()}</h2>
                            <p class="text-sm text-zinc-400 mt-1 max-w-md">{m.recon_subdomain_guide_desc()}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-blue-500/20 text-blue-300 border border-blue-500/20 shadow-sm' : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5 border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 relative custom-scrollbar">
                {#if activeTab === 0}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-blue-300 mb-4">Infrastructure Surface Mapping</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-zinc-300 text-sm leading-relaxed">
                                    Subdomain discovery leverages high-speed passive reconnaissance to map a target's infrastructure surface. Filtered domains remove common false positives (e.g., Azure or Google CDN noise).
                                </p>
                                <div class="bg-red-500/5 border border-red-500/20 rounded-xl p-6">
                                    <h3 class="text-sm font-bold text-red-400 mb-2 tracking-widest uppercase">Subdomain Takeover Risk</h3>
                                    <p class="text-xs text-gray-400 mb-4">If a subdomain has a CNAME record pointing to an unclaimed external service (e.g., <code>staging.target.com &rarr; app.herokuapps.com</code>), attackers can register the endpoint and serve malicious content.</p>
                                    <div class="text-xs border-l-2 border-red-500 pl-3 text-red-300">
                                        <strong>SecOps:</strong> Regularly audit all CNAME records. Monitor for dangling DNS pointers.
                                    </div>
                                </div>
                            </div>
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 relative overflow-hidden shadow-inner h-full flex flex-col justify-center">
                                <div class="absolute inset-0 bg-[linear-gradient(to_right,#3b82f610_1px,transparent_1px),linear-gradient(to_bottom,#3b82f610_1px,transparent_1px)] bg-size-[24px_24px]"></div>
                                <div class="relative z-10 space-y-4">
                                    <div class="flex items-center gap-3 bg-zinc-900/80 p-3 rounded-lg border border-white/10 backdrop-blur">
                                        <div class="bg-blue-500/20 p-2 rounded text-blue-400"><Network size={16}/></div>
                                        <div>
                                            <p class="text-sm font-bold text-zinc-200">api.target.com</p>
                                            <p class="text-xs text-emerald-400">Active (A: 192.168.1.10)</p>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-3 bg-zinc-900/80 p-3 rounded-lg border border-red-500/30 shadow-[0_0_15px_rgba(239,68,68,0.15)] backdrop-blur ml-6">
                                        <div class="bg-red-500/20 p-2 rounded text-red-400"><AlertTriangle size={16}/></div>
                                        <div>
                                            <p class="text-sm font-bold text-red-300">staging.target.com</p>
                                            <p class="text-xs text-red-400/80">Vulnerable: Dangling CNAME (herokuapps)</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-blue-300">Discovery Techniques</h3>
                        </div>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">Understanding passive vs. active enumeration tactics for identifying exposed perimeter assets.</p>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
                            <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-indigo-500">
                                <div class="flex items-center gap-2 mb-3">
                                    <Search class="size-5 text-indigo-400" />
                                    <h4 class="font-bold text-gray-200">Passive Enumeration</h4>
                                </div>
                                <p class="text-xs text-gray-400 leading-relaxed">Uses <strong>Certificate Transparency (CT)</strong> logs, search engine caches, and WHOIS data to discover subdomains without sending any traffic to the target. Completely stealth.</p>
                            </div>
                            
                            <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-red-500">
                                <div class="flex items-center gap-2 mb-3">
                                    <AlertTriangle class="size-5 text-red-400" />
                                    <h4 class="font-bold text-gray-200">Active Brute-Forcing</h4>
                                </div>
                                <p class="text-xs text-gray-400 leading-relaxed">DNS resolution brute-force against wordlists (e.g., SecLists). Noisy but discovers internal-only subdomains not indexed in public CT logs or search engines.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Securing the Perimeter</h3>

                        <div class="space-y-4">
                            <div class="p-5 rounded-xl border border-emerald-500/20 bg-emerald-500/5 hover:border-emerald-500/40 transition-colors">
                                <h4 class="font-bold text-emerald-300 mb-2 flex items-center gap-2">
                                    <ShieldCheck class="size-5" /> Secure Developer Protocols
                                </h4>
                                <ul class="space-y-3 text-xs text-gray-400 mt-4 pl-2">
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0"/> Implement wildcard DNS monitoring to alert on new unapproved subdomains.</li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0"/> Use tools like <code>can-i-take-over-xyz</code> to audit CNAME records continuously.</li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0"/> Restrict zone transfers (AXFR) on your authoritative DNS servers to prevent complete zone leaking.</li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0"/> Remove unused subdomain records immediately upon decommissioning cloud services or marketing campaigns.</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-blue-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-zinc-500 hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-white/5 rounded mx-1 text-zinc-400 border border-white/10 font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button 
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-white/5 hover:bg-white/10 text-zinc-300 text-sm font-medium rounded-xl border border-white/5 hover:border-white/10 transition-all focus:ring-2 focus:ring-white/10 outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}
                    
                    {#if activeTab < tabs.length - 1}
                        <button 
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(59,130,246,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-blue-500/50 outline-none"
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
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
