<script lang="ts">
	import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
	import * as m from '$lib/paraglide/messages';
	import { Network, AlertTriangle, ShieldCheck, Search } from 'lucide-svelte';

	let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title={m.recon_subdomain_guide_title()} icon={Network}>
	<div class="space-y-8 text-gray-300">

		<!-- Introduction -->
		<section>
			<p class="text-sm leading-relaxed text-gray-400">
				{m.recon_subdomain_guide_desc()}
			</p>
		</section>

		<!-- Passive vs Active -->
		<section class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-indigo-500">
				<div class="flex items-center gap-2 mb-3">
					<Search class="size-5 text-indigo-400" />
					<h3 class="font-bold text-gray-200">Passive Enumeration</h3>
				</div>
				<p class="text-xs text-gray-400">Uses Certificate Transparency (CT) logs, search engine caches, and WHOIS data to discover subdomains without sending any traffic to the target. Completely stealth.</p>
			</div>
			
			<div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-red-500">
				<div class="flex items-center gap-2 mb-3">
					<AlertTriangle class="size-5 text-red-400" />
					<h3 class="font-bold text-gray-200">Active Brute-Forcing</h3>
				</div>
				<p class="text-xs text-gray-400">DNS resolution brute-force against wordlists (e.g., SecLists). Noisy but discovers internal-only subdomains not indexed in public CT logs or search engines.</p>
			</div>
		</section>

		<!-- Subdomain Takeover -->
		<section class="bg-red-500/5 border border-red-500/20 rounded-xl p-6">
			<h3 class="text-lg font-bold text-red-400 mb-4 tracking-widest uppercase">Subdomain Takeover Risk</h3>
			<p class="text-xs text-gray-400 mb-4">If a subdomain has a CNAME record pointing to an unclaimed external service (e.g., <code>staging.target.com → app.herokuapps.com</code> where the Heroku app was deleted), attackers can register the endpoint and serve malicious content under the legitimate domain.</p>
			<div class="text-xs border-l-2 border-red-500 pl-3 text-red-300">
				<strong>SecOps:</strong> Regularly audit all CNAME records. Remove DNS entries for deprecated services. Monitor for dangling DNS pointers.
			</div>
		</section>

		<!-- Defense Section -->
		<section class="p-4 border-l-4 border-green-500 bg-green-500/5">
			<h4 class="font-bold text-green-400 mb-2 flex items-center gap-2">
				<ShieldCheck class="size-4" /> Secure Developer Protocols
			</h4>
			<p class="text-xs text-gray-400">
				Implement wildcard DNS monitoring. Use tools like <code>can-i-take-over-xyz</code> to audit CNAME records. Restrict zone transfers (AXFR) on your authoritative DNS servers. Remove unused subdomain records immediately upon decommissioning services.
			</p>
		</section>

	</div>
</SecOpsGuideModal>
