<script lang="ts">
  import { ShieldAlert, ShieldX, ShieldCheck } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  
  let { confidence, className = '' }: { confidence: string; className?: string } = $props();
</script>

{#if confidence === 'High'}
  <span class={`inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-semibold bg-rose-500/10 text-rose-500 border border-rose-500/20 shadow-[0_0_8px_rgba(244,63,94,0.3)] ${className}`}>
    <ShieldAlert size={14} />
    {confidence}
  </span>
{:else if confidence === 'Medium'}
  <span class={`inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-semibold bg-amber-500/10 text-amber-500 border border-amber-500/20 shadow-[0_0_8px_rgba(245,158,11,0.2)] ${className}`}>
    <ShieldX size={14} />
    {confidence}
  </span>
{:else}
  <span class={`inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20 ${className}`}>
    <ShieldCheck size={14} />
    {confidence}
  </span>
{/if}
