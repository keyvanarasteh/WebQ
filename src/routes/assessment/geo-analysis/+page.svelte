<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import * as m from '$lib/paraglide/messages';
    import { Search, Map as MapIcon, Compass, BrainCircuit, Globe2, BookOpen, Fingerprint, ShieldAlert } from 'lucide-svelte';
    import { slide, fade } from 'svelte/transition';
    import ApiBotDirectives from '$lib/components/assessment/geo-analysis/ApiBotDirectives.svelte';
    import GeofencingGuide from '$lib/components/assessment/geo-analysis/GeofencingGuide.svelte';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';
    import type { ScanProgressEvent } from '$lib/types/intelligence';

    let domain = $state('');
    let isScanning = $state(false);
    let scanResult = $state<any>(null);
    let error = $state<string | null>(null);
    let showGuide = $state(false);

    let logs = $state<ScanProgressEvent[]>([]);
    let progressPercent = $state(0);

    $effect(() => {
        let unlistenP: UnlistenFn | null = null;
        listen<ScanProgressEvent>('scan-progress', (event) => {
            logs.push(event.payload);
            progressPercent = event.payload.percentage;
        }).then(u => unlistenP = u);
        return () => { if (unlistenP) unlistenP(); };
    });

    async function startScan() {
        if (!domain) {
            error = "Please enter a target domain (ex: github.com)";
            return;
        }

        isScanning = true;
        error = null;
        scanResult = null;
        logs = [];
        progressPercent = 0;

        try {
            const res = await invoke('scan_geo_analysis', { domain });
            scanResult = res as any;
        } catch (e: any) {
            error = e.toString();
        } finally {
            isScanning = false;
        }
    }

    function getGradeColor(grade: string) {
        if (grade.startsWith('A')) return 'text-red-400 bg-red-500/10 border-red-500/20'; // A means completely open to AI Scraping
        if (grade.startsWith('B')) return 'text-orange-400 bg-orange-500/10 border-orange-500/20';
        if (grade.startsWith('C')) return 'text-yellow-400 bg-yellow-500/10 border-yellow-500/20';
        return 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20'; // F means locked down (Good for privacy)
    }
</script>

