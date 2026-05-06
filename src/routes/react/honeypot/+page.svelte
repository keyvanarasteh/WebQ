<script lang="ts">
    import { Shield, Activity, Target, Terminal, Settings, Server, ShieldAlert, Cpu, Play, StopCircle, RefreshCw, Info, AlertTriangle, Eye } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onMount, onDestroy } from "svelte";
    import HoneypotDocModal from "$lib/components/ui/HoneypotDocModal.svelte";

    let isRunning = $state(false);
    let port = $state(3001);
    let statusMessage = $state("");
    let error = $state("");

    let topAttackers = $state<any[]>([]);
    let liveEvents = $state<any[]>([]);
    let payloadToTest = $state("");
    let testResult = $state<any>(null);

    let isModalOpen = $state(false);
    let modalSection = $state<"overview" | "vectors" | "profiling" | "simulation">("overview");

    // Config defaults based on HoneypotConfig
    let config = $state({
        max_payload_store: 8192,
        realistic_timing: true,
        min_delay_ms: 20,
        max_delay_ms: 180,
        fake_rsc_responses: true,
        session_tracking: true,
        session_cookie: "__Host-RSC-ID",
        log_all_requests: false,
        detection_threshold: 0.5,
        progressive_sizing: true,
    });

    let unlisten: () => void;

    onMount(async () => {
        await checkStatus();
        await fetchTopAttackers();

        unlisten = await listen("honeypot-attack-detected", (event: any) => {
            const detection = event.payload;
            // Add to the front, keep max 100
            liveEvents = [detection, ...liveEvents].slice(0, 100);
            fetchTopAttackers(); // Refresh leaderboard on new attack
        });
    });

    onDestroy(() => {
        if (unlisten) unlisten();
    });

    function openModal(section: "overview" | "vectors" | "profiling" | "simulation") {
        modalSection = section;
        isModalOpen = true;
    }

    async function checkStatus() {
        try {
            isRunning = await invoke("get_honeypot_status");
        } catch (e: any) {
            error = e.toString();
        }
    }

    async function toggleHoneypot() {
        error = "";
        try {
            if (isRunning) {
                statusMessage = await invoke("stop_honeypot");
            } else {
                await invoke("update_honeypot_config", { config });
                statusMessage = await invoke("start_honeypot", { port });
            }
            await checkStatus();
        } catch (e: any) {
            error = e.toString();
        }
    }

    async function fetchTopAttackers() {
        try {
            topAttackers = await invoke("get_top_attackers", { limit: 10 });
        } catch (e: any) {
            console.error("Failed to fetch attackers", e);
        }
    }

    async function saveConfig() {
        try {
            await invoke("update_honeypot_config", { config });
            statusMessage = "Configuration updated successfully";
            setTimeout(() => statusMessage = "", 3000);
        } catch (e: any) {
            error = e.toString();
        }
    }

    async function testPayload() {
        try {
            testResult = await invoke("test_payload_locally", { payload: payloadToTest });
        } catch (e: any) {
            error = e.toString();
        }
    }

    function formatTime(iso: string) {
        return new Date(iso).toLocaleTimeString();
    }
</script>

<HoneypotDocModal bind:isOpen={isModalOpen} section={modalSection} />

