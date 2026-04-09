<script lang="ts">
	import { onMount } from 'svelte';
	// @ts-expect-error SvelteKit virtual modules generated dynamically
	import { goto } from '$app/navigation';
	// @ts-expect-error SvelteKit virtual modules generated dynamically
	import { resolve } from '$app/paths';

	// The Assessment link from the App Sidebar natively targets this component.
	// Seamlessly bridge the user into the primary feature.
	onMount(() => {
		goto(resolve('/assessment/security-posture'), { replaceState: true });
	});
</script>

<!-- Keep layout dark while bridging -->
<div class="h-full w-full bg-qix-obsidian"></div>
