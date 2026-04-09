<script lang="ts">
  type PortSecurityProps = {
      isLoading: boolean;
      ports: string[] | undefined;
      score: number | undefined;
  };

  let { isLoading, ports, score }: PortSecurityProps = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all duration-300">
  <h3 class="text-lg font-bold text-accent mb-4">Security Assessment</h3>
  
  {#if isLoading}
      <div class="space-y-4">
          <div class="h-10 w-full bg-surface/50 rounded-lg animate-pulse"></div>
          <div class="flex gap-2">
              <div class="h-6 w-12 bg-surface rounded animate-pulse"></div>
              <div class="h-6 w-12 bg-surface rounded animate-pulse"></div>
          </div>
      </div>
  {:else}
      <!-- Output: Security Score 0-100 -->
      <!-- Visualize: NumberFlow animated counter and progress bar -->
      <div class="mb-6">
          <p class="text-5xl font-black {score && score > 70 ? 'text-green-500' : 'text-orange-500'}">{score || 0}</p>
          <p class="text-sm text-muted font-medium tracking-wide">TOTAL SECURITY SCORE</p>
      </div>

      <!-- Visualize: Open ports grid with colored danger indicators -->
      <div class="border-t border-base pt-4">
          <p class="text-xs text-muted uppercase tracking-widest mb-3">Open Ports</p>
          <div class="flex flex-wrap gap-2">
              {#if ports && ports.length > 0}
                  {#each ports as port (port)}
                      {@const isDanger = port.startsWith('21') || port.startsWith('22') || port.startsWith('3306')}
                      <span class="px-2 py-1 text-xs font-mono rounded-md border 
                          {isDanger ? 'bg-red-500/10 border-red-500/50 text-red-400' : 'bg-cyan-500/10 border-cyan-500/30 text-accent'}">
                          {port}
                      </span>
                  {/each}
              {:else}
                  <span class="text-sm text-muted">No open ports detected.</span>
              {/if}
          </div>
      </div>
  {/if}
</div>
