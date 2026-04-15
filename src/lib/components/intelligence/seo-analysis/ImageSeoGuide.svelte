<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Image, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Image Signals', icon: Image }, { id: 1, label: '2. Optimization', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Image size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_images_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_images_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Image SEO Metrics</h3>
                        <p class="text-sm text-muted mb-4">WebQ parses all <code class="text-cyan-300">&lt;img&gt;</code> tags and evaluates their SEO and performance attributes. Images are often the largest assets on a page and the primary LCP element.</p>
                        <div class="space-y-3">
                            {#each [
                                { metric: 'Alt Text Coverage', unit: '% of images', desc: 'Alt text serves dual purpose: screen reader accessibility AND Google Image Search indexation. Every non-decorative image needs descriptive, keyword-rich alt text.' },
                                { metric: 'Images With Title', unit: 'count', desc: 'The title attribute adds tooltip text on hover. Less critical than alt, but contributes to accessibility compliance.' },
                                { metric: 'Lazy Loading', unit: 'loading="lazy"', desc: 'Lazy loading defers off-screen images until they\'re needed. Reduces initial page load time and improves LCP score — Google recommends it for below-the-fold images.' },
                                { metric: 'Next-Gen Formats', unit: 'WebP / AVIF', desc: 'WebP images are 25–35% smaller than JPEG at the same quality. AVIF is even more efficient. Serving modern formats significantly reduces bandwidth and improves load speed.' },
                            ] as s}
                                <div class="p-3 bg-sunken border border-base rounded-xl">
                                    <div class="flex items-center justify-between mb-1">
                                        <p class="text-sm font-bold text-primary-text">{s.metric}</p>
                                        <code class="text-xs text-cyan-300 font-mono">{s.unit}</code>
                                    </div>
                                    <p class="text-xs text-muted leading-relaxed">{s.desc}</p>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Image Optimization Checklist</h3>
                        <div class="space-y-2 mb-4">
                            {#each [
                                { tip: 'Add descriptive alt text to all content images', detail: 'Format: "[subject] doing [action] in [context]". Avoid keyword stuffing — write for humans, not bots.' },
                                { tip: 'Specify width and height on all images', detail: 'Without dimensions, the browser cannot reserve layout space — causing Cumulative Layout Shift (CLS) as images load.' },
                                { tip: 'Convert to WebP or AVIF', detail: 'Use tools like Squoosh, Sharp, or build-time converters. Modern CDNs (Cloudflare, Cloudinary) auto-convert on delivery.' },
                                { tip: 'Add loading="lazy" to off-screen images', detail: 'The LCP image (above the fold hero) should NOT be lazy loaded — this would delay the LCP metric.' },
                                { tip: 'Use descriptive filenames', detail: '"product-blue-sneakers-nike.jpg" outranks "IMG_20241015_03.jpg" for image search.' },
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
