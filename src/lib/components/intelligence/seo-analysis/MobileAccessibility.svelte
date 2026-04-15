<script lang="ts">
  import type { MobileAccessibilityResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Smartphone, Check, X } from 'lucide-svelte';

  type Props = {
      data: MobileAccessibilityResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Smartphone class="size-5" /> {m.seo_mobile_title()}
  </h3>

  {#if isLoading}
    <div class="space-y-3 animate-pulse"><div class="h-20 bg-surface rounded"></div></div>
  {:else if data}
    <div class="space-y-2 mb-4">
        {#each [
            { label: 'Viewport Meta', active: data.viewport_present },
            { label: 'Mobile Friendly', active: data.mobile_friendly }
        ] as metric, i (i)}
            <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
                <span class="text-sm text-primary-text">{metric.label}</span>
                {#if metric.active}
                    <Check class="size-4 text-green-400" />
                {:else}
                    <X class="size-4 text-red-400" />
                {/if}
            </div>
        {/each}
    </div>

    <!-- Alt Attributes -->
    <div class="p-3 bg-sunken rounded-lg border border-base mb-2">
        <p class="text-xs text-muted font-bold uppercase mb-2">Image Alt Coverage</p>
        <div class="flex items-center justify-between">
            <span class="text-sm text-primary-text">{data.alt_attributes.images_with_alt}/{data.alt_attributes.total_images} images</span>
            <span class="text-sm font-bold {data.alt_attributes.missing_alt === 0 ? 'text-green-400' : 'text-yellow-400'}">{data.alt_attributes.alt_coverage}</span>
        </div>
    </div>

    <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
        <span class="text-sm text-primary-text">ARIA Labels</span>
        <span class="text-sm font-mono text-accent">{data.aria_labels}</span>
    </div>
  {/if}
</div>
