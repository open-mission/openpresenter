<script lang="ts">
  import { onMount } from 'svelte';
  import { cn } from '$lib/utils';
  import type { Setlist, WallpaperItem } from '$lib/types';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import { X, Image, Upload, Check, Sparkles, Folder } from 'lucide-svelte';

  interface Props {
    open: boolean;
    initialSetlist?: Setlist | null;
    onclose: () => void;
    onsave: (saved: Setlist) => void;
  }

  let { open = false, initialSetlist = null, onclose, onsave }: Props = $props();

  let title = $state('');
  let description = $state('');
  let selectedWallpaper = $state<string>('');
  let wallpaperMode = $state<'cover' | 'contain' | 'center' | 'repeat'>('cover');
  let availableWallpapers = $state<WallpaperItem[]>([]);
  let isLoading = $state(false);
  let isUploading = $state(false);
  let errorMessage = $state('');
  let fileInputRef = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (open) {
      if (initialSetlist) {
        title = initialSetlist.title;
        description = initialSetlist.description || '';
        selectedWallpaper = initialSetlist.wallpaper_path || '';
        wallpaperMode = (initialSetlist.wallpaper_mode as any) || 'cover';
      } else {
        title = '';
        description = '';
        selectedWallpaper = '/api/media/files/wallpapers/dark-ambient.svg';
        wallpaperMode = 'cover';
      }
      errorMessage = '';
      loadWallpapers();
    }
  });

  async function loadWallpapers() {
    try {
      const res = await fetch('/api/media/wallpapers');
      if (res.ok) {
        availableWallpapers = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar wallpapers:', err);
    }
  }

  async function handleFileUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;
    const file = target.files[0];

    isUploading = true;
    errorMessage = '';

    try {
      const reader = new FileReader();
      reader.onload = async () => {
        const base64 = reader.result as string;
        const res = await fetch('/api/media/upload', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content_base64: base64,
          }),
        });

        if (res.ok) {
          const data = await res.json();
          selectedWallpaper = data.url;
          await loadWallpapers();
        } else {
          errorMessage = 'Falha ao salvar papel de parede na pasta do sistema.';
        }
        isUploading = false;
      };
      reader.readAsDataURL(file);
    } catch (err) {
      errorMessage = 'Erro ao processar imagem.';
      isUploading = false;
    }
  }

  async function handleSave() {
    if (!title.trim()) {
      errorMessage = 'Por favor, insira um título para a Setlist.';
      return;
    }

    isLoading = true;
    errorMessage = '';

    const payload: Setlist = {
      id: initialSetlist?.id || `setlist-${Date.now()}`,
      title: title.trim(),
      description: description.trim() || undefined,
      wallpaper_path: selectedWallpaper || undefined,
      wallpaper_mode: wallpaperMode,
      items: initialSetlist?.items || [],
      is_active: initialSetlist?.is_active ?? true,
      created_at: initialSetlist?.created_at || '',
      updated_at: '',
    };

    try {
      const res = await fetch('/api/setlists', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      if (res.ok) {
        const saved = await res.json();
        onsave(saved);
        onclose();
      } else {
        errorMessage = 'Erro ao salvar Setlist.';
      }
    } catch (err) {
      errorMessage = 'Falha na conexão com o servidor.';
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.stopPropagation();
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4 animate-in fade-in duration-150"
    onclick={onclose}
    role="presentation"
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="w-full max-w-2xl rounded-xl border border-neutral-800 bg-neutral-900 shadow-2xl flex flex-col overflow-hidden text-neutral-100"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <div class="h-14 px-6 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-900/60">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 flex items-center justify-center">
            <Sparkles size={16} />
          </div>
          <div>
            <h2 class="text-sm font-bold text-white">
              {initialSetlist ? 'Editar Setlist' : 'Nova Setlist de Programação'}
            </h2>
            <p class="text-[11px] text-neutral-400">
              Configure os detalhes e o papel de parede para a tela do projetor
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="w-8 h-8 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={16} />
        </button>
      </div>

      <!-- Formulário -->
      <div class="p-6 space-y-5 overflow-y-auto max-h-[75vh]">
        {#if errorMessage}
          <div class="p-3 rounded-lg bg-red-950/40 border border-red-500/40 text-xs text-red-200">
            {errorMessage}
          </div>
        {/if}

        <!-- Título -->
        <div class="space-y-1.5">
          <label for="setlist-title" class="text-xs font-semibold text-neutral-300">
            Título da Setlist <span class="text-emerald-400">*</span>
          </label>
          <input
            id="setlist-title"
            type="text"
            bind:value={title}
            placeholder="Ex.: Culto de Domingo - Noite, Culto da Família, Celebração..."
            class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500/60 focus:ring-1 focus:ring-emerald-500/40"
          />
        </div>

        <!-- Descrição -->
        <div class="space-y-1.5">
          <label for="setlist-desc" class="text-xs font-semibold text-neutral-300">
            Descrição / Observações (opcional)
          </label>
          <textarea
            id="setlist-desc"
            bind:value={description}
            rows="2"
            placeholder="Ex.: Ministrante: Pr. Paulo. Tema: O Amor de Deus. Equipe de Louvor 1."
            class="w-full p-3 bg-neutral-950 border border-neutral-800 rounded-lg text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500/60 focus:ring-1 focus:ring-emerald-500/40 resize-none"
          ></textarea>
        </div>

        <!-- Papel de Parede do Projetor -->
        <div class="space-y-3 pt-2 border-t border-neutral-800">
          <div class="flex items-center justify-between">
            <div>
              <span class="text-xs font-semibold text-neutral-200 block">
                Papel de Parede da Projeção (Public)
              </span>
              <p class="text-[11px] text-neutral-400">
                Exibido na tela cheia quando nenhum slide estiver ao vivo ou em modo repouso
              </p>
            </div>

            <!-- Botão de Upload de Wallpaper -->
            <div>
              <input
                bind:this={fileInputRef}
                type="file"
                accept="image/*"
                onchange={handleFileUpload}
                class="hidden"
              />
              <Button
                variant="outline"
                size="sm"
                onclick={() => fileInputRef?.click()}
                disabled={isUploading}
                class="gap-1.5 text-xs"
              >
                <Upload size={13} />
                {isUploading ? 'Enviando...' : 'Carregar Imagem'}
              </Button>
            </div>
          </div>

          <!-- Grade de Wallpapers Disponíveis -->
          <div class="grid grid-cols-3 sm:grid-cols-4 gap-2.5">
            <!-- Opção Sem Papel de Parede (Preto Absoluto) -->
            <button
              type="button"
              onclick={() => (selectedWallpaper = '')}
              class={cn(
                'aspect-video rounded-lg border flex flex-col items-center justify-center text-center p-2 transition-all cursor-pointer relative bg-black',
                selectedWallpaper === ''
                  ? 'border-emerald-500 ring-2 ring-emerald-500/40'
                  : 'border-neutral-800 hover:border-neutral-700'
              )}
            >
              <span class="text-[11px] font-medium text-neutral-400">Preto Puro</span>
              {#if selectedWallpaper === ''}
                <div class="absolute top-1.5 right-1.5 w-4 h-4 rounded-full bg-emerald-500 text-black flex items-center justify-center">
                  <Check size={10} strokeWidth={3} />
                </div>
              {/if}
            </button>

            <!-- Wallpapers Cadastrados na pasta do sistema -->
            {#each availableWallpapers as wall}
              <button
                type="button"
                onclick={() => (selectedWallpaper = wall.url)}
                class={cn(
                  'aspect-video rounded-lg border overflow-hidden relative transition-all cursor-pointer group bg-neutral-950',
                  selectedWallpaper === wall.url
                    ? 'border-emerald-500 ring-2 ring-emerald-500/40'
                    : 'border-neutral-800 hover:border-neutral-700'
                )}
              >
                <img
                  src={wall.url}
                  alt={wall.name}
                  class="w-full h-full object-cover"
                />
                <span class="absolute bottom-0 inset-x-0 bg-black/70 backdrop-blur-xs text-[9px] text-neutral-300 px-1.5 py-0.5 truncate text-left">
                  {wall.name}
                </span>

                {#if selectedWallpaper === wall.url}
                  <div class="absolute top-1.5 right-1.5 w-4 h-4 rounded-full bg-emerald-500 text-black flex items-center justify-center shadow-md">
                    <Check size={10} strokeWidth={3} />
                  </div>
                {/if}
              </button>
            {/each}
          </div>

          <!-- Modo de Ajuste do Papel de Parede -->
          {#if selectedWallpaper}
            <div class="pt-2 flex items-center justify-between">
              <span class="text-xs text-neutral-300">Modo de Ajuste do Papel de Parede:</span>
              <div class="flex items-center gap-1 bg-neutral-950 p-0.5 rounded-lg border border-neutral-800">
                {#each [
                  { id: 'cover', label: 'Preencher' },
                  { id: 'contain', label: 'Ajustar' },
                  { id: 'center', label: 'Centralizar' },
                  { id: 'repeat', label: 'Repetir' },
                ] as mode}
                  <button
                    type="button"
                    onclick={() => (wallpaperMode = mode.id as any)}
                    class={cn(
                      'px-2.5 py-1 rounded text-[11px] font-medium transition-colors cursor-pointer',
                      wallpaperMode === mode.id
                        ? 'bg-neutral-800 text-emerald-400 font-semibold'
                        : 'text-neutral-400 hover:text-white'
                    )}
                  >
                    {mode.label}
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>

      <!-- Rodapé -->
      <div class="h-14 px-6 border-t border-neutral-800 bg-neutral-900/60 flex items-center justify-end gap-2.5 shrink-0">
        <Button variant="secondary" size="sm" onclick={onclose}>
          Cancelar
        </Button>
        <Button
          variant="default"
          size="sm"
          onclick={handleSave}
          disabled={isLoading || !title.trim()}
          class="bg-emerald-500 hover:bg-emerald-400 text-black font-semibold"
        >
          {isLoading ? 'Salvando...' : initialSetlist ? 'Atualizar Setlist' : 'Criar Setlist'}
        </Button>
      </div>
    </div>
  </div>
{/if}
