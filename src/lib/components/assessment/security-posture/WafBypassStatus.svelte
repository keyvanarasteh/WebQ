<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Server, ShieldBan, Shield, HelpCircle } from 'lucide-svelte';
    import WafBypassStatusGuide from './WafBypassStatusGuide.svelte';

    let { wafResult }: {
        wafResult: {
            detected: boolean,
            primary_waf?: { provider: string, confidence: string, detection_methods: string[] },
            all_detected: { provider: string, confidence: string, detection_methods: string[] }[]
        }
    } = $props();

    let guideOpen = $state(false);
</script>

<WafBypassStatusGuide bind:isOpen={guideOpen} />

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6">
    <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
            <Server class="w-5 h-5 text-indigo-400" />
            <h3 class="text-lg font-medium text-primary-text">{m.sec_posture_waf()}</h3>
        </div>
        <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-orange-400 hover:bg-orange-500/10 border border-transparent hover:border-orange-500/20 transition-all" title="How this works">
            <HelpCircle class="size-4" />
        </button>
    </div>

    {#if wafResult.detected && wafResult.primary_waf}
        <div class="p-4 border rounded-lg bg-indigo-500/10 border-indigo-500/20 mb-6">
            <div class="flex items-center gap-3 mb-3">
                <Shield class="w-6 h-6 text-indigo-400" />
                <h4 class="text-base font-semibold text-indigo-300">
                    {m.sec_posture_waf_detected({ provider: wafResult.primary_waf.provider })}
                </h4>
            </div>
            
            <div class="space-y-2 mt-4">
                {#each wafResult.primary_waf.detection_methods as method}
                    <div class="flex items-center gap-2 text-sm text-indigo-200/80">
                        <div class="w-1 h-1 rounded-full bg-indigo-500"></div>
                        <span>{method}</span>
                    </div>
                {/each}
            </div>
        </div>
    {:else}
        <div class="flex flex-col items-center justify-center p-8 border border-dashed rounded-lg border-glass text-center">
            <ShieldBan class="w-10 h-10 mb-4 text-muted" />
            <h4 class="text-secondary-text font-medium mb-1">{m.sec_posture_waf_none()}</h4>
            <p class="text-sm text-muted">No active firewalls detected for this target.</p>
        </div>
    {/if}
</div>
