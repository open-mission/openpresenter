<script lang="ts">
  import { EyeOff, LogOut, Moon, Square, Sun, Type, Wifi, WifiOff } from 'lucide-svelte';
  import type { RemoteAuthSession } from '$lib/types';
  import { getInitials } from '$lib/remoteData';
  import { reconnectWebSocket } from '$lib/ws';

  interface Props {
    session: RemoteAuthSession;
    isConnected: boolean;
    canControlRemote: boolean;
    isBlackout: boolean;
    isClear: boolean;
    isLightTheme: boolean;
    onToggleTheme: () => void;
    onLogout: () => void;
    onToggleBlackout: () => void;
    onToggleClearText: () => void;
    onRequestStop: () => void;
  }

  let {
    session,
    isConnected,
    canControlRemote,
    isBlackout,
    isClear,
    isLightTheme,
    onToggleTheme,
    onLogout,
    onToggleBlackout,
    onToggleClearText,
    onRequestStop,
  }: Props = $props();
</script>

<header class="shrink-0 bg-card/95 backdrop-blur-md border-b border-border pt-[env(safe-area-inset-top)]">
  <div class="max-w-md md:max-w-4xl lg:max-w-5xl mx-auto px-3 sm:px-5 py-2.5 flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
    <!-- Identidade, Status WS e Utilidades -->
    <div class="flex items-center justify-between gap-2 min-w-0">
      <div class="flex items-center gap-2.5 min-w-0">
        <div
          class="w-9 h-9 rounded-full flex items-center justify-center font-bold text-xs text-white shrink-0 shadow-xs"
          style="background-color: {session.color};"
          aria-hidden="true"
        >
          {getInitials(session.name)}
        </div>
        <div class="min-w-0">
          <div class="flex items-center gap-2 min-w-0">
            <span class="font-bold text-sm text-foreground truncate max-w-[140px] sm:max-w-[200px]">
              {session.name}
            </span>
            <button
              type="button"
              onclick={() => {
                if (!isConnected) reconnectWebSocket('remote');
              }}
              aria-label={isConnected
                ? 'Conectado ao servidor da mesa em tempo real'
                : 'Desconectado da mesa. Toque para tentar reconectar'}
              title={isConnected ? 'Conectado em tempo real' : 'Offline — toque para reconectar'}
              class="min-h-7 px-2 py-0.5 rounded-full text-[11px] font-semibold inline-flex items-center gap-1 border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring shrink-0 {isConnected
                ? 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border-emerald-600/35 dark:border-emerald-500/40 cursor-default'
                : 'bg-red-500/15 text-red-700 dark:text-red-300 border-red-600/35 dark:border-red-500/40 hover:bg-red-500/25 cursor-pointer'}"
            >
              {#if isConnected}
                <Wifi size={12} aria-hidden="true" />
                <span>Remoto</span>
              {:else}
                <WifiOff size={12} aria-hidden="true" />
                <span>Offline</span>
              {/if}
            </button>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-1.5 shrink-0 md:order-last">
        <button
          type="button"
          onclick={onToggleTheme}
          aria-label={isLightTheme ? 'Alternar para tema escuro' : 'Alternar para tema claro'}
          title={isLightTheme ? 'Mudar para tema escuro' : 'Mudar para tema claro'}
          class="h-9 w-9 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          {#if isLightTheme}
            <Moon size={16} aria-hidden="true" />
          {:else}
            <Sun size={16} aria-hidden="true" />
          {/if}
        </button>

        <button
          type="button"
          onclick={onLogout}
          aria-label="Sair ou trocar de voluntário"
          title="Sair / Trocar usuário"
          class="h-9 px-2.5 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold inline-flex items-center gap-1.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          <LogOut size={15} aria-hidden="true" />
          <span class="hidden sm:inline">Sair</span>
        </button>
      </div>
    </div>

    <!-- Controles Críticos de Saída (sempre visíveis para operadores autorizados) -->
    {#if canControlRemote}
      <div class="grid grid-cols-3 md:flex items-center gap-1.5" role="group" aria-label="Controles de tela pública">
        <button
          type="button"
          onclick={onToggleBlackout}
          aria-pressed={isBlackout}
          class="min-h-9 px-2.5 rounded-xl text-xs font-semibold border flex items-center justify-center gap-1.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {isBlackout
            ? 'bg-amber-700 hover:bg-amber-800 text-white border-amber-800 shadow-xs'
            : 'bg-secondary hover:bg-muted text-foreground border-border'}"
        >
          <EyeOff size={14} aria-hidden="true" />
          <span>Preto</span>
        </button>

        <button
          type="button"
          onclick={onToggleClearText}
          aria-pressed={isClear}
          class="min-h-9 px-2.5 rounded-xl text-xs font-semibold border flex items-center justify-center gap-1.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {isClear
            ? 'bg-sky-700 hover:bg-sky-800 text-white border-sky-800 shadow-xs'
            : 'bg-secondary hover:bg-muted text-foreground border-border'}"
        >
          <Type size={14} aria-hidden="true" />
          <span>Limpar</span>
        </button>

        <button
          type="button"
          onclick={onRequestStop}
          aria-label="Parar projeção ao vivo"
          class="min-h-9 px-2.5 rounded-xl text-xs font-semibold bg-red-500/15 hover:bg-red-500/25 text-red-700 dark:text-red-300 border border-red-600/35 dark:border-red-500/40 flex items-center justify-center gap-1.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          <Square size={14} aria-hidden="true" />
          <span>Parar</span>
        </button>
      </div>
    {/if}
  </div>
</header>
