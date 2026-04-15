<script lang="ts">
  import { FileSearch, Check, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';

  type Props = {
      data: Record<string, string> | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <FileSearch class="size-5" /> {m.seo_resources_title()}
  </h3>

  {#if isLoading}
    <div class="space-y-2 animate-pulse">
        {#each Array(4) as _, i (i)}<div class="h-10 bg-surface rounded"></div>{/each}
    </div>
  {:else if data}
    <div class="space-y-1.5">
        {#each Object.entries(data) as [file, status] (file)}
            <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
                <span class="text-sm font-mono text-primary-text">{file}</span>
                {#if status === 'Found'}
                    <Check class="size-4 text-green-400" />
                {:else}
                    <X class="size-4 text-red-400" />
                {/if}
            </div>
        {/each}
    </div>
  {/if}
</div>
