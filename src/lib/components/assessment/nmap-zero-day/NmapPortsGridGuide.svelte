<script lang="ts">
    import { X, Search, Fingerprint, ShieldCheck, ServerCog, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Port Enumeration", icon: Search },
        { id: 1, label: "2. Service Fingerprinting", icon: Fingerprint },
        { id: 2, label: "3. Exposure Reduction", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-red-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(239,68,68,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-red-950/40 via-red-900/10 to-transparent p-6 border-b border-red-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-red-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-red-500/20 rounded-xl transition-all border border-subtle hover:border-red-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-red-500/20 to-red-600/10 flex items-center justify-center border border-red-500/30 shadow-inner">
                            <ServerCog size={28} class="text-red-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Open Port Scan Results</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Live service enumeration via Nmap TCP/UDP port scanning and banner grabbing.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-red-500/20 text-red-300 border border-red-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
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
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-red-300 mb-4">How Nmap Discovers Open Ports</h3>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    Nmap (Network Mapper) is the industry-standard tool for port scanning and host discovery. It probes target systems to determine which TCP and UDP ports are listening and accepting connections, revealing the network attack surface of a host.
                                </p>
                                <div class="bg-red-950/20 border border-red-500/10 rounded-xl p-4">
                                    <h4 class="text-red-400 font-medium mb-3 flex items-center gap-2"><Search size={16}/> Primary Scan Modes</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">SYN Stealth (-sS):</strong> Sends a TCP SYN packet and waits for a SYN-ACK. Never completes the handshake, making it faster and quieter than a full connect scan. Requires root/Administrator privileges.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Version Detection (-sV):</strong> After confirming a port is open, Nmap sends protocol-specific probes and reads the banner response to identify the exact service and version running.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">UDP Scan (-sU):</strong> Scans UDP ports by sending empty UDP datagrams and watching for ICMP unreachable responses. Significantly slower than TCP scans due to rate limiting.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Terminal Mockup -->
                            <div class="bg-[#0d1117] rounded-xl border border-red-500/10 shadow-2xl overflow-hidden font-mono text-xs">
                                <div class="bg-surface/80 px-4 py-2.5 flex items-center gap-2 border-b border-subtle">
                                    <div class="w-3 h-3 rounded-full bg-red-500/60"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500/60"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500/60"></div>
                                    <span class="text-muted text-[10px] ml-2">terminal — nmap scan</span>
                                </div>
                                <div class="p-4 space-y-1 text-[11px] leading-relaxed custom-scrollbar overflow-y-auto max-h-48">
                                    <p><span class="text-green-400">$</span> <span class="text-white">sudo nmap -sS -sV -p 1-65535 target.com</span></p>
                                    <p class="text-muted mt-2">Starting Nmap 7.94 ( https://nmap.org )</p>
                                    <p class="text-muted">Nmap scan report for target.com (93.184.216.34)</p>
                                    <p class="text-muted">Host is up (0.021s latency).</p>
                                    <p class="text-muted mt-1">PORT     STATE  SERVICE    VERSION</p>
                                    <p><span class="text-emerald-400">22/tcp   open   ssh</span>       <span class="text-blue-300">OpenSSH 8.4p1</span></p>
                                    <p><span class="text-emerald-400">80/tcp   open   http</span>      <span class="text-blue-300">nginx 1.21.3</span></p>
                                    <p><span class="text-emerald-400">443/tcp  open   https</span>     <span class="text-blue-300">nginx 1.21.3</span></p>
                                    <p><span class="text-yellow-400">8080/tcp filtered http-proxy</span></p>
                                    <p class="text-muted mt-1">Nmap done: 1 IP address (1 host up)</p>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4 mb-6">
                            <h4 class="text-primary-text font-medium mb-3">Port States Explained</h4>
                            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                                <div class="bg-emerald-950/20 border border-emerald-500/20 rounded-lg p-3">
                                    <span class="text-emerald-400 font-bold text-xs uppercase tracking-wider">Open</span>
                                    <p class="text-muted text-xs mt-1 leading-relaxed">A service is actively accepting connections on this port. Represents a potential attack surface that needs analysis.</p>
                                </div>
                                <div class="bg-yellow-950/20 border border-yellow-500/20 rounded-lg p-3">
                                    <span class="text-yellow-400 font-bold text-xs uppercase tracking-wider">Filtered</span>
                                    <p class="text-muted text-xs mt-1 leading-relaxed">A firewall, filter, or network obstacle is blocking probes. Nmap cannot determine if the port is open or closed. Typically means a firewall rule is in place.</p>
                                </div>
                                <div class="bg-surface border border-subtle rounded-lg p-3">
                                    <span class="text-muted font-bold text-xs uppercase tracking-wider">Closed</span>
                                    <p class="text-muted text-xs mt-1 leading-relaxed">No application is listening, but the port is reachable. The OS responds with a TCP RST packet. Less interesting but still useful for OS fingerprinting.</p>
                                </div>
                            </div>
                        </div>

                        <div class="bg-blue-950/20 border border-blue-500/20 rounded-xl p-4">
                            <h4 class="text-blue-400 font-medium mb-2">TCP vs UDP Scanning</h4>
                            <p class="text-sm text-muted leading-relaxed">TCP scans are reliable because TCP is connection-oriented — a SYN-ACK confirms the port is open. UDP is stateless, so Nmap must infer state from whether an ICMP "port unreachable" response arrives. UDP scans are slower (often 10-20x) and less reliable, but critical for discovering DNS (53), SNMP (161), DHCP (67/68), and NTP (123) services that run exclusively over UDP.</p>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-red-300 mb-4">Service Fingerprinting & CPE Strings</h3>

                        <p class="text-sm text-muted mb-6 max-w-3xl">Once an open port is confirmed, Nmap's version detection engine (-sV) probes the service to identify exactly what software is running. This data is the critical link to vulnerability databases.</p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <div class="bg-red-950/20 border border-red-500/10 rounded-xl p-4">
                                    <h4 class="text-red-400 font-medium mb-3">CPE Identifier Format</h4>
                                    <p class="text-sm text-muted mb-3 leading-relaxed">CPE (Common Platform Enumeration) is a standardized naming scheme for software, systems, and packages. It provides a machine-readable identifier used to cross-reference against CVE databases.</p>
                                    <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs mb-3">
                                        <p class="text-muted mb-1">// Format: cpe:/type:vendor:product:version</p>
                                        <p class="text-emerald-300">cpe:/a:apache:http_server:2.4.41</p>
                                        <p class="text-muted text-[10px] mt-2">
                                            <span class="text-blue-300">a</span> = application type<br/>
                                            <span class="text-blue-300">apache</span> = vendor<br/>
                                            <span class="text-blue-300">http_server</span> = product name<br/>
                                            <span class="text-blue-300">2.4.41</span> = specific version
                                        </p>
                                    </div>
                                    <ul class="space-y-2 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span>Type <strong class="text-primary-text">a</strong> = application, <strong class="text-primary-text">o</strong> = OS, <strong class="text-primary-text">h</strong> = hardware</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span>The NVD uses CPE strings to match software versions to CVE records automatically</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span>When version is unknown, the CPE appears as <code class="text-indigo-300 bg-indigo-500/10 px-1 rounded">cpe:/a:vendor:product</code> without a version suffix</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Service Detection Output Mockup -->
                            <div class="bg-[#0d1117] rounded-xl border border-red-500/10 shadow-2xl overflow-hidden font-mono text-xs">
                                <div class="bg-surface/80 px-4 py-2.5 flex items-center justify-between border-b border-subtle">
                                    <div class="flex items-center gap-2">
                                        <Fingerprint size={14} class="text-red-400" />
                                        <span class="text-muted text-[10px]">Nmap Service Detection Output</span>
                                    </div>
                                </div>
                                <div class="p-4 space-y-3 text-[11px] leading-relaxed custom-scrollbar overflow-y-auto max-h-64">
                                    <div class="border border-emerald-500/20 rounded-lg p-2.5 bg-emerald-950/10">
                                        <p class="text-muted">PORT: <span class="text-emerald-400">443/tcp</span></p>
                                        <p class="text-muted">STATE: <span class="text-emerald-400">open</span></p>
                                        <p class="text-muted">SERVICE: <span class="text-white">https</span></p>
                                        <p class="text-muted">PRODUCT: <span class="text-white">Apache httpd</span></p>
                                        <p class="text-muted">VERSION: <span class="text-yellow-300">2.4.41</span></p>
                                        <p class="text-muted">EXTRAINFO: (Ubuntu)</p>
                                        <p class="text-muted">CPE: <span class="text-indigo-300">cpe:/a:apache:http_server:2.4.41</span></p>
                                    </div>
                                    <div class="border border-blue-500/20 rounded-lg p-2.5 bg-blue-950/10">
                                        <p class="text-muted">PORT: <span class="text-emerald-400">3306/tcp</span></p>
                                        <p class="text-muted">STATE: <span class="text-emerald-400">open</span></p>
                                        <p class="text-muted">SERVICE: <span class="text-white">mysql</span></p>
                                        <p class="text-muted">PRODUCT: <span class="text-white">MySQL</span></p>
                                        <p class="text-muted">VERSION: <span class="text-yellow-300">5.7.32</span></p>
                                        <p class="text-muted">CPE: <span class="text-indigo-300">cpe:/a:mysql:mysql:5.7.32</span></p>
                                        <p class="text-red-400 text-[10px] mt-1">⚠ MySQL exposed to internet — high risk</p>
                                    </div>
                                    <p class="text-muted text-[10px] italic">// These CPE strings are fed into NVD to fetch CVEs automatically</p>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4">
                            <h4 class="text-primary-text font-medium mb-3">How Banner Grabbing Works</h4>
                            <p class="text-sm text-muted leading-relaxed mb-3">Nmap's version detection works by sending protocol-specific probe strings to open ports and reading the response banner. For HTTP, it sends a minimal GET request and parses the <code class="text-blue-300 bg-blue-500/10 px-1 rounded">Server:</code> header. For SSH, the service immediately announces its version string (<code class="text-blue-300 bg-blue-500/10 px-1 rounded">SSH-2.0-OpenSSH_8.4</code>). For databases, Nmap uses protocol handshake packets. The response is matched against a database of ~11,000 version detection probes in <code class="text-blue-300 bg-blue-500/10 px-1 rounded">nmap-service-probes</code>.</p>
                            <div class="bg-amber-950/20 border border-amber-500/20 rounded-lg p-3">
                                <p class="text-amber-400 text-xs font-medium">Why version disclosure matters</p>
                                <p class="text-muted text-xs mt-1 leading-relaxed">Once a version is identified (e.g., Apache 2.4.41), it can be cross-referenced against the NVD to find known CVEs. An attacker with a CPE string can instantly find all public exploits for that version. This table shows you your own exposure before an attacker does.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Minimizing Port Exposure</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Every open port is a potential entry point. The principle of least exposure dictates that only ports strictly necessary for business function should be reachable from the internet. Everything else should be firewalled, closed, or tunneled.</p>

                        <!-- Firewall Rules Code Block -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">ufw / iptables</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar text-xs leading-relaxed">
                                <pre class="text-primary-text">
<span class="text-muted"># Block all inbound by default</span>
<span class="text-yellow-300">sudo ufw default deny incoming</span>
<span class="text-yellow-300">sudo ufw default allow outgoing</span>

<span class="text-muted"># Allow only required services</span>
<span class="text-green-300">sudo ufw allow 443/tcp    # HTTPS</span>
<span class="text-green-300">sudo ufw allow 80/tcp     # HTTP (for redirect)</span>

<span class="text-muted"># SSH on non-standard port + restrict to known IPs</span>
<span class="text-green-300">sudo ufw allow from 10.0.0.0/8 to any port 2222</span>

<span class="text-muted"># Block database ports from public internet</span>
<span class="text-red-400">sudo ufw deny 3306/tcp    # MySQL — internal only</span>
<span class="text-red-400">sudo ufw deny 5432/tcp    # PostgreSQL — internal only</span>
<span class="text-red-400">sudo ufw deny 6379/tcp    # Redis — never public!</span>

<span class="text-yellow-300">sudo ufw enable && sudo ufw status verbose</span></pre>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <ShieldCheck size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Hide Version Banners</h4>
                                    <p class="text-xs text-muted leading-relaxed">Suppress server version strings. In Nginx: <code class="text-emerald-300 bg-emerald-500/10 px-1 rounded">server_tokens off</code>. In Apache: <code class="text-emerald-300 bg-emerald-500/10 px-1 rounded">ServerTokens Prod</code>. This does not fix vulnerabilities but makes them harder to discover via automated scanning.</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                                    <ServerCog size={20} class="text-blue-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Move SSH to Non-Standard Port</h4>
                                    <p class="text-xs text-muted leading-relaxed">Changing SSH from port 22 to a high-numbered port (e.g., 2222 or 49152+) eliminates the vast majority of automated credential-stuffing bots that exclusively target port 22. Combine with key-only auth and fail2ban.</p>
                                </div>
                            </div>
                            <div class="bg-purple-950/10 border border-purple-500/20 rounded-xl p-5 flex gap-4 hover:border-purple-500/40 transition-colors">
                                <div class="bg-purple-500/10 p-2 rounded-lg h-fit">
                                    <Search size={20} class="text-purple-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-purple-300 mb-1.5">Port Knocking</h4>
                                    <p class="text-xs text-muted leading-relaxed">Port knocking hides admin services entirely. A secret sequence of "knocks" (connection attempts on specific ports in order) triggers the firewall to temporarily open the real port. Tools: <code class="text-purple-300 bg-purple-500/10 px-1 rounded">knockd</code>.</p>
                                </div>
                            </div>
                            <div class="bg-amber-950/10 border border-amber-500/20 rounded-xl p-5 flex gap-4 hover:border-amber-500/40 transition-colors">
                                <div class="bg-amber-500/10 p-2 rounded-lg h-fit">
                                    <ShieldCheck size={20} class="text-amber-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-amber-300 mb-1.5">VPN Tunnel for Admin Ports</h4>
                                    <p class="text-xs text-muted leading-relaxed">Admin interfaces (database, admin panels, monitoring) should never be internet-facing. Use WireGuard or OpenVPN so management ports are only reachable after VPN authentication. This eliminates the attack surface entirely.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-red-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
    .custom-scrollbar::-webkit-scrollbar { width: 8px; height: 8px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background-color: rgba(255,255,255,0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background-color: rgba(255,255,255,0.2); }
</style>
