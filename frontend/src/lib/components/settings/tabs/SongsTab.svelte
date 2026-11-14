<script lang="ts">
  import { BookOpen, Download, RefreshCw, Check, AlertCircle, Upload } from 'lucide-svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { readHttpError } from '$lib/media';
  import { notifySuccess, notifyError } from '$lib/toast';
  import type { SongSettings } from '$lib/types';

  interface Props {
    songSettings: SongSettings;
    onSelectLinesPerSlide: (lines: number) => Promise<void> | void;
  }

  let { songSettings, onSelectLinesPerSlide }: Props = $props();

  let isInstallingHarpa = $state(false);
  let harpaInstallMessage = $state<string | null>(null);
  let harpaInstallSuccess = $state<boolean | null>(null);

  let settingsImportTags = $state('');
  let isSettingsImporting = $state(false);
  let settingsImportResult = $state<string | null>(null);
  let settingsImportSuccess = $state<boolean | null>(null);
  let settingsSongFileInputEl = $state<HTMLInputElement | null>(null);

  const lineOptions = [
    {
      lines: 2,
      label: '2 Linhas / Slide',
      desc: 'Padrão ideal para leitura em congregação e retorno de palco.',
    },
    {
      lines: 1,
      label: '1 Linha / Slide',
      desc: 'Minimalista e de leitura rápida. Ótimo para transmissões e lower thirds.',
    },
    {
      lines: 3,
      label: '3 Linhas / Slide',
      desc: 'Estrofes médias em telas grandes com mais espaço vertical.',
    },
    {
      lines: 4,
      label: '4 Linhas / Slide',
      desc: 'Estrofes completas de 4 versos por slide.',
    },
    {
      lines: 0,
      label: 'Sem divisão automática',
      desc: 'Preserva estritamente as quebras originais sem reformatar.',
    },
  ];

  async function handleInstallHarpa() {
    isInstallingHarpa = true;
    harpaInstallMessage = null;
    harpaInstallSuccess = null;
    try {
      const res = await fetch('/api/songs/harpa/install', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          lines_per_slide: songSettings.lines_per_slide,
        }),
      });
      if (!res.ok) {
        const msg = await readHttpError(res, 'Falha ao instalar a Harpa Cristã.');
        harpaInstallSuccess = false;
        harpaInstallMessage = msg;
        notifyError('Erro ao instalar Harpa Cristã', msg);
        return;
      }

      const data = await res.json();
      if (data.success) {
        harpaInstallSuccess = true;
        harpaInstallMessage =
          data.message || '540 hinos da Harpa Cristã instalados com sucesso!';
        notifySuccess('Harpa Cristã Instalada!', harpaInstallMessage ?? undefined);
        if (typeof window !== 'undefined') {
          window.dispatchEvent(new CustomEvent('openpresenter:songs-changed'));
        }
      } else {
        harpaInstallSuccess = false;
        harpaInstallMessage = data.message || 'Falha ao instalar a Harpa Cristã.';
        notifyError('Erro ao instalar Harpa Cristã', harpaInstallMessage ?? undefined);
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      harpaInstallSuccess = false;
      harpaInstallMessage = `Erro de conexão: ${msg}`;
      notifyError('Erro de conexão', msg);
    } finally {
      isInstallingHarpa = false;
    }
  }

  function triggerSettingsSongImport() {
    settingsSongFileInputEl?.click();
  }

  async function handleSettingsSongFilesSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = input.files;
    if (!files || files.length === 0) return;

    isSettingsImporting = true;
    settingsImportResult = null;
    settingsImportSuccess = null;

    let totalImported = 0;
    const failedFiles: string[] = [];
    const tags = settingsImportTags
      .split(/[,;\s]+/)
      .map((t) => t.trim().toLowerCase().replace(/^#/, ''))
      .filter(Boolean);

    try {
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        const content = await file.text();
        const res = await fetch('/api/songs/import', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content,
            tags,
            lines_per_slide: songSettings.lines_per_slide,
          }),
        });
        if (res.ok) {
          const data = await res.json();
          totalImported += data.imported_count || 1;
        } else {
          const msg = await readHttpError(res, `Falha em ${file.name}`);
          failedFiles.push(`${file.name}: ${msg}`);
        }
      }

      if (failedFiles.length === 0) {
        settingsImportSuccess = true;
        settingsImportResult = `${totalImported} música(s) importada(s) com sucesso!`;
        notifySuccess(settingsImportResult);
      } else {
        settingsImportSuccess = false;
        settingsImportResult = `${totalImported} importada(s); falhas: ${failedFiles.join('; ')}`;
        notifyError('Falha parcial na importação', settingsImportResult);
      }

      if (totalImported > 0 && typeof window !== 'undefined') {
        window.dispatchEvent(new CustomEvent('openpresenter:songs-changed'));
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      settingsImportSuccess = false;
      settingsImportResult = `Erro ao importar: ${msg}`;
      notifyError('Erro na importação', msg);
    } finally {
      isSettingsImporting = false;
      input.value = '';
    }
  }
