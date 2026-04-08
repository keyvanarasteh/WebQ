<script lang="ts">
  type SecurityHeaderInfo = {
      strict_transport_security: boolean;
      x_frame_options: boolean;
      x_content_type_options: boolean;
      content_security_policy: boolean;
  };
  
  let { data, isLoading } = $props<{ data: SecurityHeaderInfo | undefined, isLoading: boolean }>();
</script>

<div class="col-span-full bg-gray-50/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm mt-6">
  <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400 mb-4">Security Headers Assessment (OWASP)</h3>
  
  {#if isLoading}
    <div class="h-24 bg-gray-200 dark:bg-[#27272a] rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
        {#each [
            { name: 'Strict-Transport-Security', val: data.strict_transport_security, desc: 'Prevents MiTM attacks routing HTTP -> HTTPS' },
            { name: 'X-Frame-Options', val: data.x_frame_options, desc: 'Prevents Clickjacking UI redressing' },
            { name: 'X-Content-Type-Options', val: data.x_content_type_options, desc: 'Mitigates MIME type sniffing execution' },
            { name: 'Content-Security-Policy', val: data.content_security_policy, desc: 'Core defense against Cross-site Scripting (XSS)' },
        ] as header (header.name)}
            <div class="p-4 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] hover:border-gray-300 dark:hover:border-[#3f3f46] transition-colors rounded-lg flex flex-col justify-between h-full">
                <div class="mb-4">
                    <h4 class="text-sm font-bold text-gray-900 dark:text-gray-200 mb-1">{header.name}</h4>
                    <p class="text-xs text-gray-500 leading-relaxed">{header.desc}</p>
                </div>
                <div>
                    {#if header.val}
                        <span class="inline-flex items-center px-2 py-1 bg-green-500/10 text-green-500 dark:text-green-400 text-xs font-black rounded border border-green-500/20 uppercase tracking-widest">Configured</span>
                    {:else}
                        <span class="inline-flex items-center px-2 py-1 bg-red-500/10 text-red-500 text-xs font-black rounded border border-red-500/20 uppercase tracking-widest">Missing</span>
                    {/if}
                </div>
            </div>
        {/each}
    </div>
  {:else}
      <div class="text-gray-500 text-sm py-4">Awaiting scan data...</div>
  {/if}
</div>
