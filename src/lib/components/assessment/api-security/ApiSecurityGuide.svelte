<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Network, ShieldCheck, Database, Braces } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { show = $bindable(false) } = $props();

    function close() {
        show = false;
    }
</script>

{#if show}
    <div 
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
        transition:fade={{ duration: 200 }}
    >
        <div 
            class="bg-[#0f1115] border border-white/10 rounded-2xl w-full max-w-2xl overflow-hidden shadow-2xl relative"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
        >
            <!-- Glowing Header -->
            <div class="relative bg-gradient-to-r from-rose-900/40 via-red-900/20 to-transparent p-6 border-b border-white/10 overflow-hidden">
                <div class="absolute -right-10 -top-10 w-40 h-40 bg-rose-500/20 blur-3xl rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-1.5 text-zinc-400 hover:text-white bg-white/5 hover:bg-white/10 rounded-lg transition-colors border border-white/5"
                    aria-label="Close API guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex items-center gap-4 relative z-10">
                    <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-rose-500/20 to-red-600/10 flex items-center justify-center border border-rose-500/30">
                        <Braces size={24} class="text-rose-400" />
                    </div>
                    <div>
                        <h2 class="text-xl font-medium text-white">{m.sec_api_guide_title()}</h2>
                        <p class="text-sm text-zinc-400 mt-1">{m.sec_api_guide_mechanics()}</p>
                    </div>
                </div>
            </div>

            <!-- Content Area -->
            <div class="p-6 space-y-6">
                <!-- Mechanics -->
                <div class="p-4 bg-white/[0.02] border border-white/5 rounded-xl">
                    <p class="text-sm text-zinc-300 leading-relaxed">
                        {m.sec_api_guide_desc()}
                    </p>
                </div>

                <!-- Three pillars of API Security -->
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <!-- Injection flaws -->
                    <div class="bg-black/40 border border-white/5 p-4 rounded-xl relative overflow-hidden group hover:border-emerald-500/30 transition-colors">
                        <div class="absolute inset-0 bg-gradient-to-b from-emerald-500/5 to-transparent pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity"></div>
                        <Database size={20} class="text-emerald-400 mb-3" />
                        <h4 class="text-sm font-medium text-white mb-2">Input Validation</h4>
                        <p class="text-xs text-zinc-500 leading-relaxed">
                            Treat every JSON field as hostile. Require strict structural matching (Schema Validation) before routing to DB logic.
                        </p>
                    </div>

                    <!-- Rate Limiting -->
                    <div class="bg-black/40 border border-white/5 p-4 rounded-xl relative overflow-hidden group hover:border-blue-500/30 transition-colors">
                        <div class="absolute inset-0 bg-gradient-to-b from-blue-500/5 to-transparent pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity"></div>
                        <Network size={20} class="text-blue-400 mb-3" />
                        <h4 class="text-sm font-medium text-white mb-2">Rate Restrict</h4>
                        <p class="text-xs text-zinc-500 leading-relaxed">
                            Stop automated enumeration tools. Ban IP addresses dynamically that trigger more than 5 404s/403s inside a minute.
                        </p>
                    </div>

                    <!-- Auth -->
                    <div class="bg-black/40 border border-white/5 p-4 rounded-xl relative overflow-hidden group hover:border-yellow-500/30 transition-colors">
                        <div class="absolute inset-0 bg-gradient-to-b from-yellow-500/5 to-transparent pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity"></div>
                        <ShieldCheck size={20} class="text-yellow-400 mb-3" />
                        <h4 class="text-sm font-medium text-white mb-2">Auth Gateways</h4>
                        <p class="text-xs text-zinc-500 leading-relaxed">
                            BOLA (Broken Object Level Authorization) is fatal. Enforce multi-layer JWT claims and Mutual TLS to prevent bypassing.
                        </p>
                    </div>
                </div>

                <!-- SecOps Mitigation Box -->
                <div class="bg-blue-500/5 border border-blue-500/20 rounded-xl p-4 flex gap-4 mt-8">
                    <div class="flex-shrink-0 mt-0.5 mt-1">
                        <ShieldCheck size={20} class="text-blue-400" />
                    </div>
                    <div>
                        <h4 class="text-sm font-medium text-blue-400 mb-1.5">{m.sec_api_guide_secops()}</h4>
                        <p class="text-xs leading-relaxed text-blue-200/70">
                            {m.sec_api_guide_secops_desc()}
                        </p>
                    </div>
                </div>
            </div>
            
            <!-- Footer -->
            <div class="bg-black/40 p-4 border-t border-white/5 flex justify-end">
                <button 
                    onclick={close}
                    class="px-5 py-2 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-lg border border-white/10 transition-colors"
                >
                    Acknowledge
                </button>
            </div>
        </div>
    </div>
{/if}