</script>

<div class="space-y-6 w-full max-w-5xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Catálogo de Músicas e Harpa Cristã</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Gerencie o acervo de louvores, instale a Harpa Cristã completa (540 hinos) e defina o padrão de divisão de slides.
    </p>
  </div>

  <!-- HARPA CRISTÃ -->
  <div class="p-4 rounded-xl border border-border bg-card space-y-4">
    <div class="flex items-start justify-between gap-3">
      <div class="flex items-start gap-3">
        <div class="w-10 h-10 rounded-lg bg-emerald-500/15 border border-emerald-500/30 flex items-center justify-center text-emerald-700 dark:text-emerald-400 shrink-0 mt-0.5">
          <BookOpen size={20} />
        </div>
        <div>
          <div class="flex flex-wrap items-center gap-2">
            <h4 class="text-sm font-bold text-foreground">Harpa Cristã Completa</h4>
            <Badge variant="outline" class="text-[11px] text-emerald-700 dark:text-emerald-400 border-emerald-500/30 font-mono">
              540 Hinos
            </Badge>
            <Badge variant="secondary" class="text-[11px]">
              Oficial
            </Badge>
          </div>
          <p class="text-xs text-muted-foreground mt-1 leading-relaxed">
            Instale todos os 540 hinos tradicionais da Harpa Cristã com numeração, estrofes, refrãos e classificação automática de tags (<code class="text-primary font-mono text-[11px]">#harpa</code>, <code class="text-primary font-mono text-[11px]">#hinos</code>, <code class="text-primary font-mono text-[11px]">#tradicional</code>).
          </p>
        </div>
      </div>
    </div>

    <div class="flex flex-wrap items-center gap-3 pt-2 border-t border-border">
      <Button
        variant="default"
        size="sm"
        onclick={handleInstallHarpa}
        disabled={isInstallingHarpa}
        class="text-xs gap-1.5 font-semibold"
      >
        {#if isInstallingHarpa}
          <RefreshCw size={13} class="animate-spin text-primary-foreground" />
          <span>Instalando 540 Hinos...</span>
        {:else}
          <Download size={13} />
          <span>Instalar Harpa Cristã (540 Hinos)</span>
        {/if}
      </Button>

      {#if harpaInstallMessage}
        <div
          role={harpaInstallSuccess ? 'status' : 'alert'}
          class="flex items-center gap-1.5 text-xs font-medium {harpaInstallSuccess
            ? 'text-emerald-700 dark:text-emerald-400'
            : 'text-red-700 dark:text-red-300'}"
        >
          {#if harpaInstallSuccess}
            <Check size={14} class="shrink-0" />
          {:else}
            <AlertCircle size={14} class="shrink-0" />
          {/if}
          <span>{harpaInstallMessage}</span>
        </div>
      {/if}
    </div>
  </div>

  <!-- PADRÃO DE LINHAS POR SLIDE -->
  <div class="p-4 rounded-xl border border-border bg-card space-y-4">
    <div>
      <div class="flex flex-wrap items-center gap-2">
        <h4 class="text-sm font-bold text-foreground">Padrão de Linhas por Slide</h4>
        <Badge variant="outline" class="text-[11px] text-primary border-primary/30">
          Recomendado: 2 linhas
        </Badge>
      </div>
      <p class="text-xs text-muted-foreground mt-1 leading-relaxed">
        Define o número padrão de linhas de letra em cada slide ao importar arquivos, pesquisar músicas e dividir letras no editor.
      </p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2.5">
      {#each lineOptions as opt}
        {@const selected = songSettings.lines_per_slide === opt.lines}
        <button
          type="button"
          aria-pressed={selected}
          onclick={() => onSelectLinesPerSlide(opt.lines)}
          class="p-3 rounded-lg border text-left transition-all cursor-pointer flex flex-col justify-between focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selected
            ? 'border-primary bg-primary/10 ring-1 ring-primary'
            : 'border-border bg-background/70 hover:bg-muted/50 text-muted-foreground'}"
        >
          <div class="flex items-center justify-between mb-1 w-full">
            <span class="text-xs font-bold {selected ? 'text-primary' : 'text-foreground'}">
              {opt.label}
            </span>
            {#if selected}
              <Check size={14} class="text-primary shrink-0" />
            {/if}
          </div>
          <p class="text-[11px] text-muted-foreground leading-snug">
            {opt.desc}
          </p>
        </button>
      {/each}
    </div>
  </div>

  <!-- IMPORTAÇÃO DE ARQUIVOS -->
  <div class="p-4 rounded-xl border border-border bg-card space-y-4">
    <div>
      <h4 class="text-sm font-bold text-foreground">Importar Arquivos de Músicas</h4>
      <p class="text-xs text-muted-foreground mt-1 leading-relaxed">
        Importe músicas individuais ou coletâneas em lote a partir de arquivos JSON, texto puro (.txt) ou Markdown (.md).
      </p>
    </div>

    <div class="space-y-3">
      <div>
        <label for="settings-import-tags-input" class="block text-xs font-medium text-foreground mb-1">
          Tags automáticas para as músicas importadas (opcional)
        </label>
        <input
          id="settings-import-tags-input"
          type="text"
          bind:value={settingsImportTags}
          placeholder="Ex: louvor, jovens, especial, culto"
          class="w-full h-8 px-3 bg-background border border-border rounded-md text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        />
        <p class="text-[11px] text-muted-foreground mt-1">
          Separe as tags por vírgula. Serão adicionadas ao catálogo para facilitar busca e filtragem.
        </p>
      </div>

      <div class="flex flex-wrap items-center gap-3 pt-1">
        <Button
          variant="outline"
          size="sm"
          onclick={triggerSettingsSongImport}
          disabled={isSettingsImporting}
          class="text-xs gap-1.5"
        >
          {#if isSettingsImporting}
            <RefreshCw size={13} class="animate-spin text-primary" />
            <span>Processando arquivos...</span>
          {:else}
            <Upload size={13} class="text-primary" />
            <span>Selecionar Arquivos (.json, .txt, .md)</span>
          {/if}
        </Button>
        <input
          type="file"
          bind:this={settingsSongFileInputEl}
          onchange={handleSettingsSongFilesSelected}
          accept=".json,.txt,.md,text/plain,application/json,text/markdown"
          multiple
          class="hidden"
        />

        {#if settingsImportResult}
          <span
            role={settingsImportSuccess ? 'status' : 'alert'}
            class="text-xs font-medium {settingsImportSuccess
              ? 'text-emerald-700 dark:text-emerald-400'
              : 'text-red-700 dark:text-red-300'}"
          >
            {settingsImportResult}
          </span>
        {/if}
      </div>
    </div>
  </div>
</div>
