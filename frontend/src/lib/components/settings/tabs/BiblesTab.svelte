<script lang="ts">
  import {
    BookOpen,
    Upload,
    RefreshCw,
    AlertCircle,
    Check,
    Edit3,
    Trash2,
  } from 'lucide-svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import BibleEditModal from '../BibleEditModal.svelte';
  import { readFileAsBase64, readHttpError } from '$lib/media';
  import { notifySuccess, notifyError } from '$lib/toast';
  import type { BibleVersionSummary } from '$lib/types';
  import type { SettingsConfirmRequest } from '../types';

  interface Props {
    biblesList: BibleVersionSummary[];
    isLoadingBibles: boolean;
    bibleError: string;
    onReloadBibles: () => Promise<void>;
    onDismissError: () => void;
    onRequestConfirm: (req: SettingsConfirmRequest) => void;
  }

  let {
    biblesList,
    isLoadingBibles,
    bibleError,
    onReloadBibles,
    onDismissError,
    onRequestConfirm,
  }: Props = $props();

  let actionError = $state('');
  let isImportingBible = $state(false);
  let importProgress = $state({
    current: 0,
    total: 0,
    currentFileName: '',
    successCount: 0,
    failedCount: 0,
  });
  let showPathImport = $state(false);
  let importPathInput = $state('');
  let bibleFileInput = $state<HTMLInputElement | null>(null);

  let isEditingBible = $state(false);
  let editingBibleId = $state<string | null>(null);
  let editBibleName = $state('');
  let editBibleAbbr = $state('');
  let editBibleIsDefault = $state(false);
  let isSavingEdit = $state(false);
  let editError = $state('');

  const visibleError = $derived(actionError || bibleError);

  function dismissVisibleError() {
    actionError = '';
    onDismissError();
  }

  async function handleBibleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const files = Array.from(input.files);

    isImportingBible = true;
    actionError = '';
    importProgress = {
      current: 0,
      total: files.length,
      currentFileName: '',
      successCount: 0,
      failedCount: 0,
    };

    const failedFiles: { name: string; error: string }[] = [];
    let successCount = 0;

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      importProgress = {
        current: i + 1,
        total: files.length,
        currentFileName: file.name,
        successCount,
        failedCount: failedFiles.length,
      };

      try {
        const base64 = await readFileAsBase64(file);
        const res = await fetch('/api/bibles/import', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content_base64: base64,
          }),
        });

        if (!res.ok) {
          if (res.status === 413) {
            throw new Error('O arquivo excede o limite máximo de tamanho.');
          }
          const msg = await readHttpError(res, 'Falha ao importar Bíblia SQLite');
          if (msg.includes('length limit exceeded')) {
            throw new Error('O arquivo excede o limite máximo de tamanho.');
          }
          throw new Error(msg);
        }

        successCount++;
        notifySuccess('Bíblia importada', `${file.name} (${i + 1}/${files.length})`);
        await onReloadBibles();
      } catch (err) {
        const errorMsg =
          err instanceof Error ? err.message : 'Erro ao importar arquivo de Bíblia';
        failedFiles.push({ name: file.name, error: errorMsg });
        notifyError('Falha na importação', `${file.name}: ${errorMsg}`);
      }
    }

    if (bibleFileInput) {
      bibleFileInput.value = '';
    }
    isImportingBible = false;

    if (failedFiles.length === 0) {
      notifySuccess(
        'Importação concluída',
        files.length === 1
          ? 'Versão da Bíblia importada com sucesso.'
          : `Todas as ${files.length} versões da Bíblia foram importadas com sucesso.`
      );
    } else {
      actionError = `${successCount} de ${files.length} versão(ões) importada(s) com sucesso. Falhas: ${failedFiles
        .map((f) => `${f.name} (${f.error})`)
        .join('; ')}`;
    }
  }

  async function handleImportBibleByPath() {
    const raw = importPathInput.trim();
    if (!raw) {
      actionError = 'Informe o caminho absoluto do arquivo SQLite da Bíblia.';
      return;
    }
    const paths = raw
      .split(/[\n,;]+/)
      .map((p) => p.trim())
      .filter((p) => p.length > 0);

    if (paths.length === 0) return;

    isImportingBible = true;
    actionError = '';
    importProgress = {
      current: 0,
      total: paths.length,
      currentFileName: '',
      successCount: 0,
      failedCount: 0,
    };

    const failedPaths: { path: string; error: string }[] = [];
    let successCount = 0;

    for (let i = 0; i < paths.length; i++) {
      const p = paths[i];
      const filename = p.split(/[/\\]/).pop() || p;
      importProgress = {
        current: i + 1,
        total: paths.length,
        currentFileName: filename,
        successCount,
        failedCount: failedPaths.length,
      };

      try {
        const res = await fetch('/api/bibles/import', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            source_path: p,
          }),
        });

        if (!res.ok) {
          throw new Error(await readHttpError(res, 'Falha ao importar Bíblia a partir do caminho'));
        }

        successCount++;
        notifySuccess('Bíblia importada', `${filename} (${i + 1}/${paths.length})`);
        await onReloadBibles();
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : 'Erro ao importar';
        failedPaths.push({ path: p, error: errorMsg });
        notifyError('Falha na importação', `${filename}: ${errorMsg}`);
      }
    }

    importPathInput = '';
    showPathImport = false;
    isImportingBible = false;

    if (failedPaths.length === 0) {
      notifySuccess(
        'Importação concluída',
        paths.length === 1
          ? 'Versão da Bíblia importada com sucesso.'
          : `Todas as ${paths.length} versões da Bíblia foram importadas com sucesso.`
      );
    } else {
      actionError = `${successCount} de ${paths.length} versão(ões) importada(s) com sucesso. Falhas: ${failedPaths
        .map((f) => `${f.path} (${f.error})`)
        .join('; ')}`;
    }
  }

  function startEditBible(bible: BibleVersionSummary) {
    editingBibleId = bible.id;
    editBibleName = bible.name;
    editBibleAbbr = bible.abbreviation;
    editBibleIsDefault = bible.is_default;
    editError = '';
    isEditingBible = true;
  }

  function closeEditBible() {
    isEditingBible = false;
    editingBibleId = null;
    editError = '';
  }

  async function handleSaveEditBible() {
    if (!editingBibleId) return;
    isSavingEdit = true;
    editError = '';
    try {
      const res = await fetch(`/api/bibles/${editingBibleId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: editBibleName.trim() || undefined,
          abbreviation: editBibleAbbr.trim() || undefined,
          is_default: editBibleIsDefault,
        }),
      });
      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao atualizar versão da Bíblia'));
      }
      isEditingBible = false;
      editingBibleId = null;
      await onReloadBibles();
      notifySuccess('Versão atualizada', 'Alterações salvas com sucesso.');
    } catch (err) {
      editError = err instanceof Error ? err.message : 'Erro ao salvar alterações da Bíblia';
    } finally {
      isSavingEdit = false;
    }
  }

  async function handleSetDefaultBible(id: string) {
    actionError = '';
    try {
      const res = await fetch(`/api/bibles/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ is_default: true }),
      });
      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao definir Bíblia padrão'));
      }
      await onReloadBibles();
      notifySuccess('Bíblia padrão atualizada');
    } catch (err) {
      actionError = err instanceof Error ? err.message : 'Erro ao definir Bíblia padrão.';
    }
  }

  function requestDeleteBible(bible: BibleVersionSummary) {
    onRequestConfirm({
      title: 'Excluir Versão da Bíblia',
      description: `Tem certeza que deseja excluir a versão "${bible.name}" (${bible.abbreviation})? O arquivo SQLite importado será removido da base do aplicativo.`,
      confirmText: 'Excluir Bíblia',
      variant: 'danger',
      onConfirm: async () => {
        actionError = '';
        const res = await fetch(`/api/bibles/${bible.id}`, { method: 'DELETE' });
        if (!res.ok) {
          const msg = await readHttpError(res, 'Falha ao excluir Bíblia');
          actionError = msg;
          throw new Error(msg);
        }
        await onReloadBibles();
        notifySuccess('Bíblia removida', bible.name);
      },
    });
  }
