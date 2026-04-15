<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
  import * as m from '$lib/paraglide/messages';
  import type { SeoAnalysisResult } from '$lib/types/intelligence';
  import { Search, HelpCircle } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';

  // Components — all 13 sections
  import SeoScoreGauge from '$lib/components/intelligence/seo-analysis/SeoScoreGauge.svelte';
  import BasicSeoOverview from '$lib/components/intelligence/seo-analysis/BasicSeoOverview.svelte';
  import ContentAnalysis from '$lib/components/intelligence/seo-analysis/ContentAnalysis.svelte';
  import TechnicalSeoCard from '$lib/components/intelligence/seo-analysis/TechnicalSeoCard.svelte';
  import SocialMediaCard from '$lib/components/intelligence/seo-analysis/SocialMediaCard.svelte';
  import AnalyticsTracker from '$lib/components/intelligence/seo-analysis/AnalyticsTracker.svelte';
  import PerformanceCard from '$lib/components/intelligence/seo-analysis/PerformanceCard.svelte';
  import MobileAccessibility from '$lib/components/intelligence/seo-analysis/MobileAccessibility.svelte';
  import SeoResourcesCheck from '$lib/components/intelligence/seo-analysis/SeoResourcesCheck.svelte';
  import SchemaMarkup from '$lib/components/intelligence/seo-analysis/SchemaMarkup.svelte';
  import LinkAnalysis from '$lib/components/intelligence/seo-analysis/LinkAnalysis.svelte';
  import ImageSeoCard from '$lib/components/intelligence/seo-analysis/ImageSeoCard.svelte';
  import PageSpeedFactors from '$lib/components/intelligence/seo-analysis/PageSpeedFactors.svelte';
  import SeoGuide from '$lib/components/recon/guides/SeoGuide.svelte';

  let targetDomain = $state('');
  let scanResult = $state<SeoAnalysisResult | null>(null);
  let scanError = $state<string | null>(null);
  let showGuide = $state(false);
  
  async function performScan() {
      if (!targetDomain) return;
      appState.setScanning(true, 'SEO ANALYSIS');
      scanError = null;
      
      try {
          scanResult = await invoke<SeoAnalysisResult>('scan_seo_analysis', { url: targetDomain });
      } catch (e) {
          scanError = e instanceof Error ? e.message : String(e);
          console.error('SEO Analysis failed:', e);
      } finally {
          appState.setScanning(false, '');
      }
  }
</script>

<div class="space-y-6 max-w-7xl mx-auto w-full">
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-base pb-6">
      <div class="flex items-center gap-3">
          <div>
              <h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.intel_seo_title()}</h1>
              <p class="text-muted mt-2">{m.intel_seo_desc()}</p>
          </div>
          <button
              onclick={() => showGuide = true}
              class="p-2 ml-2 transition-colors border rounded-lg bg-surface border-base text-muted hover:text-primary-text"
              title="View SecOps Guide"
          >
              <HelpCircle class="w-4 h-4" />
          </button>
      </div>

      <div class="flex items-center gap-2 w-full md:w-96">
          <div class="relative w-full">
              <Search class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-accent" />
              <input 
                  type="text" 
                  bind:value={targetDomain} 
                  onkeydown={(e) => e.key === 'Enter' && performScan()}
                  placeholder="Enter URL (e.g. https://example.com)"
                  class="w-full bg-background border border-base rounded-lg py-2 pl-10 pr-4 text-primary-text focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-on-accent font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? m.intel_seo_auditing() : m.intel_seo_audit_btn()}
          </button>
      </div>
  </div>

  <!-- Error State -->
  {#if scanError}
      <div class="p-4 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm font-mono">
          ⚠ SEO Analysis Error: {scanError}
      </div>
  {/if}

  {#if scanResult || appState.isScanning}
      <!-- Row 1: Score + Basic SEO + Content -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div class="lg:col-span-1">
              <SeoScoreGauge data={scanResult?.seo_score} isLoading={appState.isScanning} />
          </div>
          <div class="lg:col-span-1">
              <BasicSeoOverview data={scanResult?.basic_seo} isLoading={appState.isScanning} />
          </div>
          <div class="lg:col-span-2">
              <ContentAnalysis data={scanResult?.content_analysis} isLoading={appState.isScanning} />
          </div>
      </div>

      <!-- Row 2: Technical + Performance + Links -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <TechnicalSeoCard data={scanResult?.technical_seo} isLoading={appState.isScanning} />
          <PerformanceCard data={scanResult?.performance} isLoading={appState.isScanning} />
          <LinkAnalysis data={scanResult?.link_analysis} isLoading={appState.isScanning} />
      </div>

      <!-- Row 3: Social + Analytics + Resources -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <SocialMediaCard data={scanResult?.social_media} isLoading={appState.isScanning} />
          <AnalyticsTracker data={scanResult?.analytics} isLoading={appState.isScanning} />
          <SeoResourcesCheck data={scanResult?.seo_resources} isLoading={appState.isScanning} />
      </div>

      <!-- Row 4: Mobile + Schema + Images + Speed -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <MobileAccessibility data={scanResult?.mobile_accessibility} isLoading={appState.isScanning} />
          <SchemaMarkup data={scanResult?.schema_markup} isLoading={appState.isScanning} />
          <ImageSeoCard data={scanResult?.image_seo} isLoading={appState.isScanning} />
          <PageSpeedFactors data={scanResult?.page_speed_factors} isLoading={appState.isScanning} />
      </div>
  {/if}
</div>

<SeoGuide bind:isOpen={showGuide} />
