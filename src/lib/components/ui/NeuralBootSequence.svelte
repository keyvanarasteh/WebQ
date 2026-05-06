<script lang="ts">
	import { onMount } from 'svelte';
	import './NeuralBootSequence.css';

	let { onComplete = () => {} } = $props();

	let show = $state(true);
	let fadeOut = $state(false);
	let decodedText = $state('');
	let statusText = $state('VULNERABILITY ANALYSIS ENGINE');

	onMount(() => {
		const targetWord = 'WEBQ';
		const chars = '01X';
		let iterations = 0;
		const maxIterations = 20;

		const interval = setInterval(() => {
			decodedText = targetWord.split('').map((letter, index) => {
				if (index < iterations / (maxIterations / targetWord.length)) {
					return letter;
				}
				return chars[Math.floor(Math.random() * chars.length)];
			}).join('');

			iterations++;

			if (iterations >= maxIterations) {
				clearInterval(interval);
				decodedText = targetWord;
				statusText = 'SYSTEM READY';
				
				setTimeout(() => {
					fadeOut = true;
				}, 600);

				setTimeout(() => {
					show = false;
					onComplete();
				}, 1200);
			}
		}, 30);

		return () => clearInterval(interval);
	});
</script>

{#if show}
	<div id="minimalBoot" class:fade-out={fadeOut}>
		<div class="center-stage">
			<div class="line-container">
				<div class="expanding-line"></div>
			</div>
			
			<div class="logo-container">
				<div class="logo-text">{decodedText}</div>
			</div>
			
			<div class="status-text">{statusText}</div>
		</div>
	</div>
{/if}
