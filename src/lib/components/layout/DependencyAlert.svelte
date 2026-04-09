<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { AlertTriangle, X } from 'lucide-svelte';

  type DependencyStatus = {
      nmap: boolean;
      dig: boolean;
      openssl: boolean;
  };

  let missingDeps: string[] = $state([]);
  let dismissed = $state(false);

  onMount(async () => {
      try {
          const status: DependencyStatus = await invoke('check_dependencies');
          let missing = [];
          if (!status.nmap) missing.push('nmap');
          if (!status.dig) missing.push('dnsutils');
          if (!status.openssl) missing.push('openssl');
          
          missingDeps = missing;
      } catch (e) {
          console.error("Failed to check system dependencies", e);
      }
  });
</script>

{#if missingDeps.length > 0 && !dismissed}
  <div class="bg-red-500/10 border-b border-red-500/20 px-4 py-3 flex items-start sm:items-center justify-between gap-4 w-full z-50">
      <div class="flex items-start sm:items-center gap-3">
          <AlertTriangle class="w-5 h-5 text-red-500 shrink-0 mt-0.5 sm:mt-0" />
          <div class="text-sm text-red-200">
              <span class="font-bold text-red-500 uppercase tracking-widest text-xs mr-2 border border-red-500/50 bg-red-950/20 rounded px-1.5 py-0.5">Missing Dependencies</span> 
              Core features will be limited. Please install: 
              <span class="font-mono bg-background px-2 py-0.5 rounded text-red-300 ml-1 border border-red-900/50">
                  sudo apt install {missingDeps.join(' ')}
              </span>
          </div>
      </div>
      <button 
          onclick={() => dismissed = true} 
          class="text-red-400 hover:text-red-200 hover:bg-red-500/10 transition-colors p-1.5 rounded-md shrink-0"
      >
          <X class="w-4 h-4" />
      </button>
  </div>
{/if}
