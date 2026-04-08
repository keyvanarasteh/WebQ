<script lang="ts">
	import { Play, FileText, Info, Users, ShieldAlert, Fingerprint } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	interface Props {
		onClose?: () => void;
	}

	let { onClose }: Props = $props();
</script>

<div class="h-full w-full bg-[#18181b]/95 border-l border-indigo-500/20 shadow-2xl overflow-y-auto w-full lg:w-96 rounded-lg pointer-events-auto">
	<div class="p-6 h-full flex flex-col">
		<div class="flex items-center justify-between mb-6">
			<div class="flex items-center gap-3">
				<div class="bg-indigo-500/10 p-2 rounded-lg border border-indigo-500/20">
					<Users class="text-indigo-400 size-5" />
				</div>
				<h2 class="text-xl font-bold text-white tracking-widest uppercase">{m.recon_contact_guide_title()}</h2>
			</div>
			{#if onClose}
				<button onclick={onClose} class="text-slate-400 hover:text-white transition-colors">
					<span class="sr-only">Close Guide</span>
					<Fingerprint size={20} />
				</button>
			{/if}
		</div>

		<div class="prose prose-invert prose-sm max-w-none flex-grow">
			<div class="bg-indigo-950/30 p-4 rounded-lg border border-indigo-500/20 mb-6">
				<p class="text-indigo-200 mt-0 mb-0 font-fira text-sm leading-relaxed">
					{m.recon_contact_guide_desc()}
				</p>
			</div>

			<h3 class="text-indigo-300 flex items-center gap-2 tracking-wider mt-6 mb-3 uppercase text-xs font-bold font-mono">
				<FileText size={14} />
				{m.secops_context()}
			</h3>
			<ul class="space-y-3 font-fira text-sm text-slate-300 marker:text-indigo-500 pl-4 mb-6">
				<li><strong class="text-white">OSINT Harvesting:</strong> Attackers compile employee contacts to launch targeted phishing (spear-phishing) campaigns or social engineering attacks against your company.</li>
				<li><strong class="text-white">Social Footprints:</strong> Matching corporate emails to platforms like LinkedIn, GitHub, or Twitter provides adversaries insight into tech stacks, corporate hierarchies, and vulnerable personnel.</li>
				<li><strong class="text-white">Leak Protection:</strong> Scanning your public contact footprint allows you to preemptively identify overly permissive directory listings before an adversary exploits them.</li>
			</ul>

			<h3 class="text-amber-400 flex items-center gap-2 tracking-wider mt-6 mb-3 uppercase text-xs font-bold font-mono">
				<ShieldAlert size={14} />
				{m.security_risks()}
			</h3>
			<ul class="space-y-3 font-fira text-sm text-slate-300 marker:text-amber-500 pl-4 mb-6">
				<li><strong class="text-white">Spear Phishing:</strong> High-value targets (executives, HR, IT admins) exposed in public directories are prime targets.</li>
				<li><strong class="text-white">Credential Stuffing:</strong> Emails scraped from contact pages are frequently checked against large password breach databases for reuse.</li>
				<li><strong class="text-white">Social Engineering:</strong> Phone numbers exposed permit vishing (voice phishing) attacks to bypass MFA or extract sensitive credentials via support tickets.</li>
			</ul>
		</div>

		<div class="mt-auto pt-6 border-t border-slate-800">
			<div class="flex gap-2">
				<div class="w-1/3 bg-indigo-500/10 rounded overflow-hidden">
					<div class="h-1 bg-indigo-500 w-full animate-pulse"></div>
				</div>
				<div class="w-1/3 bg-slate-800 rounded"></div>
				<div class="w-1/3 bg-slate-800 rounded"></div>
			</div>
			<p class="text-xs text-center text-slate-500 mt-4 tracking-widest uppercase font-mono">
				{m.recon_guide_tip()}
			</p>
		</div>
	</div>
</div>
