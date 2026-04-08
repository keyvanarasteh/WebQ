<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";

  let { children } = $props();
</script>

<div class="flex h-screen w-full bg-[#09090b] text-white antialiased overflow-hidden">
  <Sidebar />
  <div class="flex-grow flex flex-col overflow-hidden">
    <Topbar />
    <main class="flex-1 overflow-y-auto p-6 relative">
      {@render children()}
    </main>
  </div>
</div>
