<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Smartphone, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Mobile Signals', icon: Smartphone }, { id: 1, label: '2. Accessibility', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Smartphone size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_mobile_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_mobile_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Mobile-First Indexing</h3>
                        <p class="text-sm text-muted mb-4">Since 2021, Google uses only the mobile version of pages for indexing and ranking. A desktop-only site is effectively invisible in most searches — over 60% of web traffic comes from mobile devices.</p>
                        <div class="space-y-3">
                            {#each [
                                { check: 'Viewport Meta Tag', required: true, desc: 'Without <meta name="viewport" content="width=device-width">, the page renders at desktop width on mobile, causing a terrible UX and immediate ranking penalty.' },
                                { check: 'Responsive Images', required: true, desc: 'Images should use srcset/sizes or max-width:100% CSS to scale with viewport. Fixed-width images break mobile layouts.' },
                                { check: 'Touch Target Sizes', required: false, desc: 'Google recommends at least 44×44px for interactive elements. Too-small buttons cause accidental taps and high bounce rates.' },
                                { check: 'No Horizontal Scroll', required: true, desc: 'Content wider than the viewport causes Google\'s mobile crawler to mark the page as not mobile-friendly.' },
                            ] as c}
                                <div class="flex items-start gap-3 p-3 bg-sunken border border-base rounded-xl">
                                    <span class="text-xs font-bold px-2 py-0.5 rounded shrink-0 {c.required ? 'bg-red-500/10 text-red-400 border border-red-500/30' : 'bg-yellow-500/10 text-yellow-400 border border-yellow-500/30'}">{c.required ? 'Required' : 'Advisory'}</span>
                                    <div>
                                        <p class="text-sm font-bold text-primary-text">{c.check}</p>
                                        <p class="text-xs text-muted mt-0.5">{c.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Web Accessibility (WCAG)</h3>
                        <p class="text-sm text-muted mb-4">Accessibility improves SEO (better crawlability), expands your audience, and is legally required in many jurisdictions (ADA, EAA). WebQ checks for structural accessibility indicators.</p>
                        <div class="space-y-3">
                            {#each [
                                { check: 'ARIA Labels', desc: 'aria-label and aria-labelledby attributes allow screen readers to describe interactive elements that lack visible text labels.' },
                                { check: 'Alt Text on Images', desc: 'Descriptive alt text is used by screen readers AND by Google Image Search for indexation. Missing alt text fails both accessibility and SEO.' },
                                { check: 'Form Labels', desc: '<label> elements associated with inputs are required for screen reader compatibility and improve form UX for all users.' },
                                { check: 'Heading Hierarchy', desc: 'Proper H1–H6 structure creates a logical document outline readable by screen readers and search engines alike.' },
                            ] as c}
                                <div class="p-3 bg-sunken border border-base rounded-xl">
                                    <p class="text-sm font-bold text-primary-text mb-1">{c.check}</p>
                                    <p class="text-xs text-muted">{c.desc}</p>
                                </div>
                            {/each}
                        </div>
                        <div class="mt-4 flex gap-2 text-sm text-muted"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>For a full accessibility audit, use axe DevTools or WAVE alongside WebQ's structural checks.</span></div>
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
