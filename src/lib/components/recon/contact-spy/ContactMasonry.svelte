<script lang="ts">
	import { Mail, Phone, Share2, Search, Link2, Users } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

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

{#if isLoading}
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		{#each Array(3) as _, i (i)}
			<div class="bg-[#18181b] border border-[#27272a] rounded-lg p-6 animate-pulse">
				<div class="h-6 w-32 bg-[#27272a] rounded mb-6"></div>
				<div class="space-y-4">
					{#each Array(4) as _, j (j)}
						<div class="h-10 w-full bg-[#27272a] rounded"></div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
{:else if results}
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 items-start">
		<!-- Emails Card -->
		<div class="bg-[#18181b] border border-rose-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-rose-500/5">
			<div class="flex items-center justify-between mb-6 border-b border-rose-500/10 pb-4">
				<div class="flex items-center gap-3">
					<div class="bg-rose-500/10 p-2 rounded-lg">
						<Mail class="text-rose-400 size-5" />
					</div>
					<h3 class="text-lg font-bold text-white tracking-widest uppercase">{m.recon_contact_emails()}</h3>
				</div>
				<span class="text-2xl font-black text-rose-500">{results.total_emails}</span>
			</div>
			
			{#if results.emails.length > 0}
				<ul class="space-y-3">
					{#each results.emails as email, i (i)}
						<li class="flex items-center gap-3 bg-[#09090b] p-3 rounded-md border border-[#27272a] group hover:border-rose-500/30 transition-colors">
							<span class="text-xs text-slate-500 font-mono">{String(i + 1).padStart(2, '0')}</span>
							<span class="text-slate-300 text-sm font-fira truncate">{email}</span>
						</li>
					{/each}
				</ul>
			{:else}
				<div class="flex flex-col items-center justify-center py-8 text-slate-500">
					<Search class="size-8 mb-2 opacity-50" />
					<p class="text-sm font-fira">{m.recon_contact_no_emails()}</p>
				</div>
			{/if}
		</div>

		<!-- Phones Card -->
		<div class="bg-[#18181b] border border-amber-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-amber-500/5">
			<div class="flex items-center justify-between mb-6 border-b border-amber-500/10 pb-4">
				<div class="flex items-center gap-3">
					<div class="bg-amber-500/10 p-2 rounded-lg">
						<Phone class="text-amber-400 size-5" />
					</div>
					<h3 class="text-lg font-bold text-white tracking-widest uppercase">{m.recon_contact_phones()}</h3>
				</div>
				<span class="text-2xl font-black text-amber-500">{results.total_phones}</span>
			</div>
			
			{#if results.phones.length > 0}
				<ul class="space-y-3">
					{#each results.phones as phone, i (i)}
						<li class="flex items-center gap-3 bg-[#09090b] p-3 rounded-md border border-[#27272a] group hover:border-amber-500/30 transition-colors">
							<span class="text-xs text-slate-500 font-mono">{String(i + 1).padStart(2, '0')}</span>
							<span class="text-slate-300 text-sm font-mono truncate">{phone}</span>
						</li>
					{/each}
				</ul>
			{:else}
				<div class="flex flex-col items-center justify-center py-8 text-slate-500">
					<Search class="size-8 mb-2 opacity-50" />
					<p class="text-sm font-fira">{m.recon_contact_no_phones()}</p>
				</div>
			{/if}
		</div>

		<!-- Social Media Card -->
		<div class="bg-[#18181b] border border-indigo-500/20 rounded-lg p-6 flex flex-col shadow-lg shadow-indigo-500/5 lg:row-span-2">
			<div class="flex items-center justify-between mb-6 border-b border-indigo-500/10 pb-4">
				<div class="flex items-center gap-3">
					<div class="bg-indigo-500/10 p-2 rounded-lg">
						<Share2 class="text-indigo-400 size-5" />
					</div>
					<h3 class="text-lg font-bold text-white tracking-widest uppercase">{m.recon_contact_social()}</h3>
				</div>
				<span class="text-2xl font-black text-indigo-500">{results.total_social_media}</span>
			</div>
			
			{#if results.social_media.length > 0}
				<!-- Grouped by platform -->
				<div class="space-y-6">
					{#each Object.entries(results.social_media_by_platform) as [platform, profiles] (platform)}
						<div>
							<h4 class="text-xs font-black text-slate-400 tracking-widest uppercase mb-3 px-1 border-b border-[#27272a] inline-block pb-1">
								{platform} <span class="bg-[#27272a] text-xs px-2 py-0.5 rounded-full ml-2 text-white">{profiles.length}</span>
							</h4>
							<ul class="space-y-2">
								{#each profiles as profile, i (i + profile.url)}
									<li class="bg-[#09090b] p-3 rounded-md border border-[#27272a] hover:border-indigo-500/30 transition-colors">
										<div class="flex justify-between items-start gap-4">
											<div class="flex flex-col overflow-hidden">
												<span class="text-indigo-300 text-sm font-bold truncate">{profile.username}</span>
												{#if profile.found_on}
													<span class="text-[10px] text-slate-500 truncate mt-1 break-all flex items-center gap-1">
														<Link2 size={10} class="shrink-0" /> {profile.found_on}
													</span>
												{/if}
											</div>
											<a 
												href={profile.url} 
												target="_blank" 
												rel="noopener noreferrer"
												class="text-xs bg-indigo-500/10 text-indigo-400 px-2 py-1 rounded hover:bg-indigo-500 hover:text-white transition-colors shrink-0"
											>
												View
											</a>
										</div>
									</li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
			{:else}
				<div class="flex flex-col items-center justify-center py-8 text-slate-500 h-full">
					<Search class="size-8 mb-2 opacity-50" />
					<p class="text-sm font-fira">{m.recon_contact_no_social()}</p>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center justify-center py-20 text-slate-500 border border-dashed border-[#27272a] rounded-lg bg-[#18181b]/50">
		<Users class="size-12 mb-4 opacity-50" />
		<h3 class="text-lg font-bold text-white mb-2">{m.recon_contact_waiting()}</h3>
		<p class="text-sm font-fira max-w-md text-center">{m.recon_contact_waiting_desc()}</p>
	</div>
{/if}
