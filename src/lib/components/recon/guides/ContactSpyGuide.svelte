<script lang="ts">
	import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
	import { Play, FileText, Info, Users, ShieldAlert, Fingerprint } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title={m.recon_contact_guide_title()} icon={Users}>
	<div class="space-y-8 text-gray-300">
		<section>
			<p class="text-sm leading-relaxed text-gray-400">
				{m.recon_contact_guide_desc()}
			</p>
		</section>

		<section class="bg-indigo-950/20 border border-indigo-500/20 rounded-xl p-6">
			<h3 class="text-indigo-400 flex items-center gap-2 tracking-wider mb-4 uppercase text-xs font-bold font-mono">
				<FileText size={14} />
				{m.secops_context()}
			</h3>
			<ul class="space-y-3 font-fira text-sm text-slate-300 marker:text-indigo-500 pl-4">
				<li><strong class="text-white">OSINT Harvesting:</strong> Attackers compile employee contacts to launch targeted phishing (spear-phishing) campaigns or social engineering attacks against your company.</li>
				<li><strong class="text-white">Social Footprints:</strong> Matching corporate emails to platforms like LinkedIn, GitHub, or Twitter provides adversaries insight into tech stacks, corporate hierarchies, and vulnerable personnel.</li>
				<li><strong class="text-white">Leak Protection:</strong> Scanning your public contact footprint allows you to preemptively identify overly permissive directory listings before an adversary exploits them.</li>
			</ul>
		</section>

		<section class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden p-6">
			<h3 class="text-amber-400 flex items-center gap-2 tracking-wider mb-4 uppercase text-xs font-bold font-mono">
				<ShieldAlert size={14} />
				{m.security_risks()}
			</h3>
			<ul class="space-y-3 font-fira text-sm text-slate-300 marker:text-amber-500 pl-4">
				<li><strong class="text-white">Spear Phishing:</strong> High-value targets (executives, HR, IT admins) exposed in public directories are prime targets.</li>
				<li><strong class="text-white">Credential Stuffing:</strong> Emails scraped from contact pages are frequently checked against large password breach databases for reuse.</li>
				<li><strong class="text-white">Social Engineering:</strong> Phone numbers exposed permit vishing (voice phishing) attacks to bypass MFA or extract sensitive credentials via support tickets.</li>
			</ul>
		</section>
		
		<p class="text-xs text-center text-slate-500 mt-4 tracking-widest uppercase font-mono">
			{m.recon_guide_tip()}
		</p>
	</div>
</SecOpsGuideModal>
