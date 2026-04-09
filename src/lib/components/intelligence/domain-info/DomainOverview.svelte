<script lang="ts">
  type ParsedResultProps = {
      isLoading: boolean;
      result: any | null; // Typed to DomainInfoResult
  };

  let { isLoading, result }: ParsedResultProps = $props();
</script>

<div class="bg-white/5 bg-background border border-base border-base rounded-xl p-6 shadow-sm dark:shadow-md h-full relative overflow-hidden transition-all duration-300">
  
  {#if isLoading}
      <div class="animate-pulse space-y-4">
          <div class="h-6 w-1/3 bg-gray-300 bg-surface rounded"></div>
          <div class="grid grid-cols-2 gap-4 mt-6">
              <div class="h-16 w-full bg-surface bg-surface/50 rounded-lg"></div>
              <div class="h-16 w-full bg-surface bg-surface/50 rounded-lg"></div>
          </div>
      </div>
  {:else if result}
      <!-- Input: whois, ipv4, web_server -->
      <!-- Output & Visualize: Luxe Card with Metric Grids -->
      <h3 class="text-lg font-bold text-inverse text-accent mb-4">Infrastructure Overview</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="p-4 rounded-lg bg-gray-100 bg-surface/30 border border-transparent border-base">
              <p class="text-xs text-muted uppercase font-semibold">Registrar</p>
              <p class="text-sm text-gray-800 text-primary-text mt-1 font-mono">{result.whois.registrar}</p>
          </div>
          <!-- More metric key-value cards will go here representing memory/web_server/ipv4 -->
          <!-- Responsive design ensures it wraps properly on mobile via grid-cols-1 -->
      </div>
  {/if}
</div>
