<script lang="ts">
    import { X, Terminal, Code2, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Live Detection", icon: Terminal },
        { id: 1, label: "2. Reading Entries", icon: Code2 },
        { id: 2, label: "3. Triage Protocol", icon: ShieldCheck }
    ];

    function nextTab() {
        if (activeTab < tabs.length - 1) activeTab++;
    }
    function prevTab() {
        if (activeTab > 0) activeTab--;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div
            class="bg-[#0A0C10] border border-rose-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(244,63,94,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-rose-950/40 via-rose-900/10 to-transparent p-6 border-b border-rose-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-rose-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-rose-500/20 rounded-xl transition-all border border-subtle hover:border-rose-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-rose-500/20 to-rose-600/10 flex items-center justify-center border border-rose-500/30 shadow-inner">
                            <Terminal size={28} class="text-rose-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">API Vulnerability Log</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Live stream of injection attempts, detected payloads, and endpoint exposure evidence.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-rose-500/20 text-rose-300 border border-rose-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 custom-scrollbar relative">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-300 mb-4">Understanding the Live Feed</h3>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-sm text-primary-text leading-relaxed">
                                    The <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-red-500/10 border border-red-500/20 text-[10px] font-mono text-red-400"><span class="w-1.5 h-1.5 rounded-full bg-red-400 animate-pulse inline-block"></span>LIVE FEED</span> badge means the fuzzer is actively running and streaming vulnerability events as they are confirmed. Each entry represents a single endpoint-parameter-payload combination that produced a detectable anomaly.
                                </p>
                                <div class="bg-rose-950/20 border border-rose-500/20 rounded-xl p-4">
                                    <h4 class="text-rose-400 font-medium mb-3 flex items-center gap-2"><Terminal size={14}/> Severity Levels</h4>
                                    <div class="space-y-2 text-sm">
                                        <div class="flex items-center gap-3">
                                            <span class="px-2 py-0.5 text-[10px] font-bold tracking-wider rounded border text-red-400 bg-red-400/10 border-red-400/20 shrink-0">CRITICAL</span>
                                            <span class="text-xs text-muted">RCE potential, direct data exfiltration (SQLi with output, LFI reading sensitive files)</span>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <span class="px-2 py-0.5 text-[10px] font-bold tracking-wider rounded border text-orange-400 bg-orange-400/10 border-orange-400/20 shrink-0">HIGH</span>
                                            <span class="text-xs text-muted">Significant attack surface — confirmed injection point without full exploitation proof</span>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <span class="px-2 py-0.5 text-[10px] font-bold tracking-wider rounded border text-yellow-400 bg-yellow-400/10 border-yellow-400/20 shrink-0">MEDIUM</span>
                                            <span class="text-xs text-muted">Reflected/stored XSS, indirect SSRF, information disclosure in error messages</span>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <span class="px-2 py-0.5 text-[10px] font-bold tracking-wider rounded border text-emerald-400 bg-emerald-400/10 border-emerald-400/20 shrink-0">LOW</span>
                                            <span class="text-xs text-muted">Informational: verbose stack traces, server version headers, non-exploitable misconfigs</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <!-- Sample log entry mockup -->
                            <div class="bg-[#0d1117] rounded-xl border border-rose-500/10 overflow-hidden font-mono text-xs">
                                <div class="bg-surface/80 px-4 py-2.5 border-b border-subtle flex items-center gap-2">
                                    <div class="w-2 h-2 rounded-full bg-red-400 animate-pulse"></div>
                                    <span class="text-muted text-[10px]">Live event stream</span>
                                </div>
                                <div class="p-4 space-y-3">
                                    <div class="border border-red-500/20 rounded-lg p-3 bg-red-500/5">
                                        <div class="flex items-center gap-2 mb-2">
                                            <span class="px-1.5 py-0.5 text-[9px] font-bold rounded border text-red-400 bg-red-400/10 border-red-400/20">CRITICAL</span>
                                            <span class="text-primary-text font-bold">SQL_INJECTION</span>
                                            <span class="text-muted">- Error Based</span>
                                        </div>
                                        <p class="text-teal-400">/api/v1/users/search</p>
                                        <p class="text-muted mt-1">Param: <span class="text-orange-400">?q=</span></p>
                                        <p class="text-red-400 mt-1 break-all">' OR 1=1--</p>
                                        <p class="text-muted mt-2 text-[10px] border-t border-red-500/10 pt-2">EVIDENCE: You have an error in your SQL syntax near 'OR 1=1'</p>
                                        <p class="text-muted mt-1">CONFIDENCE: <span class="text-emerald-400">HIGH</span></p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4 text-xs text-muted leading-relaxed">
                            <ArrowRight size={12} class="text-rose-400 inline mr-1" />
                            When no scan is active the log shows "No Vulnerabilities Injected into Matrix Log" — this is expected. Entries only appear when the fuzzer actively finds and confirms a vulnerability during an ongoing scan.
                        </div>
                    </div>

                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-300 mb-4">Anatomy of a Log Entry</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed">Each log entry is a structured vulnerability record with eight fields. Understanding each field tells you exactly what was found, where, and how reliably.</p>

                        <!-- Annotated entry -->
                        <div class="bg-[#0d1117] rounded-xl border border-rose-500/20 overflow-hidden font-mono text-xs mb-6 relative">
                            <div class="absolute top-0 right-0 bg-rose-500/10 text-rose-400 px-4 py-1.5 rounded-bl-xl text-[10px] font-bold border-b border-l border-rose-500/30 tracking-widest uppercase">Sample Entry</div>
                            <div class="p-5 pt-10 space-y-2">
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">severity</span>
                                    <span class="px-2 py-0.5 text-[9px] font-bold rounded border text-red-400 bg-red-400/10 border-red-400/20">CRITICAL</span>
                                    <span class="text-muted text-[10px] ml-2">→ Exploitation risk level</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">vuln_type</span>
                                    <span class="text-primary-text font-bold">SQL_INJECTION</span>
                                    <span class="text-muted text-[10px] ml-2">→ OWASP category</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">subtype</span>
                                    <span class="text-blue-300">Error Based</span>
                                    <span class="text-muted text-[10px] ml-2">→ Detection technique used</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">endpoint</span>
                                    <span class="text-teal-400">/api/v1/users/search</span>
                                    <span class="text-muted text-[10px] ml-2">→ Vulnerable URL path</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">parameter</span>
                                    <span class="text-orange-400">q</span>
                                    <span class="text-muted text-[10px] ml-2">→ Vulnerable query/body parameter</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">payload</span>
                                    <span class="text-red-400 break-all">' OR 1=1--</span>
                                    <span class="text-muted text-[10px] ml-2">→ Exact string that triggered the finding</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">confidence</span>
                                    <span class="text-emerald-400">HIGH</span>
                                    <span class="text-muted text-[10px] ml-2">→ Reliability of the detection signal</span>
                                </div>
                                <div class="flex items-start gap-3">
                                    <span class="text-muted shrink-0 w-20">evidence</span>
                                    <span class="text-muted break-all">You have an error in your SQL syntax...</span>
                                    <span class="text-muted text-[10px] ml-2">→ Server response fragment proving the finding</span>
                                </div>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-rose-950/10 border border-rose-500/20 rounded-xl p-4">
                                <h4 class="text-xs font-semibold text-rose-300 mb-2 uppercase tracking-wider">Subtypes Explained</h4>
                                <div class="space-y-1.5 text-xs text-muted">
                                    <p><span class="text-primary-text">Error Based</span> — DB error messages leaked in response body</p>
                                    <p><span class="text-primary-text">Time Based</span> — response delay on <code class="text-rose-300 bg-glass px-1 rounded">SLEEP(5)</code> payload (blind)</p>
                                    <p><span class="text-primary-text">Reflected</span> — XSS payload echoed back unencoded</p>
                                    <p><span class="text-primary-text">Stored</span> — payload persisted and retrieved later</p>
                                    <p><span class="text-primary-text">Out-of-band</span> — DNS/HTTP callback from target server</p>
                                </div>
                            </div>
                            <div class="bg-rose-950/10 border border-rose-500/20 rounded-xl p-4">
                                <h4 class="text-xs font-semibold text-rose-300 mb-2 uppercase tracking-wider">Confidence Levels</h4>
                                <div class="space-y-1.5 text-xs text-muted">
                                    <p><span class="text-emerald-400">HIGH</span> — direct evidence in response (error text, file content)</p>
                                    <p><span class="text-yellow-400">MEDIUM</span> — strong heuristic match (status change, body length delta)</p>
                                    <p><span class="text-orange-400">LOW</span> — timing anomaly only, may be noise</p>
                                </div>
                            </div>
                        </div>
                    </div>

                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Triage Protocol</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed">Not all findings require the same urgency. Use this triage flow to prioritise remediation work once a scan completes.</p>

                        <!-- Decision flowchart as a visual list -->
                        <div class="relative mb-6">
                            <div class="absolute left-4 top-0 bottom-0 w-px bg-gradient-to-b from-red-500/50 via-orange-500/30 to-emerald-500/20"></div>

                            <div class="space-y-4 pl-10">
                                <div class="relative">
                                    <div class="absolute -left-6 top-3 w-4 h-4 rounded-full bg-red-500 border-2 border-[#0A0C10] flex items-center justify-center shadow-[0_0_8px_rgba(239,68,68,0.6)]"></div>
                                    <div class="bg-red-950/20 border border-red-500/30 rounded-xl p-4">
                                        <h4 class="text-sm font-semibold text-red-300 mb-1">Step 1 — Fix CRITICAL + HIGH confidence first</h4>
                                        <p class="text-xs text-muted leading-relaxed">These are confirmed exploitable findings. CRITICAL SQLi with HIGH confidence means an attacker can extract your database right now. Patch immediately, rotate secrets, review access logs for prior exploitation.</p>
                                        <div class="mt-2 flex flex-wrap gap-2">
                                            <span class="text-[10px] bg-red-500/10 text-red-400 border border-red-500/20 px-2 py-0.5 rounded font-mono">CRITICAL + HIGH conf</span>
                                            <span class="text-[10px] bg-red-500/10 text-red-400 border border-red-500/20 px-2 py-0.5 rounded font-mono">CRITICAL + MEDIUM conf</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="relative">
                                    <div class="absolute -left-6 top-3 w-4 h-4 rounded-full bg-orange-500 border-2 border-[#0A0C10] flex items-center justify-center shadow-[0_0_8px_rgba(249,115,22,0.5)]"></div>
                                    <div class="bg-orange-950/20 border border-orange-500/30 rounded-xl p-4">
                                        <h4 class="text-sm font-semibold text-orange-300 mb-1">Step 2 — Investigate HIGH severity findings</h4>
                                        <p class="text-xs text-muted leading-relaxed">Manually verify the payload in a test environment. HIGH confidence findings are likely real but may need context — a reflected XSS might only trigger in specific browsers or with user interaction (stored XSS is more severe).</p>
                                    </div>
                                </div>

                                <div class="relative">
                                    <div class="absolute -left-6 top-3 w-4 h-4 rounded-full bg-yellow-500 border-2 border-[#0A0C10]"></div>
                                    <div class="bg-yellow-950/20 border border-yellow-500/30 rounded-xl p-4">
                                        <h4 class="text-sm font-semibold text-yellow-300 mb-1">Step 3 — Review MEDIUM findings in next sprint</h4>
                                        <p class="text-xs text-muted leading-relaxed">These may require chaining with another vulnerability to exploit. Examples: SSRF that only reaches localhost, or XSS in a low-traffic admin panel. Assign to the team's next security sprint.</p>
                                    </div>
                                </div>

                                <div class="relative">
                                    <div class="absolute -left-6 top-3 w-4 h-4 rounded-full bg-emerald-500 border-2 border-[#0A0C10]"></div>
                                    <div class="bg-emerald-950/20 border border-emerald-500/30 rounded-xl p-4">
                                        <h4 class="text-sm font-semibold text-emerald-300 mb-1">Step 4 — Log LOW findings as tech debt</h4>
                                        <p class="text-xs text-muted leading-relaxed">Information disclosure findings (verbose errors, version banners) are low-urgency but easy to fix. Track them in your backlog and address in routine maintenance windows.</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-glass border border-subtle rounded-xl p-4 text-xs text-muted leading-relaxed">
                            <strong class="text-primary-text">High vs Medium confidence:</strong> A HIGH confidence finding has direct evidence (error text, file content in body). A MEDIUM confidence finding relies on heuristics like response-size delta or timing. Always manually verify MEDIUM findings before patching — they have a higher false-positive rate (~15%).
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-rose-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-muted hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-glass rounded mx-1 text-muted border border-glass font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle hover:border-glass transition-all focus:ring-2 focus:ring-glass outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}

                    {#if activeTab < tabs.length - 1}
                        <button
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-rose-600 hover:bg-rose-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(244,63,94,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-rose-500/50 outline-none"
                        >
                            Continue <ChevronRight size={16} />
                        </button>
                    {:else}
                        <button
                            onclick={close}
                            class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(16,185,129,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-emerald-500/50 outline-none"
                        >
                            Complete
                        </button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 8px; height: 8px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background-color: rgba(255,255,255,0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background-color: rgba(255,255,255,0.2); }
</style>
