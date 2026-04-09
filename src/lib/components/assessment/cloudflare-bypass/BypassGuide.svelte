<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Network, Search, ShieldCheck, Database, Globe, CloudFog, CloudRain, Crosshair, ArrowRight, ChevronRight, ChevronLeft, Server } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Proxy Architecture", icon: CloudFog },
        { id: 1, label: "2. Recon & Leakage", icon: Search },
        { id: 2, label: "3. Fortification", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-violet-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(139,92,246,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-violet-950/40 via-purple-900/10 to-transparent p-6 border-b border-violet-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-violet-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-zinc-400 hover:text-white bg-white/5 hover:bg-violet-500/20 rounded-xl transition-all border border-white/5 hover:border-violet-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-violet-500/20 to-purple-600/10 flex items-center justify-center border border-violet-500/30 shadow-inner">
                            <CloudRain size={28} class="text-violet-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-white">{m.sec_cfbypass_title ? m.sec_cfbypass_title() : 'WAF & CDN Bypass Mechanics'}</h2>
                            <p class="text-sm text-zinc-400 mt-1 max-w-md">{m.sec_cfbypass_guide_mechanics ? m.sec_cfbypass_guide_mechanics() : 'Discovering origin servers hidden behind reverse proxies like Cloudflare.'}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-violet-500/20 text-violet-300 border border-violet-500/20 shadow-sm' : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-violet-300 mb-4">The Proxy Shield</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-zinc-300 text-sm leading-relaxed">
                                    Web Application Firewalls (WAFs) like Cloudflare, Fastly, or Akamai operate as reverse proxies. By updating DNS to point to the WAF edge network, the true IP address of the origin server is hidden from public view.
                                </p>
                                <div class="bg-violet-950/20 border border-violet-500/10 rounded-xl p-4">
                                    <h4 class="text-violet-400 font-medium mb-3 flex items-center gap-2"><Network size={16}/> The Target Objective</h4>
                                    <ul class="space-y-3 text-sm text-zinc-400">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-violet-500 mt-1 shrink-0"/> <span><strong>Protection:</strong> WAFs block SQLi, XSS, and DDoS at the edge. They never reach the server.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-violet-500 mt-1 shrink-0"/> <span><strong>The Vulnerability:</strong> If an attacker discovers the real origin IP, they can route traffic directly to the server, bypassing *all* WAF rules completely.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Visual Diagram -->
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner h-full">
                                <div class="flex items-center justify-between w-full mb-6 relative py-4">
                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-12 h-12 rounded-full bg-zinc-800 border-2 border-zinc-600 flex items-center justify-center text-zinc-300 mb-2 shadow-lg">Attacker</div>
                                    </div>
                                    
                                    <div class="absolute top-[35%] left-10 w-[calc(100%-5rem)] h-0 border-t-2 border-dashed border-red-500/30">
                                        <div class="w-1/2 h-[2px] bg-red-500 shadow-[0_0_10px_#ef4444] animate-[pulse_2s_ease-in-out_infinite]"></div>
                                    </div>

                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-14 h-14 rounded-lg bg-orange-950 border-2 border-orange-500 flex flex-col items-center justify-center text-orange-300 mb-2 shadow-[0_0_15px_rgba(249,115,22,0.4)] relative">
                                            <CloudFog size={24} />
                                            <div class="absolute -top-2 -right-2 bg-red-500 text-white text-[10px] px-1 font-bold rounded">BLOCKED</div>
                                        </div>
                                        <span class="text-xs text-orange-400 font-bold">WAF (Edge)</span>
                                    </div>

                                    <div class="absolute top-[35%] right-10 w-1/4 h-0 border-t-2 border-dashed border-zinc-700"></div>

                                    <div class="flex flex-col items-center z-10 w-24">
                                        <div class="w-12 h-12 rounded-full bg-zinc-900 border-2 border-zinc-700 flex items-center justify-center mb-2"><Server size={18} class="text-zinc-400"/></div>
                                        <span class="text-xs text-zinc-500 w-full text-center">Hidden Origin<br>172.x.x.x</span>
                                    </div>
                                    
                                    <!-- Bypass Path -->
                                    <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 5;">
                                        <!-- Curved path from attacker overriding WAF to origin -->
                                        <path d="M 40,30 Q 150,-40 280,30" fill="none" class="stroke-violet-500" stroke-width="2" stroke-dasharray="6,4"></path>
                                        <!-- Animated dot along path -->
                                    </svg>
                                </div>
                                <div class="text-center text-xs text-violet-400 mt-6 bg-violet-500/10 px-4 py-2 rounded-full border border-violet-500/30 font-medium tracking-wide">Goal: Bypass the WAF and directly hit the Origin IP</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-violet-300">Reconnaissance Vectors</h3>
                        </div>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">Attackers assemble OSINT tools and scan engines like Shodan or Censys to scour the IPv4 space for breadcrumbs that reveal the true origin IP.</p>
                        
                        <!-- Dashboard Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-violet-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm shadow-[0_0_30px_rgba(0,0,0,0.5)] flex flex-col">
                            <div class="bg-zinc-900/80 px-4 py-3 flex items-center justify-between border-b border-white/5 backdrop-blur">
                                <div class="flex items-center gap-2">
                                    <Search size={14} class="text-violet-400" />
                                    <span class="text-zinc-400 text-xs">Censys Search Query</span>
                                </div>
                                <div class="bg-black/50 border border-white/5 rounded px-2 py-1 text-zinc-500 text-[10px]">
                                    services.tls.certificates.leaf.names: "target.com"
                                </div>
                            </div>
                            
                            <div class="p-5 flex-1 space-y-4 text-zinc-300 h-64 overflow-y-auto custom-scrollbar">
                                <p class="text-zinc-500 italic mb-2">// WAFs hide DNS, but if the Origin Server exposes HTTPS publicly on port 443 with the target.com SSL certificate, internet-wide scanners will catalog it.</p>
                                
                                <div class="bg-black/40 border border-white/5 rounded-lg p-3 hover:border-violet-500/30 transition-colors">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="flex items-center gap-2">
                                            <Globe size={14} class="text-emerald-400" />
                                            <span class="text-zinc-200 font-bold">142.250.xxx.xxx</span>
                                        </div>
                                        <span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400">AWS / EC2</span>
                                    </div>
                                    <div class="pl-5 text-xs space-y-1">
                                        <p><span class="text-zinc-500">Port:</span> <span class="text-blue-300">443 / HTTPS</span></p>
                                        <p><span class="text-zinc-500">Cert:</span> <span class="text-emerald-300 font-bold">CN=target.com</span> (Match!)</p>
                                        <p class="text-zinc-500 text-[10px] mt-2">Status: Found unproxied origin binding target SSL cert.</p>
                                    </div>
                                </div>

                                <div class="bg-blue-900/10 border border-blue-500/20 rounded-lg p-3">
                                    <p class="text-blue-400 font-bold mb-1">Other Leakage Channels:</p>
                                    <ul class="text-zinc-400 text-xs space-y-1 list-disc list-inside ml-2">
                                        <li><strong>Historical DNS:</strong> SecurityTrails showing previous A records before Cloudflare.</li>
                                        <li><strong>Misconfigured Subdomains:</strong> `mail.target.com` pointing to the same server but unproxied (grey-clouded).</li>
                                        <li><strong>Email Headers:</strong> Outbound emails from the app including `Received: from x.x.x.x`.</li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Fortification Steps</h3>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">To securely proxy traffic, the origin must be configured to exclusively accept inbound connections from strictly the CDN edge nodes, rejecting all direct public access.</p>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">AWS Security Group</div>
                            <div class="p-6 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-zinc-300 leading-relaxed">
<span class="text-zinc-500 italic"># ❌ BAD: Allowing all inbound HTTPS</span>
<span class="text-teal-400">Ingress</span>: 
  <span class="text-orange-300">Port</span>: 443
  <span class="text-orange-300">Source</span>: <span class="text-red-400">0.0.0.0/0</span> <span class="text-zinc-500 italic"># Bypasses WAF!</span>

<span class="text-zinc-500 italic"># ✅ SECURE: Whitelisting only Cloudflare IP ranges</span>
<span class="text-teal-400">Ingress</span>:
  <span class="text-orange-300">Port</span>: 443
  <span class="text-orange-300">Source_CIDR</span>: 
    - <span class="text-green-300">173.245.48.0/20</span>
    - <span class="text-green-300">103.21.244.0/22</span>
    - <span class="text-green-300">... (auto-updated via lambda/cron)</span></pre>
                            </div>
                        </div>

                        <!-- Mitigation Checklist -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <ShieldCheck size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Authenticated Origin Pulls</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Enable mTLS. Cloudflare provides a client certificate; the origin Nginx/Apache verifies `ssl_client_verify ON` so only CF can connect.</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                                    <Globe size={20} class="text-blue-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Proxy ALL Subdomains</h4>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">Never leave subdomains like `cpanel` or `mail` on the same physical server. An attacker looking up `mail.domain.com` will find the shared origin IP.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-violet-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-violet-600 hover:bg-violet-500 text-white text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(139,92,246,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-violet-500/50 outline-none"
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
