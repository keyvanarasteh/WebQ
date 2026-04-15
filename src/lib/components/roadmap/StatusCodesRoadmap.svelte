<script lang="ts">
  import { Info, CheckCircle, ArrowRightLeft, AlertTriangle, ServerCrash, X } from "lucide-svelte";
  import { fade, fly } from 'svelte/transition';

  type HttpCode = {
    code: number;
    label: string;
    description: string;
  };

  type Category = {
    title: string;
    colorClasses: string;
    icon: any;
    codes: HttpCode[];
  };

  const statusCategories: Category[] = [
    {
      title: "1XX Informational",
      colorClasses: "text-blue-400 bg-blue-500/10 border-blue-500/30 hover:bg-blue-500/20",
      icon: Info,
      codes: [
        { code: 100, label: "Continue", description: "The server has received the request headers and the client should proceed to send the request body." },
        { code: 101, label: "Switching Protocols", description: "The requester has asked the server to switch protocols and the server has agreed to do so." }
      ]
    },
    {
      title: "2XX Success",
      colorClasses: "text-emerald-400 bg-emerald-500/10 border-emerald-500/30 hover:bg-emerald-500/20",
      icon: CheckCircle,
      codes: [
        { code: 200, label: "OK", description: "Standard response for successful HTTP requests." },
        { code: 201, label: "Created", description: "The request has been fulfilled, resulting in the creation of a new resource." },
        { code: 202, label: "Accepted", description: "The request has been accepted for processing, but the processing has not been completed." },
        { code: 204, label: "No Content", description: "The server successfully processed the request and is not returning any content." }
      ]
    },
    {
      title: "3XX Redirection",
      colorClasses: "text-cyan-400 bg-cyan-500/10 border-cyan-500/30 hover:bg-cyan-500/20",
      icon: ArrowRightLeft,
      codes: [
        { code: 301, label: "Moved Permanently", description: "This and all future requests should be directed to the given URI." },
        { code: 302, label: "Found", description: "Tells the client to look at (browse to) another URL." },
        { code: 304, label: "Not Modified", description: "Indicates that the resource has not been modified since the version specified by the request headers." },
        { code: 307, label: "Temporary Redirect", description: "The request should be repeated with another URI; however, future requests should still use the original URI." },
        { code: 308, label: "Permanent Redirect", description: "The request and all future requests should be repeated using another URI." }
      ]
    },
    {
      title: "4XX Client Errors",
      colorClasses: "text-amber-400 bg-amber-500/10 border-amber-500/30 hover:bg-amber-500/20",
      icon: AlertTriangle,
      codes: [
        { code: 400, label: "Bad Request", description: "The server cannot or will not process the request due to an apparent client error." },
        { code: 401, label: "Unauthorized", description: "Similar to 403 Forbidden, but specifically for use when authentication is required and has failed or has not yet been provided." },
        { code: 403, label: "Forbidden", description: "The request contained valid data and was understood by the server, but the server is refusing action (e.g. lacks credentials)." },
        { code: 404, label: "Not Found", description: "The requested resource could not be found but may be available in the future." },
        { code: 405, label: "Method Not Allowed", description: "A request method is not supported for the requested resource." },
        { code: 408, label: "Request Timeout", description: "The server timed out waiting for the request." },
        { code: 429, label: "Too Many Requests", description: "The user has sent too many requests in a given amount of time (rate limiting)." }
      ]
    },
    {
      title: "5XX Server Errors",
      colorClasses: "text-rose-400 bg-rose-500/10 border-rose-500/30 hover:bg-rose-500/20",
      icon: ServerCrash,
      codes: [
        { code: 500, label: "Internal Server Error", description: "A generic error message, given when an unexpected condition was encountered and no more specific message is suitable." },
        { code: 501, label: "Not Implemented", description: "The server either does not recognize the request method, or it lacks the ability to fulfill the request." },
        { code: 502, label: "Bad Gateway", description: "The server was acting as a gateway or proxy and received an invalid response from the upstream server." },
        { code: 503, label: "Service Unavailable", description: "The server cannot handle the request (because it is overloaded or down for maintenance)." },
        { code: 504, label: "Gateway Timeout", description: "The server was acting as a gateway or proxy and did not receive a timely response from the upstream server." }
      ]
    }
  ];

  let selectedCode = $state<HttpCode | null>(null);
  let selectedCategoryColor = $state<string>("");

  const totalCodes = statusCategories.reduce((acc, cat) => acc + cat.codes.length, 0);

  function openModal(code: HttpCode, categoryColor: string) {
    selectedCode = code;
    selectedCategoryColor = categoryColor;
  }

  function closeModal() {
    selectedCode = null;
  }
