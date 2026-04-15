<script lang="ts">
    import { X, Crosshair, Search, ShieldAlert, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Passive Analysis", icon: Search },
        { id: 1, label: "2. Vulnerability Types", icon: ShieldAlert },
        { id: 2, label: "3. Triage Workflow", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-orange-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(249,115,22,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-orange-950/40 via-orange-900/10 to-transparent p-6 border-b border-orange-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-orange-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-orange-500/20 rounded-xl transition-all border border-subtle hover:border-orange-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-orange-500/20 to-orange-600/10 flex items-center justify-center border border-orange-500/30 shadow-inner">
                            <Crosshair size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Static Vulnerability Scan</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Passive structural vulnerability detection without active exploitation attempts.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-orange-500/20 text-orange-300 border border-orange-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">What is Passive/Static Scanning?</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Passive (static) scanning analyzes the target's HTTP responses as they naturally occur — no exploit payloads, no fuzzing, no brute force. The scanner observes what the server reveals about itself and compares those signals against known vulnerability patterns to identify configuration weaknesses.
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <div class="p-4 bg-emerald-950/10 border border-emerald-500/20 rounded-xl">
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-3 flex items-center gap-2"><Search size={14}/> What Passive Scanning Covers</h4>
                                    <ul class="space-y-2 text-xs text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>HTTP response header analysis (missing security headers, version banners)</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>Technology detection (server type, framework, CMS from signatures)</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>Known misconfiguration patterns (CORS wildcards, insecure cookie attributes)</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>Structural exposure signals (directory listing, debug mode indicators)</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>SSL/TLS protocol and cipher assessment</span></li>
                                    </ul>
                                </div>
                            </div>

                            <div class="space-y-4">
                                <div class="p-4 bg-red-950/10 border border-red-500/20 rounded-xl">
                                    <h4 class="text-sm font-semibold text-red-300 mb-3 flex items-center gap-2"><Crosshair size={14}/> What Active Fuzzing Does (Not This Module)</h4>
                                    <ul class="space-y-2 text-xs text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>Sends malformed/attack inputs to find injection vulnerabilities</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>Attempts authentication bypass, directory traversal payloads</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>Brute-forces hidden endpoints and parameters</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>Generates significant server load and log noise</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>Covered by the API Security module in this tool</span></li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <!-- Signal Trigger Examples -->
                        <div class="bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-6">
                            <div class="bg-surface/80 px-4 py-2 flex items-center gap-2 border-b border-subtle">
                                <Search size={12} class="text-orange-400" />
                                <span class="text-muted text-xs">Signal → Finding mapping examples</span>
                            </div>
                            <div class="p-4 space-y-3">
                                {#each [
                                    { signal: "Server: Apache/2.4.51", finding: "Information Disclosure: Server version exposed", severity: "Medium" },
                                    { signal: "No Strict-Transport-Security header", finding: "Missing Security Control: HSTS absent", severity: "High" },
                                    { signal: "Access-Control-Allow-Origin: *", finding: "CORS Misconfiguration: Wildcard ACAO", severity: "Medium" },
                                    { signal: "Set-Cookie: id=xyz (no flags)", finding: "Insecure Cookie: Missing HttpOnly, Secure, SameSite", severity: "High" },
                                    { signal: "TLS 1.0 protocol negotiated", finding: "Weak Transport: TLS 1.0 enabled (POODLE risk)", severity: "Critical" }
                                ] as item}
                                    <div class="flex items-start gap-3">
                                        <div class="flex-1">
                                            <div class="text-cyan-300 text-[10px] mb-0.5">Observed: {item.signal}</div>
                                            <div class="text-orange-300 text-[10px]">→ {item.finding}</div>
                                        </div>
                                        <span class="text-[9px] uppercase font-bold px-1.5 py-0.5 rounded border shrink-0 {
                                            item.severity === 'Critical' ? 'text-red-400 border-red-500/30 bg-red-500/10' :
                                            item.severity === 'High' ? 'text-orange-400 border-orange-500/30 bg-orange-500/10' :
                                            'text-yellow-400 border-yellow-500/30 bg-yellow-500/10'
                                        }">{item.severity}</span>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Vulnerability Type Reference</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Passive scanning identifies structural vulnerability categories. Each type has a specific severity classification based on its direct exploitability and potential impact — not just its theoretical risk.
                        </p>

                        <div class="space-y-3 mb-8">
                            {#each [
                                {
                                    type: "Information Disclosure",
                                    severity: "Medium",
                                    color: "yellow",
                                    desc: "Server version strings, technology stack headers, error messages with stack traces, or internal IP addresses in responses. Enables attackers to rapidly identify applicable CVEs for the exact software version.",
                                    example: "Server: nginx/1.18.0, X-Powered-By: PHP/7.4.3"
                                },
                                {
                                    type: "SSL/TLS Misconfiguration",
                                    severity: "Critical–High",
                                    color: "red",
                                    desc: "Deprecated protocol versions (TLS 1.0/1.1), weak cipher suites (RC4, 3DES, NULL), expired or self-signed certificates, missing HSTS. Allows network-level interception or decryption of encrypted traffic.",
                                    example: "Protocol: TLSv1.0, Cipher: DES-CBC3-SHA"
                                },
                                {
                                    type: "Missing Security Controls",
                                    severity: "High–Critical",
                                    color: "red",
                                    desc: "Absent security headers (CSP, HSTS, X-Frame-Options), no HTTPS redirect, or no WAF protection. Each missing control re-enables a specific attack class (XSS, clickjacking, MitM). High severity when multiple controls are simultaneously absent.",
                                    example: "No Content-Security-Policy, No X-Frame-Options"
                                },
                                {
                                    type: "Clickjacking Exposure",
                                    severity: "Medium",
                                    color: "yellow",
                                    desc: "Page can be embedded in iframes from any origin (no X-Frame-Options or CSP frame-ancestors directive). Risk varies by page sensitivity — a public homepage is lower risk than an account settings or payment page.",
                                    example: "No X-Frame-Options header on authenticated pages"
                                },
                                {
                                    type: "Directory / File Exposure",
                                    severity: "Critical",
                                    color: "red",
                                    desc: "Exposed .git directories (attacker can reconstruct source code), .env files (API keys, database credentials), backup files (.sql.gz, .zip), or directory listing enabled. These are immediate critical findings requiring instant remediation.",
                                    example: "/.git/config accessible, /.env returns 200 OK"
                                }
                            ] as vuln}
                                <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                    <div class="flex items-center justify-between mb-2">
                                        <h4 class="text-sm font-semibold text-primary-text">{vuln.type}</h4>
                                        <span class="text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded border {
                                            vuln.color === 'red' ? 'text-red-400 border-red-500/30 bg-red-500/10' :
                                            'text-yellow-400 border-yellow-500/30 bg-yellow-500/10'
                                        }">{vuln.severity}</span>
                                    </div>
                                    <p class="text-xs text-muted leading-relaxed mb-2">{vuln.desc}</p>
                                    <code class="text-[10px] font-mono text-cyan-300/70 bg-surface/50 px-2 py-1 rounded border border-subtle block">e.g., {vuln.example}</code>
                                </div>
                            {/each}
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm">Severity Classification Criteria</h4>
                            <p class="text-xs text-muted leading-relaxed">Severity is assigned based on <strong class="text-primary-text">exploitability</strong> (how easy to exploit), <strong class="text-primary-text">impact</strong> (what an attacker gains), and <strong class="text-primary-text">scope</strong> (how many users/systems are affected). Context matters: an exposed .env file on a staging server with no sensitive data is lower actual risk than the same finding on production.</p>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Vulnerability Triage Workflow</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Raw vulnerability findings need to be triaged — prioritized and contextualized — before remediation. A structured triage process prevents both under-reaction (ignoring critical findings) and over-reaction (panicking about low-risk theoretical vulnerabilities).
                        </p>

                        <!-- Triage Steps -->
                        <div class="space-y-3 mb-8">
                            {#each [
                                { step: "1", title: "Sort by Severity", desc: "Critical and High findings first. Critical = immediate exploitation risk. Don't spend time on Medium findings while a .env file is publicly accessible." },
                                { step: "2", title: "Assess Exploitability in Context", desc: "A finding that is 'High' in theory may be 'Low' in your context. Missing X-Frame-Options on your public blog is lower risk than on your payment page. A server banner on an internal tool behind VPN is very low actual risk." },
                                { step: "3", title: "Check for Compensating Controls", desc: "Does a WAF already block the attack this finding enables? Is the affected endpoint behind authentication? Compensating controls reduce effective severity. Document them." },
                                { step: "4", title: "Assign to Team with Context", desc: "Don't just file 'fix missing CSP header'. Assign with: what the header does, specific risk on this domain, example correct configuration, and a priority deadline (24h for critical, 1 week for high, 1 month for medium)." }
                            ] as step}
                                <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                    <div class="w-8 h-8 rounded-full bg-orange-500/20 border border-orange-500/30 flex items-center justify-center shrink-0 font-bold text-orange-300 text-sm">
                                        {step.step}
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-primary-text mb-1">{step.title}</h4>
                                        <p class="text-xs text-muted leading-relaxed">{step.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>

                        <!-- Decision Matrix -->
                        <div class="bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden mb-6">
                            <div class="bg-surface/80 px-4 py-2 border-b border-subtle">
                                <span class="text-orange-400 font-medium text-xs">Priority Decision Matrix</span>
                            </div>
                            <div class="overflow-x-auto">
                                <table class="w-full text-xs font-mono">
                                    <thead>
                                        <tr class="border-b border-subtle">
                                            <th class="text-left px-4 py-2 text-muted text-[10px] uppercase tracking-wider">Finding Severity</th>
                                            <th class="text-left px-4 py-2 text-muted text-[10px] uppercase tracking-wider">Exploitable?</th>
                                            <th class="text-left px-4 py-2 text-muted text-[10px] uppercase tracking-wider">Compensating Control?</th>
                                            <th class="text-left px-4 py-2 text-muted text-[10px] uppercase tracking-wider">Action</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-subtle">
                                        {#each [
                                            { sev: "Critical", scolor: "red", exp: "Yes", comp: "No", action: "Fix within 24 hours", acolor: "red" },
                                            { sev: "Critical", scolor: "red", exp: "Yes", comp: "WAF blocks", action: "Fix this sprint + document WAF rule", acolor: "orange" },
                                            { sev: "High", scolor: "orange", exp: "Yes", comp: "No", action: "Fix within 1 week", acolor: "orange" },
                                            { sev: "High", scolor: "orange", exp: "Conditional", comp: "Partial", action: "Fix within 1 month", acolor: "yellow" },
                                            { sev: "Medium", scolor: "yellow", exp: "Theoretical", comp: "Yes", action: "Schedule next quarter", acolor: "emerald" },
                                            { sev: "Low", scolor: "emerald", exp: "No", comp: "N/A", action: "Log and monitor", acolor: "emerald" }
                                        ] as row}
                                            <tr class="hover:bg-surface/30 transition-colors">
                                                <td class="px-4 py-2.5 font-bold text-{row.scolor}-400">{row.sev}</td>
                                                <td class="px-4 py-2.5 text-muted">{row.exp}</td>
                                                <td class="px-4 py-2.5 text-muted">{row.comp}</td>
                                                <td class="px-4 py-2.5 text-{row.acolor}-300">{row.action}</td>
                                            </tr>
                                        {/each}
                                    </tbody>
                                </table>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-2">"Clean" vs Actually Secure</h4>
                                <p class="text-xs text-muted leading-relaxed">A "Clean" passive scan result means no structural misconfigurations were detected. It does NOT mean the application has no vulnerabilities. Business logic flaws, authentication bypasses, and injection points in API parameters require active testing — which passive scanning intentionally skips.</p>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-2">When to Escalate to Pen Testing</h4>
                                <p class="text-xs text-muted leading-relaxed">Passive scanning is a starting point, not a comprehensive audit. Escalate to active penetration testing when: handling PCI/HIPAA data, pre-launch for significant user-facing products, after a security incident, or as part of annual compliance requirements. Pen testing actively confirms exploitability.</p>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-orange-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-orange-600 hover:bg-orange-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(249,115,22,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-orange-500/50 outline-none"
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
