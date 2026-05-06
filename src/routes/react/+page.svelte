<script lang="ts">
    import { Shield, Activity, Target, Terminal, FileCode, AlertTriangle, Info } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import ReactDocModal from "$lib/components/ui/ReactDocModal.svelte";

    let target = $state("https://arsonex.com");
    let command = $state("whoami");
    
    let sourceLeakResult = $state<any>(null);
    let rceCommandResult = $state<any>(null);
    let rceFullResult = $state<any>(null);
    
    let loadingLeak = $state(false);
    let loadingRceCmd = $state(false);
    let loadingRceFull = $state(false);
    let error = $state("");

    let isModalOpen = $state(false);
    let modalSection = $state<"target" | "leak" | "rce_command" | "rce_full">("target");

    function openModal(section: "target" | "leak" | "rce_command" | "rce_full") {
        modalSection = section;
        isModalOpen = true;
    }

    async function runSourceLeak() {
        loadingLeak = true;
        error = "";
        try {
            sourceLeakResult = await invoke("scan_react_source_leak", { domain: target });
        } catch (e: unknown) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            loadingLeak = false;
        }
    }

    async function runRceCommand() {
        loadingRceCmd = true;
        error = "";
        try {
            rceCommandResult = await invoke("scan_react_rce_command", { domain: target, command });
        } catch (e: unknown) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            loadingRceCmd = false;
        }
    }

    async function runRceFull() {
        loadingRceFull = true;
        error = "";
        try {
            rceFullResult = await invoke("scan_react_rce_full", { domain: target });
        } catch (e: unknown) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            loadingRceFull = false;
        }
    }
</script>

<ReactDocModal bind:isOpen={isModalOpen} section={modalSection} />

