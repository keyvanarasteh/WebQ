<script lang="ts">
  import { ShieldPlus, Network, Globe, BookOpenCheck } from 'lucide-svelte';
  import DomainsRoadmap from '$lib/components/roadmap/DomainsRoadmap.svelte';
  import PortsRoadmap from '$lib/components/roadmap/PortsRoadmap.svelte';
  import HeadersRoadmap from '$lib/components/roadmap/HeadersRoadmap.svelte';
  import StatusCodesRoadmap from '$lib/components/roadmap/StatusCodesRoadmap.svelte';
  import { ServerCrash } from 'lucide-svelte';

  let activeTab = $state<'domains' | 'ports' | 'headers' | 'status_codes'>('domains');
</script>

<svelte:head>
  <title>Infrastructure Roadmap | WebQ</title>
</svelte:head>

<div class="max-w-7xl mx-auto w-full px-4 py-8 animate-fade-in space-y-12 mb-20">
  <!-- Header -->
  <div>
      <h1 class="text-4xl font-black text-primary-text tracking-widest uppercase flex items-center gap-4">
          <ShieldPlus class="size-8 text-cyan-400" />
          Infrastructure Roadmap
      </h1>
      <p class="text-muted mt-3 text-lg max-w-3xl">
          Complete transparent tracking of our TCP intelligence integrations. Web-Analyzer connects directly to native remote systems for real-time security scanning and footprinting.
      </p>
  </div>

  <!-- Segmented Tabs -->
  <div class="inline-flex bg-base/50 p-1 rounded-xl border border-base backdrop-blur-sm shadow-xl flex-wrap">
      <button 
          onclick={() => activeTab = 'domains'}
          class="px-5 sm:px-8 py-2.5 rounded-lg text-xs sm:text-sm font-bold uppercase tracking-widest flex items-center gap-2 transition-all {activeTab === 'domains' ? 'bg-cyan-500/20 text-primary-text border border-cyan-500/30' : 'text-muted hover:text-primary-text hover:bg-surface'}"
      >
          <Globe class="size-4" />
          WHOIS TLD Matrix
      </button>
      <button 
          onclick={() => activeTab = 'ports'}
          class="px-5 sm:px-8 py-2.5 rounded-lg text-xs sm:text-sm font-bold uppercase tracking-widest flex items-center gap-2 transition-all {activeTab === 'ports' ? 'bg-magenta-500/20 text-primary-text border border-magenta-500/30' : 'text-muted hover:text-primary-text hover:bg-surface'}"
      >
          <Network class="size-4" />
          OSINT Port Matrix
      </button>
      <button 
          onclick={() => activeTab = 'headers'}
          class="px-5 sm:px-8 py-2.5 rounded-lg text-xs sm:text-sm font-bold uppercase tracking-widest flex items-center gap-2 transition-all {activeTab === 'headers' ? 'bg-green-500/20 text-primary-text border border-green-500/30' : 'text-muted hover:text-primary-text hover:bg-surface'}"
      >
          <BookOpenCheck class="size-4" />
          Security Headers Matrix
      </button>
      <button 
          onclick={() => activeTab = 'status_codes'}
          class="px-5 sm:px-8 py-2.5 rounded-lg text-xs sm:text-sm font-bold uppercase tracking-widest flex items-center gap-2 transition-all {activeTab === 'status_codes' ? 'bg-rose-500/20 text-primary-text border border-rose-500/30' : 'text-muted hover:text-primary-text hover:bg-surface'}"
      >
          <ServerCrash class="size-4" />
          HTTP Codes Matrix
      </button>
  </div>

  <div class="relative w-full">
      {#if activeTab === 'domains'}
          <DomainsRoadmap />
      {:else if activeTab === 'ports'}
          <PortsRoadmap />
      {:else if activeTab === 'headers'}
          <HeadersRoadmap />
      {:else}
          <StatusCodesRoadmap />
      {/if}
  </div>
</div>
