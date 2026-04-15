<script lang="ts">
  import type { ContentAnalysisResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { FileText, HelpCircle } from 'lucide-svelte';
  import ContentAnalysisGuide from './ContentAnalysisGuide.svelte';

  type Props = {
      data: ContentAnalysisResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<ContentAnalysisGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><FileText class="size-5" /> {m.seo_content_analysis_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_content_analysis_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="space-y-3 animate-pulse">
        <div class="h-10 bg-surface rounded"></div>
        <div class="h-10 bg-surface rounded"></div>
    </div>
  {:else if data}
    <!-- Word Count & Ratio -->
    <div class="grid grid-cols-3 gap-2 mb-4">
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-primary-text">{data.word_count}</p>
            <p class="text-xs text-muted">Words</p>
            <span class="text-xs {data.word_count_status === 'Good' ? 'text-green-400' : 'text-yellow-400'}">{data.word_count_status}</span>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-primary-text">{data.paragraphs}</p>
            <p class="text-xs text-muted">Paragraphs</p>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-accent">{data.text_to_html_ratio}</p>
            <p class="text-xs text-muted">Text:HTML</p>
        </div>
    </div>

    <!-- Heading Hierarchy -->
    <div class="mb-4">
        <p class="text-xs text-muted font-bold uppercase mb-2">Heading Structure</p>
        <div class="space-y-1">
            {#each Object.entries(data.headings) as [tag, info] (tag)}
                <div class="flex items-center gap-2 p-2 bg-sunken rounded border border-base">
                    <span class="text-xs font-mono text-accent font-bold w-8">{tag}</span>
                    <span class="text-xs text-muted">×{info.count}</span>
                    <span class="text-xs text-primary-text truncate flex-1">{info.texts[0] ?? ''}</span>
                </div>
            {/each}
        </div>
        {#if data.heading_issues.length > 0}
            <div class="mt-2 space-y-1">
                {#each data.heading_issues as issue, i (i)}
                    <p class="text-xs text-yellow-400 flex items-center gap-1">⚠ {issue}</p>
                {/each}
            </div>
        {/if}
    </div>

    <!-- Top Keywords -->
    {#if data.top_keywords.length > 0}
        <div>
            <p class="text-xs text-muted font-bold uppercase mb-2">Top Keywords</p>
            <div class="space-y-1">
                {#each data.top_keywords as kw (kw.word)}
                    <div class="flex items-center justify-between p-2 bg-sunken rounded border border-base">
                        <span class="text-sm font-mono text-primary-text">{kw.word}</span>
                        <div class="flex items-center gap-2">
                            <span class="text-xs text-muted">×{kw.count}</span>
                            <span class="text-xs text-accent bg-cyan-500/10 px-1.5 py-0.5 rounded">{kw.density}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <FileText class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
