<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Award, BarChart2, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. Score Breakdown', icon: Award },
        { id: 1, label: '2. Grade Scale', icon: BarChart2 },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>

            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30" aria-label="Close"><X size={18}/></button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30">
                            <Award size={28} class="text-accent"/>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_seo_score_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_seo_score_subtitle()}</p>
                        </div>
                    </div>
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}">
                                <tab.icon size={14}/><span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">How the SEO Score is Computed</h3>
                        <p class="text-sm text-muted mb-5">WebQ's SEO score aggregates signals across 13 technical and content categories. Each category contributes weighted points to a total score out of 100.</p>
                        <div class="space-y-2">
                            {#each [
                                { cat: 'Basic Meta Tags', weight: '20', desc: 'Title tag, meta description, canonical, robots, viewport' },
                                { cat: 'Technical SEO', weight: '15', desc: 'Sitemap, robots.txt, HTTPS, breadcrumbs, hreflang' },
                                { cat: 'Content Quality', weight: '15', desc: 'H1 tags, word count, heading structure, keyword density' },
                                { cat: 'Performance', weight: '12', desc: 'Load speed, minification, compression, caching headers' },
                                { cat: 'Social Signals', weight: '10', desc: 'OpenGraph tags, Twitter Card completeness' },
                                { cat: 'Link Architecture', weight: '10', desc: 'Internal linking depth, broken links, nofollow balance' },
                                { cat: 'Image SEO', weight: '8', desc: 'Alt text coverage, lazy loading, responsive images' },
                                { cat: 'Mobile & Accessibility', weight: '5', desc: 'Viewport, touch targets, ARIA labels, contrast' },
                                { cat: 'Schema Markup', weight: '5', desc: 'JSON-LD structured data, rich result eligibility' },
                            ] as c}
                                <div class="flex items-center gap-3 p-2.5 bg-sunken rounded-lg border border-base">
                                    <span class="font-mono text-sm font-bold text-cyan-400 shrink-0 w-8 text-right">{c.weight}</span>
                                    <div>
                                        <p class="text-sm font-medium text-primary-text">{c.cat}</p>
                                        <p class="text-xs text-muted">{c.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Grade Scale Reference</h3>
                        <div class="space-y-3 mb-6">
                            {#each [
                                { grade: 'A', range: '90–100', color: 'text-green-400 border-green-500', bg: 'bg-green-950/10 border-green-500/20', desc: 'Excellent. All critical SEO signals are present and optimized. Ready for competitive indexing.' },
                                { grade: 'B', range: '75–89', color: 'text-cyan-400 border-cyan-500', bg: 'bg-cyan-950/10 border-cyan-500/20', desc: 'Good. Minor gaps in coverage. Low-hanging improvements available in meta tags or performance.' },
                                { grade: 'C', range: '60–74', color: 'text-yellow-400 border-yellow-500', bg: 'bg-yellow-950/10 border-yellow-500/20', desc: 'Average. Likely missing critical elements like structured data, social tags, or sitemap.' },
                                { grade: 'D', range: '40–59', color: 'text-orange-400 border-orange-500', bg: 'bg-orange-950/10 border-orange-500/20', desc: 'Poor. Multiple major issues affecting crawlability and indexation rankings.' },
                                { grade: 'F', range: '0–39', color: 'text-red-400 border-red-500', bg: 'bg-red-950/10 border-red-500/20', desc: 'Critical. Fundamental SEO infrastructure is missing. Crawler may not index the site.' },
                            ] as g}
                                <div class="{g.bg} border rounded-xl p-4 flex items-start gap-4">
                                    <span class="text-2xl font-black {g.color} border-2 rounded-xl w-12 h-12 flex items-center justify-center shrink-0">{g.grade}</span>
                                    <div>
                                        <p class="text-sm font-bold text-primary-text">{g.range} points</p>
                                        <p class="text-xs text-muted mt-1">{g.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}
            </div>

            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded mx-1 border border-glass font-mono">Esc</kbd> or click outside to dismiss</div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button onclick={prevTab} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle transition-all"><ChevronLeft size={16}/> Previous</button>
                    {:else}<div></div>{/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={nextTab} class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
