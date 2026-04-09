<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Database, BrainCircuit, Globe, Code2 } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { show = $bindable(false) } = $props();

    function close() {
        show = false;
    }
</script>

{#if show}
    <div 
        class="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
        transition:fade={{ duration: 200 }}
    >
        <div 
            class="bg-[#0f1115] border border-white/10 rounded-2xl w-full max-w-2xl overflow-hidden shadow-2xl relative"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
        >
            <!-- Glowing Header -->
            <div class="relative bg-gradient-to-r from-blue-900/40 via-indigo-900/20 to-transparent p-6 border-b border-white/10 overflow-hidden">
                <div class="absolute -right-10 -top-10 w-40 h-40 bg-blue-500/20 blur-3xl rounded-full pointer-events-none"></div>
                
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-1.5 text-zinc-400 hover:text-white bg-white/5 hover:bg-white/10 rounded-lg transition-colors border border-white/5 z-20"
                    aria-label="Close Guide Context"
                >
                    <X size={18} />
                </button>

                <div class="flex items-center gap-4 relative z-10">
                    <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-blue-500/20 to-indigo-600/10 flex items-center justify-center border border-blue-500/30">
                        <BrainCircuit size={24} class="text-blue-400" />
                    </div>
                    <div>
                        <h2 class="text-xl font-medium text-white">{m.sec_geo_guide_title()}</h2>
                        <p class="text-sm text-zinc-400 mt-1">{m.sec_geo_guide_mechanics()}</p>
                    </div>
                </div>
            </div>

            <div class="p-6 space-y-6">
                <!-- Descriptive context -->
                <div class="p-4 bg-white/[0.02] border border-white/5 rounded-xl text-sm text-zinc-300 leading-relaxed">
                    {m.sec_geo_guide_desc()}
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <!-- Protocol 1: LLMS.txt -->
                    <div class="bg-black/40 border border-white/5 p-4 rounded-xl relative overflow-hidden group hover:border-violet-500/30 transition-colors">
                        <div class="absolute inset-0 bg-gradient-to-b from-violet-500/5 to-transparent pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity"></div>
                        <Database size={20} class="text-violet-400 mb-3" />
                        <h4 class="text-sm font-medium text-white mb-2">llms.txt Standard</h4>
                        <p class="text-xs text-zinc-500 leading-relaxed">
                            A newly proposed mechanism similar to robots.txt, but it provides explicit markdown paths designed to easily feed ChatGPT and Claude context without scraping HTML.
                        </p>
                    </div>

                    <!-- Protocol 2: WebMCP -->
                    <div class="bg-black/40 border border-white/5 p-4 rounded-xl relative overflow-hidden group hover:border-teal-500/30 transition-colors">
                        <div class="absolute inset-0 bg-gradient-to-b from-teal-500/5 to-transparent pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity"></div>
                        <Code2 size={20} class="text-teal-400 mb-3" />
                        <h4 class="text-sm font-medium text-white mb-2">Model Context Protocol</h4>
                        <p class="text-xs text-zinc-500 leading-relaxed">
                            WebMCP establishes a standardized interface allowing websites to directly inject context back into an AI assistant, crossing the browser/agent boundary automatically.
                        </p>
                    </div>
                </div>

                <!-- Mitigation SecOps box -->
                <div class="bg-red-500/5 border border-red-500/20 rounded-xl p-4 flex gap-4 mt-8">
                    <div class="flex-shrink-0 mt-1">
                        <Globe size={20} class="text-red-400" />
                    </div>
                    <div>
                        <h4 class="text-sm font-medium text-red-400 mb-1.5">{m.sec_geo_guide_secops()}</h4>
                        <p class="text-xs leading-relaxed text-red-200/70">
                            {m.sec_geo_guide_secops_desc()}
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
                    Dismiss
                </button>
            </div>
        </div>
    </div>
{/if}
