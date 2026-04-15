<script lang="ts">
    import { GitMerge, Activity, AlertOctagon, HelpCircle } from 'lucide-svelte';
    import HttpMethodsInfoGuide from './HttpMethodsInfoGuide.svelte';

    let { methods }: {
        methods: {
            methods_detected: boolean,
            allowed_methods: string[],
            dangerous_methods: string[],
            security_risk: string
        }
    } = $props();

    function getRiskColor(risk: string) {
        if (risk === 'High' || risk === 'Critical') return 'text-red-400 bg-red-400/10 border-red-400/20';
        if (risk === 'Medium') return 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20';
        return 'text-emerald-400 bg-emerald-400/10 border-emerald-400/20';
    }

    let guideOpen = $state(false);
</script>

<HttpMethodsInfoGuide bind:isOpen={guideOpen} />

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6">
    <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
            <GitMerge class="w-5 h-5 text-violet-400" />
            <h3 class="text-lg font-medium text-primary-text">HTTP Methods</h3>
        </div>
        <div class="flex items-center gap-2">
            <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-orange-400 hover:bg-orange-500/10 border border-transparent hover:border-orange-500/20 transition-all" title="How this works">
                <HelpCircle class="size-4" />
            </button>
            <div class={`px-2.5 py-1 text-xs uppercase font-bold rounded-md border ${getRiskColor(methods.security_risk)}`}>
                {methods.security_risk} Risk
            </div>
        </div>
    </div>

    {#if methods.methods_detected}
        <div class="space-y-4">
            {#if methods.dangerous_methods.length > 0}
                <div>
                    <h4 class="text-xs font-semibold text-muted uppercase tracking-widest mb-2 flex items-center gap-2">
                        <AlertOctagon size={12} class="text-red-400" /> Dangerous Discovered
                    </h4>
                    <div class="flex flex-wrap gap-2">
                        {#each methods.dangerous_methods as m}
                            <span class="px-2.5 py-1 bg-red-500/10 border border-red-500/20 text-red-300 font-mono text-xs rounded shadow-inner">
                                {m}
                            </span>
                        {/each}
                    </div>
                </div>
            {/if}

            <div>
                <h4 class="text-xs font-semibold text-muted uppercase tracking-widest mb-2 flex items-center gap-2">
                    <Activity size={12} class="text-emerald-400" /> Permitted Methods
                </h4>
                <div class="flex flex-wrap gap-2">
                    {#each methods.allowed_methods as m}
                        <span class="px-2.5 py-1 bg-surface border border-subtle text-primary-text font-mono text-xs rounded shadow-inner">
                            {m}
                        </span>
                    {/each}
                </div>
            </div>
            
            <p class="text-xs text-muted leading-relaxed mt-2 p-3 bg-surface/30 rounded-lg border border-subtle/50 border-dashed">
                Insecure HTTP methods (like PUT, DELETE, TRACE) should be explicitly blocked by the backend API firewall or load balancer if they are not explicitly required by business rules.
            </p>
        </div>
    {:else}
        <div class="text-center py-6 text-sm text-muted">
            Could not accurately enumerate permitted HTTP methods. Server returned obfuscated data.
        </div>
    {/if}
</div>
