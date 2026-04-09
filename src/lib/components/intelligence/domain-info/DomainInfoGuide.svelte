<script lang="ts">
    import { X, Globe, Lock, Server, AlertTriangle, ArrowRight, Shield, ChevronRight, ChevronLeft, Fingerprint, History } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. WHOIS Intel", icon: Globe },
        { id: 1, label: "2. SSL & CT Logs", icon: Lock },
        { id: 2, label: "3. Port Matrix", icon: Server }
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
        class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div 
            class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30 shadow-inner">
                            <Globe size={28} class="text-accent" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Domain Intelligence</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Infrastructure Profiling & WHOIS Reconnaissance</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">WHOIS Database Forensics</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    Domain Intelligence is the first step in any penetration test. By querying WHOIS databases, analysts extract the ownership, registration dates, and infrastructure blueprints.
                                </p>
                                <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                                    <h4 class="text-accent font-medium mb-3 flex items-center gap-2"><Fingerprint size={16}/> Essential Indicators</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/> <span><strong>Burner Domains:</strong> Domains registered less than 30 days ago are highly suspicious (often phishing or C2 infrastructure).</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/> <span><strong>Registrar Choice:</strong> Attackers favor specific bulletproof registrars that ignore DMCA/Abuse reports.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/> <span><strong>Name Servers:</strong> Revealing NS records immediately maps the hosting/CDN provider (e.g., Cloudflare, AWS Route53).</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Terminal Mockup -->
                            <div class="bg-[#0d1117] border border-subtle rounded-xl p-5 shadow-inner h-full font-mono text-xs flex flex-col justify-center">
                                <p class="text-emerald-400 flex items-center gap-2 mb-2 font-semibold"><span>➜</span> <span class="text-blue-400">whois</span> target.com</p>
                                <div class="text-muted space-y-1 bg-glass p-3 rounded-lg border border-subtle">
                                    <p>Domain Name: TARGET.COM</p>
                                    <p>Registry Domain ID: 123456789_DOMAIN_COM-VRSN</p>
                                    <p>Registrar WHOIS Server: whois.godaddy.com</p>
                                    <p>Registrar: GoDaddy.com, LLC</p>
                                    <p class="text-red-400 font-bold bg-red-500/10 px-1 inline-block mt-1">Creation Date: 2024-10-15T12:00:00Z</p> <span class="text-red-500/70 ml-2">&lt;-- 12 Days Old!</span>
                                    <p>Updated Date: 2024-10-15T12:00:00Z</p>
                                    <p>Registrant Name: REDACTED FOR PRIVACY</p>
                                    <p class="text-orange-400 mt-2">Name Server: NS1.AWS.COM</p>
                                    <p class="text-orange-400">Name Server: NS2.AWS.COM</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-cyan-300">SSL & Certificate Transparency</h3>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Certificate Transparency (CT) logs passively enumerate all subdomains ever issued a certificate. This allows attackers to discover hidden internal infrastructure without sending a single active packet to your network.</p>
                        
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
                            <div class="bg-sunken border border-base p-5 rounded-xl border-t-2 border-t-emerald-500 shadow-lg">
                                <div class="flex items-center gap-2 mb-3">
                                    <Lock class="size-5 text-emerald-400" />
                                    <h3 class="font-bold text-primary-text">SSL Vulnerabilities</h3>
                                </div>
                                <p class="text-sm text-muted leading-relaxed">Expired certificates break browser trust, opening avenues for Man-in-the-Middle (MitM) attacks. Supporting outdated suites (TLS 1.0/1.1) exposes traffic to known cryptographic downgrades.</p>
                            </div>
                            
                            <div class="bg-sunken border border-base p-5 rounded-xl border-t-2 border-t-orange-500 shadow-lg">
                                <div class="flex items-center gap-2 mb-3">
                                    <History class="size-5 text-orange-400" />
                                    <h3 class="font-bold text-primary-text">CT Log Leaks</h3>
                                </div>
                                <p class="text-sm text-muted leading-relaxed">Whenever Let's Encrypt or DigiCert issues a wildcard or specific cert (e.g., `dev-db-01.target.com`), it's permanently logged. Tools like `crt.sh` query this public ledger instantly.</p>
                            </div>
                        </div>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-cyan-500/30 overflow-hidden font-mono text-sm shadow-[0_0_20px_rgba(6,182,212,0.1)]">
                            <div class="absolute top-0 right-0 bg-cyan-500/10 text-accent px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-cyan-500/30">crt.sh Automation</div>
                            <div class="p-6 overflow-x-auto pt-10 text-muted custom-scrollbar">
                                <p class="text-emerald-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">curl</span> -s "https://crt.sh/?q=%25.target.com&output=json" | jq -r '.[].name_value' | sort -u</p>
                                <div class="mt-2 text-rose-300">
                                    <p>api.target.com</p>
                                    <p>dev-staging-auth.target.com <span class="text-muted italic">&lt;-- Hidden environment found!</span></p>
                                    <p>vpn.target.com</p>
                                    <p>wiki.internal.target.com</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-rose-400 mb-4">Port Security Matrix</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Open ports are direct pathways to running services. Exposing administrative protocols directly to the internet guarantees continuous brute-force attacks from global botnets.</p>

                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
                            <div class="bg-glass border border-emerald-500/30 p-4 rounded-xl text-center relative overflow-hidden group">
                                <div class="absolute inset-x-0 bottom-0 h-1 bg-emerald-500"></div>
                                <span class="font-mono text-emerald-400 text-lg font-bold">80/443</span>
                                <p class="text-xs text-muted mt-2 font-medium">HTTP(S)</p>
                                <p class="text-[10px] text-muted mt-1">Expected Public</p>
                            </div>
                            <div class="bg-glass border border-rose-500/50 p-4 rounded-xl text-center relative overflow-hidden">
                                <div class="absolute inset-x-0 bottom-0 h-1 bg-rose-500"></div>
                                <span class="font-mono text-rose-400 text-lg font-bold">22</span>
                                <p class="text-xs text-muted mt-2 font-medium">SSH</p>
                                <p class="text-[10px] text-rose-500 mt-1 font-bold">High Risk</p>
                            </div>
                            <div class="bg-glass border border-orange-500/50 p-4 rounded-xl text-center relative overflow-hidden">
                                <div class="absolute inset-x-0 bottom-0 h-1 bg-orange-500"></div>
                                <span class="font-mono text-orange-400 text-lg font-bold">3389</span>
                                <p class="text-xs text-muted mt-2 font-medium">RDP</p>
                                <p class="text-[10px] text-orange-500 mt-1 font-bold">Ransomware Vector</p>
                            </div>
                            <div class="bg-glass border border-rose-500/50 p-4 rounded-xl text-center relative overflow-hidden">
                                <div class="absolute inset-x-0 bottom-0 h-1 bg-rose-500"></div>
                                <span class="font-mono text-rose-400 text-lg font-bold">3306</span>
                                <p class="text-xs text-muted mt-2 font-medium">MySQL</p>
                                <p class="text-[10px] text-rose-500 mt-1 font-bold">Data Exfil Risk</p>
                            </div>
                        </div>

                        <!-- Mitigation Checklist -->
                        <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex flex-col md:flex-row gap-4 hover:border-emerald-500/40 transition-colors">
                            <div class="bg-emerald-500/10 p-3 rounded-xl h-fit">
                                <Shield size={24} class="text-emerald-400 shrink-0"/>
                            </div>
                            <div>
                                <h4 class="text-base font-semibold text-emerald-300 mb-2">Defensive Engineering</h4>
                                <ul class="space-y-2 text-sm text-emerald-200/80">
                                    <li class="flex gap-2"><ArrowRight size={16} class="text-emerald-500 shrink-0 mt-0.5" /> <strong>WHOIS Privacy:</strong> Mask registrar details to prevent OSINT spear-phishing tailored to network founders.</li>
                                    <li class="flex gap-2"><ArrowRight size={16} class="text-emerald-500 shrink-0 mt-0.5" /> <strong>Strict SSL/TLS:</strong> Deny TLS 1.0/1.1 entirely. Enforce modern CIPHER blocks at the NGNIX/Cloudflare edge.</li>
                                    <li class="flex gap-2"><ArrowRight size={16} class="text-emerald-500 shrink-0 mt-0.5" /> <strong>Zero Trust Ports:</strong> Place all admin ports (22, 3389) behind a VPN or identity-aware proxy (like Tailscale/Cloudflare Access).</li>
                                </ul>
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
