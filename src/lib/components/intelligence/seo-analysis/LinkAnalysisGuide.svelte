<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Link, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Link Types', icon: Link }, { id: 1, label: '2. Strategy', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Link size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_links_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_links_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Link Architecture Analysis</h3>
                        <p class="text-sm text-muted mb-4">WebQ counts and categorizes all anchor tags on the scanned page — internal links, external links, and nofollow markers. Links are the primary mechanism for distributing PageRank across a site.</p>
                        <div class="space-y-3">
                            {#each [
                                { type: 'Internal Links', color: 'border-t-cyan-500', desc: 'Links pointing to other pages on the same domain. Internal links distribute "link equity" (PageRank) and help crawlers discover all pages. Pages with zero internal links are called "orphan pages" — they may never be discovered.' },
                                { type: 'External Links', color: 'border-t-purple-500', desc: 'Links pointing to other domains. Relevant external links signal to Google that the content is well-researched and connected to authoritative sources. Too many external links can dilute the page\'s own authority.' },
                                { type: 'Nofollow Links', color: 'border-t-orange-500', desc: 'Links with rel="nofollow" or rel="sponsored". These do not pass PageRank. Use for paid links, user-generated content (UGC), and untrusted URLs to comply with Google\'s link scheme policies.' },
                            ] as t}
                                <div class="p-4 bg-sunken border border-base rounded-xl border-t-2 {t.color}">
                                    <p class="text-sm font-bold text-primary-text mb-1">{t.type}</p>
                                    <p class="text-xs text-muted leading-relaxed">{t.desc}</p>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Link Strategy Best Practices</h3>
                        <div class="space-y-2 mb-5">
                            {#each [
                                { tip: 'Use descriptive anchor text', detail: 'Anchor text is a major relevance signal. "Click here" links waste the opportunity. "Best SEO practices guide" tells Google exactly what the linked page is about.' },
                                { tip: 'Avoid orphan pages', detail: 'Every important page should have at least 2–3 internal links pointing to it. Run a crawl (Screaming Frog, Sitebulb) to find orphaned pages.' },
                                { tip: 'Link to high-authority external sources', detail: 'Citing authoritative sources (Wikipedia, gov sites, research papers) adds credibility signals — Google correlates linked-to authority with content quality.' },
                                { tip: 'Keep link depth shallow', detail: 'Important pages should be reachable within 3 clicks from the homepage. Deep pages (click depth > 5) receive less crawl budget and rank weaker.' },
                            ] as t, i}
                                <div class="flex gap-3 p-3 bg-sunken border border-base rounded-xl">
                                    <span class="text-xs font-mono font-bold text-cyan-400 shrink-0 w-5 mt-0.5">{i + 1}.</span>
                                    <div>
                                        <p class="text-sm font-medium text-primary-text">{t.tip}</p>
                                        <p class="text-xs text-muted mt-0.5">{t.detail}</p>
                                    </div>
                                </div>
                            {/each}
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
