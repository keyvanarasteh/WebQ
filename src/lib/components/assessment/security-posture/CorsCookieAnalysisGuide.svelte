<script lang="ts">
    import { X, Network, Cookie, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. CORS Policy", icon: Network },
        { id: 1, label: "2. Cookie Security Flags", icon: Cookie },
        { id: 2, label: "3. Hardening", icon: ShieldCheck }
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
                            <Network size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">CORS & Cookie Security</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Cross-origin policy enforcement and session cookie attribute analysis.</p>
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
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Same-Origin Policy & CORS</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            The <strong class="text-primary-text">Same-Origin Policy (SOP)</strong> is a fundamental browser security rule: JavaScript running on <code class="text-cyan-300">site-a.com</code> cannot read responses from <code class="text-cyan-300">site-b.com</code>. This prevents malicious sites from reading your banking session data or email. An "origin" is defined as the combination of protocol + hostname + port.
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <div class="bg-orange-950/20 border border-orange-500/10 rounded-xl p-4">
                                    <h4 class="text-sm font-semibold text-orange-300 mb-3">What CORS Headers Do</h4>
                                    <p class="text-xs text-muted mb-3 leading-relaxed">CORS (Cross-Origin Resource Sharing) is a mechanism that selectively relaxes SOP. Your server sends response headers that tell the browser which other origins are permitted to read the response.</p>
                                    <div class="space-y-2 text-xs">
                                        <div class="p-2 bg-surface/50 rounded border border-subtle font-mono">
                                            <div class="text-muted mb-1"># Safe: explicit allowlist</div>
                                            <div class="text-emerald-300">Access-Control-Allow-Origin: https://app.example.com</div>
                                        </div>
                                        <div class="p-2 bg-red-950/10 rounded border border-red-500/20 font-mono">
                                            <div class="text-muted mb-1"># Dangerous: open wildcard</div>
                                            <div class="text-red-300">Access-Control-Allow-Origin: *</div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="space-y-4">
                                <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-4">
                                    <h4 class="text-sm font-semibold text-red-300 mb-3 flex items-center gap-2"><ArrowRight size={12}/> Wildcard + Credentials = Critical Vulnerability</h4>
                                    <p class="text-xs text-muted mb-3 leading-relaxed">A wildcard <code class="text-cyan-300">ACAO: *</code> alone is not critical — browsers block credential sharing with wildcards. But when combined with <code class="text-cyan-300">Access-Control-Allow-Credentials: true</code>, any website can make authenticated API requests AS the victim user.</p>
                                    <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[11px] border border-subtle">
                                        <div class="text-muted mb-1">// Attacker site sends:</div>
                                        <div class="text-orange-300">fetch('https://api.target.com/user/profile', {"{"}</div>
                                        <div class="text-orange-300 ml-4">credentials: 'include'</div>
                                        <div class="text-orange-300">{"}"}).then(r =&gt; r.json())</div>
                                        <div class="text-muted mt-2 ml-1">.then(data =&gt; exfiltrate(data))</div>
                                        <div class="text-red-400 text-[10px] mt-2">// Browser sends victim's session cookies!</div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm">The CORS Preflight (OPTIONS) Request</h4>
                            <p class="text-xs text-muted leading-relaxed">For non-simple requests (PUT, DELETE, custom headers), browsers send an <code class="text-cyan-300">OPTIONS</code> preflight request first to ask the server "can origin X send a DELETE request?" The server's <code class="text-cyan-300">Access-Control-Allow-Methods</code> and <code class="text-cyan-300">Access-Control-Allow-Headers</code> responses determine whether the real request proceeds. A misconfigured server that always returns 200 OK to OPTIONS is effectively bypassing CORS checks.</p>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Cookie Security Attributes</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Session cookies are the keys to your users' accounts. The <code class="text-cyan-300">Set-Cookie</code> response header can include security attributes that restrict how browsers handle the cookie. Missing even one attribute opens specific attack vectors.
                        </p>

                        <!-- Ideal Cookie Header -->
                        <div class="bg-[#0d1117] rounded-xl border border-emerald-500/20 overflow-hidden font-mono text-xs mb-6">
                            <div class="bg-surface/80 px-4 py-2 border-b border-subtle">
                                <span class="text-emerald-400 font-medium text-xs">Ideal Secure Session Cookie</span>
                            </div>
                            <div class="p-4">
                                <span class="text-cyan-300">Set-Cookie: </span><span class="text-emerald-300">sessionId=abc123xyz; </span><span class="text-orange-300">HttpOnly; </span><span class="text-blue-300">Secure; </span><span class="text-fuchsia-300">SameSite=Lax; </span><span class="text-yellow-300">Path=/; Max-Age=3600</span>
                            </div>
                        </div>

                        <div class="space-y-3 mb-8">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <div class="flex items-start gap-3">
                                    <span class="text-orange-300 font-mono text-xs font-bold px-2 py-1 bg-orange-500/10 rounded border border-orange-500/20 shrink-0 mt-0.5">HttpOnly</span>
                                    <div>
                                        <div class="text-sm font-medium text-primary-text mb-1">Prevents JavaScript access to the cookie</div>
                                        <p class="text-xs text-muted leading-relaxed">Without it: <code class="text-red-300">document.cookie</code> in any XSS payload reads your session token and sends it to the attacker's server. With HttpOnly, the browser never exposes the cookie to JS — XSS can execute but cannot steal the session.</p>
                                    </div>
                                </div>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <div class="flex items-start gap-3">
                                    <span class="text-blue-300 font-mono text-xs font-bold px-2 py-1 bg-blue-500/10 rounded border border-blue-500/20 shrink-0 mt-0.5">Secure</span>
                                    <div>
                                        <div class="text-sm font-medium text-primary-text mb-1">HTTPS-only transmission</div>
                                        <p class="text-xs text-muted leading-relaxed">Without it: if a user ever visits <code class="text-cyan-300">http://</code> (not https) — even if accidentally — the browser will include the session cookie in the plaintext request. A network observer (coffee shop MitM) can read it. With Secure, the cookie is never sent over HTTP.</p>
                                    </div>
                                </div>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <div class="flex items-start gap-3">
                                    <span class="text-fuchsia-300 font-mono text-xs font-bold px-2 py-1 bg-fuchsia-500/10 rounded border border-fuchsia-500/20 shrink-0 mt-0.5">SameSite</span>
                                    <div>
                                        <div class="text-sm font-medium text-primary-text mb-1">CSRF protection — controls cross-site sending</div>
                                        <div class="space-y-2 text-xs text-muted">
                                            <div><span class="text-emerald-300 font-semibold">Strict:</span> Cookie only sent on same-site requests. Breaks OAuth flows and any top-level navigation from external links.</div>
                                            <div><span class="text-orange-300 font-semibold">Lax (recommended):</span> Cookie sent on same-site requests AND top-level GET navigations from other sites. Blocks CSRF on POST/PUT/DELETE but allows normal link clicks to work.</div>
                                            <div><span class="text-red-300 font-semibold">None:</span> Cookie sent on ALL cross-origin requests (must pair with Secure). Required for third-party contexts like embedded iframes — but exposes CSRF risk if your endpoints are not CSRF-token protected.</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">CORS & Cookie Hardening</h3>

                        <!-- CORS Nginx Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-5">
                            <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-orange-500/30 tracking-widest uppercase">nginx CORS Config</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># Strict CORS — explicit origin allowlist, never wildcard</span>
<span class="text-cyan-300">map</span> $http_origin $cors_origin {"{"}<span class="text-muted">
    # Only allow known frontend origins</span>
    ~^https://(www\.)?myapp\.com$ $http_origin;
    default <span class="text-red-400">""</span>;
{"}"}

<span class="text-cyan-300">add_header</span> Access-Control-Allow-Origin $cors_origin always;
<span class="text-cyan-300">add_header</span> Access-Control-Allow-Methods <span class="text-emerald-300">"GET, POST, OPTIONS"</span> always;
<span class="text-cyan-300">add_header</span> Access-Control-Allow-Headers <span class="text-emerald-300">"Authorization, Content-Type"</span> always;
<span class="text-muted"># NEVER: Access-Control-Allow-Origin: *  with credentials</span></pre>
                            </div>
                        </div>

                        <!-- Cookie Hardening Examples -->
                        <div class="space-y-4 mb-6">
                            <h4 class="text-sm font-semibold text-primary-text">Cookie Hardening — Framework Examples</h4>
                            <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs">
                                <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-3 py-1 rounded-bl-lg text-xs font-bold border-b border-l border-orange-500/30 uppercase tracking-wider">Express.js (Node)</div>
                                <div class="p-4 pt-8 overflow-x-auto custom-scrollbar">
                                    <pre class="text-primary-text leading-relaxed">
app.use(session({"{"}<span class="text-muted">
    secret: process.env.SESSION_SECRET,</span>
    <span class="text-cyan-300">cookie: {"{"}</span>
        <span class="text-orange-300">httpOnly: true</span>,    <span class="text-muted">// block JS access</span>
        <span class="text-blue-300">secure: true</span>,       <span class="text-muted">// HTTPS only</span>
        <span class="text-fuchsia-300">sameSite: 'lax'</span>,   <span class="text-muted">// CSRF protection</span>
        maxAge: 3600000    <span class="text-muted">// 1 hour</span>
    {"}"}
{"}"}))</pre>
                                </div>
                            </div>

                            <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs">
                                <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-3 py-1 rounded-bl-lg text-xs font-bold border-b border-l border-orange-500/30 uppercase tracking-wider">Django (Python)</div>
                                <div class="p-4 pt-8 overflow-x-auto custom-scrollbar">
                                    <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># settings.py</span>
<span class="text-cyan-300">SESSION_COOKIE_HTTPONLY</span> = <span class="text-orange-300">True</span>
<span class="text-cyan-300">SESSION_COOKIE_SECURE</span>  = <span class="text-blue-300">True</span>
<span class="text-cyan-300">SESSION_COOKIE_SAMESITE</span> = <span class="text-fuchsia-300">'Lax'</span>
<span class="text-cyan-300">CSRF_COOKIE_SECURE</span>     = <span class="text-blue-300">True</span>
<span class="text-cyan-300">CSRF_COOKIE_HTTPONLY</span>   = <span class="text-orange-300">True</span></pre>
                                </div>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm flex items-center gap-2"><ShieldCheck size={14}/> Session Fixation: Regenerate After Login</h4>
                            <p class="text-xs text-muted leading-relaxed">Session fixation attacks trick a user into using an attacker-controlled session ID before login. After the user authenticates, the attacker's session ID is now authenticated. The fix: always call your framework's <code class="text-cyan-300">session.regenerate()</code> (Express) or <code class="text-cyan-300">request.session.cycle_key()</code> (Django) immediately after successful login. This issues a fresh session ID and invalidates the old one.</p>
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
