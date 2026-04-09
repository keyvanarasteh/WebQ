<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import * as m from '$lib/paraglide/messages';
    import { Search, ShieldAlert, CheckCircle2, AlertCircle, Terminal, FileCode2, Info, Network } from 'lucide-svelte';
    import { slide, fade } from 'svelte/transition';
    import ApiFuzzerRadar from '$lib/components/assessment/api-security/ApiFuzzerRadar.svelte';
    import ApiVulnLog from '$lib/components/assessment/api-security/ApiVulnLog.svelte';
    import ApiSecurityGuide from '$lib/components/assessment/api-security/ApiSecurityGuide.svelte';

    let domain = $state('');
    let isScanning = $state(false);
    let scanResult = $state<any>(null);
    let error = $state<string | null>(null);
    let showGuide = $state(false);
    
    // Fuzzing simulation
    let currentReqs = $state(0);
    let simInterval: ReturnType<typeof setInterval>;

    async function startScan() {
        if (!domain) {
            error = "Please enter a target domain / API root (ex: api.example.com)";
            return;
        }

        isScanning = true;
        error = null;
        scanResult = null;
        currentReqs = 0;

        // Simulate a rapid fuzzer Req/s indicator during loading
        simInterval = setInterval(() => {
            currentReqs = Math.floor(Math.random() * 400) + 150; // Random req/s between 150-550
        }, 150);

        try {
            const res = await invoke('scan_api_security', { domain });
            scanResult = res as any;
        } catch (e: any) {
            error = e.toString();
        } finally {
            isScanning = false;
            clearInterval(simInterval);
        }
    }
</script>

