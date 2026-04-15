<script lang="ts">
	import { Mail, Phone, Share2, Search, Users, HelpCircle, UserX } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	import EmailHarvester from './EmailHarvester.svelte';
	import SocialOsintBox from './SocialOsintBox.svelte';
	import ContactSpyGuide from '$lib/components/recon/guides/ContactSpyGuide.svelte';

	interface SocialProfile {
		platform: string;
		username: string;
		url: string;
		found_on?: string;
	}

	interface ContactSpyResult {
		domain: string;
		emails: string[];
		phones: string[];
		social_media: SocialProfile[];
		social_media_by_platform: Record<string, SocialProfile[]>;
		pages_scanned: number;
		total_emails: number;
		total_phones: number;
		total_social_media: number;
	}

	interface Props {
		results: ContactSpyResult | null;
		isLoading: boolean;
	}

	let { results, isLoading }: Props = $props();
	let isGuideOpen = $state(false);
</script>

<ContactSpyGuide bind:isOpen={isGuideOpen} />

<div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start h-full pb-8">
	<!-- Left Side: Basic OSINT (Emails & Phones) -->
	<div class="lg:col-span-8 grid grid-cols-1 md:grid-cols-2 gap-6 w-full">
		<!-- Emails Card -->
		<div class="bg-elevated border border-rose-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-rose-500/5 min-h-[250px]">
			<div class="flex items-center justify-between mb-6 border-b border-rose-500/10 pb-4">
				<div class="flex items-center gap-3">
					<div class="bg-rose-500/10 p-2 rounded-lg">
						<Mail class="text-rose-400 size-5" />
					</div>
					<h3 class="text-lg font-bold text-primary-text tracking-widest uppercase">{m.recon_contact_emails()}</h3>
					<button onclick={() => isGuideOpen = true} class="p-1 hover:bg-rose-500/10 rounded-full text-rose-400 transition-colors" title="Guide">
						<HelpCircle class="size-4" />
					</button>
				</div>
				<span class="text-2xl font-black text-rose-500">{results?.total_emails || 0}</span>
			</div>
			
			{#if isLoading && !results}
				<div class="space-y-4 animate-pulse">
					{#each Array(4) as _, j (j)}
						<div class="h-10 w-full bg-surface rounded"></div>
					{/each}
				</div>
			{:else if results && results.emails.length > 0}
				<EmailHarvester emails={results.emails} />
			{:else if results}
				<div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50 h-full">
					<Search class="size-8 mb-2 opacity-50" />
					<p class="text-sm font-fira">{m.recon_contact_no_emails()}</p>
				</div>
			{:else}
				<div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50 h-full flex-grow flex-1">
					<UserX class="size-8 mb-2 opacity-30 text-rose-500" />
					<p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">Emails Waiting</p>
					<p class="text-xs max-w-[200px] text-center">Run contact spy to harvest</p>
				</div>
			{/if}
		</div>

		<!-- Phones Card -->
		<div class="bg-elevated border border-amber-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-amber-500/5 min-h-[250px]">
			<div class="flex items-center justify-between mb-6 border-b border-amber-500/10 pb-4">
				<div class="flex items-center gap-3">
					<div class="bg-amber-500/10 p-2 rounded-lg">
						<Phone class="text-amber-400 size-5" />
					</div>
					<h3 class="text-lg font-bold text-primary-text tracking-widest uppercase">{m.recon_contact_phones()}</h3>
					<button onclick={() => isGuideOpen = true} class="p-1 hover:bg-amber-500/10 rounded-full text-amber-400 transition-colors" title="Guide">
						<HelpCircle class="size-4" />
					</button>
				</div>
				<span class="text-2xl font-black text-amber-500">{results?.total_phones || 0}</span>
			</div>
			
			{#if isLoading && !results}
				<div class="space-y-4 animate-pulse">
					{#each Array(4) as _, j (j)}
						<div class="h-10 w-full bg-surface rounded"></div>
					{/each}
				</div>
			{:else if results && results.phones.length > 0}
				<ul class="space-y-3">
					{#each results.phones as phone, i (i)}
						<li class="flex items-center gap-3 bg-background p-3 rounded-md border border-base group hover:border-amber-500/30 transition-colors shadow shadow-black/40">
							<span class="text-xs text-muted font-mono">{String(i + 1).padStart(2, '0')}</span>
							<span class="text-secondary-text text-sm font-mono truncate">{phone}</span>
						</li>
					{/each}
				</ul>
			{:else if results}
				<div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50 h-full">
					<Search class="size-8 mb-2 opacity-50" />
					<p class="text-sm font-fira">{m.recon_contact_no_phones()}</p>
				</div>
			{:else}
				<div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50 h-full flex-grow flex-1">
					<UserX class="size-8 mb-2 opacity-30 text-amber-500" />
					<p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">Phones Waiting</p>
					<p class="text-xs max-w-[200px] text-center">Run contact spy to harvest</p>
				</div>
			{/if}
		</div>
	</div>

	<!-- Right Side: Advanced OSINT (Social Profiles) -->
	<div class="lg:col-span-4 flex flex-col gap-6 w-full">
		<div class="bg-elevated border border-indigo-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-indigo-500/5 min-h-[250px]">
			<div class="flex items-center justify-between mb-6 border-b border-indigo-500/10 pb-4">
				<div class="flex items-center gap-3">
					<div class="bg-indigo-500/10 p-2 rounded-lg">
						<Share2 class="text-indigo-400 size-5" />
					</div>
					<h3 class="text-lg font-bold text-primary-text tracking-widest uppercase">{m.recon_contact_social()}</h3>
					<button onclick={() => isGuideOpen = true} class="p-1 hover:bg-indigo-500/10 rounded-full text-indigo-400 transition-colors" title="Guide">
						<HelpCircle class="size-4" />
					</button>
				</div>
				<span class="text-2xl font-black text-indigo-500">{results?.total_social_media || 0}</span>
			</div>
			
			{#if isLoading && !results}
				<div class="space-y-4 animate-pulse">
					{#each Array(4) as _, j (j)}
						<div class="h-10 w-full bg-surface rounded"></div>
					{/each}
				</div>
			{:else if results && results.social_media.length > 0}
				<SocialOsintBox profiles={results.social_media_by_platform} />
			{:else if results}
				<div class="flex flex-col items-center justify-center py-8 text-muted h-full border border-dashed border-base rounded-lg bg-background/50">
					<Search class="size-8 mb-2 opacity-50" />
					<p class="text-sm font-fira">{m.recon_contact_no_social()}</p>
				</div>
			{:else}
				<div class="flex flex-col items-center justify-center py-8 text-muted h-full border border-dashed border-base rounded-lg bg-background/50 flex-grow flex-1">
					<UserX class="size-8 mb-2 opacity-30 text-indigo-500" />
					<p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">Socials Waiting</p>
					<p class="text-xs max-w-[200px] text-center">Run contact spy to harvest</p>
				</div>
			{/if}
		</div>
	</div>
</div>