<div class="space-y-6 max-w-6xl mx-auto p-6">
    <div class="text-center space-y-4 mb-10">
        <h1 class="text-5xl font-black tracking-tight text-primary-text">React2Shell <span class="text-accent drop-shadow-[0_0_15px_rgba(239,68,68,0.5)]">CVE-2025-55182</span></h1>
        <p class="text-muted text-lg">Simulated exploitation module for RSC insecure deserialization research.</p>
    </div>

    <!-- Target Configuration -->
    <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg mb-8 relative">
        <button 
            onclick={() => openModal('target')}
            class="absolute top-4 right-4 p-2 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors border border-blue-500/20 flex items-center gap-2"
            title="View Exploit Surface Documentation"
        >
            <Info class="w-4 h-4" />
            <span class="text-xs font-bold uppercase tracking-wider hidden sm:inline">Attack Vectors</span>
        </button>
        <h2 class="text-xl font-bold text-primary-text mb-4 flex items-center gap-2">
            <Target class="w-5 h-5 text-red-400" /> Target Configuration
        </h2>
        <div class="flex flex-col md:flex-row gap-4">
            <div class="flex-1">
                <label class="block text-sm font-medium text-muted mb-1" for="target">Target URL</label>
                <input id="target" type="text" bind:value={target} class="w-full bg-background border border-border rounded-lg px-4 py-2 text-primary-text focus:outline-none focus:border-red-500/50 focus:ring-1 focus:ring-red-500/50" />
            </div>
            <div class="flex-1">
                <label class="block text-sm font-medium text-muted mb-1" for="command">RCE Command</label>
                <input id="command" type="text" bind:value={command} class="w-full bg-background border border-border rounded-lg px-4 py-2 text-primary-text focus:outline-none focus:border-red-500/50 focus:ring-1 focus:ring-red-500/50" />
            </div>
        </div>
        {#if error}
            <div class="mt-4 p-3 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm flex items-center gap-2">
                <AlertTriangle class="w-4 h-4" /> {error}
            </div>
        {/if}
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        <!-- Source Leak Card -->
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg relative overflow-hidden flex flex-col">
            <div class="absolute top-4 right-4 z-10 flex items-center gap-2">
                <button 
                    onclick={() => openModal('leak')}
                    class="p-1 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded transition-colors border border-blue-500/20 flex items-center justify-center"
                    title="View Source Leak Documentation"
                >
                    <Info class="w-4 h-4" />
                </button>
                <span class="bg-yellow-500/20 text-yellow-400 text-xs px-2 py-1 rounded-md font-medium border border-yellow-500/30">Simulated</span>
            </div>
            <div class="flex items-center gap-3 mb-4">
                <div class="p-2 bg-background rounded-lg shadow-inner">
                    <FileCode class="w-5 h-5 text-blue-400" />
                </div>
                <h2 class="text-lg font-bold text-primary-text">Source Leak</h2>
            </div>
            <p class="text-sm text-muted mb-6 flex-1">Extract environmental variables and secrets via unauthenticated source mapping leakage.</p>
            <button onclick={runSourceLeak} disabled={loadingLeak} class="w-full py-2 bg-blue-500/20 hover:bg-blue-500/30 text-blue-400 border border-blue-500/30 rounded-lg transition-colors font-medium disabled:opacity-50">
                {loadingLeak ? 'Extracting...' : 'Execute Leak'}
            </button>

            {#if sourceLeakResult}
                <div class="mt-4 p-4 bg-background rounded-lg border border-border/50 overflow-x-auto">
                    <p class="text-xs text-green-400 mb-2">Success: {sourceLeakResult.success}</p>
                    <p class="text-xs text-muted mb-2">Bytes leaked: {sourceLeakResult.bytes_leaked}</p>
                    <h3 class="text-xs font-bold text-primary-text mt-3 mb-1">Findings:</h3>
                    {#each sourceLeakResult.findings as finding (finding.pattern)}
                        <div class="text-xs font-mono mb-2 p-2 bg-surface/50 rounded border border-border/50">
                            <span class="text-purple-400">{finding.pattern}</span><br/>
                            <span class="text-red-400 break-all">{finding.matched}</span>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>

        <!-- RCE Command Card -->
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg relative overflow-hidden flex flex-col">
            <div class="absolute top-4 right-4 z-10 flex items-center gap-2">
                <button 
                    onclick={() => openModal('rce_command')}
                    class="p-1 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded transition-colors border border-blue-500/20 flex items-center justify-center"
                    title="View RCE Command Documentation"
                >
                    <Info class="w-4 h-4" />
                </button>
                <span class="bg-yellow-500/20 text-yellow-400 text-xs px-2 py-1 rounded-md font-medium border border-yellow-500/30">Simulated</span>
            </div>
            <div class="flex items-center gap-3 mb-4">
                <div class="p-2 bg-background rounded-lg shadow-inner">
                    <Terminal class="w-5 h-5 text-green-400" />
                </div>
                <h2 class="text-lg font-bold text-primary-text">RCE Command</h2>
            </div>
            <p class="text-sm text-muted mb-6 flex-1">Execute a single arbitrary shell command against the vulnerable Flight endpoint.</p>
            <button onclick={runRceCommand} disabled={loadingRceCmd} class="w-full py-2 bg-green-500/20 hover:bg-green-500/30 text-green-400 border border-green-500/30 rounded-lg transition-colors font-medium disabled:opacity-50">
                {loadingRceCmd ? 'Executing...' : 'Run Command'}
            </button>

            {#if rceCommandResult}
                <div class="mt-4 p-4 bg-background rounded-lg border border-border/50 overflow-x-auto">
                    <p class="text-xs text-muted mb-1 font-mono">$ {rceCommandResult.command}</p>
                    <pre class="text-xs text-green-400 font-mono whitespace-pre-wrap mt-2">{rceCommandResult.output || rceCommandResult.error}</pre>
                    <p class="text-xs text-muted mt-2">Exit Code: <span class={rceCommandResult.exit_code === 0 ? "text-green-400" : "text-red-400"}>{rceCommandResult.exit_code}</span></p>
                </div>
            {/if}
        </div>

        <!-- Full RCE Chain Card -->
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg relative overflow-hidden flex flex-col">
            <div class="absolute top-4 right-4 z-10 flex items-center gap-2">
                <button 
                    onclick={() => openModal('rce_full')}
                    class="p-1 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded transition-colors border border-blue-500/20 flex items-center justify-center"
                    title="View Full RCE Chain Documentation"
                >
                    <Info class="w-4 h-4" />
                </button>
                <span class="bg-yellow-500/20 text-yellow-400 text-xs px-2 py-1 rounded-md font-medium border border-yellow-500/30">Simulated</span>
            </div>
            <div class="flex items-center gap-3 mb-4">
                <div class="p-2 bg-background rounded-lg shadow-inner">
                    <Shield class="w-5 h-5 text-red-500" />
                </div>
                <h2 class="text-lg font-bold text-primary-text">Full RCE Chain</h2>
            </div>
            <p class="text-sm text-muted mb-6 flex-1">Orchestrate the complete attack chain: recon, execution, and proof-of-concept verification.</p>
            <button onclick={runRceFull} disabled={loadingRceFull} class="w-full py-2 bg-red-500/20 hover:bg-red-500/30 text-red-400 border border-red-500/30 rounded-lg transition-colors font-medium disabled:opacity-50">
                {loadingRceFull ? 'Exploiting...' : 'Launch Exploit'}
            </button>

            {#if rceFullResult}
                <div class="mt-4 p-4 bg-background rounded-lg border border-border/50 overflow-x-auto">
                    <div class="flex items-center justify-between mb-2">
                        <span class="text-xs font-bold text-primary-text">Status:</span>
                        <span class="text-xs px-2 py-0.5 rounded {rceFullResult.success ? 'bg-green-500/20 text-green-400' : 'bg-red-500/20 text-red-400'}">{rceFullResult.success ? 'PWNED' : 'FAILED'}</span>
                    </div>
                    <div class="flex items-center justify-between mb-4">
                        <span class="text-xs font-bold text-primary-text">PoC Created:</span>
                        <span class="text-xs text-muted">{rceFullResult.poc_file_created ? 'Yes' : 'No'}</span>
                    </div>
                    <h3 class="text-xs font-bold text-primary-text mb-2">Execution Log:</h3>
                    <div class="space-y-3">
                        {#each rceFullResult.command_outputs as cmd, i (i)}
                            <div class="text-xs font-mono p-2 bg-surface/50 rounded border border-border/50">
                                <div class="text-muted mb-1">$ {cmd.command}</div>
                                <div class={cmd.exit_code === 0 ? "text-green-400" : "text-red-400"}>{cmd.output || cmd.error}</div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>
