<script lang="ts">
  type SslStatusProps = {
      isLoading: boolean;
      ssl: any | null; // SslInfo
  };

  let { isLoading, ssl }: SslStatusProps = $props();
</script>

<div class="bg-white/5 bg-background border border-base border-base rounded-xl p-6 shadow-sm dark:shadow-md transition-all duration-300">
  <h3 class="text-lg font-bold text-inverse text-accent mb-4">SSL & Cryptography</h3>

  {#if isLoading}
      <div class="flex items-center justify-center h-32">
          <!-- Circular loader skeleton -->
          <div class="w-16 h-16 border-4 border-base border-base border-t-cyan-500 rounded-full animate-spin"></div>
      </div>
  {:else if ssl}
      <!-- Input: ssl.days_until_expiry -->
      <!-- Output/Visualize: Circular Gauge showing days remaining -->
      <div class="flex flex-col items-center">
          <div class="relative w-24 h-24 flex items-center justify-center rounded-full border-4 {ssl.days_until_expiry > 30 ? 'border-green-500' : 'border-red-500'}">
              <span class="text-2xl font-bold text-primary-text">{ssl.days_until_expiry || '?'}</span>
          </div>
          <p class="text-sm mt-3 text-muted">Days Remaining</p>
      </div>
  {/if}
</div>
