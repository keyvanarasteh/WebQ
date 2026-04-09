<script lang="ts">
	import { Mail, Phone, Share2, Search, Users } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	import EmailHarvester from './EmailHarvester.svelte';
	import SocialOsintBox from './SocialOsintBox.svelte';

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
</script>

{#if isLoading && !results}
	<div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start h-full pb-8">
		<div class="lg:col-span-8 grid grid-cols-1 md:grid-cols-2 gap-6 w-full">
			{#each Array(2) as _, i (i)}
				<div class="bg-elevated border border-base rounded-lg p-6 animate-pulse">
					<div class="h-6 w-32 bg-surface rounded mb-6"></div>
					<div class="space-y-4">
						{#each Array(4) as _, j (j)}
							<div class="h-10 w-full bg-surface rounded"></div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
		<div class="lg:col-span-4 bg-elevated border border-base rounded-lg p-6 animate-pulse w-full">
			<div class="h-6 w-32 bg-surface rounded mb-6"></div>
			<div class="space-y-4">
				{#each Array(4) as _, j (j)}
					<div class="h-10 w-full bg-surface rounded"></div>
				{/each}
			</div>
		</div>
	</div>
{:else if results}
	<!-- Masonry layout using grid -->
	<div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start h-full pb-8">
		<!-- Left Side: Basic OSINT (Emails & Phones) -->
		<div class="lg:col-span-8 grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Emails Card -->
			<div class="bg-elevated border border-rose-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-rose-500/5">
				<div class="flex items-center justify-between mb-6 border-b border-rose-500/10 pb-4">
					<div class="flex items-center gap-3">
						<div class="bg-rose-500/10 p-2 rounded-lg">
							<Mail class="text-rose-400 size-5" />
						</div>
						<h3 class="text-lg font-bold text-primary-text tracking-widest uppercase">{m.recon_contact_emails()}</h3>
					</div>
					<span class="text-2xl font-black text-rose-500">{results.total_emails}</span>
				</div>
				
				{#if results.emails.length > 0}
					<EmailHarvester emails={results.emails} />
				{:else}
					<div class="flex flex-col items-center justify-center py-8 text-muted">
						<Search class="size-8 mb-2 opacity-50" />
						<p class="text-sm font-fira">{m.recon_contact_no_emails()}</p>
					</div>
				{/if}
			</div>

			<!-- Phones Card -->
			<div class="bg-elevated border border-amber-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-amber-500/5">
				<div class="flex items-center justify-between mb-6 border-b border-amber-500/10 pb-4">
					<div class="flex items-center gap-3">
						<div class="bg-amber-500/10 p-2 rounded-lg">
							<Phone class="text-amber-400 size-5" />
						</div>
						<h3 class="text-lg font-bold text-primary-text tracking-widest uppercase">{m.recon_contact_phones()}</h3>
					</div>
					<span class="text-2xl font-black text-amber-500">{results.total_phones}</span>
				</div>
				
				{#if results.phones.length > 0}
					<ul class="space-y-3">
						{#each results.phones as phone, i (i)}
							<li class="flex items-center gap-3 bg-background p-3 rounded-md border border-base group hover:border-amber-500/30 transition-colors shadow shadow-black/40">
								<span class="text-xs text-muted font-mono">{String(i + 1).padStart(2, '0')}</span>
								<span class="text-secondary-text text-sm font-mono truncate">{phone}</span>
							</li>
						{/each}
					</ul>
				{:else}
					<div class="flex flex-col items-center justify-center py-8 text-muted">
						<Search class="size-8 mb-2 opacity-50" />
						<p class="text-sm font-fira">{m.recon_contact_no_phones()}</p>
					</div>
				{/if}
			</div>
		</div>

		<!-- Right Side: Advanced OSINT (Social Profiles) -->
		<div class="lg:col-span-4 flex flex-col gap-6 w-full">
			<div class="bg-elevated border border-indigo-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-indigo-500/5">
				<div class="flex items-center justify-between mb-6 border-b border-indigo-500/10 pb-4">
					<div class="flex items-center gap-3">
						<div class="bg-indigo-500/10 p-2 rounded-lg">
							<Share2 class="text-indigo-400 size-5" />
						</div>
						<h3 class="text-lg font-bold text-primary-text tracking-widest uppercase">{m.recon_contact_social()}</h3>
					</div>
					<span class="text-2xl font-black text-indigo-500">{results.total_social_media}</span>
				</div>
				
				{#if results.social_media.length > 0}
					<SocialOsintBox profiles={results.social_media_by_platform} />
				{:else}
					<div class="flex flex-col items-center justify-center py-8 text-muted h-full">
						<Search class="size-8 mb-2 opacity-50" />
						<p class="text-sm font-fira">{m.recon_contact_no_social()}</p>
					</div>
				{/if}
			</div>
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center justify-center py-20 text-muted border border-dashed border-base rounded-lg bg-elevated/50 shadow-inner">
		<Users class="size-12 mb-4 opacity-50" />
		<h3 class="text-lg font-bold text-primary-text mb-2">{m.recon_contact_waiting()}</h3>
		<p class="text-sm font-fira max-w-md text-center">{m.recon_contact_waiting_desc()}</p>
	</div>
{/if}