</script>

<div class="space-y-6 w-full max-w-5xl">
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 pb-3 border-b border-border">
    <div>
      <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
        <BookOpen size={16} class="text-primary shrink-0" />
        <span>Gerenciamento de Bíblias (SQLite)</span>
      </h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Gerencie e importe versões da Bíblia em formato SQLite (.sqlite). Os arquivos são copiados automaticamente para a pasta local de dados.
      </p>
    </div>

    <div class="flex items-center gap-2 shrink-0">
      <input
        type="file"
        multiple
        accept=".sqlite,.db,.sqlite3"
        bind:this={bibleFileInput}
        onchange={handleBibleFileSelect}
        class="hidden"
      />
      <Button
        variant="outline"
        size="sm"
        class="text-xs gap-1.5"
        aria-expanded={showPathImport}
        onclick={() => (showPathImport = !showPathImport)}
      >
        Por Caminho...
      </Button>
      <Button
        variant="default"
        size="sm"
        class="text-xs gap-1.5"
        disabled={isImportingBible}
        onclick={() => bibleFileInput?.click()}
      >
        {#if isImportingBible}
          <RefreshCw size={13} class="animate-spin" />
          <span>Importando ({importProgress.current}/{importProgress.total})...</span>
        {:else}
          <Upload size={13} />
          <span>Importar Bíblias (.sqlite)</span>
        {/if}
      </Button>
    </div>
  </div>

  {#if isImportingBible}
    <div
      role="status"
      aria-live="polite"
      class="p-3.5 rounded-xl border border-primary/30 bg-primary/10 flex flex-col gap-2.5"
    >
      <div class="flex items-center justify-between text-xs font-semibold text-primary">
        <span class="flex items-center gap-2">
          <RefreshCw size={14} class="animate-spin text-primary shrink-0" />
          <span>
            Importando versões da Bíblia ({importProgress.current} de {importProgress.total})
          </span>
        </span>
        <span class="font-mono text-xs font-bold">
          {importProgress.total > 0
            ? Math.round((importProgress.current / importProgress.total) * 100)
            : 0}%
        </span>
      </div>

      <div class="w-full bg-muted rounded-full h-2 overflow-hidden">
        <div
          class="bg-primary h-full transition-all duration-300 rounded-full"
          style="width: {importProgress.total > 0
            ? Math.max(8, Math.round((importProgress.current / importProgress.total) * 100))
            : 0}%"
        ></div>
      </div>

      <div class="flex items-center justify-between text-[11px] text-muted-foreground gap-2">
        <span class="truncate max-w-[80%]">
          Arquivo atual: <strong class="text-foreground font-mono">{importProgress.currentFileName || 'Preparando...'}</strong>
        </span>
        {#if importProgress.successCount > 0}
          <span class="text-emerald-700 dark:text-emerald-400 font-medium shrink-0">
            {importProgress.successCount} concluída(s)
          </span>
        {/if}
      </div>
    </div>
  {/if}

  {#if visibleError}
    <div
      role="alert"
      class="p-3 rounded-lg border border-red-500/40 bg-red-500/10 text-red-700 dark:text-red-300 text-xs flex items-center justify-between gap-2"
    >
      <div class="flex items-center gap-2">
        <AlertCircle size={15} class="shrink-0" />
        <span>{visibleError}</span>
      </div>
      <div class="flex items-center gap-2 shrink-0">
        <button
          type="button"
          class="text-xs font-medium underline hover:opacity-80 cursor-pointer min-h-7 px-1.5 rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          onclick={onReloadBibles}
        >
          Tentar novamente
        </button>
        <button
          type="button"
          class="text-xs font-medium underline hover:opacity-80 cursor-pointer min-h-7 px-1.5 rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          onclick={dismissVisibleError}
        >
          Fechar
        </button>
      </div>
    </div>
  {/if}

  {#if showPathImport}
    <div class="p-4 rounded-xl border border-border bg-card space-y-3">
      <div class="flex items-center justify-between gap-2">
        <label for="bible-path-import-input" class="text-xs font-semibold text-foreground">
          Importar por Caminho de Arquivo Local
        </label>
        <button
          type="button"
          class="text-xs text-muted-foreground hover:text-foreground cursor-pointer min-h-7 px-2 rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          onclick={() => (showPathImport = false)}
        >
          Cancelar
        </button>
      </div>
      <div class="flex gap-2">
        <input
          id="bible-path-import-input"
          type="text"
          bind:value={importPathInput}
          placeholder="/caminho/para/biblia1.sqlite, /caminho/para/biblia2.sqlite"
          class="flex-1 h-8 px-3 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        />
        <Button
          variant="default"
          size="sm"
          class="text-xs"
          disabled={isImportingBible}
          onclick={handleImportBibleByPath}
        >
          {isImportingBible ? 'Copiando...' : 'Importar'}
        </Button>
      </div>
      <p class="text-[11px] text-muted-foreground">
        Separe múltiplos caminhos por vírgula ou nova linha. Os arquivos serão validados e copiados sequencialmente para a pasta de dados do OpenPresenter (<code class="text-foreground font-mono">bibles/</code>).
      </p>
    </div>
  {/if}

  <div class="space-y-3">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-foreground uppercase tracking-wider">
        Versões Instaladas ({biblesList.length})
      </span>
      {#if isLoadingBibles}
        <span class="text-[11px] text-muted-foreground animate-pulse">Carregando...</span>
      {/if}
    </div>

    {#if biblesList.length === 0 && !isLoadingBibles}
      <div class="p-8 rounded-xl border border-dashed border-border bg-muted/20 text-center space-y-3">
        <div class="w-10 h-10 rounded-full bg-primary/10 text-primary flex items-center justify-center mx-auto">
          <BookOpen size={20} />
        </div>
        <div class="max-w-sm mx-auto">
          <h4 class="text-xs font-semibold text-foreground">Nenhuma versão bíblica instalada</h4>
          <p class="text-[11px] text-muted-foreground mt-1">
            Importe um arquivo de Bíblia SQLite (.sqlite) para habilitar a pesquisa de passagens, projeção de versículos e controle de slides.
          </p>
        </div>
        <Button
          variant="outline"
          size="sm"
          class="text-xs gap-1.5"
          onclick={() => bibleFileInput?.click()}
        >
          <Upload size={13} />
          Selecionar Arquivo(s) SQLite
        </Button>
      </div>
    {:else}
      <div class="grid grid-cols-1 gap-2.5">
        {#each biblesList as bible (bible.id)}
          <div class="p-3.5 rounded-xl border border-border bg-card hover:bg-muted/30 transition-colors flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-9 h-9 rounded-lg bg-primary/15 border border-primary/25 text-primary flex items-center justify-center font-bold text-xs shrink-0 uppercase tracking-wider">
                {bible.abbreviation || 'BIB'}
              </div>

              <div class="min-w-0">
                <div class="flex flex-wrap items-center gap-2">
                  <span class="text-xs font-bold text-foreground truncate">{bible.name}</span>
                  <Badge variant="outline" class="text-[11px] uppercase font-mono py-0 px-1.5">
                    {bible.abbreviation}
                  </Badge>
                  {#if bible.is_default}
                    <span class="text-[11px] px-2 py-0.5 rounded-full bg-emerald-500/15 text-emerald-700 dark:text-emerald-400 font-semibold border border-emerald-500/30 inline-flex items-center gap-1">
                      <Check size={11} />
                      Padrão
                    </span>
                  {/if}
                </div>
                <div class="text-[11px] text-muted-foreground mt-0.5 flex flex-wrap items-center gap-2 font-mono">
                  <span>{bible.book_count} livros</span>
                  <span>•</span>
                  <span>{bible.verse_count.toLocaleString('pt-BR')} versículos</span>
                  <span>•</span>
                  <span class="truncate max-w-[200px]" title={bible.filename}>{bible.filename}</span>
                </div>
              </div>
            </div>

            <div class="flex items-center gap-1.5 shrink-0 self-end sm:self-auto">
              {#if !bible.is_default}
                <Button
                  variant="ghost"
                  size="sm"
                  onclick={() => handleSetDefaultBible(bible.id)}
                  class="h-7 px-2.5 text-xs text-muted-foreground hover:text-emerald-700 dark:hover:text-emerald-400"
                  title="Definir como versão padrão para projeção rápida"
                >
                  Tornar Padrão
                </Button>
              {/if}

              <button
                type="button"
                onclick={() => startEditBible(bible)}
                class="w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                aria-label="Editar versão {bible.name}"
                title="Renomear versão e abreviação"
              >
                <Edit3 size={14} />
              </button>

              <button
                type="button"
                onclick={() => requestDeleteBible(bible)}
                class="w-7 h-7 rounded-md text-muted-foreground hover:text-red-700 dark:hover:text-red-400 hover:bg-red-500/10 flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                aria-label="Remover Bíblia {bible.name}"
                title="Remover Bíblia do app"
              >
                <Trash2 size={14} />
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<BibleEditModal
  open={isEditingBible}
  name={editBibleName}
  abbr={editBibleAbbr}
  isDefault={editBibleIsDefault}
  error={editError}
  isSaving={isSavingEdit}
  onUpdateName={(val) => (editBibleName = val)}
  onUpdateAbbr={(val) => (editBibleAbbr = val)}
  onUpdateIsDefault={(val) => (editBibleIsDefault = val)}
  onCancel={closeEditBible}
  onSave={handleSaveEditBible}
/>
