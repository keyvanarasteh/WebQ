<script lang="ts">
	import { onMount } from 'svelte';
	import './NeuralBootSequence.css';

	let { onComplete = () => {} } = $props();

	let splashClass = $state('');
	let show = $state(true);

	onMount(() => {
		// Timing sequence based on cinematic 3D loading overlay
		const timers = [
			setTimeout(() => {
				splashClass = 'split';
			}, 1800),
			setTimeout(() => {
				splashClass = 'split explode';
			}, 2500),
			setTimeout(() => {
				splashClass = 'split explode fade-out';
			}, 3000),
			setTimeout(() => {
				show = false;
				onComplete();
			}, 3600)
		];

		return () => {
			timers.forEach(clearTimeout);
		};
	});
</script>

{#if show}
	<div id="splashScreen" class="{splashClass}">
		<div class="glassmorphism-sweep"></div>
		<div class="cyber-grid"></div>
		
		<div class="tech-ring"></div>
		<div class="tech-ring inner"></div>
		<div class="tech-ring reverse"></div>
		
		<div class="shockwave"></div>
		
		<div class="cyber-lines">
			<!-- Lines -->
			<div class="line" style="transform: rotate(30deg); animation-delay: 0.4s;"></div>
			<div class="line" style="transform: rotate(150deg); animation-delay: 0.45s;"></div>
			<div class="line" style="transform: rotate(270deg); animation-delay: 0.5s;"></div>
			<div class="line" style="transform: rotate(210deg); animation-delay: 0.55s;"></div>
			<div class="line" style="transform: rotate(330deg); animation-delay: 0.6s;"></div>
			
			<!-- Nodes -->
			<div class="node" style="top: calc(50% + 10vw); left: calc(50% + 17.32vw); animation-delay: 0.6s;"></div>
			<div class="node" style="top: calc(50% + 10vw); left: calc(50% - 17.32vw); animation-delay: 0.65s;"></div>
			<div class="node" style="top: calc(50% - 20vw); left: 50%; animation-delay: 0.7s;"></div>
			<div class="node" style="top: calc(50% - 10vw); left: calc(50% - 17.32vw); animation-delay: 0.75s;"></div>
			<div class="node" style="top: calc(50% - 10vw); left: calc(50% + 17.32vw); animation-delay: 0.8s;"></div>
		</div>
		
		<div class="core-dot"></div>
		
		<div class="micro-terminal">
			<div class="mt-line">> MOUNTING WEBQ_CORE v1.0.0...</div>
			<div class="mt-line">> ESTABLISHING RUST_IPC BRIDGES...</div>
			<div class="mt-line">> INITIALIZING SYSTEM DIAGNOSTICS... [OK]</div>
			<div class="mt-line">> BYPASSING NEURAL BLOCKS...</div>
		</div>
		
		<div class="splash-logo">
			<span class="logo-p1">W</span><span class="logo-p2">EB</span><span class="logo-p3">Q</span>
		</div>
	</div>
{/if}
