<script lang="ts">
    import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
    import * as m from '$lib/paraglide/messages';
    import { Fingerprint, ShieldAlert, Target, AlertTriangle } from 'lucide-svelte';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title={m.sec_nmap_title()} icon={Fingerprint}>
    <div class="space-y-8 text-gray-300">
        
        <!-- Introduction -->
        <section>
            <p class="text-sm leading-relaxed text-gray-400">
                <strong class="text-white">Port scanning</strong> is the cornerstone of network reconnaissance. Tools like <strong>Nmap</strong> probe open ports and cross-reference running services against CVE databases to identify zero-day and known vulnerabilities in real-time.
            </p>
        </section>

        <!-- How Nmap Works -->
        <section class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden">
            <div class="bg-red-500/10 border-b border-[#27272a] px-6 py-4 flex items-center gap-3">
                <Fingerprint class="size-5 text-red-400" />
                <h3 class="font-bold text-white tracking-widest uppercase">{m.sec_nmap_guide_mechanics()}</h3>
            </div>
            
            <div class="p-6 space-y-4">
                <div>
                    <h4 class="text-sm font-bold text-red-400 mb-1">1. SYN Scan (Stealth Scan)</h4>
                    <p class="text-xs text-gray-400">Sends a SYN packet to each port. If the target responds with SYN/ACK, the port is open. The scan never completes the TCP handshake, making it harder to detect in logs.</p>
                    <div class="mt-2 bg-[#09090b] border border-[#27272a] p-3 rounded font-mono text-xs text-gray-300">
                        nmap -sS -p 1-65535 target.com
                    </div>
                </div>

                <div>
                    <h4 class="text-sm font-bold text-red-400 mb-1">2. Service Version Detection</h4>
                    <p class="text-xs text-gray-400">The <code>-sV</code> flag probes open ports to determine the exact service version (e.g., Apache/2.4.49, OpenSSH 8.2). This version is then mapped to CVE entries for vulnerability matching.</p>
                </div>

                <div>
                    <h4 class="text-sm font-bold text-red-400 mb-1">3. CVE Cross-Reference</h4>
                    <p class="text-xs text-gray-400">WebQ queries NIST NVD and Exploit-DB APIs to match the detected service version against known vulnerabilities, providing CVSS severity scores and exploit availability.</p>
                </div>
            </div>
        </section>

        <!-- Defense Section -->
        <section class="bg-emerald-500/5 border border-emerald-500/20 rounded-xl p-6">
            <div class="flex items-center gap-3 mb-4">
                <ShieldAlert class="size-5 text-emerald-400" />
                <h3 class="font-bold text-emerald-400 tracking-widest uppercase">{m.sec_nmap_guide_secops()}</h3>
            </div>
            <p class="text-xs text-gray-400 mb-4">{m.sec_nmap_guide_secops_desc()}</p>
            
            <div class="space-y-2">
                <div class="flex items-start gap-2">
                    <Target class="size-4 text-emerald-500 mt-0.5 shrink-0" />
                    <p class="text-xs text-gray-400">Close all unnecessary ports — only expose 80/443 on production servers.</p>
                </div>
                <div class="flex items-start gap-2">
                    <Target class="size-4 text-emerald-500 mt-0.5 shrink-0" />
                    <p class="text-xs text-gray-400">Keep all services updated to the latest patch level to eliminate known CVEs.</p>
                </div>
                <div class="flex items-start gap-2">
                    <AlertTriangle class="size-4 text-orange-400 mt-0.5 shrink-0" />
                    <p class="text-xs text-gray-400">Implement IDS/IPS (Intrusion Detection/Prevention Systems) like Snort or Suricata to detect port scanning activity.</p>
                </div>
            </div>
        </section>

    </div>
</SecOpsGuideModal>
