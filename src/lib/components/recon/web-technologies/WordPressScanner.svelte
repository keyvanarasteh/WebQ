<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  type WpUser = { id: number, name: string, slug: string };
  type WordPressAnalysis = {
      is_wordpress: boolean;
      version: string | null;
      themes: string[];
      plugins: string[];
      users: WpUser[];
  };
  
  let { data, isLoading } = $props<{ data: WordPressAnalysis | undefined, isLoading: boolean }>();
</script>

<div class="bg-background/5 bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent">{m.wp_cms_fingerprinting()}</h3>
      {#if data?.is_wordpress}
          <span class="px-2 py-1 bg-blue-500/10 text-blue-500 border border-blue-500/30 rounded-md text-[10px] font-black tracking-widest animate-pulse">{m.wp_detected()}</span>
      {/if}
  </div>
  
  {#if isLoading}
    <div class="h-32 bg-surface rounded animate-pulse"></div>
  {:else if data}
      {#if data.is_wordpress}
          <div class="space-y-4">
              <div class="p-4 bg-background border border-base rounded-lg border-l-4 border-l-blue-500">
                  <p class="text-xs text-muted font-bold mb-1">{m.wp_version_detection()}</p>
                  <p class="text-sm font-mono text-inverse text-primary-text">{data.version || m.wp_version_hidden()}</p>
              </div>

              {#if data.users.length > 0}
                  <div class="p-4 bg-background border border-base rounded-lg">
                      <p class="text-xs text-red-500 font-bold mb-2 uppercase tracking-wide">{m.wp_exposed_users()}</p>
                      <ul class="space-y-1">
                          {#each data.users as user (user.id)}
                              <li class="text-xs md:text-sm text-inverse text-primary-text border-b border-base pb-1 font-mono last:border-0 last:pb-0">
                                  ID: {user.id} <span class="mx-2 text-muted">|</span> <span class="text-cyan-600 text-accent">{user.name}</span> <span class="text-muted">(@{user.slug})</span>
                              </li>
                          {/each}
                      </ul>
                  </div>
              {/if}
          </div>
      {:else}
          <div class="text-muted text-sm py-4 flex items-center justify-center border border-base rounded-lg bg-background min-h-[140px]">
              {m.wp_no_cms()}
          </div>
      {/if}
  {:else}
      <div class="text-muted text-sm py-4">{m.tech_awaiting()}</div>
  {/if}
</div>
