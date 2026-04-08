<script lang="ts">
  import { Info } from 'lucide-svelte';
  import SeoGuide from '$lib/components/recon/guides/SeoGuide.svelte';
  import * as m from '$lib/paraglide/messages';
  type BasicSeoResult = {
      title: string;
      description: string;
      keywords: string;
      h1_count: number;
      heading_structure: boolean;
  };

  let { data, isLoading } = $props<{ data: BasicSeoResult | undefined, isLoading: boolean }>();
  let isGuideOpen = $state(false);
</script>

<div class="bg-white/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm h-full">
  <SeoGuide bind:isOpen={isGuideOpen} />
  
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400">{m.seo_basic_meta_data()}</h3>
      <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-cyan-500/10 rounded-full text-cyan-500 transition-colors" title={m.secops_guide_title()}><Info class="size-4" /></button>
  </div>
  
  {#if isLoading}
    <div class="space-y-4 animate-pulse">
        <div class="h-10 bg-gray-200 dark:bg-[#27272a] rounded"></div>
        <div class="h-16 bg-gray-200 dark:bg-[#27272a] rounded"></div>
    </div>
  {:else if data}
    <div class="grid grid-cols-1 gap-4">
        <!-- Title -->
        <div class="p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a]">
            <p class="text-xs text-gray-500 font-bold mb-1">{m.seo_page_title()}</p>
            <p class="text-sm {data.title ? 'text-gray-900 dark:text-white' : 'text-red-500'}">{data.title || m.seo_missing_title()}</p>
        </div>
        
        <!-- Description -->
        <div class="p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a]">
            <p class="text-xs text-gray-500 font-bold mb-1">{m.seo_meta_desc()}</p>
            <p class="text-sm {data.description ? 'text-gray-900 dark:text-white' : 'text-red-500'}">{data.description || m.seo_missing_desc()}</p>
        </div>

        <!-- H1 Tags -->
        <div class="p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a] flex justify-between items-center">
            <div>
                <p class="text-xs text-gray-500 font-bold mb-1">{m.seo_h1_tags()}</p>
                <p class="text-sm text-gray-900 dark:text-white">{m.seo_h1_count({ count: data.h1_count })}</p>
            </div>
            {#if data.h1_count === 1}
                <span class="px-2 py-1 bg-green-500/10 text-green-500 dark:text-green-400 text-xs rounded border border-green-500/30">{m.seo_tag_optimal()}</span>
            {:else if data.h1_count > 1}
                <span class="px-2 py-1 bg-yellow-500/10 text-yellow-600 dark:text-yellow-500 text-xs rounded border border-yellow-500/30">{m.seo_tag_multiple()}</span>
            {:else}
                <span class="px-2 py-1 bg-red-500/10 text-red-500 text-xs rounded border border-red-500/30">{m.seo_tag_missing()}</span>
            {/if}
        </div>
    </div>
  {:else}
      <div class="text-gray-500 text-sm py-4">{m.seo_no_data()}</div>
  {/if}
</div>
