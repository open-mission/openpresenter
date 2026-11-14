<script lang="ts">
  import {
    HardDrive,
    Check,
    Copy,
    FolderOpen,
    Download,
    Upload,
    RefreshCw,
    AlertCircle,
    CheckCircle2,
    AlertTriangle,
    Folder,
    FolderPlus,
    Trash2,
  } from 'lucide-svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { readHttpError } from '$lib/media';
  import { notifySuccess, notifyError } from '$lib/toast';
  import type { MediaFolder, SystemPaths } from '$lib/types';
  import type { BackupInspection, SettingsConfirmRequest } from '../types';

  interface Props {
    systemPaths: SystemPaths | null;
    isLoadingPaths: boolean;
    pathsError: string;
    mediaFolders: MediaFolder[];
    isLoadingFolders: boolean;
    foldersError: string;
    onRetryPaths: () => void;
    onReloadFolders: () => Promise<void>;
    onRequestConfirm: (req: SettingsConfirmRequest) => void;
  }

  let {
    systemPaths,
    isLoadingPaths,
    pathsError,
    mediaFolders,
    isLoadingFolders,
    foldersError,
    onRetryPaths,
    onReloadFolders,
    onRequestConfirm,
  }: Props = $props();

  let newFolderName = $state('');
  let newFolderPath = $state('');
  let folderError = $state('');
  let isAddingFolder = $state(false);
  let isCopiedDataDir = $state(false);
  let folderPickerInput = $state<HTMLInputElement | null>(null);

  let isExportingBackup = $state(false);
  let isInspectingBackup = $state(false);
  let isRestoringBackup = $state(false);
  let backupFileInput = $state<HTMLInputElement | null>(null);
  let inspectedBackup = $state<BackupInspection | null>(null);
  let backupFileToRestore = $state<File | null>(null);
  let backupError = $state<string | null>(null);
  let backupSuccessMessage = $state<string | null>(null);

  async function handleCopyText(text: string) {
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
      isCopiedDataDir = true;
      setTimeout(() => (isCopiedDataDir = false), 2000);
    } catch {
      notifyError('Não foi possível copiar o caminho');
    }
  }

  async function handleOpenInFinder(path: string) {
    if (!path) return;
    try {
      const res = await fetch('/api/system/open-path', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ path }),
      });
      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao abrir pasta no sistema'));
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Erro ao abrir pasta';
      notifyError('Falha ao abrir diretório', msg);
    }
  }

  async function handleNativePickFolder() {
    folderError = '';
    try {
      const res = await fetch('/api/system/pick-folder', { method: 'POST' });
      if (res.ok) {
        const data = await res.json();
        if (data.path) {
          newFolderPath = data.path;
          if (!newFolderName && data.name) {
            newFolderName = data.name;
          }
          return;
        }
      }
    } catch {
      // Fallback para seletor HTML
    }
    folderPickerInput?.click();
  }

  function handleHtmlFolderSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      const relative = file.webkitRelativePath || '';
      const folderName = relative.split('/')[0] || file.name;
      if (!newFolderName) newFolderName = folderName;
      if (!newFolderPath) newFolderPath = folderName;
    }
  }

  async function handleAddFolder() {
    const p = newFolderPath.trim();
    if (!p) {
      folderError = 'Informe o caminho absoluto da pasta no seu computador.';
      return;
    }

    isAddingFolder = true;
    folderError = '';

    try {
      const res = await fetch('/api/media/folders', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: newFolderName.trim() || undefined,
          path: p,
          kind: 'images',
        }),
      });

      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao mapear pasta'));
      }

      newFolderName = '';
      newFolderPath = '';
      await onReloadFolders();
      notifySuccess('Pasta mapeada', 'Diretório indexado para fotos, vídeos e áudios.');
    } catch (err) {
      folderError = err instanceof Error ? err.message : 'Erro ao validar e adicionar pasta';
    } finally {
      isAddingFolder = false;
    }
  }

  function requestDeleteFolder(folder: MediaFolder) {
    onRequestConfirm({
      title: 'Desvincular Pasta Externa',
      description: `Remover o vínculo de "${folder.name}" (${folder.path})? Os arquivos originais no computador não serão apagados.`,
      confirmText: 'Desvincular Pasta',
      variant: 'danger',
      onConfirm: async () => {
        const res = await fetch(`/api/media/folders/${folder.id}`, { method: 'DELETE' });
        if (!res.ok) {
          throw new Error(await readHttpError(res, 'Falha ao desvincular pasta'));
        }
        await onReloadFolders();
        notifySuccess('Pasta desvinculada', folder.name);
      },
    });
  }

  async function handleExportBackup() {
    isExportingBackup = true;
    backupError = null;
    backupSuccessMessage = null;
    try {
      const res = await fetch('/api/backup/export');
      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao gerar pacote de backup'));
      }
      const blob = await res.blob();
      const contentDisposition = res.headers.get('content-disposition');
      let filename = 'open-presenter-backup.opbackup';
      if (contentDisposition) {
        const match = contentDisposition.match(/filename="?([^"]+)"?/);
        if (match && match[1]) filename = match[1];
      }
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      window.URL.revokeObjectURL(url);
      notifySuccess('Backup exportado com sucesso!', `Arquivo salvo como ${filename}`);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Erro ao exportar backup';
      backupError = msg;
      notifyError('Erro na exportação', msg);
    } finally {
      isExportingBackup = false;
    }
  }

  function handleSelectBackupFile() {
    backupError = null;
    backupSuccessMessage = null;
    backupFileInput?.click();
  }

  async function handleBackupFileSelected(event: Event) {
    const target = event.target as HTMLInputElement;
    const files = target.files;
    if (!files || files.length === 0) return;

    const file = files[0];
    backupFileToRestore = file;
    isInspectingBackup = true;
    backupError = null;
    backupSuccessMessage = null;

    try {
      const res = await fetch('/api/backup/inspect', {
        method: 'POST',
        headers: { 'Content-Type': 'application/zip' },
        body: file,
      });

      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao inspecionar arquivo de backup'));
      }

      const inspectResult = (await res.json()) as BackupInspection;
      if (!inspectResult.valid) {
        throw new Error(inspectResult.error || 'Arquivo de backup inválido ou corrompido.');
      }

      inspectedBackup = inspectResult;
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Erro ao analisar arquivo de backup';
      backupError = msg;
      inspectedBackup = null;
      backupFileToRestore = null;
      notifyError('Arquivo inválido', msg);
    } finally {
      isInspectingBackup = false;
      target.value = '';
    }
  }

  async function executeBackupRestore() {
    if (!backupFileToRestore) return;

    isRestoringBackup = true;
    backupError = null;
    backupSuccessMessage = null;

    try {
      const res = await fetch('/api/backup/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/zip' },
        body: backupFileToRestore,
      });

      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao restaurar dados'));
      }

      const result = await res.json();
      inspectedBackup = null;
      backupFileToRestore = null;
      backupSuccessMessage = result.message || 'Dados restaurados com sucesso!';
      notifySuccess(
        'Restauração concluída!',
        'Os dados foram aplicados com sucesso. A página será atualizada.'
      );

      onRetryPaths();
      await onReloadFolders();

      setTimeout(() => {
        window.location.reload();
      }, 1500);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Erro ao restaurar backup';
      backupError = msg;
      notifyError('Erro na restauração', msg);
      throw new Error(msg);
    } finally {
      isRestoringBackup = false;
    }
  }

  function requestConfirmRestore() {
    if (!inspectedBackup || !backupFileToRestore) return;
    onRequestConfirm({
      title: 'Confirmar Restauração Completa',
      description: `A restauração de "${inspectedBackup.filename}" substituirá o banco SQLite atual e sincronizará todas as mídias do pacote. A aplicação será recarregada ao finalizar.`,
      confirmText: 'Sim, Substituir e Restaurar',
      variant: 'danger',
      onConfirm: executeBackupRestore,
    });
  }

  function handleCancelRestore() {
    inspectedBackup = null;
    backupFileToRestore = null;
    backupError = null;
  }
