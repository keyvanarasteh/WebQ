<script lang="ts">
    import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
    import * as m from '$lib/paraglide/messages';
    import { Shield, Network, Crosshair, AlertTriangle } from 'lucide-svelte';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title={m.sec_cfbypass_title()} icon={Shield}>
    <div class="space-y-8 text-gray-300">
        
        <!-- Introduction -->
        <section>
            <p class="text-sm leading-relaxed text-gray-400">
                <strong class="text-white">Cloudflare</strong> and similar WAFs (Web Application Firewalls) act as a reverse proxy. When a domain is protected, all DNS resolution points to Cloudflare IPs, hiding the actual origin server. This module discovers techniques to bypass that protective shield and reach the real infrastructure.
            </p>
        </section>

        <!-- Bypass Mechanics -->
        <section class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden">
            <div class="bg-teal-500/10 border-b border-[#27272a] px-6 py-4 flex items-center gap-3">
                <Network class="size-5 text-teal-400" />
                <h3 class="font-bold text-white tracking-widest uppercase">{m.sec_cfbypass_guide_mechanics()}</h3>
            </div>
            
            <div class="p-6 space-y-4">
                <div>
                    <h4 class="text-sm font-bold text-teal-400 mb-1">1. Historical DNS Records</h4>
                    <p class="text-xs text-gray-400">Before Cloudflare was enabled, the domain's A records pointed directly to the origin IP. Databases like SecurityTrails, ViewDNS, and Netcraft store historical DNS snapshots that may still reveal the origin.</p>
                    <div class="mt-2 text-xs border-l-2 border-red-500 pl-3 text-red-300">
                        <strong>SecOps:</strong> Once an origin IP is found, attackers bypass all WAF rules by connecting directly — rendering CDN-level protections completely null.
                    </div>
                </div>

                <div>
                    <h4 class="text-sm font-bold text-teal-400 mb-1">2. Subdomain Leakage</h4>
                    <p class="text-xs text-gray-400">Often organizations protect <code>www.target.com</code> but leave subdomains like <code>staging.target.com</code>, <code>mail.target.com</code>, or <code>cpanel.target.com</code> unproxied, leaking the real origin IP directly.</p>
                </div>

                <div>
                    <h4 class="text-sm font-bold text-teal-400 mb-1">3. SSL Certificate Matching</h4>
                    <p class="text-xs text-gray-400">Tools like Censys and Shodan index SSL certificates across the entire IPv4 space. By searching for the target's certificate fingerprint, you can find the real server's IP even behind Cloudflare.</p>
                </div>
            </div>
        </section>

        <!-- Defense Section -->
        <section class="bg-emerald-500/5 border border-emerald-500/20 rounded-xl p-6">
            <div class="flex items-center gap-3 mb-4">
                <Shield class="size-5 text-emerald-400" />
                <h3 class="font-bold text-emerald-400 tracking-widest uppercase">{m.sec_cfbypass_guide_secops()}</h3>
            </div>
            <p class="text-xs text-gray-400 mb-4">{m.sec_cfbypass_guide_secops_desc()}</p>
            
            <div class="space-y-2">
                <div class="flex items-start gap-2">
                    <Crosshair class="size-4 text-emerald-500 mt-0.5 shrink-0" />
                    <p class="text-xs text-gray-400">Block all traffic not from Cloudflare IP ranges at the firewall level (iptables/security groups).</p>
                </div>
                <div class="flex items-start gap-2">
                    <Crosshair class="size-4 text-emerald-500 mt-0.5 shrink-0" />
                    <p class="text-xs text-gray-400">Use Cloudflare Origin certificates and strict SSL mode to prevent direct HTTPS connections.</p>
                </div>
                <div class="flex items-start gap-2">
                    <AlertTriangle class="size-4 text-orange-400 mt-0.5 shrink-0" />
                    <p class="text-xs text-gray-400">Ensure ALL subdomains are proxied — a single unproxied subdomain leaks your origin.</p>
                </div>
            </div>
        </section>

    </div>
</SecOpsGuideModal>
