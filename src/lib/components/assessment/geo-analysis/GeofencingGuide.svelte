<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Globe, MapPin, Network, Code2, ArrowRight, ChevronRight, ChevronLeft, ShieldAlert, Navigation, Database } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Fundamentals", icon: Globe },
        { id: 1, label: "2. Bypass Attacks", icon: Network },
        { id: 2, label: "3. Mitigation", icon: ShieldAlert }
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
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-white/5 hover:bg-cyan-500/20 rounded-xl transition-all border border-white/5 hover:border-cyan-500/30 font-medium"
                    aria-label="Close geo guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30 shadow-inner">
                            <MapPin size={28} class="text-accent" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.sec_geo_guide_title ? m.sec_geo_guide_title() : 'Geofencing & Geo-IP Analysis'}</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">{m.sec_geo_guide_mechanics ? m.sec_geo_guide_mechanics() : 'Understanding regional restrictions, WAF bypassing, and IP spoofing mechanics.'}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">How IP Geolocation Works</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    Geofencing involves restricting or allowing access to services based on the geographical location derived from an IP address. Security teams often block entire countries to mitigate high-volume credential stuffing and DDoS attacks.
                                </p>
                                <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                                    <h4 class="text-accent font-medium mb-3 flex items-center gap-2"><Globe size={16}/> The Mechanisms</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/> <span><strong>MaxMind/MaxCDN databases:</strong> Standard mapping of IP blocks to geographical coordinates and ASNs.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/> <span><strong>Cloud WAFs (Cloudflare/AWS):</strong> Native rules filtering requests before they reach the origin server based on <code>CF-IPCountry</code>.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/> <span><strong>Regulatory Compliance:</strong> Used for GDPR, embargoed nations (OFAC), or regional licensing.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Visual Diagram -->
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner h-full">
                                <div class="flex items-center justify-between w-full mb-6 relative py-4">
                                    <div class="flex flex-col items-center z-10 w-24">
                                        <div class="w-12 h-12 rounded-full bg-surface border-2 border-zinc-600 flex items-center justify-center text-primary-text mb-2 overflow-hidden relative">
                                            <div class="absolute bottom-1 right-1 w-3 h-3 bg-red-500 rounded-full"></div>
                                            <Globe size={24} class="text-muted" />
                                        </div>
                                        <span class="text-xs text-muted">Blocked Region</span>
                                    </div>
                                    
                                    <div class="absolute top-[40%] left-14 w-[calc(100%-7rem)] h-0 border-t-2 border-dashed border-red-500/30">
                                        <div class="w-1/2 h-[2px] bg-red-500 shadow-[0_0_10px_#ef4444] animate-[pulse_2s_ease-in-out_infinite]"></div>
                                    </div>

                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-14 h-14 rounded-lg bg-red-950 border-2 border-red-500 flex flex-col items-center justify-center text-red-300 mb-2 shadow-[0_0_15px_rgba(239,68,68,0.4)]">
                                            <ShieldAlert size={20} class="mb-1" />
                                        </div>
                                        <span class="text-xs text-red-400 font-bold">WAF Rule</span>
                                    </div>

                                    <div class="absolute top-[40%] right-10 w-1/4 h-0 border-t-2 border-dashed border-zinc-700"></div>

                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-12 h-12 rounded-full bg-surface border-2 border-zinc-700 flex items-center justify-center mb-2"><Database size={18} class="text-zinc-600"/></div>
                                        <span class="text-xs text-muted">Origin</span>
                                    </div>
                                </div>
                                <div class="text-center text-xs text-accent mt-4 bg-cyan-500/10 px-4 py-2 rounded-full border border-cyan-500/30 font-medium tracking-wide">Threat vectors from restricted IP ranges dropped at the Edge</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-cyan-300">Bypassing Geofences</h3>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Attackers manipulate HTTP headers or utilize distributed infrastructure to spoof their perceived location and bypass WAF-enforced geographical blocks.</p>
                        
                        <!-- Terminal Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm ">
                            <div class="bg-surface/80 px-4 py-3 flex items-center gap-2 border-b border-white/5 backdrop-blur">
                                <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                                <span class="ml-2 text-muted text-xs tracking-wider">attacker@kali:~/recon</span>
                            </div>
                            <div class="p-5 space-y-3 text-primary-text h-72 overflow-y-auto custom-scrollbar">
                                <div class="opacity-70 mb-4">
                                    <p class="text-red-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">curl</span> -I https://financial-portal.com</p>
                                    <p class="text-muted">HTTP/2 403 Forbidden</p>
                                    <p class="text-muted">cf-ray: 8...-AMS</p>
                                    <p class="text-red-400/80 text-xs mt-1"># Request originating from a blocked country (e.g. RU/CN/IR) gets dropped by WAF</p>
                                </div>

                                <div class="border-t border-white/5 pt-4">
                                    <p class="text-emerald-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">curl</span> -I -H "X-Forwarded-For: 8.8.8.8" https://financial-portal.com</p>
                                    <p class="text-muted mt-1">HTTP/2 200 OK</p>
                                    <p class="text-muted">content-type: text/html; charset=utf-8</p>
                                    <p class="text-accent/80 text-xs mt-2 border-l-2 border-cyan-500/50 pl-2"># If the origin server misconfigures IP validation, trusting the X-Forwarded-For header blindly, it registers the IP as 8.8.8.8 (USA).</p>
                                </div>
                                
                                <div class="mt-4 bg-blue-900/10 p-3 rounded-lg border border-blue-500/20">
                                    <p class="text-blue-300 font-bold mb-1 flex items-center gap-2"><Navigation size={14}/> Common Header Exploits</p>
                                    <p class="text-muted">- X-Forwarded-For: 12.34.56.78</p>
                                    <p class="text-muted">- X-Originating-IP: 12.34.56.78</p>
                                    <p class="text-muted">- True-Client-IP: 12.34.56.78</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Hardening Configurations</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Mitigating geofencing bypass involves dropping untrusted HTTP headers, explicitly trusting only known Edge endpoints, and mapping routing logic carefully.</p>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">Nginx Configuration</div>
                            <div class="p-6 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted italic"># ❌ VULNERABLE: Blindly trusting X-Forwarded-For from any source</span>