</script>

<div class="space-y-6 w-full max-w-5xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Arquivos e Pastas do Sistema</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Estrutura de dados local do OpenPresenter e integração com diretórios externos de fotos, vídeos e áudios.
    </p>
  </div>

  <!-- Input invisível de fallback para seletor de pasta no navegador -->
  <input
    type="file"
    bind:this={folderPickerInput}
    onchange={handleHtmlFolderSelected}
    webkitdirectory
    class="hidden"
  />

  {#if pathsError}
    <div
      role="alert"
      class="p-3 rounded-lg bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center justify-between gap-2"
    >
      <div class="flex items-center gap-2">
        <AlertCircle size={15} class="shrink-0" />
        <span>{pathsError}</span>
      </div>
      <Button variant="outline" size="sm" onclick={onRetryPaths} class="text-xs h-7 shrink-0">
        Tentar novamente
      </Button>
    </div>
  {:else if isLoadingPaths && !systemPaths}
    <div class="p-4 rounded-xl border border-border bg-card text-xs text-muted-foreground flex items-center gap-2">
      <RefreshCw size={14} class="animate-spin text-primary shrink-0" />
      <span>Carregando caminhos do sistema...</span>
    </div>
  {/if}

  {#if systemPaths}
    <div class="p-4 rounded-xl border border-primary/30 bg-primary/5 space-y-3">
      <div class="flex items-center justify-between gap-2">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-primary/15 text-primary flex items-center justify-center shrink-0">
            <HardDrive size={18} />
          </div>
          <div>
            <h4 class="text-xs font-bold text-foreground">Pasta de Dados do Sistema (Data Dir)</h4>
            <p class="text-[11px] text-muted-foreground">
              Onde ficam salvos o banco de dados SQLite, arquivos e mídias do usuário
            </p>
          </div>
        </div>
        <Badge variant="outline" class="text-[11px] text-primary border-primary/40 font-mono">
          data_dir
        </Badge>
      </div>

      <div class="p-2.5 rounded-lg bg-background border border-border flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <code class="text-xs font-mono text-foreground break-all select-all flex-1">
          {systemPaths.data_dir}
        </code>

        <div class="flex items-center gap-1.5 shrink-0 self-end sm:self-auto">
          <Button
            variant="outline"
            size="sm"
            onclick={() => handleCopyText(systemPaths?.data_dir || '')}
            class="h-7 text-xs gap-1"
            title="Copiar caminho da pasta de dados"
          >
            {#if isCopiedDataDir}
              <Check size={12} class="text-emerald-700 dark:text-emerald-400" />
              <span class="text-emerald-700 dark:text-emerald-400">Copiado</span>
            {:else}
              <Copy size={12} />
              <span>Copiar</span>
            {/if}
          </Button>

          <Button
            variant="outline"
            size="sm"
            onclick={() => handleOpenInFinder(systemPaths?.data_dir || '')}
            class="h-7 text-xs gap-1"
            title="Abrir pasta no Finder / Explorador de Arquivos"
          >
            <FolderOpen size={12} />
            <span>Abrir</span>
          </Button>
        </div>
      </div>
    </div>

    <div class="space-y-2.5">
      <span class="text-xs font-semibold text-foreground uppercase tracking-wider block">
        Subdiretórios da Aplicação
      </span>
      <div class="grid grid-cols-1 gap-2 text-xs">
        <div class="p-2.5 rounded-lg border border-border bg-card flex items-center justify-between gap-3">
          <div class="min-w-0 flex-1">
            <div class="font-semibold text-foreground">Banco de Dados SQLite</div>
            <div class="text-[11px] text-muted-foreground font-mono mt-0.5 truncate" title={systemPaths.db_path}>
              {systemPaths.db_path}
            </div>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <Badge variant="outline" class="text-[11px] font-mono">openpresenter.db</Badge>
            <button
              type="button"
              onclick={() => handleOpenInFinder(systemPaths?.data_dir || '')}
              class="w-7 h-7 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              aria-label="Abrir pasta do banco de dados"
              title="Abrir no Finder"
            >
              <FolderOpen size={14} />
            </button>
          </div>
        </div>

        <div class="p-2.5 rounded-lg border border-border bg-card flex items-center justify-between gap-3">
          <div class="min-w-0 flex-1">
            <div class="font-semibold text-foreground">Imagens do Aplicativo</div>
            <div class="text-[11px] text-muted-foreground font-mono mt-0.5 truncate" title={systemPaths.images_path}>
              {systemPaths.images_path}
            </div>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <Badge variant="outline" class="text-[11px] font-mono">media/images</Badge>
            <button
              type="button"
              onclick={() => handleOpenInFinder(systemPaths?.images_path || '')}
              class="w-7 h-7 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              aria-label="Abrir pasta de imagens do aplicativo"
              title="Abrir no Finder"
            >
              <FolderOpen size={14} />
            </button>
          </div>
        </div>

        <div class="p-2.5 rounded-lg border border-border bg-card flex items-center justify-between gap-3">
          <div class="min-w-0 flex-1">
            <div class="font-semibold text-foreground">Papéis de Parede</div>
            <div class="text-[11px] text-muted-foreground font-mono mt-0.5 truncate" title={systemPaths.wallpapers_path}>
              {systemPaths.wallpapers_path}
            </div>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <Badge variant="outline" class="text-[11px] font-mono">media/wallpapers</Badge>
            <button
              type="button"
              onclick={() => handleOpenInFinder(systemPaths?.wallpapers_path || '')}
              class="w-7 h-7 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              aria-label="Abrir pasta de papéis de parede"
              title="Abrir no Finder"
            >
              <FolderOpen size={14} />
            </button>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Backup & Migração de Instalação -->
  <div class="pt-4 border-t border-border space-y-3">
    <div>
      <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
        <HardDrive size={16} class="text-primary shrink-0" />
        <span>Backup & Migração de Instalação</span>
      </h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Mova sua instalação para outro computador ou guarde uma cópia de segurança completa contendo músicas, setlists, equipe, mídias importadas e configurações.
      </p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3 flex flex-col justify-between">
        <div class="space-y-1.5">
          <div class="flex items-center gap-2">
            <div class="w-7 h-7 rounded-lg bg-primary/15 text-primary flex items-center justify-center shrink-0">
              <Download size={14} />
            </div>
            <h4 class="text-xs font-bold text-foreground">Exportar Dados (.opbackup)</h4>
          </div>
          <p class="text-[11px] text-muted-foreground leading-relaxed">
            Gera um arquivo portátil contendo todo o banco SQLite e arquivos de imagens, papéis de parede, vídeos, áudios, bíblias e fontes.
          </p>
        </div>

        <div class="pt-1">
          <Button
            variant="default"
            size="sm"
            onclick={handleExportBackup}
            disabled={isExportingBackup}
            class="w-full gap-2 text-xs font-semibold"
          >
            {#if isExportingBackup}
              <RefreshCw size={13} class="animate-spin" />
              <span>Gerando pacote...</span>
            {:else}
              <Download size={13} />
              <span>Exportar Todos os Dados</span>
            {/if}
          </Button>
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3 flex flex-col justify-between">
        <div class="space-y-1.5">
          <div class="flex items-center gap-2">
            <div class="w-7 h-7 rounded-lg bg-emerald-500/15 text-emerald-700 dark:text-emerald-400 flex items-center justify-center shrink-0">
              <Upload size={14} />
            </div>
            <h4 class="text-xs font-bold text-foreground">Importar / Restaurar Instalação</h4>
          </div>
          <p class="text-[11px] text-muted-foreground leading-relaxed">
            Restaure os dados de outro computador a partir de um arquivo <code class="font-mono text-[11px] bg-background px-1 py-0.5 rounded border border-border">.opbackup</code> ou <code class="font-mono text-[11px] bg-background px-1 py-0.5 rounded border border-border">.zip</code>.
          </p>
        </div>

        <div class="pt-1">
          <input
            type="file"
            accept=".opbackup,.zip"
            bind:this={backupFileInput}
            onchange={handleBackupFileSelected}
            class="hidden"
          />
          <Button
            variant="outline"
            size="sm"
            onclick={handleSelectBackupFile}
            disabled={isInspectingBackup || isRestoringBackup}
            class="w-full gap-2 text-xs font-semibold hover:border-emerald-500/50"
          >
            {#if isInspectingBackup}
              <RefreshCw size={13} class="animate-spin text-emerald-700 dark:text-emerald-400" />
              <span>Analisando arquivo...</span>
            {:else}
              <Upload size={13} />
              <span>Selecionar Arquivo de Backup...</span>
            {/if}
          </Button>
        </div>
      </div>
    </div>

    {#if backupError}
      <div
        role="alert"
        class="p-3 rounded-lg bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center gap-2"
      >
        <AlertCircle size={14} class="shrink-0" />
        <span>{backupError}</span>
      </div>
    {/if}

    {#if backupSuccessMessage}
      <div
        role="status"
        class="p-3 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-emerald-700 dark:text-emerald-400 text-xs flex items-center gap-2"
      >
        <CheckCircle2 size={14} class="shrink-0" />
        <span>{backupSuccessMessage}</span>
      </div>
    {/if}

    {#if inspectedBackup}
      <div class="p-4 rounded-xl border border-amber-500/40 bg-amber-500/5 space-y-3.5">
        <div class="flex items-start justify-between gap-3">
          <div class="flex items-center gap-2.5">
            <div class="w-8 h-8 rounded-lg bg-amber-500/20 text-amber-700 dark:text-amber-400 flex items-center justify-center shrink-0">
              <AlertTriangle size={18} />
            </div>
            <div>
              <h4 class="text-xs font-bold text-foreground">Confirmar Restauração de Dados</h4>
              <p class="text-[11px] text-muted-foreground">
                Revise o conteúdo do pacote antes de aplicar as alterações.
              </p>
            </div>
          </div>
          <Badge variant="outline" class="text-[11px] text-amber-700 dark:text-amber-300 border-amber-500/40 font-mono">
            {inspectedBackup.filename}
          </Badge>
        </div>

        {#if inspectedBackup.manifest}
          {@const m = inspectedBackup.manifest}
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
            <div class="p-2.5 rounded-lg bg-background border border-border">
              <span class="text-[11px] text-muted-foreground block">Músicas</span>
              <span class="font-bold text-foreground text-sm">{m.stats.songs_count}</span>
            </div>
            <div class="p-2.5 rounded-lg bg-background border border-border">
              <span class="text-[11px] text-muted-foreground block">Setlists</span>
              <span class="font-bold text-foreground text-sm">{m.stats.setlists_count}</span>
            </div>
            <div class="p-2.5 rounded-lg bg-background border border-border">
              <span class="text-[11px] text-muted-foreground block">Voluntários</span>
              <span class="font-bold text-foreground text-sm">{m.stats.team_members_count}</span>
            </div>
            <div class="p-2.5 rounded-lg bg-background border border-border">
              <span class="text-[11px] text-muted-foreground block">Mídias (Arquivos)</span>
              <span class="font-bold text-foreground text-sm">
                {m.stats.images_count + m.stats.videos_count + m.stats.audios_count + m.stats.wallpapers_count}
              </span>
            </div>
          </div>

          <div class="text-[11px] text-muted-foreground flex flex-wrap items-center justify-between gap-2 px-1">
            <span>
              Criado em: {new Date(m.created_at).toLocaleString('pt-BR')} (OpenPresenter v{m.app_version})
            </span>
            <span>Tamanho: {(inspectedBackup.file_size_bytes / (1024 * 1024)).toFixed(1)} MB</span>
          </div>
        {/if}

        <div class="p-2.5 rounded-lg bg-amber-500/10 border border-amber-500/25 text-[11px] text-amber-800 dark:text-amber-300 leading-relaxed">
          <strong>Aviso:</strong> A restauração substituirá os dados atuais do banco SQLite e sincronizará os arquivos de mídia. A aplicação será recarregada automaticamente após a conclusão.
        </div>

        <div class="flex items-center justify-end gap-2 pt-1">
          <Button
            variant="outline"
            size="sm"
            onclick={handleCancelRestore}
            disabled={isRestoringBackup}
            class="text-xs"
          >
            Cancelar
          </Button>
          <Button
            variant="destructive"
            size="sm"
            onclick={requestConfirmRestore}
            disabled={isRestoringBackup}
            class="gap-1.5 text-xs font-semibold"
          >
            {#if isRestoringBackup}
              <RefreshCw size={13} class="animate-spin" />
              <span>Restaurando dados...</span>
            {:else}
              <CheckCircle2 size={13} />
              <span>Substituir e Restaurar Dados</span>
            {/if}
          </Button>
        </div>
      </div>
    {/if}
  </div>

  <!-- Pastas Externas Mapeadas -->
  <div class="pt-4 border-t border-border space-y-3">
    <div>
      <h3 class="text-sm font-semibold text-foreground">Pastas de Mídia Externas Mapeadas</h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Mapeie diretórios do seu computador (fotos, vídeos ou áudios). O OpenPresenter indexa os arquivos diretamente sem copiar nem duplicar espaço em disco.
      </p>
    </div>

    <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
        <div>
          <label for="folder-name-input" class="block text-[11px] text-muted-foreground mb-1">
            Rótulo da Pasta
          </label>
          <input
            id="folder-name-input"
            type="text"
            bind:value={newFolderName}
            placeholder="Ex: Mídias do Culto (Fotos, Vídeos, Áudios)"
            class="w-full h-8 px-2.5 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>
        <div>
          <label for="folder-path-input" class="block text-[11px] text-muted-foreground mb-1">
            Caminho no Computador *
          </label>
          <div class="flex items-center gap-1.5">
            <input
              id="folder-path-input"
              type="text"
              bind:value={newFolderPath}
              placeholder="Ex: /Users/usuario/Imagens/Culto"
              class="flex-1 h-8 px-2.5 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring font-mono"
            />
            <Button
              variant="outline"
              size="sm"
              onclick={handleNativePickFolder}
              class="h-8 text-xs gap-1.5 shrink-0"
              title="Selecionar pasta graficamente no computador"
            >
              <Folder size={13} class="text-primary" />
              <span>Selecionar...</span>
            </Button>
          </div>
        </div>
      </div>

      {#if folderError}
        <div
          role="alert"
          class="p-2 rounded bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center gap-1.5"
        >
          <AlertCircle size={13} class="shrink-0" />
          <span>{folderError}</span>
        </div>
      {/if}

      <div class="flex justify-end">
        <Button
          variant="default"
          size="sm"
          onclick={handleAddFolder}
          disabled={isAddingFolder}
          class="gap-1.5 text-xs font-semibold"
        >
          <FolderPlus size={13} />
          <span>{isAddingFolder ? 'Validando...' : 'Mapear Pasta'}</span>
        </Button>
      </div>
    </div>

    <div class="space-y-2">
      {#if foldersError}
        <div
          role="alert"
          class="p-3 rounded-lg bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center justify-between gap-2"
        >
          <span>{foldersError}</span>
          <Button variant="outline" size="sm" onclick={onReloadFolders} class="h-7 text-xs">
            Recarregar
          </Button>
        </div>
      {:else if isLoadingFolders && mediaFolders.length === 0}
        <div class="p-4 rounded-lg border border-border bg-card text-center text-xs text-muted-foreground">
          Carregando pastas mapeadas...
        </div>
      {:else if mediaFolders.length === 0}
        <div class="p-4 rounded-lg border border-dashed border-border text-center text-xs text-muted-foreground">
          Nenhuma pasta externa mapeada ainda. Clique em "Selecionar..." para importar sua primeira pasta.
        </div>
      {:else}
        {#each mediaFolders as folder (folder.id)}
          <div class="p-3 rounded-lg border border-border bg-card flex items-center justify-between gap-3">
            <div class="flex items-center gap-2.5 min-w-0 flex-1">
              <Folder size={16} class="text-primary shrink-0" />
              <div class="min-w-0 flex-1">
                <div class="text-xs font-semibold text-foreground truncate">{folder.name}</div>
                <div class="text-[11px] text-muted-foreground font-mono truncate" title={folder.path}>
                  {folder.path}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-1.5 shrink-0">
              <Button
                variant="outline"
                size="sm"
                onclick={() => handleOpenInFinder(folder.path)}
                class="h-7 text-[11px] gap-1 px-2"
                title="Abrir no Finder"
              >
                <FolderOpen size={12} />
                <span>Abrir</span>
              </Button>

              <button
                type="button"
                onclick={() => requestDeleteFolder(folder)}
                aria-label="Desvincular pasta {folder.name}"
                title="Remover pasta mapeada"
                class="w-7 h-7 rounded-md text-muted-foreground hover:text-red-700 dark:hover:text-red-400 hover:bg-red-500/10 flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              >
                <Trash2 size={14} />
              </button>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>
