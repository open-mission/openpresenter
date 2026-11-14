<script lang="ts">
  import {
    Sparkles,
    RefreshCw,
    AlertTriangle,
    CheckCircle2,
    RotateCcw,
    Download,
    ExternalLink,
  } from 'lucide-svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import type { UpdateCheckResponse, UpdaterStatusPayload } from '$lib/updater';

  interface Props {
    updateInfo: UpdateCheckResponse | null;
    updaterStatus: UpdaterStatusPayload | null;
    isCheckingUpdate: boolean;
    isRestarting: boolean;
    updaterError: string;
    onCheckUpdates: () => void;
    onStartUpdate: () => void;
    onRequestRestart: () => void;
  }

  let {
    updateInfo,
    updaterStatus,
    isCheckingUpdate,
    isRestarting,
    updaterError,
    onCheckUpdates,
    onStartUpdate,
    onRequestRestart,
  }: Props = $props();
</script>

<div class="space-y-6 w-full max-w-5xl">
  <!-- Banner da Marca Adaptável Dark/Light -->
  <div class="p-6 rounded-xl border border-border bg-card/60 text-center space-y-3">
    <img
      src="/branding/open-presenter-wordmark.svg"
      alt="OpenPresenter"
      class="h-10 mx-auto block dark:hidden"
    />
    <img
      src="/branding/open-presenter-wordmark-dark.svg"
      alt="OpenPresenter"
      class="h-10 mx-auto hidden dark:block"
    />
    <p class="text-xs text-muted-foreground">
      Sistema moderno e aberto de projeção para igrejas e eventos.
    </p>
    <div class="flex items-center justify-center gap-2 pt-1 flex-wrap">
      <Badge variant="secondary" class="text-[11px] font-mono">
        v{updateInfo?.current_version || '0.6.0'}
      </Badge>
      <Badge variant="outline" class="text-[11px] text-primary border-primary/40">GPL-3.0</Badge>
      <Badge variant="outline" class="text-[11px] text-muted-foreground">Rust · Axum · Svelte · SQLite</Badge>
    </div>
  </div>

  <!-- Grid com Atualizações e Documentação Offline -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 items-start">
    <!-- Auto-Updater -->
    <div class="p-5 rounded-xl border border-border bg-card/60 space-y-4">
      <div class="flex items-center justify-between gap-2">
        <div class="flex items-center gap-2">
          <Sparkles size={16} class="text-primary" />
          <h4 class="text-xs font-semibold text-foreground">Atualizações do Sistema</h4>
        </div>
        <Button
          variant="outline"
          size="sm"
          class="h-7 text-xs gap-1.5"
          onclick={onCheckUpdates}
          disabled={isCheckingUpdate}
        >
          <RefreshCw size={12} class={isCheckingUpdate ? 'animate-spin' : ''} />
          <span>{isCheckingUpdate ? 'Verificando...' : 'Verificar Atualizações'}</span>
        </Button>
      </div>

      {#if updaterError}
        <div
          role="alert"
          class="p-3 rounded-lg bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-start gap-2"
        >
          <AlertTriangle size={15} class="shrink-0 mt-0.5" />
          <span>{updaterError}</span>
        </div>
      {/if}

      {#if updaterStatus?.stage?.type === 'ReadyToRestart'}
        <div class="p-4 rounded-xl bg-emerald-500/15 border border-emerald-500/35 text-emerald-800 dark:text-emerald-200 text-xs space-y-3">
          <div class="flex items-center gap-2 font-semibold text-emerald-700 dark:text-emerald-300">
            <CheckCircle2 size={16} />
            <span>Atualização Instalada com Sucesso!</span>
          </div>
          <p class="text-[11px]">
            A nova versão foi gravada no executável. Reinicie o aplicativo OpenPresenter para aplicar todas as novidades.
          </p>
          <div class="pt-1">
            <Button
              variant="default"
              size="sm"
              class="bg-emerald-600 hover:bg-emerald-700 text-white font-medium text-xs h-8 gap-2 shadow-sm"
              onclick={onRequestRestart}
              disabled={isRestarting}
            >
              <RotateCcw size={14} class={isRestarting ? 'animate-spin' : ''} />
              <span>{isRestarting ? 'Reiniciando Servidor...' : 'Reiniciar Aplicativo Agora'}</span>
            </Button>
          </div>
        </div>
      {:else if updaterStatus?.stage?.type === 'Downloading'}
        <div class="p-4 rounded-xl border border-border bg-card space-y-3">
          <div class="flex items-center justify-between text-xs">
            <span class="text-foreground font-medium flex items-center gap-1.5">
              <Download size={14} class="text-primary" />
              Baixando nova versão...
            </span>
            <span class="font-mono text-primary font-bold">
              {updaterStatus.stage.percent}%
            </span>
          </div>
          <div class="w-full h-2 rounded-full bg-muted overflow-hidden">
            <div
              class="h-full bg-primary transition-all duration-200 rounded-full"
              style="width: {updaterStatus.stage.percent}%"
            ></div>
          </div>
          <div class="text-[11px] text-muted-foreground font-mono text-right">
            {(updaterStatus.stage.downloaded_bytes / (1024 * 1024)).toFixed(1)} MB / {(updaterStatus.stage.total_bytes / (1024 * 1024)).toFixed(1)} MB
          </div>
        </div>
      {:else if updaterStatus?.stage?.type === 'Extracting'}
        <div class="p-4 rounded-xl border border-border bg-card flex items-center gap-3 text-xs text-foreground">
          <RefreshCw size={16} class="animate-spin text-primary shrink-0" />
          <span>Extraindo arquivos e aplicando a substituição atômica no executável...</span>
        </div>
      {:else if updaterStatus?.stage?.type === 'Error'}
        <div role="alert" class="p-4 rounded-xl bg-red-500/10 border border-red-500/35 text-xs text-red-700 dark:text-red-300 space-y-2">
          <div class="flex items-center gap-2 font-semibold">
            <AlertTriangle size={15} />
            <span>Erro ao aplicar atualização</span>
          </div>
          <p class="text-[11px]">{updaterStatus.stage.message}</p>
          <Button variant="outline" size="sm" class="h-7 text-xs" onclick={onStartUpdate}>
            Tentar Novamente
          </Button>
        </div>
      {:else if updateInfo?.update_available}
        <div class="p-4 rounded-xl border border-emerald-500/35 bg-emerald-500/10 space-y-3">
          <div class="flex items-center justify-between gap-2">
            <div class="flex items-center gap-2">
              <Badge variant="default" class="bg-emerald-600 text-white hover:bg-emerald-600 text-[11px] font-bold">
                NOVO
              </Badge>
              <span class="text-xs font-bold text-foreground">Versão {updateInfo.latest_version} disponível</span>
            </div>
            {#if updateInfo.published_at}
              <span class="text-[11px] text-muted-foreground">
                {new Date(updateInfo.published_at).toLocaleDateString('pt-BR')}
              </span>
            {/if}
          </div>

          <div class="text-xs text-foreground font-medium">
            {updateInfo.release_name}
          </div>

          {#if updateInfo.release_notes}
            <div class="p-2.5 rounded-lg bg-background border border-border max-h-36 overflow-y-auto text-[11px] text-foreground/90 font-mono whitespace-pre-wrap leading-relaxed">
              {updateInfo.release_notes}
            </div>
          {/if}

          <div class="flex items-center justify-between pt-1 text-[11px] text-muted-foreground">
            <span>Arquivo: {updateInfo.asset_name || 'Pacote multiplataforma'}</span>
            {#if updateInfo.asset_size}
              <span>{(updateInfo.asset_size / (1024 * 1024)).toFixed(1)} MB</span>
            {/if}
          </div>

          <div class="pt-2 flex items-center justify-end gap-2">
            <Button
              variant="default"
              size="sm"
              class="gap-1.5 font-semibold bg-emerald-600 hover:bg-emerald-700 text-white"
              onclick={onStartUpdate}
              disabled={!updateInfo.download_url}
            >
              <Download size={14} />
              <span>Baixar e Atualizar Agora</span>
            </Button>
          </div>
        </div>
      {:else if updateInfo && !updateInfo.update_available}
        <div class="p-3.5 rounded-lg border border-border bg-card flex items-center gap-2.5 text-xs text-foreground">
          <CheckCircle2 size={16} class="text-emerald-600 dark:text-emerald-400 shrink-0" />
          <span>O OpenPresenter está na versão mais recente (v{updateInfo.current_version}).</span>
        </div>
      {:else}
        <div class="p-3.5 rounded-lg border border-border bg-card text-xs text-muted-foreground flex items-center justify-between gap-2">
          <span>Nenhuma verificação de atualização realizada nesta sessão.</span>
          <Button variant="ghost" size="sm" class="min-h-7 text-xs shrink-0" onclick={onCheckUpdates}>
            Checar agora
          </Button>
        </div>
      {/if}
    </div>

    <!-- Documentação da API -->
    <div class="p-5 rounded-xl border border-border bg-card/60 space-y-4">
      <div>
        <div class="flex items-center gap-2 flex-wrap">
          <h4 class="text-xs font-semibold text-foreground">Documentação da API & Integrações</h4>
          <Badge variant="outline" class="text-[11px] text-purple-700 dark:text-purple-300 border-purple-500/35">
            100% Offline
          </Badge>
        </div>
        <p class="text-[11px] text-muted-foreground mt-1 leading-relaxed">
          O OpenPresenter embute documentação OpenAPI 3.0 interativa que funciona 100% offline para integrar escalas ou automações externas via REST e WebSockets.
        </p>
      </div>

      <div class="flex flex-wrap items-center gap-2 pt-1">
        <a
          href="/docs"
          target="_blank"
          rel="noreferrer"
          class="inline-flex min-h-8 items-center gap-1.5 px-3 py-1.5 rounded-lg bg-purple-500/15 hover:bg-purple-500/25 text-purple-700 dark:text-purple-300 border border-purple-500/35 text-xs font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <ExternalLink size={13} />
          <span>Abrir Scalar (/docs)</span>
        </a>
        <a
          href="/swagger"
          target="_blank"
          rel="noreferrer"
          class="inline-flex min-h-8 items-center gap-1.5 px-3 py-1.5 rounded-lg bg-muted hover:bg-muted/80 text-foreground border border-border text-xs font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <ExternalLink size={13} />
          <span>Abrir Swagger (/swagger)</span>
        </a>
        <a
          href="/api/openapi.json"
          target="_blank"
          rel="noreferrer"
          class="inline-flex min-h-8 items-center gap-1.5 px-3 py-1.5 rounded-lg bg-card hover:bg-muted text-muted-foreground hover:text-foreground border border-border text-xs font-mono transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <span>openapi.json</span>
        </a>
      </div>
    </div>
  </div>

  <!-- Reinicialização Segura (Com Confirmação) -->
  <div class="p-4 rounded-xl border border-border bg-card/60">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div>
        <h4 class="text-xs font-semibold text-foreground">Reiniciar Servidor e Aplicação</h4>
        <p class="text-[11px] text-muted-foreground mt-0.5">
          Recarrega o executável do OpenPresenter e reinicia os serviços mantendo dados e configurações salvos. Exige confirmação.
        </p>
      </div>
      <Button
        variant="outline"
        size="sm"
        class="h-8 text-xs gap-1.5 shrink-0 border-amber-500/40 text-amber-800 dark:text-amber-300 hover:bg-amber-500/10"
        onclick={onRequestRestart}
        disabled={isRestarting}
      >
        <RotateCcw size={12} class={isRestarting ? 'animate-spin' : ''} />
        <span>{isRestarting ? 'Reiniciando...' : 'Reiniciar Servidor'}</span>
      </Button>
    </div>
  </div>

  <div class="text-[11px] text-muted-foreground space-y-1.5 text-center">
    <p>
      Repositório de código aberto:
      <a
        href="https://github.com/open-mission/openpresenter"
        target="_blank"
        rel="noreferrer"
        class="inline-flex min-h-7 items-center px-1.5 py-1 text-primary hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring rounded"
      >
        github.com/open-mission/openpresenter
      </a>
    </p>
    <p>Todos os seus dados (banco SQLite, mídias e bíblias) permanecem preservados em qualquer atualização.</p>
  </div>
</div>
