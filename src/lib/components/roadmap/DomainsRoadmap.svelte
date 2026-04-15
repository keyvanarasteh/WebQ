<script lang="ts">
  import { CheckCircle, Globe, MapPin, Sparkles } from "lucide-svelte";

  const tldCategories = [
    {
      name: "Core gTLDs",
      icon: Globe,
      domains: [
        { code: "com", status: "supported" },
        { code: "net", status: "supported" },
        { code: "org", status: "supported" },
        { code: "edu", status: "supported" },
        { code: "gov", status: "supported" },
        { code: "mil", status: "supported" },
        { code: "int", status: "supported" },
        { code: "info", status: "supported" },
        { code: "biz", status: "supported" },
        { code: "name", status: "supported" },
        { code: "pro", status: "supported" },
        { code: "aero", status: "supported" },
        { code: "coop", status: "supported" },
        { code: "museum", status: "supported" },
        { code: "arpa", status: "supported" },
      ],
    },
    {
      name: "New Highly Active gTLDs",
      icon: Sparkles,
      domains: [
        { code: "xyz", status: "supported" },
        { code: "top", status: "supported" },
        { code: "club", status: "supported" },
        { code: "vip", status: "supported" },
        { code: "app", status: "supported" },
        { code: "dev", status: "supported" },
        { code: "shop", status: "supported" },
        { code: "store", status: "supported" },
        { code: "site", status: "supported" },
        { code: "online", status: "supported" },
        { code: "tech", status: "supported" },
        { code: "ai", status: "supported" },
        { code: "io", status: "supported" },
        { code: "me", status: "supported" },
        { code: "tv", status: "supported" },
        { code: "cc", status: "supported" },
        { code: "website", status: "supported" },
        { code: "space", status: "supported" },
        { code: "press", status: "supported" },
        { code: "design", status: "supported" },
        { code: "agency", status: "supported" },
        { code: "photography", status: "supported" },
        { code: "email", status: "supported" },
        { code: "network", status: "supported" },
        { code: "today", status: "supported" },
        { code: "icu", status: "supported" },
        { code: "wang", status: "supported" },
        { code: "win", status: "supported" },
        { code: "mobi", status: "supported" },
        { code: "asia", status: "supported" },
        { code: "tel", status: "supported" },
        { code: "cloud", status: "supported" },
        { code: "global", status: "supported" },
        { code: "host", status: "supported" },
        { code: "link", status: "supported" },
      ],
    },
    {
      name: "Country Code TLDs (ccTLDs)",
      icon: MapPin,
      domains: [
        { code: "ac", status: "supported" },
        { code: "ae", status: "supported" },
        { code: "am", status: "supported" },
        { code: "at", status: "supported" },
        { code: "au", label: "au / com.au", status: "supported" },
        { code: "be", status: "supported" },
        { code: "br", label: "br / com.br", status: "supported" },
        { code: "by", status: "supported" },
        { code: "ca", status: "supported" },
        { code: "ch", status: "supported" },
        { code: "cl", status: "supported" },
        { code: "cn", label: "cn / com.cn", status: "supported" },
        { code: "co", status: "supported" },
        { code: "cz", status: "supported" },
        { code: "de", status: "supported" },
        { code: "dk", status: "supported" },
        { code: "dz", status: "supported" },
        { code: "es", status: "supported" },
        { code: "eu", status: "supported" },
        { code: "fi", status: "supported" },
        { code: "fr", status: "supported" },
        { code: "hk", status: "supported" },
        { code: "hr", status: "supported" },
        { code: "hu", status: "supported" },
        { code: "id", label: "id / co.id", status: "supported" },
        { code: "ie", status: "supported" },
        { code: "il", label: "il / co.il", status: "supported" },
        { code: "in", label: "in / co.in", status: "supported" },
        { code: "ir", status: "supported" },
        { code: "is", status: "supported" },
        { code: "it", status: "supported" },
        { code: "jp", label: "jp / co.jp", status: "supported" },
        { code: "kr", label: "kr / co.kr", status: "supported" },
        { code: "kz", status: "supported" },
        { code: "lt", status: "supported" },
        { code: "lu", status: "supported" },
        { code: "lv", status: "supported" },
        { code: "ma", status: "supported" },
        { code: "mx", label: "mx / com.mx", status: "supported" },
        { code: "nl", status: "supported" },
        { code: "no", status: "supported" },
        { code: "nz", label: "nz / co.nz", status: "supported" },
        { code: "pt", status: "supported" },
        { code: "pl", label: "pl / com.pl", status: "supported" },
        { code: "ro", status: "supported" },
        { code: "rs", status: "supported" },
        { code: "ru", status: "supported" },
        { code: "se", status: "supported" },
        { code: "sg", label: "sg / com.sg", status: "supported" },
        { code: "si", status: "supported" },
        { code: "sk", status: "supported" },
        { code: "su", status: "supported" },
        { code: "th", label: "th / co.th", status: "supported" },
        { code: "tr", label: "tr / com.tr", status: "supported" },
        { code: "tw", label: "tw / com.tw", status: "supported" },
        { code: "ua", label: "ua / com.ua", status: "supported" },
        { code: "uk", label: "uk / co.uk", status: "supported" },
        { code: "us", status: "supported" },
        { code: "za", label: "za / co.za", status: "supported" },
      ],
    },
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
        Total Integrated
      </p>
      <p class="text-5xl font-black text-primary-text mt-2">109 TLDs</p>
    </div>
    <div
      class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden"
    >
      <div
        class="absolute -right-4 -top-4 w-24 h-24 bg-green-500/10 rounded-full blur-2xl"
      ></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">
        Global Top 100
      </p>
      <p class="text-5xl font-black text-green-400 mt-2">100%</p>
    </div>
    <div
      class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden"
    >
      <div
        class="absolute -right-4 -top-4 w-24 h-24 bg-purple-500/10 rounded-full blur-2xl"
      ></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">
        Parser Type
      </p>
      <p class="text-3xl font-black text-purple-400 mt-4">Native Port 43 TCP</p>
    </div>
  </div>

  <div class="space-y-10 border-t border-base pt-10">
    {#each tldCategories as category (category.name)}
      <div>
        <h3
          class="text-xl font-bold text-accent uppercase tracking-widest flex items-center gap-3 mb-6"
        >
          {#if category.icon}<category.icon class="size-6 text-cyan-400" />{/if}
          {category.name}
        </h3>
        <div
          class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3"
        >
          {#each category.domains as domain (domain.code)}
            <div
              class="flex items-center justify-between p-2.5 rounded-lg border bg-surface/50 transition-all border-green-500/30 hover:bg-green-500/5 group cursor-default"
            >
              <span class="text-sm font-mono text-primary-text font-semibold"
                >.{domain.label || domain.code}</span
              >
              <CheckCircle class="size-4 text-green-400 shrink-0" />
            </div>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>
