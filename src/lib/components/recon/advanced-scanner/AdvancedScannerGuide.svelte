<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, FileSearch, ShieldAlert, Cpu, Terminal, Search, AlignLeft, AlertCircle, Play, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Fingerprinting", icon: Cpu },
        { id: 1, label: "2. Path Fuzzing", icon: FileSearch },
        { id: 2, label: "3. Vuln Profiling", icon: ShieldAlert }
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
        class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-black/80 backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div 
            class="bg-[#0A0C10] border border-teal-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(20,184,166,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-teal-950/40 via-indigo-900/10 to-transparent p-6 border-b border-teal-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-teal-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-white/5 hover:bg-teal-500/20 rounded-xl transition-all border border-white/5 hover:border-teal-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-teal-500/20 to-indigo-600/10 flex items-center justify-center border border-teal-500/30 shadow-inner">
                            <FileSearch size={28} class="text-teal-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.recon_scanner_guide_title ? m.recon_scanner_guide_title() : 'Advanced Deep Scanning'}</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">{m.recon_scanner_subtitle ? m.recon_scanner_subtitle() : 'Automated path fuzzing, fingerprinting, and surface discovery'}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-teal-500/20 text-teal-300 border border-teal-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-teal-300 mb-4">Technology Fingerprinting</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Advanced scanners begin by stripping the target bare. They analyze HTTP headers, injected cookies, and unique DOM signatures to identify the exact technology stack running beneath.</p>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-teal-500/30 shadow-[0_0_30px_rgba(20,184,166,0.1)] overflow-hidden font-mono text-sm mb-6 flex flex-col md:flex-row">
                            <div class="bg-surface/50 p-4 border-b md:border-b-0 md:border-r border-teal-500/20 w-full md:w-1/3 flex flex-col justify-center">
                                <div class="flex items-center gap-2 mb-2">
                                    <AlignLeft size={16} class="text-teal-400"/>
                                    <span class="text-teal-300 font-bold text-xs uppercase tracking-wider">Raw Response</span>
                                </div>
                                <p class="text-xs text-muted italic mb-4">Analyzing response anomalies...</p>
                                <div class="space-y-1 text-xs">
                                    <p class="text-orange-300">HTTP/2 200 OK</p>
                                    <p class="text-primary-text">Server: <span class="text-red-400 font-bold bg-red-400/10 px-1 rounded">cloudflare</span></p>
                                    <p class="text-primary-text">X-Powered-By: <span class="text-red-400 font-bold bg-red-400/10 px-1 rounded">PHP/8.1.2</span></p>
                                    <p class="text-primary-text">Set-Cookie: <span class="text-red-400 font-bold bg-red-400/10 px-1 rounded">PHPSESSID</span>=abc123</p>
                                </div>
                            </div>
                            <div class="p-6 relative flex-1">
                                <div class="absolute top-0 right-0 bg-teal-500/10 text-teal-400 px-4 py-1.5 rounded-bl-xl text-[10px] font-bold border-b border-l border-teal-500/30 tracking-widest uppercase">Scanner Inference Engine</div>
                                <div class="mt-4 space-y-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-8 h-8 rounded bg-orange-500/20 border border-orange-500/30 flex items-center justify-center text-orange-400 font-bold text-xs">CF</div>
                                        <div>
                                            <p class="text-sm font-semibold text-zinc-200">WAF Detected</p>
                                            <p class="text-xs text-muted">Cloudflare edge routing identified.</p>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-3">
                                        <div class="w-8 h-8 rounded bg-blue-500/20 border border-blue-500/30 flex items-center justify-center text-blue-400 font-bold text-xs">PHP</div>
                                        <div>
                                            <p class="text-sm font-semibold text-zinc-200">Backend Language</p>
                                            <p class="text-xs text-muted">PHP 8.1.2 (Potentially vulnerable to deserialization)</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-teal-300">Path Fuzzing & Forced Browsing</h3>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Attackers don't navigate websites; they brute-force them. By leveraging massive wordlists (like SecLists), scanners discover hidden administrative panels, `.git` exposures, and unlinked backup files.</p>
                        
                        <!-- Terminal Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-teal-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm shadow-[0_0_30px_rgba(0,0,0,0.5)]">
                            <div class="bg-surface/80 px-4 py-3 flex items-center justify-between border-b border-white/5 backdrop-blur">
                                <div class="flex items-center gap-2">
                                    <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                                    <span class="ml-2 text-muted text-xs tracking-wider">ffuf / gobuster simulation</span>
                                </div>
                                <span class="text-[10px] text-teal-500 font-bold uppercase tracking-widest px-2 py-0.5 bg-teal-500/10 rounded">Concurrent Threads: 40</span>
                            </div>
                            <div class="p-5 space-y-2 text-primary-text h-72 overflow-y-auto custom-scrollbar">
                                <p class="text-emerald-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">ffuf</span> -w /usr/share/seclists/Discovery/Web-Content/common.txt -u https://target.com/FUZZ</p>
                                <p class="text-muted mb-4">________________________________________________</p>
                                
                                <div class="flex items-center gap-4 text-emerald-400"><span class="w-16">200 OK</span> <span class="text-primary-text">/about</span> <span class="text-zinc-600 text-xs">(Size: 4502)</span></div>
                                <div class="flex items-center gap-4 text-emerald-400"><span class="w-16">200 OK</span> <span class="text-primary-text">/contact</span> <span class="text-zinc-600 text-xs">(Size: 3102)</span></div>
                                <div class="flex items-center gap-4 text-orange-400"><span class="w-16">301 MOVD</span> <span class="text-primary-text">/admin</span> <span class="text-zinc-600 text-xs">-> /admin/login.php</span></div>
                                <div class="flex items-center gap-4 text-orange-400"><span class="w-16">403 FORB</span> <span class="text-primary-text">/api/v1</span> <span class="text-zinc-600 text-xs">(Size: 152)</span></div>
                                <div class="flex items-center gap-4 text-orange-400"><span class="w-16">301 MOVD</span> <span class="text-primary-text">/.git</span> <span class="text-zinc-600 text-xs bg-red-500/20 text-red-400 px-1 rounded ml-2">HIGH RISK!</span></div>
                                <div class="flex items-center gap-4 text-emerald-400"><span class="w-16">200 OK</span> <span class="text-primary-text">/backup.zip</span> <span class="text-zinc-600 text-xs bg-red-500/20 text-red-400 px-1 rounded ml-2">SOURCE EXFILTRATION!</span></div>
                                
                                <p class="text-muted mt-4 border-t border-white/5 pt-2">:: Progress: [4614/4614] :: Job [1/1] :: 450 req/sec ::</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-indigo-400 mb-4">Vulnerability Profiling (CVEs)</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Modern automated scanners employ YAML-based templates (like Nuclei) to spray massive payloads identifying 1-day vulnerabilities instantly across thousands of subdomains.</p>

                        <!-- Mitigation Checklist -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
                            <div class="bg-indigo-950/10 border border-indigo-500/20 rounded-xl p-5 flex gap-4 hover:border-indigo-500/40 transition-colors">
                                <div class="bg-indigo-500/10 p-2 rounded-lg h-fit">
                                    <AlertCircle size={20} class="text-indigo-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-indigo-300 mb-1.5">Template Matching</h4>
                                    <p class="text-xs text-indigo-200/70 leading-relaxed">Scanners send highly specific payloads (e.g., Log4j JNDI strings) in HTTP headers. If an Out-of-Band (OAST) DNS ping is received, the vulnerability is confirmed.</p>
                                </div>
                            </div>
                            <div class="bg-teal-950/10 border border-teal-500/20 rounded-xl p-5 flex gap-4 hover:border-teal-500/40 transition-colors">
                                <div class="bg-teal-500/10 p-2 rounded-lg h-fit">
                                    <ShieldAlert size={20} class="text-teal-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-teal-300 mb-1.5">SecOps Defense</h4>
                                    <p class="text-xs text-teal-200/70 leading-relaxed">Defending against this requires severe Web Application Firewall (WAF) rate-limiting, honeypot endpoints, and continuous patch management (DevSecOps).</p>
                                </div>
                            </div>
                        </div>
                        
                        <div class="bg-indigo-900/20 border-l-4 border-indigo-500 p-4 rounded-r-xl">
                            <p class="text-sm text-indigo-200 font-medium mb-1">Zero-Day vs One-Day</p>
                            <p class="text-xs text-indigo-300/80 leading-relaxed">Scanners primarily find "One-Days" (known CVEs lacking patches). Zero-Days require manual human ingenuity, but automated recon provides the exact software versions needed to begin that research.</p>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-teal-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-muted hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-white/5 rounded mx-1 text-muted border border-white/10 font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button 
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-white/5 hover:bg-white/10 text-primary-text text-sm font-medium rounded-xl border border-white/5 hover:border-white/10 transition-all focus:ring-2 focus:ring-white/10 outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}
                    
                    {#if activeTab < tabs.length - 1}
                        <button 
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-teal-600 hover:bg-teal-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(20,184,166,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-teal-500/50 outline-none"
                        >
                            Continue <ChevronRight size={16} />
                        </button>
                    {:else}
                        <button 
                            onclick={close}
                            class="flex items-center gap-1.5 px-8 py-2.5 bg-indigo-500 hover:bg-indigo-400 text-primary-text text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(99,102,241,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-indigo-500/50 outline-none"
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
