<script lang="ts">
    import { X, Globe, Search, ShieldCheck, Network, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Record Types", icon: Globe },
        { id: 1, label: "2. Recon Value", icon: Search },
        { id: 2, label: "3. DNS Security", icon: ShieldCheck }
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
                            <Network size={28} class="text-red-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">DNS Resolution Records</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">IPv4 and IPv6 address resolution mapped from target DNS A/AAAA records.</p>
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
                        <h3 class="text-lg font-semibold text-red-300 mb-4">A and AAAA Record Types</h3>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    DNS (Domain Name System) translates human-readable domain names into IP addresses that computers use to route traffic. The A and AAAA record types are the foundational address records that map a hostname to a specific IP address.
                                </p>
                                <div class="grid grid-cols-1 gap-3">
                                    <div class="bg-red-950/20 border border-red-500/10 rounded-xl p-4">
                                        <h4 class="text-red-400 font-medium mb-3 flex items-center gap-2"><Globe size={16}/> A Record (IPv4)</h4>
                                        <ul class="space-y-2 text-sm text-muted">
                                            <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span>Maps a hostname to a <strong class="text-primary-text">32-bit IPv4 address</strong> (e.g., <code class="text-cyan-300 bg-cyan-500/10 px-1 rounded">93.184.216.34</code>). The "A" stands for "Address".</span></li>
                                            <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span>IPv4 addresses are written in dot-decimal notation: four octets from 0–255. The total address space is ~4.3 billion addresses — now exhausted, driving IPv6 adoption.</span></li>
                                            <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span>Multiple A records for the same domain provide DNS-based load balancing — each lookup may return a different IP via round-robin rotation.</span></li>
                                        </ul>
                                    </div>
                                    <div class="bg-blue-950/20 border border-blue-500/10 rounded-xl p-4">
                                        <h4 class="text-blue-400 font-medium mb-3 flex items-center gap-2"><Globe size={16}/> AAAA Record (IPv6)</h4>
                                        <ul class="space-y-2 text-sm text-muted">
                                            <li class="flex items-start gap-2"><ArrowRight size={14} class="text-blue-500 mt-1 shrink-0"/><span>Maps a hostname to a <strong class="text-primary-text">128-bit IPv6 address</strong> (e.g., <code class="text-cyan-300 bg-cyan-500/10 px-1 rounded">2606:2800:220:1:248:1893:25c8:1946</code>). Four "A"s because it's four times the size.</span></li>
                                            <li class="flex items-start gap-2"><ArrowRight size={14} class="text-blue-500 mt-1 shrink-0"/><span>Written as eight groups of four hexadecimal digits. Consecutive groups of zeros can be collapsed with <code class="text-blue-300 bg-blue-500/10 px-1 rounded">::</code> notation.</span></li>
                                            <li class="flex items-start gap-2"><ArrowRight size={14} class="text-blue-500 mt-1 shrink-0"/><span>IPv6 provides 340 undecillion unique addresses, effectively eliminating exhaustion concerns. Modern networks should support dual-stack (both A and AAAA).</span></li>
                                        </ul>
                                    </div>
                                </div>
                            </div>

                            <!-- DNS Lookup Terminal Mockup -->
                            <div class="bg-[#0d1117] rounded-xl border border-red-500/10 shadow-2xl overflow-hidden font-mono text-xs">
                                <div class="bg-surface/80 px-4 py-2.5 flex items-center gap-2 border-b border-subtle">
                                    <div class="w-3 h-3 rounded-full bg-red-500/60"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500/60"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500/60"></div>
                                    <span class="text-muted text-[10px] ml-2">terminal — dig lookups</span>
                                </div>
                                <div class="p-4 space-y-3 text-[11px] leading-relaxed custom-scrollbar overflow-y-auto max-h-72">
                                    <p><span class="text-green-400">$</span> <span class="text-white">dig A target.com +short</span></p>
                                    <p class="text-cyan-300 pl-2">93.184.216.34</p>
                                    <p class="mt-2"><span class="text-green-400">$</span> <span class="text-white">dig AAAA target.com +short</span></p>
                                    <p class="text-cyan-300 pl-2">2606:2800:220:1:248:1893:25c8:1946</p>
                                    <div class="border-t border-subtle pt-3">
                                        <p><span class="text-green-400">$</span> <span class="text-white">dig target.com ANY</span></p>
                                        <p class="text-muted mt-1">; QUESTION SECTION:</p>
                                        <p class="text-muted">;target.com. IN ANY</p>
                                        <p class="text-muted mt-1">; ANSWER SECTION:</p>
                                        <p><span class="text-yellow-300">target.com.</span> <span class="text-muted">3600 IN A</span> <span class="text-cyan-300">93.184.216.34</span></p>
                                        <p><span class="text-yellow-300">target.com.</span> <span class="text-muted">3600 IN AAAA</span> <span class="text-cyan-300">2606:2800::1946</span></p>
                                        <p><span class="text-yellow-300">target.com.</span> <span class="text-muted">3600 IN MX 10</span> <span class="text-purple-300">mail.target.com.</span></p>
                                        <p class="text-muted mt-1">; TTL: 3600 seconds (1 hour)</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4">
                            <h4 class="text-primary-text font-medium mb-2">Recursive DNS Resolution</h4>
                            <p class="text-sm text-muted leading-relaxed mb-2">When a DNS lookup occurs, the resolver contacts a root nameserver (there are 13 root server clusters), which directs it to the Top-Level Domain (TLD) nameserver (e.g., <code class="text-blue-300 bg-blue-500/10 px-1 rounded">.com</code>), which directs it to the authoritative nameserver for the domain. The authoritative NS returns the final A/AAAA record.</p>
                            <div class="bg-amber-950/20 border border-amber-500/20 rounded-lg p-3 mt-3">
                                <p class="text-amber-400 text-xs font-medium">TTL and OSINT Timing</p>
                                <p class="text-muted text-xs mt-1 leading-relaxed">TTL (Time to Live) is the number of seconds a DNS record is cached before requiring re-lookup. A low TTL (e.g., 60 seconds) means the target can update records quickly — useful for monitoring infrastructure changes. A high TTL (86400 = 24 hours) means IP changes propagate slowly, giving OSINT tools time to observe historical IPs.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-red-300 mb-4">Reconnaissance Value of IP Addresses</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">A resolved IP address is the entry point for a cascade of further reconnaissance techniques. Knowing the actual network address of a target enables port scanning, service fingerprinting, and infrastructure intelligence gathering.</p>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
                            <div class="bg-red-950/20 border border-red-500/10 rounded-xl p-4">
                                <h4 class="text-red-400 font-medium mb-3">IP-Enabled Recon Techniques</h4>
                                <ul class="space-y-3 text-sm text-muted">
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Geolocation:</strong> IP-to-GeoIP databases (MaxMind, ip-api.com) reveal the approximate physical location, country, city, and data center. Useful for understanding hosting provider and data sovereignty.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">ASN Lookup:</strong> Every IP block is owned by an Autonomous System Number (ASN). An ASN reveals the hosting provider (AWS AS16509, Cloudflare AS13335, etc.) and other IP ranges they own.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Reverse DNS:</strong> A PTR record lookup on the IP may reveal the server's hostname or additional internal hostnames (e.g., <code class="text-primary-text bg-surface/50 px-1 rounded">ec2-93-184-216-34.compute-1.amazonaws.com</code>).</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Shodan / Censys:</strong> These internet-wide scanners continuously scan the entire IPv4 space. Looking up a discovered IP on Shodan reveals all open ports, banners, SSL certificates, and historical data without sending a single packet to the target.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Port Scanning:</strong> With the IP confirmed, the next step is Nmap scanning to enumerate open services — which feeds directly into the CVE correlation table.</span></li>
                                </ul>
                            </div>

                            <!-- IPv6 WAF Bypass Panel -->
                            <div class="bg-blue-950/20 border border-blue-500/20 rounded-xl p-4">
                                <h4 class="text-blue-400 font-medium mb-3">IPv4 vs IPv6 — The WAF Gap</h4>
                                <p class="text-sm text-muted mb-3 leading-relaxed">This is a critical security insight: many targets configure their WAF (Cloudflare, AWS WAF, etc.) only for their IPv4 address. If an AAAA record exists pointing directly to the origin server's IPv6 address, an attacker can often bypass the WAF entirely.</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[10px] space-y-1.5">
                                    <p class="text-muted"># IPv4 — goes through WAF (Cloudflare)</p>
                                    <p><span class="text-yellow-300">target.com A</span> <span class="text-orange-300">104.21.x.x</span> <span class="text-emerald-400">← WAF IP</span></p>
                                    <p class="text-muted mt-1"># IPv6 — may bypass WAF entirely!</p>
                                    <p><span class="text-yellow-300">target.com AAAA</span> <span class="text-red-300">2a01:4f8::1</span> <span class="text-red-400">← Direct origin</span></p>
                                    <p class="text-muted mt-2 italic">curl -6 https://target.com  # Uses IPv6 direct</p>
                                </div>
                                <p class="text-muted text-xs mt-3 leading-relaxed">If an AAAA record is found pointing to a non-CDN address, test whether WAF rules apply on the IPv6 path. Many organizations deploy WAF only for IPv4 traffic.</p>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4">
                            <h4 class="text-primary-text font-medium mb-2">Dual-Stack vs IPv4-Only Targets</h4>
                            <p class="text-sm text-muted leading-relaxed">A <strong>dual-stack</strong> target responds on both IPv4 (A record) and IPv6 (AAAA record). IPv4-only targets have no AAAA record. The presence of both records increases the attack surface — each IP path may have different firewall rules, different services, and different WAF configurations. During security assessments, both IP versions should be tested independently.</p>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">DNS Security Mechanisms</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">DNS was designed in the 1980s without security in mind. Multiple security extensions have been added to address its inherent vulnerabilities, particularly spoofing and cache poisoning attacks.</p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-4">
                                    <h4 class="text-emerald-400 font-medium mb-3">DNSSEC</h4>
                                    <p class="text-sm text-muted leading-relaxed mb-3">DNSSEC (DNS Security Extensions) adds cryptographic signatures to DNS records. A chain of trust from the root zone down to the authoritative nameserver means resolvers can verify that responses have not been tampered with in transit.</p>
                                    <ul class="space-y-2 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span>Prevents <strong class="text-primary-text">DNS cache poisoning</strong> — where an attacker injects false DNS records into a resolver's cache, redirecting users to a malicious IP.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span>Each record set is signed with the zone's private DNSKEY. Validators check the signature using the public key published in the parent zone's DS record.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span>Check: <code class="text-emerald-300 bg-emerald-500/10 px-1 rounded">dig DS target.com +short</code> — a response means DNSSEC is configured.</span></li>
                                    </ul>
                                </div>

                                <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-4">
                                    <h4 class="text-blue-400 font-medium mb-3">Split-Horizon DNS</h4>
                                    <p class="text-sm text-muted leading-relaxed">Split-horizon (or split-brain) DNS serves different DNS responses to internal vs external queries for the same domain. External clients resolve <code class="text-blue-300 bg-blue-500/10 px-1 rounded">app.company.com</code> to a public CDN IP, while internal employees resolve it to a private RFC 1918 address. This hides internal infrastructure topology from external reconnaissance.</p>
                                </div>
                            </div>

                            <div class="space-y-4">
                                <div class="bg-purple-950/10 border border-purple-500/20 rounded-xl p-4">
                                    <h4 class="text-purple-400 font-medium mb-3">CAA Records</h4>
                                    <p class="text-sm text-muted leading-relaxed mb-3">CAA (Certification Authority Authorization) records specify which Certificate Authorities (CAs) are allowed to issue TLS certificates for your domain. This is a critical defense against mis-issuance attacks where an attacker tricks a CA into issuing a certificate for your domain.</p>
                                    <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[10px] space-y-1">
                                        <p class="text-muted"># Only Let's Encrypt can issue certs</p>
                                        <p><span class="text-yellow-300">target.com CAA 0 issue</span> <span class="text-green-300">"letsencrypt.org"</span></p>
                                        <p><span class="text-yellow-300">target.com CAA 0 issuewild</span> <span class="text-red-300">";"</span> <span class="text-muted"># No wildcard certs</span></p>
                                        <p><span class="text-yellow-300">target.com CAA 0 iodef</span> <span class="text-cyan-300">"mailto:security@target.com"</span></p>
                                    </div>
                                </div>

                                <div class="bg-amber-950/10 border border-amber-500/20 rounded-xl p-4">
                                    <h4 class="text-amber-400 font-medium mb-3">DNS Change Monitoring</h4>
                                    <p class="text-sm text-muted leading-relaxed mb-2">Unauthorized DNS changes are a severe attack vector — if an attacker gains access to your DNS registrar, they can redirect all traffic to a server they control. Implement monitoring strategies:</p>
                                    <ul class="space-y-2 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span>Enable registrar-level alerts for any DNS record changes.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span>Use tools like <strong class="text-primary-text">DNSTwist</strong> or SecurityTrails to monitor for changes.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span>Enable DNSSEC and Registry Lock at your registrar to prevent unauthorized transfers.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span>Use separate, MFA-protected accounts for DNS management versus regular operations.</span></li>
                                    </ul>
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
