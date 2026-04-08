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

<div class="bg-gray-50/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm mb-6">
  <div class="flex justify-between items-center mb-6 border-b border-gray-200 dark:border-[#27272a] pb-4">
      <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400">{m.val_diagnostics_title()}</h3>
      {#if isLoading}
          <span class="text-xs text-cyan-500 dark:text-cyan-400 animate-pulse font-mono font-bold tracking-widest">{m.val_running()}</span>
      {/if}
  </div>

  {#if isLoading && !stats}
      <div class="w-full h-8 bg-gray-200 dark:bg-[#27272a] rounded-full animate-pulse mt-4"></div>
  {:else if stats}
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div class="text-center p-3 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] rounded-lg">
              <p class="text-xs text-gray-500 font-black tracking-widest mb-1">{m.val_total()}</p>
              <p class="text-2xl font-black text-gray-900 dark:text-gray-200">{stats.total}</p>
          </div>
          <div class="text-center p-3 bg-green-50/50 dark:bg-[#121214] border border-green-500/20 rounded-lg">
              <p class="text-xs text-green-600 dark:text-green-500 font-black tracking-widest mb-1">{m.val_valid()}</p>
              <p class="text-2xl font-black text-green-600 dark:text-green-400">{stats.valid}</p>
          </div>
          <div class="text-center p-3 bg-red-50/50 dark:bg-[#121214] border border-red-500/20 rounded-lg">
              <p class="text-xs text-red-600 dark:text-red-500 font-black tracking-widest mb-1">{m.val_invalid()}</p>
              <p class="text-2xl font-black text-red-600 dark:text-red-400">{stats.invalid}</p>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] rounded-lg">
              <p class="text-xs text-gray-500 font-black tracking-widest mb-1">{m.val_skipped()}</p>
              <p class="text-2xl font-black text-gray-700 dark:text-gray-400">{stats.skipped}</p>
          </div>
      </div>
      
      <div class="w-full h-4 bg-gray-200 dark:bg-[#18181b] rounded-full overflow-hidden flex border border-gray-300 dark:border-[#27272a] shadow-inner">
          <div class="h-full bg-green-500 transition-all duration-1000" style="width: {validPercent}%"></div>
          <div class="h-full bg-red-500 transition-all duration-1000" style="width: {invalidPercent}%"></div>
      </div>
  {:else}
      <div class="text-gray-500 text-sm font-mono text-center py-4 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] rounded-lg">
          {m.val_prompt()}
      </div>
  {/if}
</div>
