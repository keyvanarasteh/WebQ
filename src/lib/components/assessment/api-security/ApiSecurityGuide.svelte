<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Network, ShieldCheck, Database, Braces, Terminal, Code2, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Fundamentals", icon: ShieldCheck },
        { id: 1, label: "2. Attack Vectors", icon: Terminal },
        { id: 2, label: "3. Remediation", icon: Code2 }
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
        class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6 bg-black/80 backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div 
            class="bg-[#0A0C10] border border-rose-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(225,29,72,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-rose-950/40 via-red-900/10 to-transparent p-6 border-b border-rose-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-rose-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-white/5 hover:bg-rose-500/20 rounded-xl transition-all border border-white/5 hover:border-rose-500/30 font-medium"
                    aria-label="Close API guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-rose-500/20 to-red-600/10 flex items-center justify-center border border-rose-500/30 shadow-inner">
                            <Braces size={28} class="text-rose-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.sec_api_guide_title()}</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">{m.sec_api_guide_mechanics()}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-rose-500/20 text-rose-300 border border-rose-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-white/5 border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 relative">
                {#if activeTab === 0}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-rose-300 mb-4">API Anatomy & Core Vulnerabilities</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    Modern applications rely on APIs as their backbone. While they enable rich frontend experiences, they also expose direct access controls, internal endpoints, and sensitive data structures if not explicitly protected.
                                </p>
                                <div class="bg-rose-950/20 border border-rose-500/10 rounded-xl p-4">
                                    <h4 class="text-rose-400 font-medium mb-3 flex items-center gap-2"><Network size={16}/> The Common Attack Surface</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-rose-500 mt-1 shrink-0"/> <span><strong>BOLA (Broken Object Level Auth):</strong> Accessing `/api/user/10` when logged in as User `2`.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-rose-500 mt-1 shrink-0"/> <span><strong>Mass Assignment:</strong> Sending `&#123;"role":"admin"&#125;` in a profile update payload.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-rose-500 mt-1 shrink-0"/> <span><strong>Excessive Data Exposure:</strong> Returning full DB records when only name/email is needed.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Visual Diagram -->
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner h-full">
                                <div class="flex items-center justify-between w-full mb-6 relative py-4">
                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-12 h-12 rounded-full bg-surface border-2 border-zinc-600 flex items-center justify-center text-primary-text mb-2 shadow-lg">User</div>
                                    </div>
                                    
                                    <div class="absolute top-[40%] left-10 w-[calc(100%-5rem)] h-0 border-t-2 border-dashed border-rose-500/30">
                                        <div class="w-1/2 h-[2px] bg-rose-500 shadow-[0_0_10px_#f43f5e] animate-[pulse_2s_ease-in-out_infinite]"></div>
                                    </div>

                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-14 h-14 rounded-lg bg-indigo-900 border-2 border-indigo-500 flex items-center justify-center text-indigo-300 mb-2 shadow-[0_0_15px_rgba(99,102,241,0.4)]">API</div>
                                    </div>

                                    <div class="absolute top-[40%] right-10 w-1/4 h-0 border-t-2 border-dashed border-emerald-500/30"></div>

                                    <div class="flex flex-col items-center z-10 w-20">
                                        <div class="w-12 h-12 rounded-full bg-emerald-900 border-2 border-emerald-500 flex items-center justify-center mb-2 shadow-lg"><Database size={18} class="text-emerald-400"/></div>
                                    </div>
                                </div>
                                <div class="text-center text-xs text-rose-400 mt-4 bg-rose-500/10 px-4 py-2 rounded-full border border-rose-500/30 font-medium tracking-wide">Missing Authorization Check at API Boundary</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-rose-300">Simulated BOLA Fuzzing</h3>
                            <span class="px-2.5 py-0.5 rounded-full bg-rose-500/20 text-rose-300 text-[10px] font-bold uppercase tracking-wider border border-rose-500/30">Live Trace</span>
                        </div>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Attackers use automation tools to iterate through ID sequences, looking for HTTP 200 OK responses on resources they shouldn't own.</p>
                        
                        <!-- Terminal Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-rose-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm ">
                            <div class="bg-surface/80 px-4 py-3 flex items-center gap-2 border-b border-white/5 backdrop-blur">
                                <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                                <span class="ml-2 text-muted text-xs tracking-wider">attacker@kali:~/recon</span>
                            </div>
                            <div class="p-5 space-y-3 text-primary-text h-72 overflow-y-auto custom-scrollbar">
                                <p class="text-emerald-400 flex items-center gap-2 font-semibold"><span>➜</span> <span class="text-blue-400">ffuf</span> -w ids.txt -u "https://api.target.com/v1/invoices/FUZZ" -H "Authorization: Bearer attacker_token"</p>
                                <p class="text-muted">Starting fuzzing sequence (threads: 10)</p>
                                <div class="pl-4 space-y-1.5 opacity-80">
                                    <p class="text-muted">... testing ID: 9400 <span class="text-red-400">[403 Forbidden]</span></p>
                                    <p class="text-muted">... testing ID: 9401 <span class="text-red-400">[403 Forbidden]</span></p>
                                    <p class="text-green-400 font-bold bg-green-500/10 px-3 py-1.5 rounded-md w-fit border border-green-500/30 shadow-[0_0_15px_rgba(34,197,94,0.15)] flex gap-2 items-center">
                                        <ShieldCheck size={14}/><span>... testing ID: 9402 [200 OK] (Size: 1.4KB) - BOUNTY!</span>
                                    </p>
                                    <p class="text-muted">... testing ID: 9403 <span class="text-red-400">[403 Forbidden]</span></p>
                                </div>
                                <p class="text-emerald-400 flex items-center gap-2 mt-6 font-semibold border-t border-white/5 pt-4"><span>➜</span> <span class="text-blue-400">curl</span> -H "Authorization: Bearer attacker_token" https://api.target.com/v1/invoices/9402 | jq</p>
                                <div class="pl-4 mt-2">
                                <pre class="text-orange-300 bg-black/60 p-4 rounded-xl border border-white/5 shadow-inner">
&#123;
  "invoice_id": 9402,
  "client_email": "ceo@target.com",
  "amount_usd": 150000.00,
  "status": "paid",
  "ssn_last4": "8492"
&#125;</pre>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Defense & Remediation</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Never trust client-provided IDs. Always extract the user's identity from a cryptographically signed JWT, and assert ownership in the database layer.</p>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">Rust (Axum)</div>
                            <div class="p-6 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-purple-400">pub async fn</span> <span class="text-blue-400">get_invoice</span>(
    State(db): State&lt;DbPool&gt;,
    Path(invoice_id): Path&lt;<span class="text-teal-400">i32</span>&gt;,
    Extension(claims): Extension&lt;Claims&gt; <span class="text-muted italic">// JWT Middleware sets this accurately</span>
) -> <span class="text-teal-400">Result</span>&lt;Json&lt;Invoice&gt;, AppError&gt; &#123;
    <span class="text-muted italic">// ✅ GOOD: Check ownership explicitly in query</span>
    <span class="text-purple-400">let</span> invoice = sqlx::query!(
        <span class="text-green-300">"SELECT * FROM invoices WHERE id = $1 AND owner_id = $2"</span>,
        invoice_id,
        claims.user_id <span class="text-muted italic">// Trusted server-side value extracted from token</span>
    )
    .fetch_optional(&db)
    .<span class="text-purple-400">await</span>?;

    <span class="text-purple-400">match</span> invoice &#123;
        <span class="text-teal-400">Some</span>(inv) => <span class="text-teal-400">Ok</span>(Json(inv)),
        <span class="text-teal-400">None</span> => <span class="text-teal-400">Err</span>(AppError::NotFound) <span class="text-muted italic">// Safely hide existence from attackers</span>
    &#125;
&#125;</pre>
                            </div>
                        </div>

                        <!-- Mitigation Checklist -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <ShieldCheck size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Direct Object References</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Switch from sequential INTs to v4 UUIDs to prevent enumeration. Random identifiers make crawling impossible.</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                                    <Network size={20} class="text-blue-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Rate Limiting</h4>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">Apply 429 Too Many Requests per IP/Token to slow down fuzzers. Create thresholds over time intervals.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-rose-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-muted hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-white/5 rounded mx-1 text-muted border border-white/10 font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button 
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-white/5 hover:bg-white/10 text-primary-text text-sm font-medium rounded-xl border border-white/5 hover:border-white/10 transition-all focus:ring-2 focus:ring-white/10 outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}
                    
                    {#if activeTab < tabs.length - 1}
                        <button 
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-rose-500 hover:bg-rose-400 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(225,29,72,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-rose-500/50 outline-none"
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
    /* Custom Scrollbar for Terminal */
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
