<script lang="ts">
  import {
    Plus,
    Search,
    Play,
    ListPlus,
    Upload,
    RefreshCw,
    FileText,
    X,
    ChevronDown,
    Layers,
    Boxes,
    CalendarDays,
    Bookmark,
    Pencil,
    Trash2,
    Copy,
    Hourglass,
    AlertTriangle,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import Badge from '../ui/Badge.svelte';
  import { ResizablePaneGroup, ResizablePane, ResizableHandle } from '../ui/resizable';
  import type { MenuItem } from '../ContextMenu.svelte';
  import {
    customItemsErrorStore,
    customItemsStore,
    deleteCustomItem,
    importItemsFromSetlist,
    loadCustomItems,
    saveOrUpdateCustomItem,
  } from '../../customItems';
  import { formatDuration } from '../../media';
  import { notifyError, notifySuccess } from '../../toast';
  import type { MediaItem, Slide } from '../../types';

  interface ConfirmRequest {
    title: string;
    message: string;
    confirmLabel?: string;
    onconfirm: () => Promise<void> | void;
  }

  interface Props {
    active: boolean;
    activeSetlistItems?: MediaItem[];
    onadditemtosetlist?: (item: MediaItem) => void;
    onpresentitem?: (item: MediaItem, slideIndex?: number) => void;
    onopenedititem?: (item: MediaItem) => void;
    onopenscheduleeditor?: () => void;
    onopentexteditor?: () => void;
    onopenquickitemeditor?: (mode: 'header' | 'announcement') => void;
    onopentimereditor?: (timerItem?: MediaItem) => void;
    onopencontextmenu: (x: number, y: number, items: MenuItem[]) => void;
    onrequestconfirm: (req: ConfirmRequest) => void;
  }

  let {
    active,
    activeSetlistItems = [],
    onadditemtosetlist,
    onpresentitem,
    onopenedititem,
    onopenscheduleeditor,
    onopentexteditor,
    onopenquickitemeditor,
    onopentimereditor,
    onopencontextmenu,
    onrequestconfirm,
  }: Props = $props();

  let othersSearch = $state('');
  let othersKindFilter = $state<'all' | 'schedule' | 'text' | 'announcement' | 'header' | 'timer'>('all');
  let viewingCustomItem = $state<MediaItem | null>(null);
  let isImportingFromSetlist = $state(false);
  let isAddOtherMenuOpen = $state(false);
  let addMenuTriggerEl: HTMLSpanElement | null = $state(null);
  let addMenuEl: HTMLDivElement | null = $state(null);
  let hasLoadedOnce = false;

  $effect(() => {
    if (active && !hasLoadedOnce) {
      hasLoadedOnce = true;
      loadCustomItems();
    }
  });

  $effect(() => {
    if (isAddOtherMenuOpen) {
      setTimeout(() => {
        const firstBtn = addMenuEl?.querySelector<HTMLButtonElement>('button[role="menuitem"]:not([disabled])');
        firstBtn?.focus();
      }, 10);
    }
  });

  // Contagens pré-computadas por categoria
  const kindCounts = $derived.by(() => {
    let schedule = 0;
    let text = 0;
    let announcement = 0;
    let header = 0;
    let timer = 0;
    for (const item of $customItemsStore) {
      if (item.kind === 'schedule') schedule++;
      else if (item.kind === 'text') text++;
      else if (item.kind === 'announcement') announcement++;
      else if (item.kind === 'header') header++;
      else if (item.kind === 'timer') timer++;
    }
    return {
      all: $customItemsStore.length,
      schedule,
      text,
      announcement,
      header,
      timer,
    };
  });

  const filteredCustomItems = $derived.by(() => {
    return $customItemsStore.filter((item) => {
      const matchesKind = othersKindFilter === 'all' || item.kind === othersKindFilter;
      const term = othersSearch.trim().toLowerCase();
      if (!term) return matchesKind;

      const titleMatch = item.title.toLowerCase().includes(term);
      const artistMatch = item.artist_or_ref?.toLowerCase().includes(term);
      const slideMatch = item.slides.some(
        (s) =>
          s.label?.toLowerCase().includes(term) ||
          s.content?.toLowerCase().includes(term) ||
          (s.metadata?.schedule_day && s.metadata.schedule_day.toLowerCase().includes(term)) ||
          (s.metadata?.schedule_time && s.metadata.schedule_time.toLowerCase().includes(term)) ||
          (s.metadata?.schedule_details && s.metadata.schedule_details.toLowerCase().includes(term))
      );

      return matchesKind && (titleMatch || artistMatch || slideMatch);
    });
  });

  $effect(() => {
    if (active && viewingCustomItem && !filteredCustomItems.some((i) => i.id === viewingCustomItem?.id)) {
      viewingCustomItem = null;
    }
  });

  function getItemKindBadge(kind: MediaItem['kind']) {
    switch (kind) {
      case 'schedule':
        return {
          label: 'Agenda',
          color: 'border-amber-500/35 bg-amber-500/15 text-amber-800 dark:text-amber-300',
        };
      case 'text':
        return {
          label: 'Texto',
          color: 'border-blue-500/35 bg-blue-500/15 text-blue-800 dark:text-blue-300',
        };
      case 'announcement':
        return {
          label: 'Aviso',
          color: 'border-emerald-500/35 bg-emerald-500/15 text-emerald-800 dark:text-emerald-300',
        };
      case 'header':
        return {
          label: 'Seção',
          color: 'border-cyan-500/35 bg-cyan-500/15 text-cyan-800 dark:text-cyan-300',
        };
      case 'timer':
        return {
          label: 'Cronômetro',
          color: 'border-rose-500/35 bg-rose-500/15 text-rose-800 dark:text-rose-300',
        };
      case 'video':
        return {
          label: 'Vídeo',
          color: 'border-purple-500/35 bg-purple-500/15 text-purple-800 dark:text-purple-300',
        };
      case 'audio':
        return {
          label: 'Áudio',
          color: 'border-indigo-500/35 bg-indigo-500/15 text-indigo-800 dark:text-indigo-300',
        };
      default:
        return {
          label: 'Outro',
          color: 'border-border bg-muted text-foreground',
        };
    }
  }

  async function handleImportFromSetlistClick() {
    if (!activeSetlistItems || activeSetlistItems.length === 0) {
      notifyError('Setlist Vazia', 'Não há itens na setlist ativa para importar.');
      return;
    }
    isImportingFromSetlist = true;
    try {
      const count = await importItemsFromSetlist(activeSetlistItems);
      if (count > 0) {
        notifySuccess('Importação Concluída', `${count} novo(s) item(ns) importado(s) para o catálogo Outros.`);
      } else {
        notifySuccess('Tudo Atualizado', 'Todos os itens suportados da setlist já estão no catálogo.');
      }
    } catch (err: any) {
      notifyError('Erro na Importação', err?.message || 'Não foi possível importar os itens da setlist.');
    } finally {
      isImportingFromSetlist = false;
    }
  }

  function requestDeleteCustomItem(item: MediaItem) {
    onrequestconfirm({
      title: 'Excluir Item do Catálogo',
      message: `Deseja realmente excluir "${item.title}" do catálogo Outros?`,
      confirmLabel: 'Excluir Item',
      onconfirm: async () => {
        try {
          await deleteCustomItem(item.id);
          if (viewingCustomItem?.id === item.id) {
            viewingCustomItem = null;
          }
          notifySuccess('Item Excluído', `"${item.title}" foi removido do catálogo.`);
        } catch (err: any) {
          notifyError('Erro ao Excluir', err?.message || 'Não foi possível salvar a exclusão no servidor.');
        }
      },
    });
  }

  async function handleDuplicateCustomItem(item: MediaItem) {
    const newItem: MediaItem = {
      ...JSON.parse(JSON.stringify(item)),
      id: `${item.kind || 'item'}-${Date.now()}`,
      title: `${item.title} (Cópia)`,
      slides: item.slides.map((s, idx) => ({
        ...s,
        id: `${item.kind || 'slide'}-${Date.now()}-${idx + 1}`,
      })),
    };
    try {
      await saveOrUpdateCustomItem(newItem);
      viewingCustomItem = newItem;
      notifySuccess('Item Duplicado', `"${newItem.title}" criado com sucesso.`);
    } catch (err: any) {
      notifyError('Erro ao Duplicar', err?.message || 'Não foi possível salvar o item duplicado.');
    }
  }

  function handleOtherItemContextMenu(e: MouseEvent, item: MediaItem) {
    e.preventDefault();
    e.stopPropagation();
    viewingCustomItem = item;
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'other-present-now', label: 'Apresentar Agora (Ao Vivo)', icon: Play, onclick: () => onpresentitem?.(item, 0) },
      { id: 'other-add-setlist', label: 'Adicionar à Setlist', icon: ListPlus, onclick: () => onadditemtosetlist?.(item) },
      { id: 'div-other-1', divider: true },
      { id: 'other-edit', label: 'Editar Item', icon: Pencil, onclick: () => onopenedititem?.(item) },
      { id: 'other-dup', label: 'Duplicar Item', icon: Copy, onclick: () => handleDuplicateCustomItem(item) },
      { id: 'div-other-2', divider: true },
      {
        id: 'other-del',
        label: 'Excluir do Catálogo',
        icon: Trash2,
        destructive: true,
        onclick: () => requestDeleteCustomItem(item),
      },
    ]);
  }

  function handleOtherSlideContextMenu(e: MouseEvent, item: MediaItem, slide: Slide, idx: number) {
    e.preventDefault();
    e.stopPropagation();
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'slide-title', label: slide.label || `Slide ${idx + 1}`, disabled: true },
      { id: 'div-slide-0', divider: true },
      { id: 'slide-present-live', label: 'Apresentar Este Slide Ao Vivo', icon: Play, onclick: () => onpresentitem?.(item, idx) },
      { id: 'slide-add-setlist', label: 'Adicionar Apresentação à Setlist', icon: ListPlus, onclick: () => onadditemtosetlist?.(item) },
      { id: 'div-slide-1', divider: true },
      { id: 'slide-edit-item', label: 'Editar Apresentação', icon: Pencil, onclick: () => onopenedititem?.(item) },
    ]);
  }

  function handleOtherEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'empty-header', label: 'Aba Outros', disabled: true },
      { id: 'div-empty-0', divider: true },
      { id: 'create-schedule', label: 'Nova Programação (Agenda)', icon: CalendarDays, onclick: () => onopenscheduleeditor?.() },
      { id: 'create-text', label: 'Nova Apresentação de Texto', icon: FileText, onclick: () => onopentexteditor?.() },
      { id: 'create-announcement', label: 'Novo Aviso / Comunicado', icon: Bookmark, onclick: () => onopenquickitemeditor?.('announcement') },
      { id: 'div-empty-1', divider: true },
      { id: 'reload-catalog', label: 'Recarregar Catálogo', icon: RefreshCw, onclick: () => loadCustomItems(true) },
    ]);
  }

  function handleAddMenuKeyDown(e: KeyboardEvent) {
    if (!isAddOtherMenuOpen || !addMenuEl) return;
    const items = Array.from(
      addMenuEl.querySelectorAll<HTMLButtonElement>('button[role="menuitem"]:not([disabled])')
    );
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      isAddOtherMenuOpen = false;
      addMenuTriggerEl?.querySelector('button')?.focus();
      return;
    }
    if (items.length === 0) return;
    const idx = items.indexOf(document.activeElement as HTMLButtonElement);
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      items[(idx + 1) % items.length].focus();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      items[(idx - 1 + items.length) % items.length].focus();
    } else if (e.key === 'Home') {
      e.preventDefault();
      items[0].focus();
    } else if (e.key === 'End') {
      e.preventDefault();
      items[items.length - 1].focus();
    }
  }
