<script lang="ts">
    import { X, FileText, Bot, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft, Globe } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. robots.txt", icon: FileText },
        { id: 1, label: "2. Bot Classification", icon: Bot },
        { id: 2, label: "3. Enforcement", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-cyan-950/40 via-cyan-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-cyan-600/10 flex items-center justify-center border border-cyan-500/30 shadow-inner">
                            <Globe size={28} class="text-accent" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Bot & Crawler Directives</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Access control rules governing automated agents and web crawlers.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">The robots.txt Protocol</h3>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-sm text-primary-text leading-relaxed">
                                    <code class="text-cyan-300 bg-glass px-1.5 py-0.5 rounded text-xs">robots.txt</code> is a plain-text file served at the root of a domain that declares crawling rules to automated user agents. It is a <strong>voluntary protocol</strong> — compliant bots like Googlebot and Bingbot respect it, but malicious crawlers and vulnerability scanners routinely ignore it.
                                </p>
                                <div class="bg-cyan-950/20 border border-cyan-500/20 rounded-xl p-4">
                                    <h4 class="text-cyan-400 font-medium mb-3 flex items-center gap-2"><FileText size={14}/> Key Directives</h4>
                                    <div class="space-y-2 text-xs text-muted">
                                        <div class="flex items-start gap-2">
                                            <ArrowRight size={12} class="text-cyan-500 mt-0.5 shrink-0" />
                                            <span><code class="text-cyan-300 bg-glass px-1 rounded">User-agent: *</code> — applies rule to all bots; use a specific name like <code class="text-cyan-300 bg-glass px-1 rounded">Googlebot</code> to target one crawler</span>
                                        </div>
                                        <div class="flex items-start gap-2">
                                            <ArrowRight size={12} class="text-cyan-500 mt-0.5 shrink-0" />
                                            <span><code class="text-cyan-300 bg-glass px-1 rounded">Disallow: /admin/</code> — instructs compliant bots to skip paths under <code class="text-cyan-300 bg-glass px-1 rounded">/admin/</code></span>
                                        </div>
                                        <div class="flex items-start gap-2">
                                            <ArrowRight size={12} class="text-cyan-500 mt-0.5 shrink-0" />
                                            <span><code class="text-cyan-300 bg-glass px-1 rounded">Allow: /public/</code> — explicitly permits a path, overriding a broader Disallow rule</span>
                                        </div>
                                        <div class="flex items-start gap-2">
                                            <ArrowRight size={12} class="text-cyan-500 mt-0.5 shrink-0" />
                                            <span><code class="text-cyan-300 bg-glass px-1 rounded">Crawl-delay: 10</code> — asks bots to wait 10 seconds between requests (not universally supported)</span>
                                        </div>
                                        <div class="flex items-start gap-2">
                                            <ArrowRight size={12} class="text-cyan-500 mt-0.5 shrink-0" />
                                            <span><code class="text-cyan-300 bg-glass px-1 rounded">Sitemap: https://...</code> — points crawlers to the XML sitemap for structured discovery</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="bg-red-950/20 border border-red-500/20 rounded-xl p-4 text-xs">
                                    <p class="text-red-300 font-semibold mb-1">Security Warning</p>
                                    <p class="text-muted leading-relaxed">Do <strong>not</strong> rely on <code class="text-red-300 bg-glass px-1 rounded">Disallow</code> to protect sensitive pages. The robots.txt file is itself publicly readable — listing <code class="text-red-300 bg-glass px-1 rounded">Disallow: /internal-api/</code> effectively tells attackers where your hidden endpoints are. Use proper authentication instead.</p>
                                </div>
                            </div>

                            <!-- robots.txt code example -->
                            <div class="relative bg-[#0d1117] rounded-xl border border-cyan-500/20 overflow-hidden font-mono text-xs">
                                <div class="absolute top-0 right-0 bg-cyan-500/10 text-cyan-400 px-4 py-1.5 rounded-bl-xl text-[10px] font-bold border-b border-l border-cyan-500/30 tracking-widest uppercase">robots.txt example</div>
                                <div class="p-5 pt-10 leading-relaxed">
                                    <pre class="text-primary-text">
<span class="text-muted"># Allow all major search engines</span>
<span class="text-cyan-300">User-agent: Googlebot</span>
<span class="text-emerald-300">Allow: /</span>

<span class="text-cyan-300">User-agent: Bingbot</span>
<span class="text-emerald-300">Allow: /</span>

<span class="text-muted"># Block AI training crawlers</span>
<span class="text-cyan-300">User-agent: GPTBot</span>
<span class="text-red-400">Disallow: /</span>

<span class="text-cyan-300">User-agent: CCBot</span>
<span class="text-red-400">Disallow: /</span>

<span class="text-muted"># Block all others from admin paths</span>
<span class="text-cyan-300">User-agent: *</span>
<span class="text-red-400">Disallow: /admin/</span>
<span class="text-red-400">Disallow: /api/internal/</span>
<span class="text-red-400">Disallow: /wp-admin/</span>
<span class="text-emerald-300">Allow: /api/public/</span>

<span class="text-muted"># Crawl rate limit</span>
<span class="text-cyan-300">Crawl-delay: 5</span>

<span class="text-muted"># Sitemap reference</span>
<span class="text-cyan-300">Sitemap: https://target.com/sitemap.xml</span></pre>
                                </div>
                            </div>
                        </div>
                    </div>

                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Bot Classification</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed max-w-3xl">
                            The directives panel shows each bot found in the target's robots.txt alongside its real-world classification and associated risk. Not all bots are equal — blocking Googlebot harms your SEO; blocking vulnerability scanners is essential.
                        </p>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
                            <!-- Search engines -->
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-4 hover:border-emerald-500/40 transition-colors">
                                <div class="flex items-center gap-3 mb-3">
                                    <div class="bg-emerald-500/10 p-2 rounded-lg">
                                        <Bot size={16} class="text-emerald-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-emerald-300">Search Engine Crawlers</h4>
                                        <span class="text-[10px] text-emerald-400 font-mono">Allowed — high value</span>
                                    </div>
                                </div>
                                <p class="text-xs text-muted leading-relaxed mb-2">Googlebot, Bingbot, DuckDuckBot, Yandex. These respect robots.txt strictly and drive organic traffic. Blocking them destroys search visibility.</p>
                                <div class="flex flex-wrap gap-1">
                                    {#each ['Googlebot', 'Bingbot', 'DuckDuckBot', 'Yandexbot'] as bot}
                                        <span class="text-[10px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded font-mono">{bot}</span>
                                    {/each}
                                </div>
                            </div>

                            <!-- SEO tools -->
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-4 hover:border-blue-500/40 transition-colors">
                                <div class="flex items-center gap-3 mb-3">
                                    <div class="bg-blue-500/10 p-2 rounded-lg">
                                        <Bot size={16} class="text-blue-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-blue-300">SEO / Analytics Bots</h4>
                                        <span class="text-[10px] text-blue-400 font-mono">Usually allowed — low risk</span>
                                    </div>
                                </div>
                                <p class="text-xs text-muted leading-relaxed mb-2">SemrushBot, AhrefsBot, MJ12bot. Marketing tools used for backlink analysis. Typically permitted but can be rate-limited if they cause server load.</p>
                                <div class="flex flex-wrap gap-1">
                                    {#each ['SemrushBot', 'AhrefsBot', 'MJ12bot'] as bot}
                                        <span class="text-[10px] bg-blue-500/10 text-blue-400 border border-blue-500/20 px-2 py-0.5 rounded font-mono">{bot}</span>
                                    {/each}
                                </div>
                            </div>

                            <!-- Security scanners -->
                            <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-4 hover:border-red-500/40 transition-colors">
                                <div class="flex items-center gap-3 mb-3">
                                    <div class="bg-red-500/10 p-2 rounded-lg">
                                        <Bot size={16} class="text-red-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-red-300">Security Scanners</h4>
                                        <span class="text-[10px] text-red-400 font-mono">Block — high risk</span>
                                    </div>
                                </div>
                                <p class="text-xs text-muted leading-relaxed mb-2">Nikto, sqlmap, Nessus, OpenVAS probes. These actively probe for vulnerabilities. Any robots.txt entries for these suggest prior unwanted scanning activity.</p>
                                <div class="flex flex-wrap gap-1">
                                    {#each ['Nikto', 'sqlmap', 'Nessus'] as bot}
                                        <span class="text-[10px] bg-red-500/10 text-red-400 border border-red-500/20 px-2 py-0.5 rounded font-mono">{bot}</span>
                                    {/each}
                                </div>
                            </div>

                            <!-- AI bots -->
                            <div class="bg-yellow-950/10 border border-yellow-500/20 rounded-xl p-4 hover:border-yellow-500/40 transition-colors">
                                <div class="flex items-center gap-3 mb-3">
                                    <div class="bg-yellow-500/10 p-2 rounded-lg">
                                        <Bot size={16} class="text-yellow-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-yellow-300">AI Training Bots</h4>
                                        <span class="text-[10px] text-yellow-400 font-mono">Increasingly blocked</span>
                                    </div>
                                </div>
                                <p class="text-xs text-muted leading-relaxed mb-2">GPTBot (OpenAI), CCBot (Common Crawl), Claude-Web, Google-Extended. Scrape content for AI training datasets. Blocked by a growing majority of content publishers and news sites.</p>
                                <div class="flex flex-wrap gap-1">
                                    {#each ['GPTBot', 'CCBot', 'Claude-Web', 'Google-Extended'] as bot}
                                        <span class="text-[10px] bg-yellow-500/10 text-yellow-400 border border-yellow-500/20 px-2 py-0.5 rounded font-mono">{bot}</span>
                                    {/each}
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4 text-xs text-muted leading-relaxed">
                            <ArrowRight size={12} class="text-cyan-400 inline mr-1" />
                            A robot showing as "Blocked" means the target's robots.txt has a <code class="text-cyan-300 bg-glass px-1 rounded">Disallow: /</code> for that User-agent. "Allowed" means it has <code class="text-cyan-300 bg-glass px-1 rounded">Allow: /</code> or no rule (default is allow). Remember: these are voluntary — use WAF rules for real enforcement.
                        </div>
                    </div>

                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Real Enforcement Strategy</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed max-w-3xl">
                            robots.txt is advisory only. Malicious bots and vulnerability scanners actively ignore it. For real protection you need layered enforcement: WAF rules, rate limiting, and IP reputation blocking.
                        </p>

                        <div class="space-y-4 mb-6">
                            <div class="bg-glass border border-subtle rounded-xl p-5 flex gap-4 hover:border-cyan-500/30 transition-colors">
                                <div class="bg-cyan-500/10 p-2.5 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={20} class="text-cyan-400" />
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-cyan-300 mb-2">Cloudflare Bot Management Rules</h4>
                                    <p class="text-xs text-muted leading-relaxed mb-3">Use Cloudflare's WAF to create rules that challenge or block based on the verified bot score. Cloudflare classifies traffic into "definitely automated", "likely automated", and "likely human" using browser fingerprinting and behavioural signals.</p>
                                    <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs">
                                        <span class="text-muted">// Cloudflare Rule (expression syntax)</span><br />
                                        <span class="text-cyan-300">(cf.bot_management.score lt 30)</span><br />
                                        <span class="text-muted">Action: </span><span class="text-red-400">Block</span><br /><br />
                                        <span class="text-cyan-300">(cf.bot_management.verified_bot eq false and cf.bot_management.score lt 50)</span><br />
                                        <span class="text-muted">Action: </span><span class="text-yellow-400">Managed Challenge (CAPTCHA)</span>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-glass border border-subtle rounded-xl p-5 flex gap-4 hover:border-cyan-500/30 transition-colors">
                                <div class="bg-orange-500/10 p-2.5 rounded-lg h-fit shrink-0">
                                    <Bot size={20} class="text-orange-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-orange-300 mb-2">Rate Limiting as a Complement</h4>
                                    <p class="text-xs text-muted leading-relaxed">Even legitimate crawlers should not send hundreds of requests per second. Set a rate limit at your WAF: e.g., max 60 requests/minute per IP for non-authenticated paths. Exceeding this triggers a 429 response or a JS challenge. This slows scraping without blocking legitimate users.</p>
                                </div>
                            </div>

                            <div class="bg-glass border border-subtle rounded-xl p-5 flex gap-4 hover:border-cyan-500/30 transition-colors">
                                <div class="bg-purple-500/10 p-2.5 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={20} class="text-purple-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-purple-300 mb-2">User-Agent Blocking vs IP Reputation</h4>
                                    <p class="text-xs text-muted leading-relaxed">User-Agent header blocking is trivially bypassed — any bot can spoof <code class="text-purple-300 bg-glass px-1 rounded">Googlebot</code>. IP reputation databases (Cloudflare's built-in lists, IPQualityScore, Spamhaus) are far more effective because they track the actual source network and its historical behaviour, regardless of what User-Agent is presented.</p>
                                </div>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-cyan-950/10 border border-cyan-500/20 rounded-xl p-4 text-xs">
                                <h4 class="font-semibold text-cyan-300 mb-2">User-Agent Blocking</h4>
                                <div class="space-y-1 text-muted">
                                    <p class="flex items-center gap-1"><span class="text-emerald-400">+</span> Easy to configure</p>
                                    <p class="flex items-center gap-1"><span class="text-emerald-400">+</span> Stops lazy scrapers</p>
                                    <p class="flex items-center gap-1"><span class="text-red-400">-</span> Trivially bypassed (header spoofing)</p>
                                    <p class="flex items-center gap-1"><span class="text-red-400">-</span> Blocks real Googlebot if misconfigured</p>
                                </div>
                            </div>
                            <div class="bg-cyan-950/10 border border-cyan-500/20 rounded-xl p-4 text-xs">
                                <h4 class="font-semibold text-cyan-300 mb-2">IP Reputation Blocking</h4>
                                <div class="space-y-1 text-muted">
                                    <p class="flex items-center gap-1"><span class="text-emerald-400">+</span> Cannot be spoofed by attacker</p>
                                    <p class="flex items-center gap-1"><span class="text-emerald-400">+</span> Blocks entire malicious ASNs</p>
                                    <p class="flex items-center gap-1"><span class="text-red-400">-</span> Requires subscription to threat-intel feed</p>
                                    <p class="flex items-center gap-1"><span class="text-red-400">-</span> May block shared hosting IPs</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-cyan-500/50 outline-none"
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
