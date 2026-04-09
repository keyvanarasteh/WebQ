<script lang="ts">
  import { XIcon } from 'lucide-svelte';
  import type { Snippet } from 'svelte';

  let { 
      isOpen = $bindable(false),
      title,
      icon: Icon,
      children
  } : {
      isOpen: boolean;
      title: string;
      icon: any; // Lucide Icon component
      children: Snippet;
  } = $props();

  function close() {
      isOpen = false;
  }
  
  $effect(() => {
      const handleEscape = (e: KeyboardEvent) => {
          if (e.key === 'Escape' && isOpen) close();
      };
      
      if (isOpen) document.addEventListener('keydown', handleEscape);
      return () => document.removeEventListener('keydown', handleEscape);
  });
</script>

{#if isOpen}
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div 
  class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm transition-all duration-300"
  onclick={close}
>
  <div 
      class="bg-background border flex flex-col border-base rounded-xl shadow-[0_0_50px_rgba(34,211,238,0.05)] w-full max-w-4xl max-h-[85vh] overflow-hidden"
      onclick={(e) => e.stopPropagation()}
  >
      <!-- HEADER -->
      <div class="px-6 py-4 border-b border-base bg-[#121214] flex justify-between items-center sticky top-0 z-10 shrink-0">
          <div class="flex items-center gap-3">
              {#if Icon}
                  <div class="p-2 bg-cyan-500/10 rounded-lg">
                      <Icon class="size-5 text-accent" />
                  </div>
              {/if}
              <div>
                  <h2 class="text-xl font-black text-gray-100 tracking-wide">{title}</h2>
                  <p class="text-xs font-mono text-accent/70 tracking-widest uppercase">SecOps & Secure Dev Intelligence</p>
              </div>
          </div>
          
          <button 
              onclick={close}
              class="p-2 text-muted hover:text-primary-text hover:bg-surface rounded-lg transition-colors cursor-pointer"
          >
              <XIcon class="size-5" />
          </button>
      </div>

      <!-- SCROLLABLE CONTENT AREA -->
      <div class="p-6 overflow-y-auto w-full h-full custom-scrollbar">
          {@render children()}
      </div>
  </div>
</div>
{/if}

<style>
/* Scoped minimal scrollbar for guides */
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #27272a;
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #3f3f46;
}
</style>
