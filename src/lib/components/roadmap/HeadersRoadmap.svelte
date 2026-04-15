<script lang="ts">
  import { CheckCircle, ShieldCheck } from "lucide-svelte";

  const headersMap = [
    { code: "strict-transport-security", label: "HSTS", status: "supported" },
    { code: "x-frame-options", label: "X-Frame-Options", status: "supported" },
    { code: "x-content-type-options", label: "X-Content-Type-Options", status: "supported" },
    { code: "x-xss-protection", label: "X-XSS-Protection", status: "supported" },
    { code: "content-security-policy", label: "CSP", status: "supported" },
    { code: "content-security-policy-report-only", label: "CSP Report-Only", status: "supported" },
    { code: "permissions-policy", label: "Permissions-Policy", status: "supported" },
    { code: "referrer-policy", label: "Referrer-Policy", status: "supported" },
    { code: "x-permitted-cross-domain-policies", label: "X-Permitted-Cross-Domain", status: "supported" },
    { code: "expect-ct", label: "Expect-CT", status: "supported" },
    { code: "cross-origin-embedder-policy", label: "COEP", status: "supported" },
    { code: "cross-origin-opener-policy", label: "COOP", status: "supported" },
    { code: "cross-origin-resource-policy", label: "CORP", status: "supported" },
    { code: "access-control-allow-origin", label: "CORS Allow-Origin", status: "supported" },
    { code: "server-timing", label: "Server-Timing", status: "supported" }
  ];
</script>

<div class="animate-fade-in space-y-8">
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div
      class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden"
    >
      <div
        class="absolute -right-4 -top-4 w-24 h-24 bg-cyan-500/10 rounded-full blur-2xl"
      ></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">
        Monitored Headers
      </p>
      <p class="text-5xl font-black text-primary-text mt-2">{headersMap.length}</p>
    </div>
    <div
      class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden flex flex-col justify-end"
    >
      <div
        class="absolute -right-4 -top-4 w-24 h-24 bg-green-500/10 rounded-full blur-2xl"
      ></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">
        Mapping Level
      </p>
      <p class="text-3xl font-black text-green-400 mt-2">HTTP L7 Core</p>
    </div>
    <div
      class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden flex flex-col justify-end"
    >
      <div
        class="absolute -right-4 -top-4 w-24 h-24 bg-purple-500/10 rounded-full blur-2xl"
      ></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">
        Compliance Suite
      </p>
      <p class="text-3xl font-black text-purple-400 mt-4">OWASP & W3C</p>
    </div>
  </div>

  <div class="space-y-10 border-t border-base pt-10">
    <div>
      <h3
        class="text-xl font-bold text-accent uppercase tracking-widest flex items-center gap-3 mb-6"
      >
        <ShieldCheck class="size-6 text-cyan-400" />
        Application Security Constraints
      </h3>
      <div
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4"
      >
        {#each headersMap as header (header.code)}
          <div
            class="flex items-center justify-between p-4 rounded-lg border bg-surface/50 transition-all border-green-500/30 hover:bg-green-500/5 group cursor-default"
          >
            <div class="flex flex-col gap-1">
              <span class="text-sm font-mono text-primary-text font-bold"
                >{header.label}</span
              >
              <span class="text-xs text-muted font-mono">{header.code}</span>
            </div>
            <CheckCircle class="size-5 text-green-400 shrink-0 opacity-80" />
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
