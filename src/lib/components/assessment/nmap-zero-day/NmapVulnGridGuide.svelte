<script lang="ts">
    import { X, Database, Activity, ShieldCheck, Bug, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. CVE Database", icon: Database },
        { id: 1, label: "2. CVSS Scoring", icon: Activity },
        { id: 2, label: "3. Patch Workflow", icon: ShieldCheck }
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
                            <Bug size={28} class="text-red-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">CVE Vulnerability Correlation</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Zero-day and known CVE matching against detected service versions via NVD database.</p>
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
                        <h3 class="text-lg font-semibold text-red-300 mb-4">The CVE & NVD System</h3>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    This table cross-references the service versions discovered during port scanning against the National Vulnerability Database (NVD), which is the U.S. government's authoritative repository of vulnerability data. Every CVE listed here was identified as affecting the exact software version detected on the target.
                                </p>
                                <div class="bg-red-950/20 border border-red-500/10 rounded-xl p-4">
                                    <h4 class="text-red-400 font-medium mb-3 flex items-center gap-2"><Database size={16}/> How CVEs Are Created</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">MITRE assigns CVE IDs:</strong> MITRE Corporation operates as a CVE Numbering Authority (CNA). When a security researcher or vendor reports a vulnerability, MITRE assigns a unique CVE identifier (e.g., CVE-2021-44228).</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">NVD enriches the entry:</strong> NIST's NVD takes the CVE and adds CVSS scoring, CPE applicability statements (which software versions are affected), and references to exploits or advisories.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-red-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Source: NVD</strong> means the data was fetched from the NVD REST API. Other sources may include vendor-specific databases like GitHub Advisory Database or OSV.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- CVE Entry Mockup -->
                            <div class="bg-[#0d1117] rounded-xl border border-red-500/10 shadow-2xl overflow-hidden font-mono text-xs">
                                <div class="bg-surface/80 px-4 py-2.5 flex items-center justify-between border-b border-subtle">
                                    <div class="flex items-center gap-2">
                                        <Bug size={14} class="text-red-400" />
                                        <span class="text-muted text-[10px]">Sample NVD Entry</span>
                                    </div>
                                    <span class="text-red-400 text-[10px] font-bold">CRITICAL</span>
                                </div>
                                <div class="p-4 space-y-3 text-[11px] leading-relaxed custom-scrollbar overflow-y-auto max-h-64">
                                    <div class="space-y-1.5">
                                        <p><span class="text-muted">CVE ID: </span><span class="text-red-300 font-bold">CVE-2021-44228</span></p>
                                        <p><span class="text-muted">Published: </span><span class="text-white">2021-12-10</span></p>
                                        <p><span class="text-muted">Source: </span><span class="text-blue-300">NVD (NIST)</span></p>
                                        <p><span class="text-muted">CVSS Score: </span><span class="text-red-400 font-bold">10.0 (Critical)</span></p>
                                        <p><span class="text-muted">Affects: </span><span class="text-yellow-300">log4j 2.0 – 2.14.1</span></p>
                                    </div>
                                    <div class="border-t border-subtle pt-2">
                                        <p class="text-muted text-[10px] mb-1">Description:</p>
                                        <p class="text-white leading-relaxed">Apache Log4j2 JNDI lookup allows attackers to execute arbitrary code via crafted log messages. Remote Code Execution without authentication.</p>
                                    </div>
                                    <div class="border border-red-500/20 rounded p-2 bg-red-950/20">
                                        <p class="text-red-400 text-[10px]">CPE Match: cpe:/a:apache:log4j:2.14.1</p>
                                        <p class="text-muted text-[10px] mt-1">Exploits available: YES (public PoC on GitHub)</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4">
                            <h4 class="text-primary-text font-medium mb-2">How This Table Populates</h4>
                            <p class="text-sm text-muted leading-relaxed">The scanning engine extracts CPE strings from Nmap's service detection output. These CPE strings are then used to query the NVD API, which returns all CVEs whose applicability statements match the detected version. The results are ranked by CVSS score so the most critical vulnerabilities appear first. Each row links directly to the full NVD entry for exploit references and patch advisories.</p>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-red-300 mb-4">CVSS v3 Scoring System</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">CVSS (Common Vulnerability Scoring System) v3 provides a standardized numerical score from 0 to 10 that reflects the severity of a vulnerability. The score is calculated from multiple base metrics that describe how the vulnerability is exploited and what damage it can cause.</p>

                        <!-- Score Bands -->
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-6">
                            <div class="bg-emerald-950/20 border border-emerald-500/30 rounded-xl p-4 text-center">
                                <div class="text-2xl font-bold text-emerald-400 mb-1">0–3.9</div>
                                <div class="text-emerald-400 text-xs font-bold uppercase tracking-wider mb-2">Low</div>
                                <p class="text-muted text-[11px] leading-relaxed">Minimal impact. Requires significant preconditions or local access. May still warrant patching during regular maintenance cycles.</p>
                            </div>
                            <div class="bg-yellow-950/20 border border-yellow-500/30 rounded-xl p-4 text-center">
                                <div class="text-2xl font-bold text-yellow-400 mb-1">4–6.9</div>
                                <div class="text-yellow-400 text-xs font-bold uppercase tracking-wider mb-2">Medium</div>
                                <p class="text-muted text-[11px] leading-relaxed">Significant risk. Often requires some user interaction or privileges. Should be scheduled for patching within 30–90 days.</p>
                            </div>
                            <div class="bg-orange-950/20 border border-orange-500/30 rounded-xl p-4 text-center">
                                <div class="text-2xl font-bold text-orange-400 mb-1">7–8.9</div>
                                <div class="text-orange-400 text-xs font-bold uppercase tracking-wider mb-2">High</div>
                                <p class="text-muted text-[11px] leading-relaxed">Serious vulnerability. Remote exploitation may be possible. Prioritize patching within 7–30 days. Verify exploit availability.</p>
                            </div>
                            <div class="bg-red-950/20 border border-red-500/30 rounded-xl p-4 text-center">
                                <div class="text-2xl font-bold text-red-400 mb-1">9–10</div>
                                <div class="text-red-400 text-xs font-bold uppercase tracking-wider mb-2">Critical</div>
                                <p class="text-muted text-[11px] leading-relaxed">Emergency. Often unauthenticated RCE over the network. Patch or mitigate within 24–72 hours. Active exploitation likely.</p>
                            </div>
                        </div>

                        <!-- CVSS Factors -->
                        <div class="bg-glass border border-subtle rounded-xl p-5 mb-6">
                            <h4 class="text-primary-text font-medium mb-4">Base Score Metrics</h4>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                <div class="space-y-3">
                                    <div>
                                        <div class="flex items-center justify-between mb-1">
                                            <span class="text-xs font-semibold text-primary-text">Attack Vector (AV)</span>
                                            <span class="text-[10px] text-red-400 bg-red-500/10 px-1.5 py-0.5 rounded">Network = worst</span>
                                        </div>
                                        <p class="text-xs text-muted">How the vulnerability is reached. <strong>Network (N)</strong> = exploitable remotely over internet, highest score. Physical (P) = requires physical access, lowest score.</p>
                                    </div>
                                    <div>
                                        <div class="flex items-center justify-between mb-1">
                                            <span class="text-xs font-semibold text-primary-text">Attack Complexity (AC)</span>
                                            <span class="text-[10px] text-red-400 bg-red-500/10 px-1.5 py-0.5 rounded">Low = worst</span>
                                        </div>
                                        <p class="text-xs text-muted">How hard it is to exploit. <strong>Low (L)</strong> = no special conditions needed (worst). High (H) = requires specific configuration or race condition.</p>
                                    </div>
                                    <div>
                                        <div class="flex items-center justify-between mb-1">
                                            <span class="text-xs font-semibold text-primary-text">Privileges Required (PR)</span>
                                            <span class="text-[10px] text-red-400 bg-red-500/10 px-1.5 py-0.5 rounded">None = worst</span>
                                        </div>
                                        <p class="text-xs text-muted"><strong>None (N)</strong> = no login required (unauthenticated attack). Low = standard user. High = admin account needed.</p>
                                    </div>
                                </div>
                                <div class="space-y-3">
                                    <div>
                                        <div class="flex items-center justify-between mb-1">
                                            <span class="text-xs font-semibold text-primary-text">User Interaction (UI)</span>
                                        </div>
                                        <p class="text-xs text-muted"><strong>None (N)</strong> = attacker acts alone without victim interaction. Required (R) = victim must click a link, open a file, etc.</p>
                                    </div>
                                    <div>
                                        <div class="flex items-center justify-between mb-1">
                                            <span class="text-xs font-semibold text-primary-text">CIA Impact Triad</span>
                                            <span class="text-[10px] text-red-400 bg-red-500/10 px-1.5 py-0.5 rounded">High = worst</span>
                                        </div>
                                        <p class="text-xs text-muted"><strong>Confidentiality</strong> = data disclosure. <strong>Integrity</strong> = data modification. <strong>Availability</strong> = service disruption/DoS. Each scored None/Low/High independently.</p>
                                    </div>
                                    <div class="bg-red-950/20 border border-red-500/20 rounded-lg p-3">
                                        <p class="text-red-400 text-xs font-medium">CVE-2021-44228 (Log4Shell) Breakdown</p>
                                        <p class="text-muted text-[10px] mt-1">AV:N / AC:L / PR:N / UI:N / S:C / C:H / I:H / A:H = <strong class="text-red-400">10.0 Critical</strong><br/>Network-reachable, trivial to exploit, no auth, full system compromise.</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Patch & Remediation Workflow</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Effective vulnerability management requires a structured triage-to-deploy process. Not all vulnerabilities warrant emergency response — the goal is to allocate effort proportional to actual risk.</p>

                        <!-- Prioritization Matrix -->
                        <div class="bg-glass border border-subtle rounded-xl p-5 mb-6">
                            <h4 class="text-primary-text font-medium mb-4">Triage Priority Matrix</h4>
                            <div class="overflow-x-auto custom-scrollbar">
                                <table class="w-full text-xs font-mono">
                                    <thead>
                                        <tr class="border-b border-subtle text-muted">
                                            <th class="text-left pb-2 pr-4">CVSS Score</th>
                                            <th class="text-left pb-2 pr-4">Exploit Available?</th>
                                            <th class="text-left pb-2 pr-4">Priority</th>
                                            <th class="text-left pb-2">SLA Target</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-subtle">
                                        <tr class="py-2">
                                            <td class="py-2 pr-4 text-red-400 font-bold">9.0 – 10.0</td>
                                            <td class="py-2 pr-4 text-red-300">Yes</td>
                                            <td class="py-2 pr-4"><span class="bg-red-500/20 text-red-400 px-2 py-0.5 rounded border border-red-500/30">P0 CRITICAL</span></td>
                                            <td class="py-2 text-white">24–72 hours</td>
                                        </tr>
                                        <tr>
                                            <td class="py-2 pr-4 text-red-400 font-bold">9.0 – 10.0</td>
                                            <td class="py-2 pr-4 text-muted">No</td>
                                            <td class="py-2 pr-4"><span class="bg-orange-500/20 text-orange-400 px-2 py-0.5 rounded border border-orange-500/30">P1 HIGH</span></td>
                                            <td class="py-2 text-white">7 days</td>
                                        </tr>
                                        <tr>
                                            <td class="py-2 pr-4 text-orange-400 font-bold">7.0 – 8.9</td>
                                            <td class="py-2 pr-4 text-red-300">Yes</td>
                                            <td class="py-2 pr-4"><span class="bg-orange-500/20 text-orange-400 px-2 py-0.5 rounded border border-orange-500/30">P1 HIGH</span></td>
                                            <td class="py-2 text-white">7–14 days</td>
                                        </tr>
                                        <tr>
                                            <td class="py-2 pr-4 text-orange-400 font-bold">7.0 – 8.9</td>
                                            <td class="py-2 pr-4 text-muted">No</td>
                                            <td class="py-2 pr-4"><span class="bg-yellow-500/20 text-yellow-400 px-2 py-0.5 rounded border border-yellow-500/30">P2 MEDIUM</span></td>
                                            <td class="py-2 text-white">30 days</td>
                                        </tr>
                                        <tr>
                                            <td class="py-2 pr-4 text-yellow-400 font-bold">4.0 – 6.9</td>
                                            <td class="py-2 pr-4 text-muted">Either</td>
                                            <td class="py-2 pr-4"><span class="bg-blue-500/20 text-blue-400 px-2 py-0.5 rounded border border-blue-500/30">P3 LOW</span></td>
                                            <td class="py-2 text-white">90 days</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>

                        <!-- Remediation Steps -->
                        <div class="space-y-3 mb-6">
                            {#each [
                                { step: "1", title: "Verify Exploitability", color: "red", desc: "Check NVD for exploit references (click 'Exploit Reference' links in the table). Search GitHub and ExploitDB. A critical CVE with no public exploit is lower priority than a medium with a weaponized PoC." },
                                { step: "2", title: "Confirm Patch Availability", color: "orange", desc: "Check the vendor's security advisory. Many CVEs have patches available same-day. If no patch exists, move to compensating controls immediately." },
                                { step: "3", title: "Test in Staging", color: "yellow", desc: "Never deploy security patches directly to production. Apply to a staging environment first — some patches introduce breaking changes that require application-level testing before rollout." },
                                { step: "4", title: "Deploy & Verify", color: "emerald", desc: "Deploy the patch, restart affected services, and re-run the scan to confirm the CVE no longer appears. Document the remediation with timestamp and patch version for compliance purposes." }
                            ] as item}
                                <div class="flex gap-3 p-4 bg-glass border border-subtle rounded-xl hover:border-{item.color}-500/30 transition-colors">
                                    <div class="w-7 h-7 rounded-full bg-{item.color}-500/20 border border-{item.color}-500/30 flex items-center justify-center text-{item.color}-400 text-xs font-bold shrink-0">{item.step}</div>
                                    <div>
                                        <h5 class="text-sm font-semibold text-primary-text mb-1">{item.title}</h5>
                                        <p class="text-xs text-muted leading-relaxed">{item.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>

                        <div class="bg-amber-950/20 border border-amber-500/20 rounded-xl p-4">
                            <h4 class="text-amber-400 font-medium mb-2">Compensating Controls (When Patching Isn't Possible)</h4>
                            <ul class="space-y-2 text-sm text-muted">
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span><strong class="text-primary-text">WAF Virtual Patching:</strong> Deploy a WAF rule that blocks known exploit patterns for the CVE. ModSecurity OWASP CRS rules often include these. Buys time while a patch is developed or tested.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Network Segmentation:</strong> Move the vulnerable service behind a private subnet or VPN. Remove it from the internet-facing perimeter entirely to eliminate remote exploitability.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Disable the Feature:</strong> If the vulnerability is in a specific component (e.g., a JNDI lookup, an XML parser, an upload endpoint), consider disabling that feature until a patch is available.</span></li>
                            </ul>
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
