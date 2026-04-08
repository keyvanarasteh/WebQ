<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import { HelpCircle } from 'lucide-svelte';
</script>

<div class="relative overflow-hidden rounded-xl border border-indigo-500/20 bg-slate-900/40 p-6 backdrop-blur-md">
	<div class="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-transparent"></div>
	<div class="absolute -right-12 -top-12 h-32 w-32 rounded-full bg-indigo-500/10 blur-[40px]"></div>

	<div class="relative z-10 flex items-start gap-4">
		<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl border border-indigo-500/30 bg-indigo-500/20 text-indigo-400">
			<HelpCircle size={24} />
		</div>
		<div class="flex flex-col gap-1">
			<h3 class="text-lg font-bold tracking-tight text-slate-100 font-fira">{m.recon_subdomain_guide_title()}</h3>
			<p class="text-sm leading-relaxed text-slate-400 font-inter">
				{m.recon_subdomain_guide_desc()}
			</p>
		</div>
	</div>
</div>
