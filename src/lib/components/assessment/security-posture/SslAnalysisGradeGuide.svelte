<script lang="ts">
    import { X, Lock, Shield, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. TLS Protocol Landscape", icon: Lock },
        { id: 1, label: "2. Certificate Trust Chain", icon: Shield },
        { id: 2, label: "3. TLS Hardening", icon: ShieldCheck }
    ];

    function nextTab() {
        if (activeTab < tabs.length - 1) activeTab++;
    }
    function prevTab() {
        if (activeTab > 0) activeTab--;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div
            class="bg-[#0A0C10] border border-orange-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(249,115,22,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-orange-950/40 via-orange-900/10 to-transparent p-6 border-b border-orange-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-orange-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-orange-500/20 rounded-xl transition-all border border-subtle hover:border-orange-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-orange-500/20 to-orange-600/10 flex items-center justify-center border border-orange-500/30 shadow-inner">
                            <Lock size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">SSL/TLS Configuration</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Transport layer security analysis covering protocol versions, cipher suites, and certificate validity.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-orange-500/20 text-orange-300 border border-orange-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 relative custom-scrollbar">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">TLS Protocol Versions</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            TLS (Transport Layer Security) is the cryptographic protocol that encrypts all HTTP traffic (shown as HTTPS in browsers). Not all versions are equal — older versions have known vulnerabilities that allow attackers to break the encryption and read or modify your traffic.
                        </p>

                        <div class="space-y-3 mb-8">
                            <div class="flex items-start gap-4 p-4 border border-red-500/30 bg-red-950/10 rounded-xl">
                                <div class="w-16 shrink-0 text-center">
                                    <div class="text-sm font-bold text-red-400 font-mono">TLS 1.0</div>
                                    <div class="text-xs text-red-400/60">1999</div>
                                </div>
                                <div class="flex-1">
                                    <div class="text-xs font-semibold text-red-300 mb-1">DEPRECATED — Must Disable</div>
                                    <p class="text-xs text-muted">Vulnerable to POODLE (Padding Oracle On Downgraded Legacy Encryption) and BEAST attacks. PCI DSS 3.2+ explicitly prohibits TLS 1.0 for cardholder data environments. Chrome and Firefox have blocked it since 2020.</p>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 border border-red-500/30 bg-red-950/10 rounded-xl">
                                <div class="w-16 shrink-0 text-center">
                                    <div class="text-sm font-bold text-red-400 font-mono">TLS 1.1</div>
                                    <div class="text-xs text-red-400/60">2006</div>
                                </div>
                                <div class="flex-1">
                                    <div class="text-xs font-semibold text-red-300 mb-1">DEPRECATED — Must Disable</div>
                                    <p class="text-xs text-muted">Partially mitigated BEAST but still supports weak CBC cipher modes. RFC 8996 (2021) formally deprecated TLS 1.0 and 1.1. No modern browser supports them.</p>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 border border-yellow-500/30 bg-yellow-950/10 rounded-xl">
                                <div class="w-16 shrink-0 text-center">
                                    <div class="text-sm font-bold text-yellow-400 font-mono">TLS 1.2</div>
                                    <div class="text-xs text-yellow-400/60">2008</div>
                                </div>
                                <div class="flex-1">
                                    <div class="text-xs font-semibold text-yellow-300 mb-1">Acceptable — But Upgrade</div>
                                    <p class="text-xs text-muted">Safe when configured with strong ciphers (AEAD only: AES-GCM, ChaCha20-Poly1305). Still susceptible to downgrade attacks if weaker ciphers are also enabled. Widely deployed; acceptable for compatibility.</p>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 border border-emerald-500/30 bg-emerald-950/10 rounded-xl">
                                <div class="w-16 shrink-0 text-center">
                                    <div class="text-sm font-bold text-emerald-400 font-mono">TLS 1.3</div>
                                    <div class="text-xs text-emerald-400/60">2018</div>
                                </div>
                                <div class="flex-1">
                                    <div class="text-xs font-semibold text-emerald-300 mb-1">Best — Target Configuration</div>
                                    <p class="text-xs text-muted">Removed all insecure cipher suites entirely (no RC4, 3DES, CBC with SHA-1). Perfect Forward Secrecy is mandatory by design — each session uses unique ephemeral keys so past traffic cannot be decrypted later. Faster handshake (1-RTT vs 2-RTT).</p>
                                </div>
                            </div>
                        </div>

                        <!-- Cipher Suite Explainer -->
                        <div class="bg-orange-950/20 border border-orange-500/10 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-3 text-sm">Reading a Cipher Suite Name</h4>
                            <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs border border-subtle mb-3">
                                <span class="text-blue-300">ECDHE</span>-<span class="text-orange-300">RSA</span>-<span class="text-emerald-300">AES256</span>-<span class="text-emerald-300">GCM</span>-<span class="text-cyan-300">SHA384</span>
                            </div>
                            <div class="grid grid-cols-1 sm:grid-cols-5 gap-2 text-xs">
                                <div class="text-center"><div class="text-blue-300 font-semibold mb-1">ECDHE</div><div class="text-muted">Key Exchange — Elliptic Curve Diffie-Hellman Ephemeral (PFS)</div></div>
                                <div class="text-center"><div class="text-orange-300 font-semibold mb-1">RSA</div><div class="text-muted">Authentication — server proves identity with RSA private key</div></div>
                                <div class="text-center"><div class="text-emerald-300 font-semibold mb-1">AES256</div><div class="text-muted">Bulk Encryption — 256-bit AES encrypts actual data</div></div>
                                <div class="text-center"><div class="text-emerald-300 font-semibold mb-1">GCM</div><div class="text-muted">Mode — Galois/Counter Mode (AEAD, tamper-proof)</div></div>
                                <div class="text-center"><div class="text-cyan-300 font-semibold mb-1">SHA384</div><div class="text-muted">MAC — message integrity hash</div></div>
                            </div>
                        </div>

                        <!-- Terminal Mockup -->
                        <div class="mt-6 bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs">
                            <div class="bg-surface/80 px-4 py-2 flex items-center gap-2 border-b border-subtle">
                                <div class="w-3 h-3 rounded-full bg-red-500/60"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500/60"></div>
                                <div class="w-3 h-3 rounded-full bg-emerald-500/60"></div>
                                <span class="text-muted text-xs ml-2">Terminal — openssl s_client probe</span>
                            </div>
                            <div class="p-4 text-primary-text leading-relaxed space-y-1">
                                <div><span class="text-emerald-400">$</span> openssl s_client -connect target.com:443 -tls1_3</div>
                                <div class="text-muted">...</div>
                                <div>Protocol  : <span class="text-emerald-300">TLSv1.3</span></div>
                                <div>Cipher    : <span class="text-cyan-300">TLS_AES_256_GCM_SHA384</span></div>
                                <div>Session-ID: <span class="text-muted">A3F7...</span></div>
                                <div>Verify return code: <span class="text-emerald-300">0 (ok)</span></div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Certificate Trust Chain (PKI)</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Every SSL/TLS certificate is signed by a Certificate Authority (CA). Browsers trust a fixed set of Root CAs (pre-installed in the OS). Trust flows down a chain — the Root CA signs Intermediate CAs, which sign the certificate your server presents.
                        </p>

                        <!-- PKI Chain Diagram -->
                        <div class="flex flex-col items-center mb-8 space-y-2">
                            <div class="p-3 bg-blue-950/20 border border-blue-500/30 rounded-xl text-center w-full max-w-xs">
                                <div class="text-xs font-bold text-blue-300 mb-1">Root CA</div>
                                <div class="text-xs text-muted font-mono">e.g., ISRG Root X1 (Let's Encrypt), DigiCert Global Root</div>
                                <div class="text-[10px] text-muted mt-1">Pre-trusted by your OS/browser. Self-signed. Kept offline.</div>
                            </div>
                            <div class="text-muted text-lg">↓ signs</div>
                            <div class="p-3 bg-orange-950/20 border border-orange-500/30 rounded-xl text-center w-full max-w-xs">
                                <div class="text-xs font-bold text-orange-300 mb-1">Intermediate CA</div>
                                <div class="text-xs text-muted font-mono">e.g., R3 (Let's Encrypt), DigiCert TLS RSA SHA256 2020 CA1</div>
                                <div class="text-[10px] text-muted mt-1">The issuer that actually signs end-entity certs. Online but protected.</div>
                            </div>
                            <div class="text-muted text-lg">↓ signs</div>
                            <div class="p-3 bg-emerald-950/20 border border-emerald-500/30 rounded-xl text-center w-full max-w-xs">
                                <div class="text-xs font-bold text-emerald-300 mb-1">End Entity Certificate (your server)</div>
                                <div class="text-xs text-muted font-mono">CN=target.com, SAN=*.target.com</div>
                                <div class="text-[10px] text-muted mt-1">The cert your nginx/Apache presents on port 443. Has an expiry date.</div>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">Subject vs Issuer Fields</h4>
                                <div class="space-y-2 text-xs text-muted">
                                    <div><span class="text-primary-text font-semibold">Subject:</span> The domain this certificate was issued FOR. Contains CN (Common Name) and SANs (Subject Alternative Names). A wildcard cert shows <code class="text-cyan-300">CN=*.target.com</code>.</div>
                                    <div><span class="text-primary-text font-semibold">Issuer:</span> The CA that signed this certificate. Your browser walks up the chain to verify the Issuer is ultimately trusted by a root in its trust store.</div>
                                </div>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">Let's Encrypt vs Paid CAs</h4>
                                <div class="space-y-2 text-xs text-muted">
                                    <div><span class="text-emerald-300 font-semibold">Let's Encrypt (free):</span> DV (Domain Validation) certs only. 90-day validity, auto-renew via certbot/ACME. Trusted everywhere. Ideal for most web apps.</div>
                                    <div><span class="text-orange-300 font-semibold">Paid CAs (DigiCert, Sectigo):</span> OV (Organization Validation) and EV (Extended Validation) certs. OV/EV validate the legal entity. Required in some regulated industries.</div>
                                </div>
                            </div>
                        </div>

                        <div class="p-4 bg-red-950/10 border border-red-500/20 rounded-xl mb-4">
                            <h4 class="text-sm font-semibold text-red-300 mb-2">Certificate Expiry Risk</h4>
                            <p class="text-xs text-muted leading-relaxed">An expired certificate causes browsers to block access entirely with a hard error — no "proceed" option in some browsers. Let's Encrypt certs are 90 days. Enable <strong class="text-primary-text">autorenew with certbot</strong> (<code class="text-cyan-300">certbot renew --quiet</code> in a daily cron) or use a managed service like AWS Certificate Manager which auto-renews indefinitely.</p>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm">What "cipher_strength" Means</h4>
                            <p class="text-xs text-muted leading-relaxed">The <code class="text-cyan-300">cipher_strength</code> field reports the effective key size of the bulk encryption algorithm: <span class="text-emerald-300">"strong"</span> = 256-bit AES (industry standard), <span class="text-yellow-300">"medium"</span> = 128-bit AES (acceptable), <span class="text-red-300">"weak"</span> = RC4, 3DES, or export-grade ciphers (immediately disable these).</p>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">TLS Hardening Configuration</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Most TLS misconfigurations are nginx/Apache config issues that take minutes to fix. The Mozilla SSL Configuration Generator at ssl-config.mozilla.org produces hardened configs for your specific software version — highly recommended as a starting point.
                        </p>

                        <!-- Nginx SSL Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 shadow-[0_0_30px_rgba(249,115,22,0.05)] overflow-hidden font-mono text-xs mb-6">
                            <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-orange-500/30 tracking-widest uppercase">nginx SSL Block</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># Disable weak protocols entirely</span>
<span class="text-cyan-300">ssl_protocols</span> <span class="text-emerald-300">TLSv1.2 TLSv1.3</span>;

<span class="text-muted"># TLS 1.3 uses its own cipher list (no config needed)</span>
<span class="text-muted"># TLS 1.2 cipher restriction:</span>
<span class="text-cyan-300">ssl_ciphers</span> <span class="text-emerald-300">ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305</span>;
<span class="text-cyan-300">ssl_prefer_server_ciphers</span> <span class="text-orange-300">on</span>;

<span class="text-muted"># OCSP Stapling (avoids client needing to contact CA)</span>
<span class="text-cyan-300">ssl_stapling</span> <span class="text-orange-300">on</span>;
<span class="text-cyan-300">ssl_stapling_verify</span> <span class="text-orange-300">on</span>;

<span class="text-muted"># HSTS — force HTTPS for 1 year, all subdomains, preload</span>
<span class="text-cyan-300">add_header</span> Strict-Transport-Security <span class="text-emerald-300">"max-age=31536000; includeSubDomains; preload"</span> always;</pre>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">OCSP Stapling</h4>
                                <p class="text-xs text-muted leading-relaxed">Normally, on each TLS handshake, the browser contacts the CA's OCSP server to check if your cert was revoked — adding latency and exposing visitor IPs to the CA. OCSP stapling makes your server pre-fetch and cache the OCSP response, stapling it to the handshake. Faster and more private.</p>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">HSTS Preloading</h4>
                                <p class="text-xs text-muted leading-relaxed">Adding the <code class="text-cyan-300">preload</code> directive and submitting to <span class="text-orange-300">hstspreload.org</span> hardcodes your domain into browser source code. Browsers will refuse to connect over HTTP even on the first visit — before they see your HSTS header. Requirements: 1yr max-age + includeSubDomains + preload.</p>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm flex items-center gap-2"><ShieldCheck size={14}/> HPKP is Deprecated — Use CT Logs Instead</h4>
                            <p class="text-xs text-muted leading-relaxed">HTTP Public Key Pinning (HPKP) was a header that pinned specific cert public keys — widely deployed sites bricked themselves when their certs expired with pins still active. It was removed from browsers in 2018. The modern alternative is Certificate Transparency (CT) logs monitoring: services like <span class="text-orange-300">crt.sh</span> and <span class="text-orange-300">Facebook CT Monitor</span> alert you if any CA issues an unauthorized certificate for your domain.</p>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-orange-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-muted hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-glass rounded mx-1 text-muted border border-glass font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle hover:border-glass transition-all focus:ring-2 focus:ring-glass outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}

                    {#if activeTab < tabs.length - 1}
                        <button
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-orange-600 hover:bg-orange-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(249,115,22,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-orange-500/50 outline-none"
                        >
                            Continue <ChevronRight size={16} />
                        </button>
                    {:else}
                        <button
                            onclick={close}
                            class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(16,185,129,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-emerald-500/50 outline-none"
                        >
                            Complete
                        </button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
