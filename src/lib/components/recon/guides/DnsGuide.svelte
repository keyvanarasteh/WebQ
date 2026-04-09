<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Network, Globe, Mail, ShieldAlert, ShieldCheck, ArrowRight, Server, FileQuestion, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Address Mapping", icon: Globe },
        { id: 1, label: "2. CNAME Exploits", icon: FileQuestion },
        { id: 2, label: "3. Email Spoofing", icon: Mail }
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
            class="bg-[#0A0C10] border border-purple-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(168,85,247,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-purple-950/40 via-cyan-900/10 to-transparent p-6 border-b border-purple-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-purple-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-white/5 hover:bg-purple-500/20 rounded-xl transition-all border border-white/5 hover:border-purple-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-purple-500/20 to-cyan-600/10 flex items-center justify-center border border-purple-500/30 shadow-inner">
                            <Network size={28} class="text-purple-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.dns_guide_title ? m.dns_guide_title() : 'DNS Intelligence Recon'}</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Analyzing Address Maps, CNAME Vulnerabilities, & Org Email Boundaries.</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-purple-500/20 text-purple-300 border border-purple-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-purple-300 mb-4">Direct Address Resolution (A/AAAA)</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">The Domain Name System (DNS) is the phonebook of the internet. Analyzing standard records maps the physical infrastructure underneath a company's web presence.</p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <div class="bg-purple-950/20 border border-purple-500/10 rounded-xl p-4 h-full">
                                    <h4 class="text-purple-400 font-medium mb-3 flex items-center gap-2"><Globe size={16}/> Resolution Mechanics</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-purple-500 mt-1 shrink-0"/> <span><strong>A Records:</strong> Translates a hostname (e.g., `api.target.com`) into its corresponding 32-bit IPv4 address (e.g., `192.0.2.1`).</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-purple-500 mt-1 shrink-0"/> <span><strong>AAAA Records:</strong> The exact same mechanism, but uses 128-bit IPv6 addresses.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-purple-500 mt-1 shrink-0"/> <span><strong>Exposure Risk:</strong> These records reveal exactly where the server is hosted (AWS, Azure, DigitalOcean).</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Terminal Mockup -->
                            <div class="bg-[#0d1117] rounded-xl border border-white/5 shadow-2xl overflow-hidden font-mono text-xs ">
                                <div class="bg-surface/80 px-4 py-3 flex items-center gap-2 border-b border-white/5 backdrop-blur">
                                    <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                                    <span class="ml-2 text-muted tracking-wider">attacker@kali:~</span>
                                </div>
                                <div class="p-5 space-y-3 text-primary-text">
                                    <p class="text-emerald-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">dig</span> +short A target.com</p>
                                    <div class="pl-4">
                                        <p class="text-orange-300">104.21.5.15 <span class="text-muted italic ml-2">&lt;-- Cloudflare Proxy (Shielded)</span></p>
                                        <p class="text-orange-300">172.67.10.15 <span class="text-muted italic ml-2">&lt;-- Cloudflare Proxy (Shielded)</span></p>
                                    </div>
                                    <p class="text-emerald-400 flex items-center gap-2 font-semibold mt-4"><span>➜</span> <span class="text-blue-400">dig</span> +short A dev-api.target.com</p>
                                    <div class="pl-4">
                                        <p class="text-rose-400 font-bold bg-rose-500/10 px-1 inline-block">167.99.155.62</p>  <span class="text-red-500 ml-2">&lt;-- WARNING: DigitalOcean Origin IP exposed!</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-cyan-300">Canonical Names (CNAME) & Hijacking</h3>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">A Canonical Name (`CNAME`) record creates an alias. Instead of pointing to an IP address, it points to another domain name. If mismanaged, this leads directly to Subdomain Takeovers.</p>
                        
                        <!-- Visual Chain -->
                        <div class="bg-black/50 border border-white/5 rounded-xl p-6 relative overflow-hidden shadow-inner h-full flex flex-col items-center justify-center py-10 mb-8">
                            <div class="flex items-center gap-4 w-full max-w-2xl">
                                <div class="bg-cyan-950/40 border-2 border-cyan-500 p-4 rounded-xl flex-1 text-center shadow-[0_0_20px_rgba(6,182,212,0.3)]">
                                    <h4 class="text-cyan-300 font-bold font-mono text-sm">support.target.com</h4>
                                    <p class="text-xs text-muted mt-1">Target's Subdomain</p>
                                </div>
                                <div class="flex flex-col items-center justify-center shrink-0">
                                    <span class="text-[10px] text-accent font-bold font-mono uppercase tracking-widest mb-1">CNAME Maps To</span>
                                    <ArrowRight class="text-accent" size={24} />
                                </div>
                                <div class="bg-red-950/40 border-2 border-red-500 p-4 rounded-xl flex-1 text-center shadow-[0_0_20px_rgba(239,68,68,0.3)] border-dashed relative group">
                                    <h4 class="text-red-300 font-bold font-mono text-sm">target-help.zendesk.com</h4>
                                    <p class="text-xs text-red-500 mt-1 font-bold">UNREGISTERED (404)</p>
                                    
                                    <!-- Hacker Claim -->
                                    <div class="absolute -bottom-14 left-1/2 -translate-x-1/2 w-48 bg-red-600 text-primary-text text-xs p-2 rounded shadow-xl opacity-0 group-hover:opacity-100 transition-opacity font-bold uppercase tracking-widest border border-red-400">
                                        Attacker Claims the bucket!
                                    </div>
                                </div>
                            </div>
                            <div class="text-center mt-6">
                                <p class="text-sm text-primary-text font-medium">The Dangling Record Vulnerability</p>
                                <p class="text-xs text-muted mt-2 max-w-md">If the target cancels their Zendesk subscription but forgets to remove the CNAME record, an attacker can register `target-help.zendesk.com` and instantly control `support.target.com`.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Organizational Email Security</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Email spoofing relies on missing DNS validation records. Identifying a lack of SPF/DMARC policies allows attackers to send phishing emails that appear genuinely from your executive team.</p>

                        <div class="space-y-6 mb-8">
                            <!-- SPF -->
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex flex-col md:flex-row gap-5">
                                <div class="shrink-0">
                                    <div class="bg-emerald-500/10 p-3 rounded-xl border border-emerald-500/30">
                                        <ShieldCheck size={28} class="text-emerald-400"/>
                                    </div>
                                </div>
                                <div>
                                    <h4 class="text-base font-semibold text-emerald-300 mb-1">Sender Policy Framework (SPF)</h4>
                                    <p class="text-xs text-emerald-100/60 leading-relaxed mb-3">A TXT record that dictates the cryptographic addresses authorized to send emails on behalf of this domain.</p>
                                    
                                    <div class="bg-black/60 border border-white/10 p-3 rounded font-mono text-xs flex gap-2">
                                        <span class="text-muted">target.com TXT</span>
                                        <code class="text-primary-text"><span class="text-teal-400 font-bold">v=spf1</span> include:_spf.google.com <span class="text-red-400 font-bold">~all</span></code>
                                    </div>
                                    <p class="text-[10px] text-muted mt-2"><strong class="text-emerald-500">SecOps Review:</strong> `~all` (Soft Fail) is weak. Production servers must leverage `-all` (Hard Fail) to completely reject spoofed origins.</p>
                                </div>
                            </div>
                            
                            <!-- DMARC -->
                            <div class="bg-orange-950/10 border border-orange-500/20 rounded-xl p-5 flex flex-col md:flex-row gap-5">
                                <div class="shrink-0">
                                    <div class="bg-orange-500/10 p-3 rounded-xl border border-orange-500/30">
                                        <ShieldAlert size={28} class="text-orange-400"/>
                                    </div>
                                </div>
                                <div>
                                    <h4 class="text-base font-semibold text-orange-300 mb-1">Domain-based Message Authentication (DMARC)</h4>
                                    <p class="text-xs text-orange-100/60 leading-relaxed mb-3">Tells the receiving mail server precisely what to do if SPF or DKIM validation fails.</p>
                                    
                                    <div class="bg-black/60 border border-white/10 p-3 rounded font-mono text-xs flex gap-2">
                                        <span class="text-muted">_dmarc TXT</span>
                                        <code class="text-primary-text"><span class="text-teal-400 font-bold">v=DMARC1;</span> p=<span class="text-orange-400 font-bold">quarantine</span>; rua=mailto:admin@target.com</code>
                                    </div>
                                    <p class="text-[10px] text-muted mt-2"><strong class="text-emerald-500">SecOps Review:</strong> Move policy (`p=`) from `none` -> `quarantine` -> `reject` automatically dropping spoofed external traffic.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-purple-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-purple-600 hover:bg-purple-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(168,85,247,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-purple-500/50 outline-none"
                        >
                            Continue <ChevronRight size={16} />
                        </button>
                    {:else}
                        <button 
                            onclick={close}
                            class="flex items-center gap-1.5 px-8 py-2.5 bg-cyan-500 hover:bg-cyan-400 text-cyan-950 text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(34,211,238,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-cyan-500/50 outline-none"
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