<div class="h-full flex flex-col pt-8 px-6 lg:px-10 pb-10 max-w-[1600px] mx-auto w-full custom-scrollbar overflow-y-auto">
    <!-- Header Area -->
    <div class="mb-8">
        <div class="flex items-center gap-3 mb-2">
            <div class="w-10 h-10 rounded-xl bg-rose-500/10 flex items-center justify-center border border-rose-500/20">
                <FileCode2 class="text-rose-400" size={20} />
            </div>
            <div>
                <h1 class="text-2xl font-semibold text-primary-text tracking-tight">{m.sec_api_title()}</h1>
                <p class="text-sm text-muted mt-1">{m.sec_api_desc()}</p>
            </div>
            <div class="ml-auto">
                <button 
                    onclick={() => showGuide = true}
                    class="flex items-center gap-2 px-4 py-2 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 text-sm font-medium rounded-lg border border-blue-500/20 transition-colors"
                >
                    <Info size={16} />
                    View Mechanics
                </button>
            </div>
        </div>
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
                    placeholder="https://api.target.com/v1"
                    class="w-full bg-[#0a0a0a] border border-glass text-primary-text text-sm rounded-xl focus:ring-1 focus:ring-rose-500 focus:border-rose-500 block pl-11 pr-4 py-3.5 transition-all outline-none shadow-inner"
                    disabled={isScanning}
                    onkeydown={(e) => e.key === 'Enter' && !isScanning && startScan()}
                />
            </div>
            <button 
                onclick={startScan}
                disabled={isScanning}
                class="bg-rose-600 hover:bg-rose-500 disabled:bg-surface disabled:text-muted text-primary-text px-8 py-3.5 rounded-xl font-medium transition-colors flex items-center justify-center gap-2 shadow-lg shadow-rose-900/20 disabled:shadow-none min-w-[200px]"
            >
                {#if isScanning}
                    <div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
                    Fuzzing API...
                {:else}
                    Launch Fuzzer
                {/if}
            </button>
        </div>

        {#if error}
            <div class="mt-4 p-4 bg-red-500/10 border border-red-500/20 rounded-xl flex items-start gap-3" transition:slide>
                <AlertCircle class="text-red-400 mt-0.5 shrink-0" size={18} />
                <div class="text-sm text-red-200/80 leading-relaxed font-mono break-all">
                    {error}
                </div>
            </div>
        {/if}
    </div>

    <!-- Fuzzing Indicator (Simulated) -->
    {#if isScanning}
        <div class="mb-6 p-6 border border-subtle bg-glass rounded-xl flex flex-col items-center justify-center gap-4" transition:fade>
            <Terminal size={32} class="text-rose-400 animate-pulse" />
            <div class="text-center">
                <h3 class="text-lg font-medium text-primary-text mb-1">Deep Fuzzing In Progress</h3>
                <p class="text-sm text-muted">Injecting destructive payloads and probing structural weaknesses.</p>
            </div>
            
            <div class="mt-4 flex gap-8">
                <div class="text-center">
                    <div class="text-2xl font-mono text-emerald-400">{currentReqs}</div>
                    <div class="text-xs text-muted uppercase tracking-wider mt-1">Req/s (Speed)</div>
                </div>
                <div class="text-center">
                    <div class="text-2xl font-mono text-rose-400 animate-pulse">9</div>
                    <div class="text-xs text-muted uppercase tracking-wider mt-1">Suites Active</div>
                </div>
            </div>
        </div>
    {/if}

    <!-- Scan Results -->
    {#if scanResult && !isScanning}
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6" transition:fade>
            
            <!-- Left Column: Stats & Radar -->
            <div class="lg:col-span-4 flex flex-col gap-6">
                <!-- Summary Stats -->
                <div class="grid grid-cols-2 gap-4">
                    <div class="bg-glass border border-subtle rounded-xl p-5 relative overflow-hidden backdrop-blur-md">
                        <div class="text-muted text-xs font-medium uppercase tracking-wider mb-2">{m.sec_api_endpoints_tested()}</div>
                        <div class="text-2xl font-semibold text-primary-text tracking-tight">{scanResult.endpoints_found || 0}</div>
                        <div class="absolute -bottom-2 -right-2 text-primary-text/5">
                            <Network size={64} />
                        </div>
                    </div>
                    
                    <div class="bg-glass border border-subtle rounded-xl p-5 relative overflow-hidden backdrop-blur-md">
                        <div class="text-muted text-xs font-medium uppercase tracking-wider mb-2">{m.sec_api_total_paths()}</div>
                        <div class="text-2xl font-semibold text-primary-text tracking-tight">{scanResult.total_fuzz_payloads || 0}</div>
                        <div class="absolute -bottom-2 -right-2 text-primary-text/5">
                            <Terminal size={64} />
                        </div>
                    </div>
                    
                    <div class="bg-glass border border-subtle rounded-xl p-5 relative overflow-hidden backdrop-blur-md col-span-2">
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="text-muted text-xs font-medium uppercase tracking-wider mb-2">{m.sec_api_vulns_found()}</div>
                                <div class="text-3xl font-semibold {scanResult.vulnerabilities?.length > 0 ? 'text-red-400' : 'text-emerald-400'} tracking-tight">
                                    {scanResult.vulnerabilities?.length || 0}
                                </div>
                            </div>
                            <!-- Status Icon -->
                            <div class={`w-12 h-12 rounded-full flex items-center justify-center ${scanResult.vulnerabilities?.length > 0 ? 'bg-red-500/10 text-red-400 border border-red-500/20' : 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20'}`}>
                                {#if scanResult.vulnerabilities?.length > 0}
                                    <ShieldAlert size={24} />
                                {:else}
                                    <CheckCircle2 size={24} />
                                {/if}
                            </div>
                        </div>
                        {#if scanResult.fuzz_time_ms}
                            <div class="mt-4 pt-3 border-t border-subtle text-xs text-muted flex justify-between">
                                <span>Execution Time</span>
                                <span class="font-mono text-muted">{(scanResult.fuzz_time_ms / 1000).toFixed(2)}s</span>
                            </div>
                        {/if}
                    </div>
                </div>

                <!-- Radar Chart / Distribution -->
                <div class="flex-1">
                    <ApiFuzzerRadar vulnerabilities={scanResult.vulnerabilities || []} />
                </div>
            </div>

            <!-- Right Column: Vulnerability Log Matrix -->
            <div class="lg:col-span-8 h-full flex flex-col">
                <ApiVulnLog vulnerabilities={scanResult.vulnerabilities || []} />
            </div>
            
        </div>
    {/if}
</div>

<!-- Modal Guide -->
<ApiSecurityGuide bind:isOpen={showGuide} />

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 6px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
</style>
