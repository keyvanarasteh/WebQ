<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Share2, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. OpenGraph', icon: Share2 }, { id: 1, label: '2. Twitter Cards', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Share2 size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_social_media_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_social_media_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">OpenGraph Protocol</h3>
                        <p class="text-sm text-muted mb-4">OpenGraph (OG) tags control how your pages appear when shared on Facebook, LinkedIn, Slack, and any other platform that renders link previews. Without them, platforms use arbitrary page content — often displaying ugly, irrelevant previews.</p>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-muted italic mb-2">// Essential OpenGraph tags</p>
                            <p class="text-primary-text">&lt;meta property="<span class="text-cyan-300">og:title</span>" content="Page Title" /&gt;</p>
                            <p class="text-primary-text">&lt;meta property="<span class="text-cyan-300">og:description</span>" content="Description for preview" /&gt;</p>
                            <p class="text-primary-text">&lt;meta property="<span class="text-cyan-300">og:image</span>" content="https://example.com/image.jpg" /&gt;</p>
                            <p class="text-primary-text">&lt;meta property="<span class="text-cyan-300">og:url</span>" content="https://example.com/page" /&gt;</p>
                            <p class="text-primary-text">&lt;meta property="<span class="text-cyan-300">og:type</span>" content="website" /&gt;</p>
                        </div>
                        <div class="space-y-2 text-sm text-muted">
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><code class="text-cyan-300">og:image</code> dimensions: 1200×630px is the Facebook/LinkedIn optimal size. Images smaller than 200×200 won't render as card previews.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><code class="text-cyan-300">og:title</code> and <code class="text-cyan-300">og:description</code> should be optimized for social sharing CTR, not just SEO.</span></div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Twitter / X Card Protocol</h3>
                        <p class="text-sm text-muted mb-4">Twitter (X) uses its own card system alongside OG tags. Without a <code class="text-cyan-300">twitter:card</code> tag, shared links appear as plain text — zero visual engagement.</p>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-primary-text">&lt;meta name="<span class="text-cyan-300">twitter:card</span>" content="summary_large_image" /&gt;</p>
                            <p class="text-primary-text">&lt;meta name="<span class="text-cyan-300">twitter:title</span>" content="Title" /&gt;</p>
                            <p class="text-primary-text">&lt;meta name="<span class="text-cyan-300">twitter:description</span>" content="Description" /&gt;</p>
                            <p class="text-primary-text">&lt;meta name="<span class="text-cyan-300">twitter:image</span>" content="https://…/image.jpg" /&gt;</p>
                        </div>
                        <div class="space-y-2 text-sm text-muted">
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><code class="text-cyan-300">summary_large_image</code>: Full-width hero image card. Best for content marketing and blog posts.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>If twitter: tags are missing, Twitter falls back to OG tags — so OG tags also cover Twitter's minimum requirements.</span></div>
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