<div class="h-full flex flex-col pt-8 px-6 lg:px-10 pb-10 max-w-[1600px] mx-auto w-full custom-scrollbar overflow-y-auto">
    <!-- Header Area -->
    <div class="mb-8 flex items-center justify-between">
        <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-violet-500/10 flex items-center justify-center border border-violet-500/20">
                <Globe2 class="text-violet-400" size={20} />
            </div>
            <div>
                <h1 class="text-2xl font-semibold text-primary-text tracking-tight">{m.sec_geo_title()}</h1>
                <p class="text-sm text-muted mt-1">{m.sec_geo_desc()}</p>
            </div>
        </div>
        <button 
            onclick={() => showGuide = true}
            class="flex items-center gap-2 px-4 py-2 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 text-sm font-medium rounded-lg border border-blue-500/20 transition-colors"
        >
            <BookOpen size={16} />
            AI Architectures
        </button>
    </div>

    <!-- Main Scanner Input -->
    <div class="bg-glass border border-subtle rounded-2xl p-6 backdrop-blur-xl mb-6 shadow-lg">
        <div class="flex flex-col md:flex-row gap-4">
            <div class="relative flex-1">
                <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                    <Search class="text-muted" size={18} />
                </div>
                <input 
                    type="text" 
                    bind:value={domain}
                    placeholder="https://example.com"
                    class="w-full bg-[#0a0a0a] border border-glass text-primary-text text-sm rounded-xl focus:ring-1 focus:ring-violet-500 focus:border-violet-500 block pl-11 pr-4 py-3.5 transition-all outline-none shadow-inner"
                    disabled={isScanning}
                    onkeydown={(e) => e.key === 'Enter' && !isScanning && startScan()}
                />
            </div>
            <button 
                onclick={startScan}
                disabled={isScanning}
                class="bg-violet-600 hover:bg-violet-500 disabled:bg-surface disabled:text-muted text-primary-text px-8 py-3.5 rounded-xl font-medium transition-colors flex items-center justify-center gap-2 shadow-lg shadow-violet-900/20 disabled:shadow-none min-w-[200px]"
            >
                {#if isScanning}
                    <div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
                    Gathering Context
                {:else}
                    Probe Directives
                {/if}
            </button>
        </div>

        {#if error}
            <div class="mt-4 p-4 bg-red-500/10 border border-red-500/20 rounded-xl flex items-start gap-3" transition:slide>
                <ShieldAlert class="text-red-400 mt-0.5 shrink-0" size={18} />
                <div class="text-sm text-red-200/80 leading-relaxed font-mono break-all">
                    {error}
                </div>
            </div>
        {/if}
    </div>

    <!-- Scanning Terminal output -->
    {#if isScanning}
        <div class="mb-6" transition:fade>
            <ScanTerminal {logs} {progressPercent} />
        </div>
    {/if}

    <!-- Results Structure -->
    {#if scanResult && !isScanning}
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6" transition:fade>
            
            <!-- Context Indicators column -->
            <div class="lg:col-span-4 flex flex-col gap-6">
                <!-- Massive Score Card -->
                <div class="bg-glass border border-subtle rounded-xl p-6 relative overflow-hidden backdrop-blur-md flex flex-col items-center justify-center text-center">
                    <div class="absolute -right-10 -bottom-10 opacity-5 pointer-events-none">
                        <BrainCircuit size={150} />
                    </div>
                    
                    <h3 class="text-muted text-sm uppercase tracking-wider font-medium mb-4">{m.sec_geo_readiness_score()}</h3>
                    
                    <div class="text-7xl font-light mb-2 text-primary-text">
                        {scanResult.geo_score}<span class="text-3xl text-muted">/100</span>
                    </div>

                    <div class={`mt-4 px-4 py-1.5 rounded-full border text-sm font-bold tracking-wider ${getGradeColor(scanResult.geo_grade)}`}>
                        {scanResult.geo_grade}
                    </div>
                    
                    <p class="text-xs text-muted mt-4 px-4 leading-relaxed">
                        A higher score indicates the site actively provides structural LLM sets and permits AI Crawlers (Dangerous for privacy / compute offload).
                    </p>
                </div>

                <!-- LLMS.txt status -->
                <div class="bg-glass border border-subtle rounded-xl p-5 relative overflow-hidden">
                    <div class="flex items-center justify-between mb-4">
                        <h4 class="text-primary-text font-medium flex items-center gap-2">
                            <BookOpen size={16} class="text-rose-400" />
                            {m.sec_geo_llms()}
                        </h4>
                    </div>
                    <div class="text-2xl font-light text-primary-text mb-1 tracking-tight">
                        {scanResult.llms_txt.files?.length || 0}
                    </div>
                    <div class="text-xs text-muted uppercase tracking-wider mb-4">{m.sec_geo_llms_found()}</div>
                    
                    {#if scanResult.llms_txt.files && scanResult.llms_txt.files.length > 0}
                        <div class="space-y-2 mt-2">
                            {#each scanResult.llms_txt.files as file}
                                <div class="px-3 py-2 bg-glass rounded-lg text-xs font-mono text-primary-text break-all border border-subtle">
                                    {file}
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <!-- WebMCP status -->
                <div class="bg-glass border border-subtle rounded-xl p-5 relative overflow-hidden">
                    <div class="flex items-center justify-between mb-4">
                        <h4 class="text-primary-text font-medium flex items-center gap-2">
                            <Fingerprint size={16} class="text-teal-400" />
                            {m.sec_geo_webmcp()}
                        </h4>
                    </div>
                    
                    <div class="text-2xl font-light text-primary-text mb-1 tracking-tight">
                        {scanResult.webmcp.endpoints?.length + scanResult.webmcp.html_features?.length || 0}
                    </div>
                    <div class="text-xs text-muted uppercase tracking-wider mb-4">{m.sec_geo_webmcp_capabilities()}</div>

                    <div class="space-y-2">
                        {#each (scanResult.webmcp.endpoints || []) as ep}
                            <div class="px-3 py-2 bg-teal-500/10 rounded-lg text-xs font-mono text-teal-400 break-all border border-teal-500/20">
                                {ep}
                            </div>
                        {/each}
                        {#each (scanResult.webmcp.html_features || []) as f}
                            <div class="px-3 py-2 bg-glass rounded-lg text-xs font-mono text-primary-text border border-subtle">
                                [HTML] {f}
                            </div>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Bot Framework Directives Panel -->
            <div class="lg:col-span-8 h-full flex flex-col">
                <ApiBotDirectives bots={scanResult.ai_crawler_directives?.bots || {}} />
            </div>

        </div>
    {/if}
</div>

<!-- Modal Guide -->
<GeofencingGuide bind:isOpen={showGuide} />

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 6px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
</style>
