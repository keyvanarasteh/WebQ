<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  type ValidationStats = {
      total: number;
      valid: number;
      invalid: number;
      skipped: number;
  };
  
  let { stats, isLoading } = $props<{ stats: ValidationStats | undefined, isLoading: boolean }>();
  
  let validPercent = $derived(stats && stats.total > 0 ? (stats.valid / stats.total) * 100 : 0);
  let invalidPercent = $derived(stats && stats.total > 0 ? (stats.invalid / stats.total) * 100 : 0);
</script>

<div class="bg-background/5 bg-background border border-base border-base rounded-xl p-6 shadow-sm mb-6">
  <div class="flex justify-between items-center mb-6 border-b border-base border-base pb-4">
      <h3 class="text-lg font-bold text-inverse text-accent">{m.val_diagnostics_title()}</h3>
      {#if isLoading}
          <span class="text-xs text-accent text-accent animate-pulse font-mono font-bold tracking-widest">{m.val_running()}</span>
      {/if}
  </div>

  {#if isLoading && !stats}
      <div class="w-full h-8 bg-surface bg-surface rounded-full animate-pulse mt-4"></div>
  {:else if stats}
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div class="text-center p-3 bg-background dark:bg-[#121214] border border-base border-base rounded-lg">
              <p class="text-xs text-muted font-black tracking-widest mb-1">{m.val_total()}</p>
              <p class="text-2xl font-black text-inverse text-primary-text">{stats.total}</p>
          </div>
          <div class="text-center p-3 bg-green-50/50 dark:bg-[#121214] border border-green-500/20 rounded-lg">
              <p class="text-xs text-green-600 dark:text-green-500 font-black tracking-widest mb-1">{m.val_valid()}</p>
              <p class="text-2xl font-black text-green-600 dark:text-green-400">{stats.valid}</p>
          </div>
          <div class="text-center p-3 bg-red-50/50 dark:bg-[#121214] border border-red-500/20 rounded-lg">
              <p class="text-xs text-red-600 dark:text-red-500 font-black tracking-widest mb-1">{m.val_invalid()}</p>
              <p class="text-2xl font-black text-red-600 dark:text-red-400">{stats.invalid}</p>
          </div>
          <div class="text-center p-3 bg-background dark:bg-[#121214] border border-base border-base rounded-lg">
              <p class="text-xs text-muted font-black tracking-widest mb-1">{m.val_skipped()}</p>
              <p class="text-2xl font-black text-gray-700 text-muted">{stats.skipped}</p>
          </div>
      </div>
      
      <div class="w-full h-4 bg-surface dark:bg-[#18181b] rounded-full overflow-hidden flex border border-gray-300 border-base shadow-inner">
          <div class="h-full bg-green-500 transition-all duration-1000" style="width: {validPercent}%"></div>
          <div class="h-full bg-red-500 transition-all duration-1000" style="width: {invalidPercent}%"></div>
      </div>
  {:else}
      <div class="text-muted text-sm font-mono text-center py-4 bg-background dark:bg-[#121214] border border-base border-base rounded-lg">
          {m.val_prompt()}
      </div>
  {/if}
</div>
