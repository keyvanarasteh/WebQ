<script lang="ts">
  import { Database, Trash2, Download, AlertTriangle } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';
  
  let showConfirmModal = $state(false);
  let cleaning = $state(false);
  
  let dbSize = $state("0.00 MB");
  let totalScans = $state(0);

  async function loadStats() {
      try {
          const stats = await invoke<{ size_string: string, total_scans: number }>('get_db_stats');
          dbSize = stats.size_string;
          totalScans = stats.total_scans;
      } catch (err) {
          console.error("Failed to load DB Stats:", err);
      }
  }

  onMount(() => {
      loadStats();
  });

  function handleBackup() {
      // Backup logic reserved
      toast.info("Database Export", { description: "Coming soon." });
  }

  async function handleClean() {
      cleaning = true;
      try {
          await invoke('nuke_history');
          toast.success("Database Erased", { description: "All historical scan records destroyed." });
          await loadStats();
      } catch (err) {
          toast.error("Database Wipe Failed", { description: String(err) });
      } finally {
          cleaning = false;
          showConfirmModal = false;
      }
  }
</script>

<section class="bg-surface/20 border border-base rounded-xl p-6 shadow-lg">
  <div class="flex items-center gap-3 mb-4">
    <Database class="w-6 h-6 text-accent" />
    <h2 class="text-xl font-semibold text-primary-text">Database Management</h2>
  </div>
  
  <div class="space-y-4">
    <div class="flex items-center justify-between p-4 bg-background rounded-lg border border-base flex-wrap gap-4">
      <div>
        <p class="font-medium text-primary-text">Local Storage Metrics</p>
        <p class="text-sm text-muted mt-1">Current SQLite DB footprint: <span class="bg-surface px-2 py-0.5 rounded border border-base text-accent font-mono">{dbSize}</span> | Scans: <span class="text-primary-text font-bold">{totalScans}</span></p>
      </div>
      <div class="flex gap-2">
        <button onclick={handleBackup} class="flex items-center gap-2 px-4 py-2 rounded-md bg-surface text-primary-text font-medium border border-base hover:bg-base transition-colors hover:text-cyan-400">
          <Download class="w-4 h-4" /> Backup DB
        </button>
        <button onclick={() => showConfirmModal = true} class="flex items-center gap-2 px-4 py-2 rounded-md bg-rose-500/10 text-rose-500 font-medium border border-rose-500/20 hover:bg-rose-500/20 transition-colors">
          <Trash2 class="w-4 h-4" /> Nuke History
        </button>
      </div>
    </div>
  </div>

  {#if showConfirmModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
        <div class="bg-background border border-rose-500/30 rounded-2xl w-full max-w-md shadow-2xl p-6 relative overflow-hidden">
            <div class="absolute -right-10 -top-10 w-32 h-32 bg-rose-500/10 rounded-full blur-2xl pointer-events-none"></div>
            
            <div class="flex items-center gap-3 mb-4 text-rose-500">
                <AlertTriangle class="w-8 h-8" />
                <h2 class="text-xl font-bold">Nuke Database?</h2>
            </div>
            
            <p class="text-muted mb-6">You are about to irreversibly delete all reconnaissance and security scan records. This action cannot be undone unless you have recently backed up your database.</p>
            
            <div class="flex justify-end gap-3 mt-8">
                <button 
                    onclick={() => showConfirmModal = false}
                    disabled={cleaning}
                    class="px-5 py-2 rounded-lg bg-surface hover:bg-base text-primary-text font-medium border border-base transition-colors"
                >
                    Cancel
                </button>
                <button 
                    onclick={handleClean}
                    disabled={cleaning}
                    class="px-5 py-2 rounded-lg bg-rose-500 text-white font-bold hover:bg-rose-600 shadow-[0_0_15px_rgba(244,63,94,0.4)] transition-all flex items-center gap-2 disabled:opacity-70"
                >
                    {#if cleaning}
                        <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                        Erasing Data...
                    {:else}
                        <Trash2 class="w-4 h-4" /> Confirm Wipe
                    {/if}
                </button>
            </div>
        </div>
    </div>
  {/if}
</section>
