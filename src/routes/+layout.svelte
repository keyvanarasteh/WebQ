<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
  import DependencyAlert from "$lib/components/layout/DependencyAlert.svelte";
  import CommandPalette from "$lib/components/ui/CommandPalette.svelte";
  import NeuralBootSequence from "$lib/components/ui/NeuralBootSequence.svelte";
  import { setLanguageTag } from "$lib/paraglide/runtime";
  import { appState } from "$lib/stores/AppState.svelte";
  import { Toaster, toast } from 'svelte-sonner';
  import { Minus, Square, X, Monitor } from 'lucide-svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // Global error handler for unhandled backend/Tauri exceptions
  if (typeof window !== "undefined") {
    window.addEventListener('unhandledrejection', (event) => {
        const err = event.reason;
        console.error("Unhandled Promise Rejection:", err);
        const msg = typeof err === 'string' ? err : (err?.message || "An unexpected error occurred");
        toast.error("Application Error", { description: msg });
    });
  }

  if (typeof localStorage !== "undefined") {
      setLanguageTag((localStorage.getItem('webq-lang') || 'en') as "en" | "tr");
      const storedAnim = localStorage.getItem('webq-animations');
      if (storedAnim !== null) {
          appState.animationsEnabled = storedAnim === 'true';
      }
  }

  $effect(() => {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      const isDark = appState.theme === 'dark' || (appState.theme === 'system' && prefersDark);
      document.documentElement.classList.toggle('dark', isDark);
      
      document.documentElement.classList.toggle('disable-animations', !appState.animationsEnabled);
      if (typeof localStorage !== "undefined") {
          localStorage.setItem('webq-animations', appState.animationsEnabled.toString());
      }
  });

  let { children } = $props();
  // Svelte 5 universal run: check sessionStorage safely
  let isBooted = $state(typeof sessionStorage !== 'undefined' ? sessionStorage.getItem('webq-booted') === 'true' : false);
  let showCloseConfirm = $state(false);

  function handleBootComplete() {
      isBooted = true;
      if (typeof sessionStorage !== 'undefined') {
          sessionStorage.setItem('webq-booted', 'true');
      }
  }
</script>

{#if !isBooted}
<NeuralBootSequence onComplete={handleBootComplete} />
{/if}

{#if isBooted}
<div class="h-10 w-full fixed top-0 left-0 z-900 select-none">
    <!-- Drag layer -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
        class="absolute inset-0 bg-background border-b border-base"
        data-tauri-drag-region
        ondblclick={() => getCurrentWindow().toggleMaximize()}
    ></div>

    <!-- Foreground content -->
    <div class="relative z-10 h-full flex items-center justify-between px-4 pointer-events-none">
        <div class="flex items-center gap-2">
            <Monitor size={14} class="text-accent" />
            <span class="text-xs font-semibold text-muted tracking-wide">WEBQ</span>
        </div>
        
        <div class="flex items-center gap-1 pointer-events-auto">
            <button onclick={() => getCurrentWindow().minimize()} class="p-1.5 rounded-md text-muted hover:bg-surface hover:text-primary-text transition-colors" title="Minimize" aria-label="Minimize window">
                <Minus size={14} />
            </button>
            <button onclick={() => getCurrentWindow().toggleMaximize()} class="p-1.5 rounded-md text-muted hover:bg-surface hover:text-primary-text transition-colors" title="Maximize" aria-label="Maximize window">
                <Square size={14} />
            </button>
            <button onclick={() => showCloseConfirm = true} class="p-1.5 rounded-md text-muted hover:bg-red-500 hover:text-white transition-colors" title="Close" aria-label="Close window">
                <X size={14} />
            </button>
        </div>
    </div>
</div>
{/if}

<div class="flex h-screen w-full pt-10 bg-background text-primary-text antialiased overflow-hidden transition-all duration-1000 {isBooted ? 'opacity-100' : 'opacity-0'}">
  {#if isBooted}
  <Sidebar />
  <div class="grow flex flex-col min-w-0 relative">
    <DependencyAlert />
    <Topbar />
    <Toaster position="top-right" richColors theme={appState.theme} expand={false} />
    <CommandPalette />
    <main class="flex-1 scroll-optimized p-6 relative z-0">
      {@render children()}
    </main>
  </div>
  {/if}
</div>

{#if showCloseConfirm}
<div class="fixed inset-0 z-9999 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
  <div class="bg-surface border border-base shadow-2xl rounded-lg p-6 max-w-sm w-full flex flex-col gap-4">
    <h3 class="text-lg font-semibold text-primary-text">Exit WebQ?</h3>
    <p class="text-sm text-secondary-text">Are you sure you want to close the application? Any ongoing intelligence scans will be terminated.</p>
    <div class="flex justify-end gap-2 mt-4">
      <button onclick={() => showCloseConfirm = false} class="px-4 py-2 text-sm rounded-md bg-sunken text-primary-text hover:brightness-110 transition-colors font-medium">Cancel</button>
      <button onclick={() => getCurrentWindow().close()} class="px-4 py-2 text-sm rounded-md bg-red-600 hover:bg-red-500 text-white transition-colors font-medium">Close App</button>
    </div>
  </div>
</div>
{/if}