<div class="space-y-6 max-w-7xl mx-auto p-6">
    <div class="text-center space-y-4 mb-10">
        <h1 class="text-5xl font-black tracking-tight text-primary-text">React2Shell <span class="text-yellow-500 drop-shadow-[0_0_15px_rgba(234,179,8,0.5)]">Honeypot</span></h1>
        <p class="text-muted text-lg">Intelligent Deception & Attack Vector Profiling Engine</p>
    </div>

    <!-- Top Control Bar -->
    <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg flex flex-col md:flex-row items-center justify-between gap-6 relative">
        <button 
            onclick={() => openModal('overview')}
            class="absolute top-4 right-4 p-2 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors border border-blue-500/20 flex items-center gap-2"
            title="View Honeypot Documentation"
        >
            <Info class="w-4 h-4" />
            <span class="text-xs font-bold uppercase tracking-wider hidden sm:inline">Architecture</span>
        </button>

        <div class="flex items-center gap-4">
            <div class="relative">
                {#if isRunning}
                    <div class="absolute inset-0 bg-green-500/20 rounded-full animate-ping"></div>
                {/if}
                <div class="p-3 bg-background rounded-xl border {isRunning ? 'border-green-500/50 shadow-[0_0_15px_rgba(34,197,94,0.3)]' : 'border-border/50'} relative z-10">
                    <Server class="w-8 h-8 {isRunning ? 'text-green-400' : 'text-muted'}" />
                </div>
            </div>
            <div>
                <h2 class="text-xl font-bold text-primary-text">Honeypot Daemon</h2>
                <div class="flex items-center gap-2 text-sm mt-1">
                    <span class="relative flex h-2.5 w-2.5">
                        {#if isRunning}
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-green-500"></span>
                        {:else}
                            <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-red-500"></span>
                        {/if}
                    </span>
                    <span class={isRunning ? "text-green-400" : "text-muted"}>{isRunning ? `Running on port ${port}` : "Stopped"}</span>
                </div>
            </div>
        </div>

        <div class="flex items-center gap-4 w-full md:w-auto mt-4 md:mt-0">
            <div class="flex-1 md:w-32">
                <label class="block text-xs font-bold text-muted uppercase tracking-wider mb-1" for="port">Listen Port</label>
                <input id="port" type="number" bind:value={port} disabled={isRunning} class="w-full bg-background border border-border rounded-lg px-4 py-2 text-primary-text focus:outline-none focus:border-yellow-500/50 disabled:opacity-50" />
            </div>
            <button 
                onclick={toggleHoneypot} 
                class="flex items-center gap-2 px-6 py-2 h-10 mt-5 rounded-lg font-bold transition-all {isRunning ? 'bg-red-500/20 text-red-400 hover:bg-red-500/30 border border-red-500/30' : 'bg-green-500/20 text-green-400 hover:bg-green-500/30 border border-green-500/30'}"
            >
                {#if isRunning}
                    <StopCircle class="w-5 h-5" /> Stop
                {:else}
                    <Play class="w-5 h-5" /> Start
                {/if}
            </button>
        </div>
    </div>

    {#if statusMessage || error}
        <div class="p-3 {error ? 'bg-red-500/10 border-red-500/30 text-red-400' : 'bg-green-500/10 border-green-500/30 text-green-400'} border rounded-lg text-sm flex items-center gap-2">
            {#if error}
                <AlertTriangle class="w-4 h-4" /> {error}
            {:else}
                <Info class="w-4 h-4" /> {statusMessage}
            {/if}
        </div>
    {/if}

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        <!-- Live Attack Feed -->
        <div class="lg:col-span-2 bg-surface/30 border border-border/50 rounded-xl flex flex-col h-[500px] shadow-lg relative overflow-hidden">
            <div class="absolute top-4 right-4 z-10">
                <button 
                    onclick={() => openModal('vectors')}
                    class="p-1.5 bg-yellow-500/10 text-yellow-500 hover:bg-yellow-500/20 rounded-lg transition-colors border border-yellow-500/20"
                    title="View Detection Categories"
                >
                    <Target class="w-4 h-4" />
                </button>
            </div>
            <div class="p-4 border-b border-border/50 bg-surface/50 flex items-center gap-3">
                <Activity class="w-5 h-5 text-yellow-500" />
                <h2 class="text-lg font-bold text-primary-text">Live Detection Feed</h2>
                <span class="px-2 py-0.5 rounded-full bg-yellow-500/10 text-yellow-500 text-xs font-bold ml-auto border border-yellow-500/20">
                    {liveEvents.length} Events
                </span>
            </div>
            
            <div class="flex-1 overflow-y-auto p-4 custom-scrollbar space-y-3">
                {#if liveEvents.length === 0}
                    <div class="h-full flex flex-col items-center justify-center text-muted opacity-50">
                        <Eye class="w-12 h-12 mb-4" />
                        <p>Awaiting incoming attacks...</p>
                    </div>
                {:else}
                    {#each liveEvents as event (event.event_id)}
                        <div class="bg-background border border-border/50 rounded-lg p-3 hover:border-yellow-500/30 transition-colors">
                            <div class="flex justify-between items-start mb-2">
                                <div class="flex items-center gap-2">
                                    {#if event.severity === 'Critical'}
                                        <span class="w-2 h-2 rounded-full bg-red-500 animate-pulse"></span>
                                        <span class="text-xs font-bold text-red-500 uppercase tracking-wider">Critical</span>
                                    {:else if event.severity === 'High'}
                                        <span class="w-2 h-2 rounded-full bg-orange-500"></span>
                                        <span class="text-xs font-bold text-orange-500 uppercase tracking-wider">High</span>
                                    {:else if event.severity === 'Medium'}
                                        <span class="w-2 h-2 rounded-full bg-yellow-500"></span>
                                        <span class="text-xs font-bold text-yellow-500 uppercase tracking-wider">Medium</span>
                                    {:else}
                                        <span class="w-2 h-2 rounded-full bg-blue-500"></span>
                                        <span class="text-xs font-bold text-blue-500 uppercase tracking-wider">Low</span>
                                    {/if}
                                    <span class="text-xs text-muted">|</span>
                                    <span class="text-xs font-mono text-primary-text">{formatTime(event.timestamp)}</span>
                                </div>
                                <span class="text-xs font-mono bg-surface px-2 py-0.5 rounded border border-border">{event.attacker_ip}</span>
                            </div>
                            
                            <div class="flex flex-wrap items-center gap-2 mb-2">
                                <span class="px-2 py-1 bg-yellow-500/10 text-yellow-400 text-xs font-bold rounded border border-yellow-500/20">
                                    {event.category}
                                </span>
                                <span class="text-xs text-muted uppercase tracking-wider">{event.subcategory}</span>
                                {#if event.mitre_id}
                                    <span class="px-2 py-0.5 bg-blue-500/10 text-blue-400 text-[10px] font-mono rounded border border-blue-500/20 ml-auto">
                                        {event.mitre_id}
                                    </span>
                                {/if}
                            </div>
                            
                            <div class="bg-surface/50 border border-border/50 rounded p-2 mt-2">
                                <div class="text-xs text-muted mb-1 flex justify-between">
                                    <span>Matched Payload</span>
                                    <span>Conf: {(event.confidence * 100).toFixed(0)}%</span>
                                </div>
                                <div class="font-mono text-xs text-red-400 break-all bg-background p-1.5 rounded border border-red-500/10">
                                    {event.matched_payload}
                                </div>
                            </div>
                        </div>
                    {/each}
                {/if}
            </div>
        </div>

        <!-- Threat Intelligence -->
        <div class="bg-surface/30 border border-border/50 rounded-xl flex flex-col h-[500px] shadow-lg relative overflow-hidden">
            <div class="absolute top-4 right-4 z-10">
                <button 
                    onclick={() => openModal('profiling')}
                    class="p-1.5 bg-purple-500/10 text-purple-400 hover:bg-purple-500/20 rounded-lg transition-colors border border-purple-500/20"
                    title="View Profiling Documentation"
                >
                    <Target class="w-4 h-4" />
                </button>
            </div>
            <div class="p-4 border-b border-border/50 bg-surface/50 flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <ShieldAlert class="w-5 h-5 text-purple-500" />
                    <h2 class="text-lg font-bold text-primary-text">Top Threats</h2>
                </div>
                <button onclick={fetchTopAttackers} class="p-1.5 hover:bg-surface rounded-lg transition-colors text-muted hover:text-primary-text">
                    <RefreshCw class="w-4 h-4" />
                </button>
            </div>
            
            <div class="flex-1 overflow-y-auto p-4 custom-scrollbar space-y-4">
                {#if topAttackers.length === 0}
                    <div class="h-full flex flex-col items-center justify-center text-muted opacity-50">
                        <Cpu class="w-12 h-12 mb-4" />
                        <p>No profiles generated yet.</p>
                    </div>
                {:else}
                    {#each topAttackers as profile}
                        <div class="bg-background border border-border/50 rounded-lg p-3 relative overflow-hidden">
                            <div class="absolute top-0 right-0 h-full w-1 bg-purple-500/50"></div>
                            
                            <div class="flex justify-between items-center mb-2">
                                <span class="font-mono text-sm font-bold text-primary-text">{profile.ip}</span>
                                <div class="flex flex-col items-end">
                                    <span class="text-xs font-bold {profile.risk_score > 70 ? 'text-red-500' : profile.risk_score > 30 ? 'text-yellow-500' : 'text-blue-500'}">
                                        Risk: {profile.risk_score.toFixed(1)}
                                    </span>
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-2 mb-2 text-xs">
                                <div class="bg-surface/50 p-1.5 rounded border border-border/50">
                                    <span class="text-muted block mb-0.5">Requests</span>
                                    <span class="font-bold text-primary-text">{profile.total_requests}</span>
                                </div>
                                <div class="bg-surface/50 p-1.5 rounded border border-border/50">
                                    <span class="text-muted block mb-0.5">Automated</span>
                                    <span class="font-bold {profile.is_automated ? 'text-yellow-500' : 'text-green-500'}">
                                        {profile.is_automated ? 'Yes (Bot)' : 'No'}
                                    </span>
                                </div>
                            </div>
                            
                            {#if profile.browser_fingerprint}
                                <div class="text-xs text-muted truncate border-t border-border/50 pt-2 mt-1">
                                    <span class="text-blue-400">{profile.browser_fingerprint.os}</span> / 
                                    <span class="text-purple-400">{profile.browser_fingerprint.browser}</span>
                                    {#if profile.browser_fingerprint.is_headless}
                                        <span class="ml-1 px-1 py-0.5 bg-red-500/10 text-red-400 rounded">Headless</span>
                                    {/if}
                                </div>
                            {/if}
                        </div>
                    {/each}
                {/if}
            </div>
        </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Configuration -->
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg relative">
            <div class="absolute top-4 right-4 z-10">
                <button 
                    onclick={() => openModal('simulation')}
                    class="p-1.5 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors border border-blue-500/20"
                    title="View Simulation Documentation"
                >
                    <Server class="w-4 h-4" />
                </button>
            </div>
            <div class="flex items-center gap-3 mb-6">
                <Settings class="w-5 h-5 text-blue-400" />
                <h2 class="text-lg font-bold text-primary-text">Honeypot Tuning</h2>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                <label class="flex items-center gap-3 p-3 bg-background border border-border/50 rounded-lg cursor-pointer hover:border-blue-500/30 transition-colors">
                    <input type="checkbox" bind:checked={config.realistic_timing} class="accent-blue-500 w-4 h-4" />
                    <div>
                        <div class="text-sm font-bold text-primary-text">Realistic Timing</div>
                        <div class="text-xs text-muted">Simulate RSC backend delay</div>
                    </div>
                </label>
                
                <label class="flex items-center gap-3 p-3 bg-background border border-border/50 rounded-lg cursor-pointer hover:border-blue-500/30 transition-colors">
                    <input type="checkbox" bind:checked={config.fake_rsc_responses} class="accent-blue-500 w-4 h-4" />
                    <div>
                        <div class="text-sm font-bold text-primary-text">Fake RSC Payloads</div>
                        <div class="text-xs text-muted">Return fake Flight protocol data</div>
                    </div>
                </label>
                
                <label class="flex items-center gap-3 p-3 bg-background border border-border/50 rounded-lg cursor-pointer hover:border-blue-500/30 transition-colors">
                    <input type="checkbox" bind:checked={config.session_tracking} class="accent-blue-500 w-4 h-4" />
                    <div>
                        <div class="text-sm font-bold text-primary-text">Session Tracking</div>
                        <div class="text-xs text-muted">Track via {config.session_cookie}</div>
                    </div>
                </label>
                
                <label class="flex items-center gap-3 p-3 bg-background border border-border/50 rounded-lg cursor-pointer hover:border-blue-500/30 transition-colors">
                    <input type="checkbox" bind:checked={config.progressive_sizing} class="accent-blue-500 w-4 h-4" />
                    <div>
                        <div class="text-sm font-bold text-primary-text">Progressive Sizing</div>
                        <div class="text-xs text-muted">Gradually increase response size</div>
                    </div>
                </label>
            </div>
            
            <div class="flex items-center gap-4">
                <div class="flex-1">
                    <label class="block text-xs font-bold text-muted uppercase mb-1" for="thresh">Detection Threshold</label>
                    <input id="thresh" type="range" min="0" max="1" step="0.1" bind:value={config.detection_threshold} class="w-full accent-blue-500" />
                    <div class="text-xs text-center text-primary-text mt-1 font-mono">{config.detection_threshold}</div>
                </div>
                <button onclick={saveConfig} class="px-6 py-2 bg-blue-500/20 text-blue-400 hover:bg-blue-500/30 border border-blue-500/30 rounded-lg font-bold transition-colors">
                    Apply Config
                </button>
            </div>
        </div>

        <!-- Payload Simulator -->
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg">
            <div class="flex items-center gap-3 mb-4">
                <Terminal class="w-5 h-5 text-green-400" />
                <h2 class="text-lg font-bold text-primary-text">Offline Payload Simulator</h2>
            </div>
            <p class="text-sm text-muted mb-4">Test raw payloads against the local detection engine without hitting the network.</p>
            
            <div class="mb-4">
                <textarea 
                    bind:value={payloadToTest} 
                    placeholder="Enter attack payload (e.g. admin' OR '1'='1)" 
                    class="w-full h-24 bg-background border border-border rounded-lg p-3 text-primary-text font-mono text-sm focus:outline-none focus:border-green-500/50 resize-none"
                ></textarea>
            </div>
            
            <button onclick={testPayload} class="w-full py-2 bg-green-500/20 text-green-400 hover:bg-green-500/30 border border-green-500/30 rounded-lg font-bold transition-colors mb-4">
                Analyze Payload
            </button>
            
            {#if testResult}
                <div class="bg-background border border-border/50 rounded-lg p-4 overflow-x-auto max-h-48 custom-scrollbar">
                    <div class="flex justify-between items-center mb-3">
                        <span class="text-sm font-bold text-primary-text">Analysis Result</span>
                        <span class="px-2 py-1 rounded text-xs font-bold {testResult.should_block ? 'bg-red-500/20 text-red-400' : 'bg-green-500/20 text-green-400'}">
                            {testResult.should_block ? 'BLOCKED' : 'PASSED'}
                        </span>
                    </div>
                    
                    {#if testResult.detections.length > 0}
                        <div class="space-y-2">
                            {#each testResult.detections as det}
                                <div class="bg-surface/50 border border-border/50 rounded p-2 text-xs">
                                    <div class="flex justify-between font-bold text-primary-text mb-1">
                                        <span>{det.category} / {det.subcategory}</span>
                                        <span class="text-yellow-400">Conf: {(det.confidence * 100).toFixed(0)}%</span>
                                    </div>
                                    <div class="font-mono text-red-400 break-all">{det.matched_payload}</div>
                                </div>
                            {/each}
                        </div>
                    {:else}
                        <div class="text-sm text-green-400 font-mono">No attack signatures detected.</div>
                    {/if}
                    
                    <div class="mt-3 pt-3 border-t border-border/50 text-xs text-muted flex justify-between font-mono">
                        <span>Suggested Status: {testResult.simulated_status}</span>
                        <span>Delay: {testResult.suggested_delay_ms}ms</span>
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>
