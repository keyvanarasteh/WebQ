<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Layers, Fingerprint, Shield, CheckCircle2, ChevronRight, ChevronLeft, Lock } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Fingerprinting", icon: Fingerprint },
        { id: 1, label: "2. Target: WordPress", icon: Shield },
        { id: 2, label: "3. Defense", icon: Lock }
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
            class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-zinc-400 hover:text-white bg-white/5 hover:bg-cyan-500/20 rounded-xl transition-all border border-white/5 hover:border-cyan-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30 shadow-inner">
                            <Layers size={28} class="text-cyan-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-white">{m.recon_tech_title ? m.recon_tech_title() : 'Web Technologies & Fingerprinting'}</h2>
                            <p class="text-sm text-zinc-400 mt-1 max-w-md">Identifying frameworks, servers, CMS, and analytics platforms.</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20 shadow-sm' : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">The First Step</h3>
                        <p class="text-sm text-zinc-400 mb-8 max-w-3xl">
                            Identifying the exact <strong class="text-white">technology stack</strong> (frameworks, servers, CMS, analytics) of a target is Step 1 in performing a vulnerability assessment. If you know the version, you know the exploits.
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div class="bg-[#121214] border border-[#27272a] p-6 rounded-xl border-t-2 border-t-cyan-500 shadow-lg">
                                <div class="flex items-center gap-2 mb-4">
                                    <Fingerprint class="size-6 text-cyan-400" />
                                    <h3 class="font-bold text-gray-200 text-lg">Passive Fingerprinting</h3>
                                </div>
                                <p class="text-sm text-gray-400 leading-relaxed">Analyzing the raw DOM, standard headers, and JavaScript global objects (e.g. <code>window.React</code> or Wappalyzer signatures) to detect tech without ringing alarms.</p>
                            </div>
                            
                            <div class="bg-[#121214] border border-[#27272a] p-6 rounded-xl border-t-2 border-t-red-500 shadow-lg">
                                <div class="flex items-center gap-2 mb-4">
                                    <Shield class="size-6 text-red-400" />
                                    <h3 class="font-bold text-gray-200 text-lg">Active Probing</h3>
                                </div>
                                <p class="text-sm text-gray-400 leading-relaxed">Sending deliberate requests to known API endpoints (e.g., <code>/wp-json/wp/v2/users</code> or <code>.env</code> paths) to forcibly extract sensitive data, exact version numbers, and plugin hashes.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-blue-400 uppercase tracking-widest">Why WordPress is Highly Targeted</h3>
                        </div>
                        
                        <div class="bg-blue-500/5 border border-blue-500/20 rounded-xl p-6 shadow-inner">
                            <ul class="space-y-6">
                                <li class="flex items-start gap-4">
                                    <div class="mt-1 bg-blue-500/10 p-2 rounded-full border border-blue-500/20">
                                        <CheckCircle2 class="size-5 text-blue-400 shrink-0" />
                                    </div>
                                    <div>
                                        <p class="text-base font-bold text-gray-200">User Enumeration via REST API</p>
                                        <p class="text-sm text-gray-400 mt-2 leading-relaxed">By default, WordPress allows unauthenticated access to the user endpoint. Attackers scrape usernames and perform targeted brute-force attacks on the <code>/wp-login.php</code> portal.</p>
                                    </div>
                                </li>
                                <li class="flex items-start gap-4">
                                    <div class="mt-1 bg-red-500/10 p-2 rounded-full border border-red-500/20">
                                        <Shield class="size-5 text-red-400 shrink-0" />
                                    </div>
                                    <div>
                                        <p class="text-base font-bold text-gray-200">Vulnerable Plugin Sprawl</p>
                                        <p class="text-sm text-gray-400 mt-2 leading-relaxed">90% of WP hacks occur via outdated 3rd-party plugins. Fingerprinting which plugins exist is a gold mine for Remote Code Execution (RCE) mapping against CVE databases.</p>
                                    </div>
                                </li>
                            </ul>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Secure Developer Protocols</h3>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">Implementing defensive measures to limit fingerprinting effectively mitigates active and passive probes.</p>

                        <div class="bg-[#0A0C10] border-l-4 border-emerald-500 p-6 rounded-r-xl border-y border-r  shadow-lg">
                            <h4 class="font-bold text-emerald-400 mb-4 flex items-center gap-2">
                                <Lock size={18} /> Remediation Steps
                            </h4>
                            <ul class="space-y-4 text-sm text-gray-300">
                                <li class="flex items-start gap-2">
                                    <ChevronRight size={16} class="text-emerald-500 mt-0.5"/>
                                    <span>Always disable <code>X-Powered-By</code> headers in Node.js, PHP, and other frameworks to obscure the backend language.</span>
                                </li>
                                <li class="flex items-start gap-2">
                                    <ChevronRight size={16} class="text-emerald-500 mt-0.5"/>
                                    <span>If using WordPress, utilize security plugins to block generic REST API access entirely for unauthenticated users.</span>
                                </li>
                                <li class="flex items-start gap-2">
                                    <ChevronRight size={16} class="text-emerald-500 mt-0.5"/>
                                    <span>Randomize administrative login paths (e.g., move away from <code>/wp-admin</code> or <code>/wp-login.php</code>).</span>
                                </li>
                                <li class="flex items-start gap-2">
                                    <ChevronRight size={16} class="text-emerald-500 mt-0.5"/>
                                    <span>Disable user enumeration queries, such as redirecting or blocking requests parameterizing <code>?author=1</code> searches.</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-white text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-cyan-500/50 outline-none"
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
