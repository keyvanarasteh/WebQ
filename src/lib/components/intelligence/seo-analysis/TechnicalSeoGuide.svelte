<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Settings, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Technical Checks', icon: Settings }, { id: 1, label: '2. Crawlability', icon: ArrowRight }];
    function next() { if (activeTab < tabs.length - 1) activeTab++; }
    function prev() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md" transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>
            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle" aria-label="Close"><X size={18}/></button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Settings size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_tech_seo_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_tech_seo_subtitle()}</p>
                        </div>
                    </div>
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button onclick={() => activeTab = tab.id} class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}">
                                <tab.icon size={14}/><span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>
            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Technical SEO Checklist</h3>
                        <p class="text-sm text-muted mb-5">Technical SEO ensures crawlers can access, understand, and index your pages. These checks are the infrastructure layer — without them, even perfect content won't rank.</p>
                        <div class="space-y-3">
                            {#each [
                                { check: 'HTTPS Active', importance: 'Ranking factor since 2014. Google treats HTTP sites as "Not Secure". Users see browser warnings.', status: 'Critical' },
                                { check: 'sitemap.xml Present', importance: 'Tells crawlers the full map of indexable URLs. Essential for large sites or sites with weak internal linking.', status: 'High' },
                                { check: 'robots.txt Present', importance: 'Controls crawler access to specific paths. Missing robots.txt means crawlers access everything — including /admin, /api, etc.', status: 'High' },
                                { check: 'Hreflang Tags', importance: 'For multilingual sites — tells Google which language version to serve to which region. Missing hreflang leads to wrong-language pages ranking in wrong countries.', status: 'Medium' },
                                { check: 'Breadcrumb Navigation', importance: 'Improves crawl path clarity and enables rich breadcrumb snippets in search results — increasing CTR.', status: 'Medium' },
                            ] as c}
                                <div class="flex items-start gap-3 p-3 bg-sunken border border-base rounded-xl">
                                    <span class="text-xs font-bold px-2 py-0.5 rounded {c.status === 'Critical' ? 'bg-red-500/10 text-red-400 border border-red-500/30' : c.status === 'High' ? 'bg-orange-500/10 text-orange-400 border border-orange-500/30' : 'bg-yellow-500/10 text-yellow-400 border border-yellow-500/30'} shrink-0">{c.status}</span>
                                    <div>
                                        <p class="text-sm font-bold text-primary-text">{c.check}</p>
                                        <p class="text-xs text-muted mt-0.5">{c.importance}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Crawler Directives & robots.txt</h3>
                        <p class="text-sm text-muted mb-5">robots.txt controls which paths bots can crawl. Misconfigured rules can accidentally block indexation of your entire site.</p>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-muted italic mb-2">// Good robots.txt</p>
                            <p class="text-primary-text">User-agent: *</p>
                            <p class="text-primary-text">Disallow: /admin/</p>
                            <p class="text-primary-text">Disallow: /api/</p>
                            <p class="text-primary-text">Allow: /</p>
                            <p class="text-primary-text mt-2">Sitemap: https://example.com/sitemap.xml</p>
                            <br/>
                            <p class="text-red-400 italic">// DANGER: blocks entire site</p>
                            <p class="text-red-300">User-agent: Googlebot</p>
                            <p class="text-red-300">Disallow: /  ← de-indexes everything!</p>
                        </div>
                        <div class="space-y-2 text-sm text-muted">
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>Always include the <code class="text-cyan-300">Sitemap:</code> directive at the end of robots.txt pointing to your sitemap URL.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>Disallow paths that should never be indexed: admin panels, API endpoints, staging dirs, session-specific URLs.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>Test your robots.txt in Google Search Console's robots.txt tester before deploying changes.</span></div>
                        </div>
                    </div>
                {/if}
            </div>
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded mx-1 border border-glass font-mono">Esc</kbd> to dismiss</div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}<button onclick={prev} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle transition-all"><ChevronLeft size={16}/> Previous</button>{:else}<div></div>{/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={next} class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
