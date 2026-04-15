<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, FileSearch, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Critical Files', icon: FileSearch }, { id: 1, label: '2. Security Notes', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><FileSearch size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_seo_resources_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_seo_resources_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">SEO-Critical File Reference</h3>
                        <div class="space-y-4">
                            <div class="p-4 bg-sunken border border-base rounded-xl border-l-2 border-l-cyan-500">
                                <code class="text-cyan-300 text-sm font-mono font-bold">/robots.txt</code>
                                <p class="text-xs text-muted mt-2">Crawler access control file. Tells search engines which paths to index or ignore. Also serves as an attack surface map — revealing admin paths, API routes, and hidden directories to anyone who reads it.</p>
                            </div>
                            <div class="p-4 bg-sunken border border-base rounded-xl border-l-2 border-l-emerald-500">
                                <code class="text-emerald-300 text-sm font-mono font-bold">/sitemap.xml</code>
                                <p class="text-xs text-muted mt-2">XML file listing all indexable URLs with optional metadata (last modified, change frequency, priority). Essential for large sites. Submitted to Google Search Console for faster discovery of new pages.</p>
                            </div>
                            <div class="p-4 bg-sunken border border-base rounded-xl border-l-2 border-l-purple-500">
                                <code class="text-purple-300 text-sm font-mono font-bold">/humans.txt</code>
                                <p class="text-xs text-muted mt-2">Optional team and technology attribution file. Reveals dev stack, team members, and CMS. From a security perspective — useful OSINT source for social engineering targets.</p>
                            </div>
                            <div class="p-4 bg-sunken border border-base rounded-xl border-l-2 border-l-orange-500">
                                <code class="text-orange-300 text-sm font-mono font-bold">/ads.txt</code>
                                <p class="text-xs text-muted mt-2">Authorized Digital Sellers file. Lists which ad platforms are permitted to sell the site's ad inventory. Absence exposes the site to domain spoofing in programmatic advertising.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-400 mb-4">Security Implications of robots.txt</h3>
                        <p class="text-sm text-muted mb-4">robots.txt is public and is always checked by security researchers and attackers during initial reconnaissance. Disallow rules reveal directory structure even when trying to hide it.</p>
                        <div class="bg-[#0d1117] rounded-xl border border-rose-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-rose-400 italic mb-2">// What attackers learn from your robots.txt</p>
                            <p class="text-primary-text">Disallow: <span class="text-red-400">/admin/</span>          ← admin panel confirmed</p>
                            <p class="text-primary-text">Disallow: <span class="text-red-400">/wp-admin/</span>       ← WordPress site confirmed</p>
                            <p class="text-primary-text">Disallow: <span class="text-red-400">/api/v1/internal/</span> ← internal API path exposed</p>
                            <p class="text-primary-text">Disallow: <span class="text-yellow-400">/staging/</span>        ← staging env disclosed</p>
                        </div>
                        <div class="flex gap-2 text-sm text-muted"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>robots.txt hides paths from crawlers but NOT from attackers. Use proper authentication and server-side access controls — never rely on robots.txt for security.</span></div>
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