</script>

<svelte:window onkeydown={handleAddMenuKeyDown} />

<div role="tabpanel" id="panel-others" aria-labelledby="tab-others" class="flex-1 flex flex-col min-w-0 overflow-hidden">
  <h2 class="sr-only">Catálogo de Agendas, Textos, Avisos e Cronômetros</h2>

  <!-- Barra Superior -->
  <div class="flex items-center justify-between gap-2 mb-2 shrink-0 flex-wrap">
    <div class="relative flex-1 min-w-[200px] max-w-md">
      <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
      <input
        type="text"
        bind:value={othersSearch}
        aria-label="Pesquisar em Outros (agenda, textos, avisos)"
        placeholder="Pesquisar em Outros (agenda, textos, avisos)..."
        class="w-full h-8 pl-8 pr-8 bg-background border border-input rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
      />
      {#if othersSearch}
        <button
          type="button"
          onclick={() => (othersSearch = '')}
          aria-label="Limpar pesquisa em Outros"
          class="absolute right-1 top-1/2 -translate-y-1/2 w-6 h-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
        >
          <X size={12} />
        </button>
      {/if}
    </div>

    <div class="flex items-center gap-1.5 flex-wrap">
      <Button
        variant="outline"
        size="sm"
        onclick={() => loadCustomItems(true)}
        aria-label="Recarregar catálogo de Outros"
        title="Recarregar catálogo"
        class="h-8 w-8 p-0"
      >
        <RefreshCw size={13} />
      </Button>

      <div class="relative">
        <span class="inline-flex" bind:this={addMenuTriggerEl}>
          <Button
            variant="default"
            size="sm"
            onclick={() => (isAddOtherMenuOpen = !isAddOtherMenuOpen)}
            title="Criar novo item ou importar para a aba Outros"
            class="h-8 text-xs gap-1.5 cursor-pointer shadow-sm"
            aria-haspopup="menu"
            aria-expanded={isAddOtherMenuOpen}
          >
            <Plus size={13} />
            <span>Adicionar</span>
            <ChevronDown
              size={11}
              class="opacity-70 transition-transform duration-200 {isAddOtherMenuOpen ? 'rotate-180' : ''}"
            />
          </Button>
        </span>

        {#if isAddOtherMenuOpen}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="fixed inset-0 z-40"
            onclick={() => (isAddOtherMenuOpen = false)}
            onkeydown={handleAddMenuKeyDown}
          ></div>

          <div
            bind:this={addMenuEl}
            class="absolute right-0 top-full mt-1.5 w-64 rounded-xl bg-popover text-popover-foreground border border-border shadow-2xl p-1 z-50 animate-in fade-in zoom-in-95 duration-150 select-none text-left"
            role="menu"
            tabindex="-1"
            onkeydown={handleAddMenuKeyDown}
          >
            <button
              type="button"
              role="menuitem"
              onclick={() => {
                isAddOtherMenuOpen = false;
                onopenscheduleeditor?.();
              }}
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium hover:bg-accent transition-colors text-left cursor-pointer group"
            >
              <CalendarDays size={14} class="shrink-0 text-amber-600 dark:text-amber-400" />
              <div class="flex flex-col min-w-0">
                <span class="font-semibold text-foreground">Programação (Agenda)</span>
                <span class="text-[11px] text-muted-foreground font-normal">Cultos semanais e eventos especiais</span>
              </div>
            </button>

            <button
              type="button"
              role="menuitem"
              onclick={() => {
                isAddOtherMenuOpen = false;
                onopentexteditor?.();
              }}
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium hover:bg-accent transition-colors text-left cursor-pointer group"
            >
              <FileText size={14} class="shrink-0 text-blue-600 dark:text-blue-400" />
              <div class="flex flex-col min-w-0">
                <span class="font-semibold text-foreground">Texto Livre</span>
                <span class="text-[11px] text-muted-foreground font-normal">Apresentação com versos ou leitura</span>
              </div>
            </button>

            <button
              type="button"
              role="menuitem"
              onclick={() => {
                isAddOtherMenuOpen = false;
                onopenquickitemeditor?.('announcement');
              }}
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium hover:bg-accent transition-colors text-left cursor-pointer group"
            >
              <Bookmark size={14} class="shrink-0 text-emerald-600 dark:text-emerald-400" />
              <div class="flex flex-col min-w-0">
                <span class="font-semibold text-foreground">Aviso / Comunicado</span>
                <span class="text-[11px] text-muted-foreground font-normal">Avisos e comunicados da equipe</span>
              </div>
            </button>

            <button
              type="button"
              role="menuitem"
              onclick={() => {
                isAddOtherMenuOpen = false;
                onopenquickitemeditor?.('header');
              }}
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium hover:bg-accent transition-colors text-left cursor-pointer group"
            >
              <Layers size={14} class="shrink-0 text-cyan-600 dark:text-cyan-400" />
              <div class="flex flex-col min-w-0">
                <span class="font-semibold text-foreground">Divisor de Seção</span>
                <span class="text-[11px] text-muted-foreground font-normal">Separador visual de blocos na setlist</span>
              </div>
            </button>

            <button
              type="button"
              role="menuitem"
              onclick={() => {
                isAddOtherMenuOpen = false;
                onopentimereditor?.();
              }}
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium hover:bg-accent transition-colors text-left cursor-pointer group"
            >
              <Hourglass size={14} class="shrink-0 text-rose-600 dark:text-rose-400" />
              <div class="flex flex-col min-w-0">
                <span class="font-semibold text-foreground">Cronômetro / Timer</span>
                <span class="text-[11px] text-muted-foreground font-normal">Contagem regressiva sincronizada com o palco</span>
              </div>
            </button>

            <div class="my-1 border-t border-border"></div>

            <button
              type="button"
              role="menuitem"
              onclick={() => {
                isAddOtherMenuOpen = false;
                handleImportFromSetlistClick();
              }}
              disabled={!activeSetlistItems || activeSetlistItems.length === 0 || isImportingFromSetlist}
              class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium hover:bg-accent disabled:opacity-40 disabled:hover:bg-transparent transition-colors text-left cursor-pointer"
              title="Salva itens criados avulsos na setlist ativa diretamente no catálogo"
            >
              <Upload size={14} class="shrink-0 text-primary {isImportingFromSetlist ? 'animate-spin' : ''}" />
              <div class="flex flex-col min-w-0">
                <span class="font-semibold text-foreground">Importar da Setlist Ativa</span>
                <span class="text-[11px] text-muted-foreground font-normal">Salva itens avulsos da setlist no catálogo</span>
              </div>
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>

  {#if $customItemsErrorStore}
    <div
      role="alert"
      class="mb-2 px-3 py-1.5 rounded-lg border border-amber-500/35 bg-amber-500/10 text-amber-800 dark:text-amber-200 flex items-center justify-between gap-2 text-xs shrink-0"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={14} class="shrink-0" />
        <span class="truncate font-medium">
          {$customItemsErrorStore} Exibindo modelos padrão locais.
        </span>
      </div>
      <Button variant="outline" size="sm" onclick={() => loadCustomItems(true)} class="h-7 px-2.5 text-[11px] shrink-0">
        Tentar novamente
      </Button>
    </div>
  {/if}

  <!-- Chips de Categoria -->
  <div class="flex items-center gap-1.5 mb-2 overflow-x-auto pb-1 text-xs shrink-0 select-none">
    {#each [
      { id: 'all', label: 'Todos', count: kindCounts.all },
      { id: 'schedule', label: 'Agenda', count: kindCounts.schedule },
      { id: 'text', label: 'Textos', count: kindCounts.text },
      { id: 'announcement', label: 'Avisos', count: kindCounts.announcement },
      { id: 'header', label: 'Seções', count: kindCounts.header },
      { id: 'timer', label: 'Cronômetros', count: kindCounts.timer }
    ] as cat (cat.id)}
      <button
        type="button"
        onclick={() => (othersKindFilter = cat.id as any)}
        class="min-h-[28px] px-2.5 py-0.5 rounded-full text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 shrink-0 {othersKindFilter === cat.id ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'bg-muted hover:bg-muted/80 text-muted-foreground hover:text-foreground'}"
      >
        <span>{cat.label}</span>
        <span class="text-[11px] font-mono opacity-85">({cat.count})</span>
      </button>
    {/each}
  </div>

  {#snippet otherItemCard(item: MediaItem)}
    {@const badge = getItemKindBadge(item.kind)}
    {@const isSelected = viewingCustomItem?.id === item.id}
    <article
      class="rounded-lg border transition-all flex flex-col justify-between group {isSelected ? 'ring-1 ring-primary border-primary bg-primary/10 shadow-xs' : 'border-border bg-card hover:bg-muted/40'}"
    >
      <button
        type="button"
        onclick={() => (viewingCustomItem = item)}
        ondblclick={() => onpresentitem?.(item, 0)}
        oncontextmenu={(e) => handleOtherItemContextMenu(e, item)}
        class="p-2.5 pb-1.5 text-left flex-1 cursor-pointer rounded-t-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
      >
        <div class="flex items-center justify-between gap-1.5 mb-1">
          <h3 class="text-xs font-bold text-foreground truncate">{item.title}</h3>
          <span class="text-[11px] px-1.5 py-0.5 rounded font-semibold border shrink-0 {badge.color}">
            {badge.label}
          </span>
        </div>

        {#if item.kind === 'schedule'}
          <p class="text-[11px] text-muted-foreground truncate">
            {#if item.slides.length > 0}
              {item.slides.map((s) => s.metadata?.schedule_day || s.label).filter(Boolean).slice(0, 3).join(' • ')}
            {:else}
              Programação de cultos
            {/if}
          </p>
        {:else if item.kind === 'timer'}
          {@const dur = parseInt(item.slides[0]?.content || '0', 10)}
          <p class="text-[11px] text-rose-700 dark:text-rose-300 font-mono mt-0.5 flex items-center gap-1">
            <Hourglass size={11} />
            <span>Duração: {formatDuration(isNaN(dur) ? 0 : dur)}</span>
          </p>
        {:else if item.slides[0]?.content}
          <p class="text-[11px] text-muted-foreground line-clamp-2 mt-0.5 italic">
            "{item.slides[0].content.slice(0, 90)}"
          </p>
        {/if}
      </button>

      <div class="flex items-center justify-between flex-wrap px-2.5 py-1.5 border-t border-border/60 text-[11px] gap-1.5">
        <span class="font-mono text-muted-foreground shrink-0">
          {item.slides.length} {item.slides.length === 1 ? 'slide' : 'slides'}
        </span>

        <div class="flex items-center gap-1 shrink-0">
          <button
            type="button"
            onclick={() => onadditemtosetlist?.(item)}
            class="h-7 px-2 rounded-md hover:bg-primary/15 text-foreground hover:text-primary flex items-center gap-1 transition-colors cursor-pointer shrink-0 whitespace-nowrap"
            title="Adicionar à Setlist"
          >
            <ListPlus size={12} class="text-primary shrink-0" />
            <span class="hidden sm:inline">+ Setlist</span>
          </button>

          <button
            type="button"
            onclick={() => onpresentitem?.(item, 0)}
            class="h-7 px-2 rounded-md bg-primary/15 hover:bg-primary/25 text-primary font-bold flex items-center gap-1 transition-colors cursor-pointer shrink-0 whitespace-nowrap"
            title="Apresentar Imediatamente"
          >
            <Play size={11} class="fill-current shrink-0" />
            <span class="hidden sm:inline">Ao Vivo</span>
          </button>

          <button
            type="button"
            onclick={() => onopenedititem?.(item)}
            aria-label="Editar {item.title}"
            class="w-7 h-7 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
            title="Editar este item"
          >
            <Pencil size={12} />
          </button>

          <button
            type="button"
            onclick={() => requestDeleteCustomItem(item)}
            aria-label="Excluir {item.title} do catálogo"
            class="w-7 h-7 rounded-md hover:bg-red-500/15 text-muted-foreground hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer shrink-0"
            title="Excluir do catálogo"
          >
            <Trash2 size={12} />
          </button>
        </div>
      </div>
    </article>
  {/snippet}

  {#snippet viewingCustomItemSheet()}
    {#if viewingCustomItem}
      {@const currentBadge = getItemKindBadge(viewingCustomItem.kind)}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        onkeydown={(e) => {
          if (e.key === 'Escape') {
            e.stopPropagation();
            viewingCustomItem = null;
          }
        }}
        class="flex-1 min-w-0 border border-border bg-card rounded-xl flex flex-col p-3.5 h-full overflow-hidden shadow-md animate-in slide-in-from-right duration-150"
      >
        <div class="flex items-center justify-between pb-2.5 border-b border-border shrink-0 gap-2 flex-wrap">
          <div class="min-w-0 mr-2">
            <div class="flex items-center gap-2 flex-wrap">
              <h3 class="text-sm font-bold text-foreground truncate">{viewingCustomItem.title}</h3>
              <span class="text-[11px] px-2 py-0.5 rounded font-semibold border shrink-0 {currentBadge.color}">
                {currentBadge.label}
              </span>
              <Badge variant="secondary" class="text-[11px] font-mono">
                {viewingCustomItem.slides.length} slides
              </Badge>
            </div>
            {#if viewingCustomItem.artist_or_ref}
              <p class="text-[11px] text-muted-foreground truncate mt-0.5">{viewingCustomItem.artist_or_ref}</p>
            {/if}
          </div>

          <div class="flex items-center gap-1.5 shrink-0 flex-wrap">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => viewingCustomItem && onadditemtosetlist?.(viewingCustomItem)}
              class="h-8 text-xs gap-1"
            >
              <ListPlus size={12} class="text-primary" />
              <span>+ Setlist</span>
            </Button>
            <Button
              variant="live"
              size="sm"
              onclick={() => viewingCustomItem && onpresentitem?.(viewingCustomItem, 0)}
              class="h-8 text-xs gap-1 font-bold"
            >
              <Play size={12} class="fill-current" />
              <span>Apresentar</span>
            </Button>
            <Button
              variant="outline"
              size="sm"
              onclick={() => viewingCustomItem && onopenedititem?.(viewingCustomItem)}
              class="h-8 text-xs gap-1"
            >
              <Pencil size={12} />
              <span>Editar</span>
            </Button>
            <button
              type="button"
              onclick={() => (viewingCustomItem = null)}
              aria-label="Fechar prévia de slides"
              class="w-8 h-8 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer"
              title="Fechar visualização de slides (Esc)"
            >
              <X size={16} />
            </button>
          </div>
        </div>

        <!-- Grade 16:9 -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="flex-1 overflow-y-auto py-3 pr-1 min-h-0" oncontextmenu={handleOtherEmptyContextMenu}>
          {#if viewingCustomItem.slides.length > 0}
            <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3">
              {#each viewingCustomItem.slides as slide, idx (slide.id || idx)}
                <article
                  oncontextmenu={(e) => viewingCustomItem && handleOtherSlideContextMenu(e, viewingCustomItem, slide, idx)}
                  class="group rounded-xl border border-border bg-card flex flex-col overflow-hidden shadow-xs hover:border-primary/60 transition-all"
                >
                  <div class="px-2.5 py-1.5 bg-muted/50 border-b border-border flex items-center justify-between gap-1 shrink-0">
                    <span class="text-[11px] font-bold text-foreground font-mono truncate">
                      {slide.label || `Slide ${idx + 1}`}
                    </span>
                    <button
                      type="button"
                      onclick={() => viewingCustomItem && onpresentitem?.(viewingCustomItem, idx)}
                      aria-label="Apresentar slide {slide.label || idx + 1} ao vivo"
                      class="h-6 px-2 rounded bg-primary/15 hover:bg-primary/25 text-primary text-[11px] font-bold flex items-center gap-1 shrink-0 cursor-pointer"
                    >
                      <Play size={10} class="fill-current" />
                      <span>Ao Vivo</span>
                    </button>
                  </div>

                  <button
                    type="button"
                    onclick={() => viewingCustomItem && onpresentitem?.(viewingCustomItem, idx)}
                    class="aspect-video w-full p-3 flex flex-col justify-center items-center text-center select-none bg-slate-950 text-slate-50 relative overflow-hidden cursor-pointer hover:bg-slate-900 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
                    title="Clique para apresentar este slide ao vivo"
                  >
                    {#if viewingCustomItem.kind === 'schedule'}
                      <div class="w-full flex flex-col items-center justify-center p-2 rounded-lg bg-white/5 border border-white/10">
                        {#if slide.metadata?.schedule_day || slide.metadata?.schedule_time}
                          <div class="flex items-center gap-1 mb-1 flex-wrap justify-center">
                            {#if slide.metadata?.schedule_day}
                              <span class="px-1.5 py-0.5 rounded text-[11px] font-bold uppercase bg-amber-500/20 text-amber-200 border border-amber-400/35">
                                {slide.metadata.schedule_day}
                              </span>
                            {/if}
                            {#if slide.metadata?.schedule_time}
                              <span class="px-1.5 py-0.5 rounded text-[11px] font-bold font-mono bg-cyan-500/20 text-cyan-200 border border-cyan-400/35">
                                {slide.metadata.schedule_time}
                              </span>
                            {/if}
                          </div>
                        {/if}
                        <h4 class="text-xs font-bold text-white line-clamp-2 leading-tight">
                          {slide.label || slide.content}
                        </h4>
                        {#if slide.metadata?.schedule_details}
                          <p class="text-[11px] text-slate-300 line-clamp-1 mt-1">
                            {slide.metadata.schedule_details}
                          </p>
                        {/if}
                      </div>
                    {:else if viewingCustomItem.kind === 'header'}
                      <div class="px-3 py-1 rounded bg-cyan-500/15 border border-cyan-400/40 text-cyan-200 font-bold text-xs uppercase tracking-wider">
                        {slide.content || viewingCustomItem.title}
                      </div>
                    {:else if viewingCustomItem.kind === 'timer'}
                      {@const secs = parseInt(slide.content || '0', 10)}
                      <div class="w-full flex flex-col items-center justify-center p-2 rounded-xl bg-rose-950/60 border border-rose-500/40 gap-1">
                        <div class="flex items-center gap-1.5 text-rose-200 font-semibold text-[11px] uppercase tracking-wider">
                          <Hourglass size={12} />
                          <span>{slide.label || 'Cronômetro'}</span>
                        </div>
                        <div class="text-3xl font-mono font-black text-rose-200 tracking-tight">
                          {formatDuration(isNaN(secs) ? 0 : secs)}
                        </div>
                        {#if slide.lower_content}
                          <p class="text-[11px] text-rose-100/90 line-clamp-2 mt-0.5 max-w-full italic">
                            {slide.lower_content}
                          </p>
                        {/if}
                      </div>
                    {:else}
                      <p class="text-xs text-slate-100 font-medium leading-relaxed line-clamp-4 whitespace-pre-wrap">
                        {slide.content}
                      </p>
                    {/if}
                  </button>
                </article>
              {/each}
            </div>
          {:else}
            <div class="h-full flex items-center justify-center text-xs text-muted-foreground">
              Este item não possui slides cadastrados.
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {/snippet}

  <div class="flex-1 flex overflow-hidden min-h-0 w-full h-full">
    {#if viewingCustomItem}
      <ResizablePaneGroup direction="horizontal" class="flex-1 w-full h-full min-h-0 overflow-hidden">
        <ResizablePane defaultSize={35} minSize={20} maxSize={60} class="flex flex-col p-1.5 pr-2 overflow-y-auto min-h-0">
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="grid grid-cols-1 gap-2" oncontextmenu={handleOtherEmptyContextMenu}>
            {#each filteredCustomItems as item (item.id)}
              {@render otherItemCard(item)}
            {/each}
          </div>
        </ResizablePane>

        <ResizableHandle withHandle class="hover:bg-primary transition-colors cursor-col-resize" />

        <ResizablePane defaultSize={65} minSize={40} class="flex flex-col pl-3 min-w-0 min-h-0 overflow-hidden">
          {@render viewingCustomItemSheet()}
        </ResizablePane>
      </ResizablePaneGroup>
    {:else}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="flex-1 overflow-y-auto flex flex-col p-1.5 min-h-0" oncontextmenu={handleOtherEmptyContextMenu}>
        {#if filteredCustomItems.length === 0}
          <div class="h-48 flex flex-col items-center justify-center text-center p-6 text-muted-foreground space-y-2">
            <Boxes size={36} class="opacity-40" />
            <p class="text-xs font-medium">Nenhum item encontrado nesta categoria.</p>
            <div class="flex items-center gap-2 pt-2">
              <Button variant="outline" size="sm" class="text-xs h-8" onclick={onopenscheduleeditor}>
                + Criar Agenda
              </Button>
              <Button variant="outline" size="sm" class="text-xs h-8" onclick={onopentexteditor}>
                + Criar Texto
              </Button>
            </div>
          </div>
        {:else}
          <div class="grid grid-cols-[repeat(auto-fill,minmax(265px,1fr))] gap-2.5">
            {#each filteredCustomItems as item (item.id)}
              {@render otherItemCard(item)}
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
