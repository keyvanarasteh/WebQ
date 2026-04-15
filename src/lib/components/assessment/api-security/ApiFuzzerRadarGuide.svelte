<script lang="ts">
    import { X, Shield, Braces, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. OWASP Vectors", icon: Shield },
        { id: 1, label: "2. Fuzzing Engine", icon: Braces },
        { id: 2, label: "3. Defense Matrix", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-rose-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(244,63,94,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-rose-950/40 via-rose-900/10 to-transparent p-6 border-b border-rose-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-rose-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-rose-500/20 rounded-xl transition-all border border-subtle hover:border-rose-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-rose-500/20 to-rose-600/10 flex items-center justify-center border border-rose-500/30 shadow-inner">
                            <Braces size={28} class="text-rose-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Vulnerability Distribution Radar</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Categorized breakdown of discovered attack surface across OWASP top vectors.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-rose-500/20 text-rose-300 border border-rose-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 custom-scrollbar relative">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-300 mb-4">OWASP Attack Vectors Tracked</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl leading-relaxed">
                            The radar chart maps five critical OWASP vulnerability categories across the target's API surface. Each bar represents the number of confirmed findings in that category, coloured red when any critical-severity instance was detected.
                        </p>

                        <div class="space-y-4 mb-6">
                            <!-- SQL Injection -->
                            <div class="bg-rose-950/20 border border-rose-500/20 rounded-xl p-4 hover:border-rose-500/40 transition-colors">
                                <div class="flex items-start gap-3">
                                    <div class="bg-red-500/10 p-2 rounded-lg shrink-0 mt-0.5">
                                        <Shield size={16} class="text-red-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-red-300 mb-1">SQL Injection</h4>
                                        <p class="text-xs text-muted leading-relaxed">Unsanitised user input is interpolated into SQL queries. The fuzzer injects payloads like <code class="text-red-300 bg-glass px-1 rounded">' OR 1=1--</code> and checks for database errors, timing anomalies, or data disclosure in responses.</p>
                                        <div class="mt-2 flex items-center gap-2">
                                            <div class="h-1.5 w-24 rounded-full bg-gradient-to-r from-red-600 to-red-400 shadow-[0_0_8px_rgba(239,68,68,0.4)]"></div>
                                            <span class="text-[10px] text-red-400 font-mono">CRITICAL severity class</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <!-- XSS -->
                            <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4 hover:border-orange-500/40 transition-colors">
                                <div class="flex items-start gap-3">
                                    <div class="bg-orange-500/10 p-2 rounded-lg shrink-0 mt-0.5">
                                        <Shield size={16} class="text-orange-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-orange-300 mb-1">Cross-Site Scripting (XSS)</h4>
                                        <p class="text-xs text-muted leading-relaxed">Script-injection payloads are reflected or stored in responses without encoding. The fuzzer submits <code class="text-orange-300 bg-glass px-1 rounded">&lt;script&gt;alert(1)&lt;/script&gt;</code> variants and detects unescaped echo-back in HTML, JSON, or header fields.</p>
                                        <div class="mt-2 flex items-center gap-2">
                                            <div class="h-1.5 w-20 rounded-full bg-gradient-to-r from-orange-600 to-orange-400 shadow-[0_0_8px_rgba(249,115,22,0.4)]"></div>
                                            <span class="text-[10px] text-orange-400 font-mono">HIGH severity class</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <!-- SSRF -->
                            <div class="bg-yellow-950/20 border border-yellow-500/20 rounded-xl p-4 hover:border-yellow-500/40 transition-colors">
                                <div class="flex items-start gap-3">
                                    <div class="bg-yellow-500/10 p-2 rounded-lg shrink-0 mt-0.5">
                                        <Shield size={16} class="text-yellow-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-yellow-300 mb-1">Server-Side Request Forgery (SSRF)</h4>
                                        <p class="text-xs text-muted leading-relaxed">Parameters that accept URLs are probed with internal addresses: <code class="text-yellow-300 bg-glass px-1 rounded">http://169.254.169.254/latest/meta-data/</code> (AWS metadata), <code class="text-yellow-300 bg-glass px-1 rounded">http://localhost:6379</code> (Redis). Response timing and body size indicate internal reachability.</p>
                                    </div>
                                </div>
                            </div>

                            <!-- LFI -->
                            <div class="bg-rose-950/20 border border-rose-500/20 rounded-xl p-4 hover:border-rose-500/40 transition-colors">
                                <div class="flex items-start gap-3">
                                    <div class="bg-rose-500/10 p-2 rounded-lg shrink-0 mt-0.5">
                                        <Shield size={16} class="text-rose-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-rose-300 mb-1">LFI / Path Traversal</h4>
                                        <p class="text-xs text-muted leading-relaxed">File-path parameters are fuzzed with traversal sequences: <code class="text-rose-300 bg-glass px-1 rounded">../../../../etc/passwd</code>, URL-encoded variants, and null-byte injection. Detection is based on known file-content signatures in the response.</p>
                                    </div>
                                </div>
                            </div>

                            <!-- Auth Bypass -->
                            <div class="bg-purple-950/20 border border-purple-500/20 rounded-xl p-4 hover:border-purple-500/40 transition-colors">
                                <div class="flex items-start gap-3">
                                    <div class="bg-purple-500/10 p-2 rounded-lg shrink-0 mt-0.5">
                                        <Shield size={16} class="text-purple-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-purple-300 mb-1">Auth Bypass / Config Exposure</h4>
                                        <p class="text-xs text-muted leading-relaxed">Endpoints are probed without credentials or with modified JWT claims. Common paths like <code class="text-purple-300 bg-glass px-1 rounded">/admin</code>, <code class="text-purple-300 bg-glass px-1 rounded">/.env</code>, and <code class="text-purple-300 bg-glass px-1 rounded">/config.json</code> are checked for 200-status disclosures.</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4 text-xs text-muted leading-relaxed">
                            <ArrowRight size={12} class="text-rose-400 inline mr-1 shrink-0" />
                            The bar width is proportional to that category's share of total findings. A fully red bar means every finding in the category was rated CRITICAL. The radar updates live as the fuzzer sends new payloads.
                        </div>
                    </div>

                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-300 mb-4">How the Fuzzing Engine Works</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl leading-relaxed">
                            The fuzzer combines endpoint discovery via crawling with a payload mutation engine. It builds a dynamic wordlist of parameter names and values, then fires thousands of crafted HTTP requests and analyses responses for vulnerability signatures.
                        </p>

                        <!-- Terminal Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-rose-500/10 shadow-2xl overflow-hidden font-mono text-xs mb-6">
                            <div class="bg-surface/80 px-4 py-3 flex items-center gap-2 border-b border-subtle">
                                <div class="flex gap-1.5">
                                    <div class="w-3 h-3 rounded-full bg-red-500/70"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500/70"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500/70"></div>
                                </div>
                                <span class="text-muted text-[10px] ml-2">webq-fuzzer — api_scan.log</span>
                            </div>
                            <div class="p-5 space-y-1.5 text-primary-text h-72 overflow-y-auto custom-scrollbar">
                                <p><span class="text-muted">[INF]</span> <span class="text-blue-300">Crawling</span> https://target.com/api — depth 3</p>
                                <p><span class="text-muted">[INF]</span> <span class="text-blue-300">Discovered</span> 47 unique endpoints</p>
                                <p><span class="text-muted">[INF]</span> <span class="text-blue-300">Parameters</span> extracted: id, q, url, file, token (23 total)</p>
                                <p class="mt-2"><span class="text-muted">[FUZ]</span> POST /api/v1/search?q=<span class="text-red-400">%27+OR+1%3D1--</span></p>
                                <p><span class="text-muted">[FUZ]</span> POST /api/v1/search?q=<span class="text-red-400">%27+UNION+SELECT+NULL--</span></p>
                                <p><span class="text-muted">[HIT]</span> <span class="text-red-400 font-bold">SQL_INJECTION</span> @ /api/v1/search param=q | evidence: You have an error in your SQL syntax</p>
                                <p class="mt-2"><span class="text-muted">[FUZ]</span> GET /api/v2/render?url=<span class="text-yellow-400">http://169.254.169.254/latest/meta-data/</span></p>
                                <p><span class="text-muted">[HIT]</span> <span class="text-yellow-400 font-bold">SSRF</span> @ /api/v2/render param=url | evidence: ami-id: ami-0abcdef123456</p>
                                <p class="mt-2"><span class="text-muted">[FUZ]</span> GET /api/v1/file?path=<span class="text-rose-400">../../../../etc/passwd</span></p>
                                <p><span class="text-muted">[HIT]</span> <span class="text-rose-400 font-bold">LFI</span> @ /api/v1/file param=path | evidence: root:x:0:0:root</p>
                                <p class="mt-2"><span class="text-muted">[FUZ]</span> GET /api/v1/render?template=<span class="text-orange-400">&lt;script&gt;alert(document.domain)&lt;/script&gt;</span></p>
                                <p><span class="text-muted">[HIT]</span> <span class="text-orange-400 font-bold">XSS</span> @ /api/v1/render param=template | evidence: reflected unencoded in body</p>
                                <p class="mt-4"><span class="text-muted">[SUM]</span> <span class="text-emerald-300">Scan complete.</span> 4 critical findings, 12 high, 7 medium</p>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                            <div class="bg-rose-950/10 border border-rose-500/20 rounded-xl p-4">
                                <h4 class="text-xs font-semibold text-rose-300 mb-2 uppercase tracking-wider">Endpoint Discovery</h4>
                                <p class="text-xs text-muted leading-relaxed">Spider crawls the target collecting form action URLs, XHR fetch calls, and link hrefs. OpenAPI/Swagger specs are parsed if detected to extract all declared routes.</p>
                            </div>
                            <div class="bg-rose-950/10 border border-rose-500/20 rounded-xl p-4">
                                <h4 class="text-xs font-semibold text-rose-300 mb-2 uppercase tracking-wider">Payload Mutation</h4>
                                <p class="text-xs text-muted leading-relaxed">Base payloads are mutated with URL encoding, double encoding, Unicode escape sequences, null bytes, and case variants to bypass naïve input filters.</p>
                            </div>
                            <div class="bg-rose-950/10 border border-rose-500/20 rounded-xl p-4">
                                <h4 class="text-xs font-semibold text-rose-300 mb-2 uppercase tracking-wider">Response Analysis</h4>
                                <p class="text-xs text-muted leading-relaxed">Each response is scored against error-message signatures, timing deltas (blind SQLi), status code changes, and body-length anomalies to assign a confidence level.</p>
                            </div>
                        </div>
                    </div>

                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Defense Matrix</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl leading-relaxed">
                            Each vulnerability category has a primary remediation strategy. Implement all five to close the attack surface the radar visualises.
                        </p>

                        <div class="space-y-3">
                            <div class="bg-glass border border-subtle rounded-xl p-4 flex gap-4 hover:border-emerald-500/30 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={18} class="text-emerald-400" />
                                </div>
                                <div class="flex-1">
                                    <div class="flex items-center justify-between mb-1">
                                        <h4 class="text-sm font-semibold text-emerald-300">SQL Injection → Parameterised Queries</h4>
                                        <span class="text-[10px] text-red-400 font-mono bg-red-500/10 px-2 py-0.5 rounded border border-red-500/20">CRITICAL</span>
                                    </div>
                                    <p class="text-xs text-muted leading-relaxed">Never concatenate user input into SQL. Use prepared statements or an ORM:</p>
                                    <div class="mt-2 bg-[#0d1117] rounded-lg p-3 font-mono text-xs">
                                        <span class="text-muted">// Bad:</span><br />
                                        <span class="text-red-400">db.query("SELECT * FROM users WHERE id=" + req.params.id)</span><br />
                                        <span class="text-muted mt-1 block">// Good:</span><br />
                                        <span class="text-emerald-300">db.query("SELECT * FROM users WHERE id = ?", [req.params.id])</span>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-glass border border-subtle rounded-xl p-4 flex gap-4 hover:border-orange-500/30 transition-colors">
                                <div class="bg-orange-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={18} class="text-orange-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-orange-300 mb-1">XSS → Output Encoding</h4>
                                    <p class="text-xs text-muted leading-relaxed">HTML-encode all user-supplied content before rendering. Use a Content Security Policy (CSP) header to restrict script sources: <code class="text-orange-300 bg-glass px-1 rounded">Content-Security-Policy: default-src 'self'</code>. Avoid <code class="text-orange-300 bg-glass px-1 rounded">innerHTML</code> — prefer <code class="text-orange-300 bg-glass px-1 rounded">textContent</code>.</p>
                                </div>
                            </div>

                            <div class="bg-glass border border-subtle rounded-xl p-4 flex gap-4 hover:border-yellow-500/30 transition-colors">
                                <div class="bg-yellow-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={18} class="text-yellow-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-yellow-300 mb-1">SSRF → Strict URL Allowlist</h4>
                                    <p class="text-xs text-muted leading-relaxed">Validate URL parameters against an explicit allowlist of permitted domains/schemes. Block all private CIDR ranges at the network layer. Use a dedicated egress proxy that enforces the allowlist rather than relying on application code.</p>
                                </div>
                            </div>

                            <div class="bg-glass border border-subtle rounded-xl p-4 flex gap-4 hover:border-rose-500/30 transition-colors">
                                <div class="bg-rose-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={18} class="text-rose-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-rose-300 mb-1">LFI / Path Traversal → Path Canonicalisation</h4>
                                    <p class="text-xs text-muted leading-relaxed">Resolve the path to its canonical form and verify it starts with the permitted base directory: <code class="text-rose-300 bg-glass px-1 rounded">path.resolve(base, input).startsWith(base)</code>. Never expose raw filesystem paths in parameters.</p>
                                </div>
                            </div>

                            <div class="bg-glass border border-subtle rounded-xl p-4 flex gap-4 hover:border-purple-500/30 transition-colors">
                                <div class="bg-purple-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={18} class="text-purple-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-purple-300 mb-1">Auth Bypass → Centralised Access Control</h4>
                                    <p class="text-xs text-muted leading-relaxed">Apply authentication middleware globally and deny-by-default. Validate JWT signatures server-side — never trust <code class="text-purple-300 bg-glass px-1 rounded">alg: none</code>. Remove debug endpoints and sensitive config files (<code class="text-purple-300 bg-glass px-1 rounded">.env</code>, <code class="text-purple-300 bg-glass px-1 rounded">config.json</code>) from production deployments.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-rose-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-rose-600 hover:bg-rose-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(244,63,94,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-rose-500/50 outline-none"
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
