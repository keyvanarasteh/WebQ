<script lang="ts">
	import { onMount } from 'svelte';
	// @ts-expect-error SvelteKit virtual modules generated dynamically
	import { goto } from '$app/navigation';
	// @ts-expect-error SvelteKit virtual modules generated dynamically
	import { resolve } from '$app/paths';

	// The Recon link from the App Sidebar natively targets this component.
	// Seamlessly bridge the user into the primary Phase 3 feature.
	onMount(() => {
		goto(resolve('/recon/subdomain-discovery'), { replaceState: true });
	});
</script>

<!-- Keep layout dark while bridging -->
<div class="h-full w-full bg-[#09090b]"></div>