</script>

<svelte:window on:keydown={(e) => e.key === 'Escape' && closeModal()} />

<div class="animate-fade-in space-y-8">
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden">
      <div class="absolute -right-4 -top-4 w-24 h-24 bg-blue-500/10 rounded-full blur-2xl"></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">HTTP Specifications</p>
      <p class="text-5xl font-black text-primary-text mt-2">{totalCodes}</p>
    </div>
    <div class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden flex flex-col justify-end">
      <div class="absolute -right-4 -top-4 w-24 h-24 bg-rose-500/10 rounded-full blur-2xl"></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">Protocol Mapping</p>
      <p class="text-3xl font-black text-rose-400 mt-2">HTTP/1.1 & HTTP/2</p>
    </div>
    <div class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden flex flex-col justify-end">
      <div class="absolute -right-4 -top-4 w-24 h-24 bg-purple-500/10 rounded-full blur-2xl"></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">Compliance</p>
      <p class="text-3xl font-black text-purple-400 mt-4">RFC 9110</p>
    </div>
  </div>

  <div class="space-y-12 border-t border-base pt-10">
    {#each statusCategories as category}
      <div class="flex flex-col gap-4">
        <h3 class="text-xl font-black text-secondary-text tracking-widest uppercase flex items-center gap-3">
          <svelte:component this={category.icon} class={`size-6 ${category.colorClasses.split(' ')[0]}`} />
          {category.title}
        </h3>
        
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {#each category.codes as httpCode (httpCode.code)}
            <button
              onclick={() => openModal(httpCode, category.colorClasses)}
              class={`flex flex-col items-start justify-center p-4 rounded-xl border bg-surface/50 transition-all font-mono text-left group cursor-pointer ${category.colorClasses}`}
            >
              <div class="flex items-center justify-between w-full">
                <span class="text-2xl font-black">{httpCode.code}</span>
                <Info class="size-4 opacity-0 group-hover:opacity-100 transition-opacity" />
              </div>
              <span class="text-sm font-bold mt-1 max-w-full truncate">{httpCode.label}</span>
            </button>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>

{#if selectedCode}
  <!-- Modal Overlay -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-background/80 backdrop-blur-md"
    in:fade={{ duration: 150 }}
    out:fade={{ duration: 150 }}
    onclick={closeModal}
  >
    <!-- Modal Content -->
    <div 
      class="bg-surface border border-base/50 rounded-2xl shadow-2xl overflow-hidden w-full max-w-lg relative"
      in:fly={{ y: 20, duration: 200 }}
      out:fly={{ y: 20, duration: 200 }}
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Modal Header -->
      <div class={`px-6 py-5 border-b border-base/50 flex items-center justify-between ${selectedCategoryColor.split(' ').slice(0,2).join(' ')}`}>
        <div class="flex items-center gap-3">
          <span class="text-3xl font-black">{selectedCode.code}</span>
          <h2 class="text-lg font-bold font-fira uppercase tracking-wide">{selectedCode.label}</h2>
        </div>
        <button onclick={closeModal} class="p-2 hover:bg-black/20 rounded-lg transition-colors">
          <X class="size-5" />
        </button>
      </div>
      
      <!-- Modal Body -->
      <div class="p-6">
        <p class="text-secondary-text text-sm leading-relaxed">
          {selectedCode.description}
        </p>
      </div>

      <!-- Action Footer -->
      <div class="bg-base/20 px-6 py-4 flex justify-end">
        <button onclick={closeModal} class="px-5 py-2 rounded-lg bg-surface border border-base text-muted hover:text-primary-text transition-colors text-sm font-bold uppercase tracking-widest">
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
