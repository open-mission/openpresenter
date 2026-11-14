<script lang="ts">
  import { Folder, AlertCircle } from 'lucide-svelte';
  import Badge from '../ui/Badge.svelte';
  import type { ImageMediaItem, MediaFolder } from '$lib/types';

  interface Props {
    folders: MediaFolder[];
    images: ImageMediaItem[];
    appImagesCount: number;
    selectedFolderId: string;
    feedbackMessage: string;
    onselectfolder: (id: string) => void;
  }

  let {
    folders,
    images,
    appImagesCount,
    selectedFolderId,
    feedbackMessage,
    onselectfolder,
  }: Props = $props();

  function handleKeydown(e: KeyboardEvent, id: string) {
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      onselectfolder(id);
    }
  }
</script>

<div class="space-y-4 max-w-xl mx-auto py-2 select-none">
  <div>
    <h3 class="text-xs font-bold text-foreground uppercase tracking-wider mb-1">
      Escolha a Pasta a Inserir na Setlist
    </h3>
    <p class="text-xs text-muted-foreground">
      Todas as imagens da pasta selecionada serão organizadas em uma apresentação sequencial de slides.
    </p>
  </div>

  {#if feedbackMessage}
    <div
      role="alert"
      class="p-3 rounded-lg bg-destructive/10 border border-destructive/30 text-xs text-destructive flex items-center gap-2.5 animate-in fade-in duration-150"
    >
      <AlertCircle size={16} class="shrink-0" aria-hidden="true" />
      <span>{feedbackMessage}</span>
    </div>
  {/if}

  <div role="radiogroup" aria-label="Pastas disponíveis para importação" class="space-y-2">
    <!-- Pasta Interna do App -->
    <div
      role="radio"
      aria-checked={selectedFolderId === 'app'}
      tabindex="0"
      onclick={() => onselectfolder('app')}
      onkeydown={(e) => handleKeydown(e, 'app')}
      class="p-3.5 rounded-xl border transition-all cursor-pointer flex items-center justify-between focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedFolderId ===
      'app'
        ? 'border-primary bg-primary/5 ring-1 ring-primary/40'
        : 'border-border bg-card/40 hover:bg-muted/40'}"
    >
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-xl bg-primary/10 border border-primary/25 text-primary flex items-center justify-center shrink-0"
          aria-hidden="true"
        >
          <Folder size={20} />
        </div>
        <div>
          <h4 class="text-xs sm:text-sm font-bold text-foreground">Imagens do Aplicativo</h4>
          <p class="text-xs text-muted-foreground">Pasta interna de mídia do OpenPresenter</p>
        </div>
      </div>
      <Badge variant="outline" class="text-xs font-mono">{appImagesCount} imagens</Badge>
    </div>

    <!-- Pastas Mapeadas do Usuário -->
    {#each folders as f (f.id)}
      {@const count = images.filter((i) => i.folder_id === f.id).length}
      <div
        role="radio"
        aria-checked={selectedFolderId === f.id}
        tabindex="0"
        onclick={() => onselectfolder(f.id)}
        onkeydown={(e) => handleKeydown(e, f.id)}
        class="p-3.5 rounded-xl border transition-all cursor-pointer flex items-center justify-between focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedFolderId ===
        f.id
          ? 'border-primary bg-primary/5 ring-1 ring-primary/40'
          : 'border-border bg-card/40 hover:bg-muted/40'}"
      >
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="w-10 h-10 rounded-xl bg-primary/10 border border-primary/25 text-primary flex items-center justify-center shrink-0"
            aria-hidden="true"
          >
            <Folder size={20} />
          </div>
          <div class="min-w-0">
            <h4 class="text-xs sm:text-sm font-bold text-foreground truncate">{f.name}</h4>
            <p class="text-[11px] text-muted-foreground font-mono truncate">{f.path}</p>
          </div>
        </div>
        <Badge variant="outline" class="text-xs font-mono shrink-0">{count} imagens</Badge>
      </div>
    {/each}
  </div>
</div>
