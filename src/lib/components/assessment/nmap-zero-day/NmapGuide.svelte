<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Search, ShieldCheck, AlertTriangle, Fingerprint, Activity, Network, Terminal, ShieldAlert, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Network Recon", icon: Activity },
        { id: 1, label: "2. CVE Mapping", icon: Terminal },
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
            class="bg-[#0A0C10] border border-red-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(239,68,68,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-red-950/40 via-rose-900/10 to-transparent p-6 border-b border-red-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-red-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-white/5 hover:bg-red-500/20 rounded-xl transition-all border border-white/5 hover:border-red-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-red-500/20 to-rose-600/10 flex items-center justify-center border border-red-500/30 shadow-inner">
                            <Fingerprint size={28} class="text-red-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.sec_nmap_title ? m.sec_nmap_title() : 'Port Scanning & Zero-Day Correlation'}</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">{m.sec_nmap_guide_mechanics ? m.sec_nmap_guide_mechanics() : 'Probing networks to identify services and mapping them to known CVEs.'}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-red-500/20 text-red-300 border border-red-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-red-300 mb-4">Stealth Scanning Basics</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    Before exploiting a target, attackers must map its exposed attack surface. Port scanners send crafted packets to determine what services are listening and filter out firewalled ports.
                                </p>
                                <div class="bg-red-950/20 border border-red-500/10 rounded-xl p-4">
                                    <h4 class="text-red-400 font-medium mb-3 flex items-center gap-2"><Activity size={16}/> SYN Scanning (Stealth)</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/> <span><strong>The Handshake:</strong> Attacker sends SYN. Target replies SYN/ACK (port open).</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/> <span><strong>The Drop:</strong> Instead of sending the final ACK, attacker sends RST (Reset). The connection drops without being fully logged by basic app-level software.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/> <span><strong>Pacing:</strong> Slow, distributed scans bypass rudimentary rate limiters.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Visual Diagram -->
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner h-full">
                                <div class="flex items-center justify-between w-full mb-6 relative py-4">
                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-12 h-12 rounded-full bg-surface border-2 border-zinc-600 flex items-center justify-center shadow-lg">
                                            <Search size={20} class="text-muted"/>
                                        </div>
                                        <span class="text-xs text-muted mt-2">Scanner</span>
                                    </div>
                                    
                                    <div class="flex-1 px-4 relative h-16">
                                        <!-- SYN -->
                                        <div class="absolute top-2 w-full flex items-center">
                                            <div class="h-[2px] bg-red-500/50 w-full relative">
                                                <div class="absolute top-0 right-0 w-3 h-3 border-t-2 border-r-2 border-red-500 transform rotate-45 -mt-[1px] -mr-[1px]"></div>
                                            </div>
                                            <span class="absolute -top-4 left-1/2 -translate-x-1/2 text-[10px] text-red-400 font-bold tracking-widest">SYN</span>
                                        </div>
                                        
                                        <!-- SYN/ACK -->
                                        <div class="absolute top-8 w-full flex items-center">
                                            <div class="h-[2px] bg-emerald-500/50 w-full relative">
                                                <div class="absolute top-0 left-0 w-3 h-3 border-b-2 border-l-2 border-emerald-500 transform rotate-45 -mt-[1px] -ml-[1px]"></div>
                                            </div>
                                            <span class="absolute -top-4 left-1/2 -translate-x-1/2 text-[10px] text-emerald-400 font-bold tracking-widest">SYN / ACK</span>
                                        </div>

                                        <!-- RST -->
                                        <div class="absolute top-14 w-full flex items-center">
                                            <div class="h-[2px] bg-orange-500/50 w-1/2 relative">
                                                <div class="absolute top-0 right-0 w-3 h-3 border-t-2 border-r-2 border-orange-500 transform rotate-45 -mt-[1px] -mr-[1px]"></div>
                                            </div>
                                            <span class="absolute -top-4 left-1/4 -translate-x-1/2 text-[10px] text-orange-400 font-bold tracking-widest">RST</span>
                                        </div>
                                    </div>

                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-14 h-14 rounded-lg bg-surface border-2 border-zinc-700 flex flex-col items-center justify-center text-primary-text shadow-xl">
                                            <Network size={20} class="text-emerald-400"/>
                                            <span class="text-[10px] mt-1 font-bold">Port 22</span>
                                        </div>
                                        <span class="text-xs text-emerald-500 mt-2 font-medium">OPEN</span>
                                    </div>
                                </div>
                                <div class="text-center text-xs text-red-400 mt-6 bg-red-500/10 px-4 py-2 rounded-full border border-red-500/30 font-medium tracking-wide">TCP half-open connection prevents application logging</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-red-300">Service Profiling & Exploitation</h3>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Once open ports are found, scanners interact with the services using hardcoded probes to extract precise software versions, which are then cross-referenced against vulnerability databases (NVD).</p>
                        
                        <!-- Terminal Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-red-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm shadow-[0_0_30px_rgba(0,0,0,0.5)]">
                            <div class="bg-surface/80 px-4 py-3 flex items-center gap-2 border-b border-white/5 backdrop-blur">
                                <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                                <span class="ml-2 text-muted text-xs tracking-wider">attacker@kali:~/recon</span>
                            </div>
                            <div class="p-5 space-y-3 text-primary-text h-72 overflow-y-auto custom-scrollbar">
                                <p class="text-emerald-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">nmap</span> -sV --script vuln 10.0.0.45</p>
                                <p class="text-muted">Starting Nmap 7.94 ( https://nmap.org )</p>
                                <div class="pl-4 mt-2">
                                    <p class="text-muted font-bold">PORT     STATE  SERVICE VERSION</p>
                                    <p class="text-primary-text">22/tcp   <span class="text-emerald-400">open</span>   ssh     OpenSSH 8.2p1 Ubuntu 4ubuntu0.5</p>
                                    <p class="text-primary-text">80/tcp   <span class="text-emerald-400">open</span>   http    <span class="text-red-400 font-bold bg-red-500/10 px-1 border border-red-500/30 rounded">Apache httpd 2.4.49</span></p>
                                </div>
                                
                                <div class="mt-4 border-t border-red-500/20 pt-4 pl-4 relative">
                                    <div class="absolute left-0 top-4 w-1 h-full bg-red-500"></div>
                                    <p class="text-red-400 font-bold mb-1">| _http-vuln-cve2021-41773:</p>
                                    <p class="text-rose-300">|   VULNERABLE: <span class="font-bold text-primary-text">Path traversal and file disclosure vulnerability</span></p>
                                    <p class="text-muted">|     State: VULNERABLE</p>
                                    <p class="text-muted">|     IDs:  CVE:CVE-2021-41773</p>
                                    <p class="text-muted">|     Risk factor: <span class="text-red-500 font-bold">High  CVSS: 7.5</span></p>
                                    <p class="text-muted">|       A flaw was found in a change made to path normalization in Apache HTTP Server.</p>
                                </div>
                                
                                <p class="text-emerald-400 flex items-center gap-2 mt-4 font-semibold border-t border-white/5 pt-4"><span>➜</span> <span class="text-blue-400">curl</span> -v --path-as-is http://10.0.0.45/cgi-bin/.%2e/.%2e/.%2e/.%2e/etc/passwd</p>
                                <div class="pl-4 mt-2 text-orange-400/80">
                                    <p>root:x:0:0:root:/root:/bin/bash</p>
                                    <p>daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin</p>
                                    <p>www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Hardening Configurations</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Mitigating port scanning and 1-day exploit chaining requires severe attack surface reduction and intrusion prevention systems.</p>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">UFW Firewall (Linux)</div>
                            <div class="p-6 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted italic"># 1. Deny all incoming traffic by default</span>
<span class="text-emerald-400">sudo</span> ufw default deny incoming

<span class="text-muted italic"># 2. Open strictly necessary ports</span>
<span class="text-emerald-400">sudo</span> ufw allow 443/tcp

<span class="text-muted italic"># 3. Restrict Admin services (SSH/22) to VPN IPs only</span>
<span class="text-emerald-400">sudo</span> ufw allow from <span class="text-orange-300">10.8.0.0/24</span> to any port 22

<span class="text-muted italic"># 4. Enable ufw rate-limiting (blocks brute-forcers)</span>
<span class="text-emerald-400">sudo</span> ufw limit 443/tcp</pre>
                            </div>
                        </div>

                        <!-- Mitigation Checklist -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <ShieldAlert size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Intrusion Prevention Server (IPS)</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Deploy Suricata or Snort. They passively monitor packet flows for anomalous scanning patterns (like sequential SYN drops) and automatically ban the origin IP.</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                                    <Terminal size={20} class="text-blue-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Version Obfuscation</h4>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">Change default service banners. Configure Nginx (`server_tokens off;`) and SSH to not broadcast their exact minor version numbers to stall automated exploitation.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-red-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-red-600 hover:bg-red-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(239,68,68,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-red-500/50 outline-none"
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
