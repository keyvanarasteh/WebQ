<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
  import DependencyAlert from "$lib/components/layout/DependencyAlert.svelte";
  import CommandPalette from "$lib/components/ui/CommandPalette.svelte";
  import { setLanguageTag } from "$lib/paraglide/runtime";
  import { appState } from "$lib/stores/AppState.svelte";
  import { Toaster, toast } from 'svelte-sonner';

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
  }

  $effect(() => {
      document.documentElement.classList.toggle('dark', appState.theme === 'dark');
  });

  let { children } = $props();
</script>

<div class="flex h-screen w-full bg-background text-primary-text antialiased overflow-hidden transition-colors duration-300">
  <Sidebar />
  <div class="grow flex flex-col overflow-hidden relative">
    <DependencyAlert />
    <Topbar />
    <Toaster position="top-right" richColors theme={appState.theme} expand={false} />
    <CommandPalette />
    <main class="flex-1 overflow-y-auto p-6 relative z-0">
      {@render children()}
    </main>
  </div>
</div>
