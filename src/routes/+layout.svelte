<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
  import { setLanguageTag } from "$lib/paraglide/runtime";
  import { appState } from "$lib/stores/AppState.svelte";

  if (typeof localStorage !== "undefined") {
      setLanguageTag((localStorage.getItem('webq-lang') || 'en') as "en" | "tr");
  }

  $effect(() => {
      document.documentElement.classList.toggle('dark', appState.theme === 'dark');
  });

  let { children } = $props();
</script>

<div class="flex h-screen w-full bg-white dark:bg-qix-obsidian text-black dark:text-white antialiased overflow-hidden transition-colors duration-300">
  <Sidebar />
  <div class="flex-grow flex flex-col overflow-hidden">
    <Topbar />
    <main class="flex-1 overflow-y-auto p-6 relative">
      {@render children()}
    </main>
  </div>
</div>