<span class="text-red-400 line-through">set_real_ip_from 0.0.0.0/0;</span>
<span class="text-red-400 line-through">real_ip_header X-Forwarded-For;</span>

<span class="text-muted italic"># ✅ SECURE: Only trust the IP address populated by Cloudflare</span>
<span class="text-purple-400">set_real_ip_from</span> <span class="text-green-300">173.245.48.0/20</span>;  <span class="text-muted italic"># Cloudflare IP Ranges</span>
<span class="text-purple-400">set_real_ip_from</span> <span class="text-green-300">103.21.244.0/22</span>;
<span class="text-purple-400">real_ip_header</span>   <span class="text-blue-400">CF-Connecting-IP</span>;

<span class="text-muted italic"># Restrict Origin Access to only allow Edge Network IPs</span>
<span class="text-purple-400">allow</span> <span class="text-green-300">173.245.48.0/20</span>;
<span class="text-purple-400">deny</span>  <span class="text-red-300">all</span>;
</pre>
                            </div>
                        </div>

                        <!-- Mitigation Checklist -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <Network size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Authenticated Origin Pulls</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Use mutual TLS (mTLS) to cryptographically verify that requests to your server originated exactly from your WAF/CDN provider.</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                                    <ShieldAlert size={20} class="text-blue-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Residential Proxies</h4>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">Geofencing naturally fails against Residential IPs (Tor/Luminati) localized in unblocked countries. Employ Behavioral Analysis tracking.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
