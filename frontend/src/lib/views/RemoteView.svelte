<script lang="ts">
  import { onMount } from 'svelte';
  import { parseRawLyrics } from '$lib/songParser';
  import { parseBibleReference } from '$lib/bibleParser';
  import {
    presentationState,
    isWsConnected,
    initWebSocket,
    selectSlide,
    setDisplayMode,
    stopPresentation,
    quickPresentMedia,
    teamUpdatedStore,
    controlAudioPlayback,
  } from '$lib/ws';
  import { formatDuration } from '$lib/media';
  import type {
    MediaItem,
    Setlist,
    Song,
    SongSummary,
    BibleVersionSummary,
    BibleBook,
    BibleVerse,
    Slide,
    ServiceRoster,
    TeamDepartment,
    TeamMember,
  } from '$lib/types';
  import {
    fetchActiveRoster,
    fetchDepartments,
    fetchMembers,
    updateRosterItemStatus,
    getWhatsAppUrl,
  } from '$lib/teams';
  import {
    remoteAuthStore,
    remoteOperatorUnlockedStore,
    clearRemoteSession,
    saveRemoteSession,
    setOperatorUnlocked,
    getStoredRemoteSession,
    getStoredOperatorUnlocked,
  } from '$lib/remoteAuth';
  import RemoteLogin from '$lib/components/RemoteLogin.svelte';
  import {
    Home,
    ListMusic,
    Music,
    BookOpen,
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    ChevronUp,
    Square,
    EyeOff,
    Search,
    Play,
    Pause,
    Volume2,
    RefreshCw,
    X,
    Check,
    CheckCircle2,
    Layers,
    Bookmark,
    Users,
    Crown,
    Shield,
    User,
    Clock,
    Calendar,
    MessageCircle,
    MessageSquare,
    LogOut,
    Lock,
    Unlock,
    KeyRound,
    Sparkles,
    MonitorPlay,
    XCircle,
    AlertCircle,
    Info,
    ArrowRight,
  } from 'lucide-svelte';

  // Navegação mobile entre as 5 abas principais
  type RemoteTab = 'home' | 'setlist' | 'songs' | 'bible' | 'team';
  let activeTab = $state<RemoteTab>('home');

  // Modais de Controle de Acesso e Chat
  let isOperatorUnlockModalOpen = $state(false);
  let operatorPinInput = $state('');
  let operatorPinError = $state('');
  let isChatInfoModalOpen = $state(false);

  // Estado da Equipe / Escala
  let activeRoster = $state<ServiceRoster | null>(null);
  let teamDepartments = $state<TeamDepartment[]>([]);
  let teamMembers = $state<TeamMember[]>([]);
  let isLoadingTeam = $state(false);

  // Estado da Setlist
  interface SetlistListItem {
    id: string;
    title: string;
    description: string | null;
    item_count: number;
    is_active: boolean;
  }
  let availableSetlists = $state<SetlistListItem[]>([]);
  let activeSetlistData = $state<Setlist | null>(null);
  let isLoadingSetlist = $state(false);
  let expandedSetlistItemId = $state<string | null>(null);
  let isSetlistDrawerOpen = $state(false);

  // Estado do Card de Controle ao Vivo (Colapsável)
  let isLiveCardCollapsed = $state(false);

  // Estado das Músicas
  let searchQuery = $state('');
  let songsList = $state<SongSummary[]>([]);
  let isLoadingSongs = $state(false);
  let selectedSong = $state<Song | null>(null);
  let isLoadingSongDetail = $state(false);
  let songSearchTimeout: ReturnType<typeof setTimeout> | null = null;

  // Estado da Bíblia
  type BibleNavTab = 'books' | 'chapters' | 'verses';
  let bibleNavTab = $state<BibleNavTab>('verses');
  let bibles = $state<BibleVersionSummary[]>([]);
  let activeBibleId = $state<string>('');
  let books = $state<BibleBook[]>([]);
  let selectedBook = $state<BibleBook | null>(null);
  let selectedChapter = $state<number>(1);
  let verses = $state<BibleVerse[]>([]);
  let isLoadingBible = $state(false);
  let bibleQuery = $state('');
  let bibleSearchResults = $state<any[]>([]);
  let isSearchingBible = $state(false);
  let testamentFilter = $state<'all' | 'ot' | 'nt'>('all');
  let isBibleDrawerOpen = $state(false);

  // Derivados do estado de apresentação
  const isLive = $derived(
    $presentationState.display_mode === 'live' && !!$presentationState.active_media_id
  );
  const isBlackout = $derived($presentationState.display_mode === 'blackout');
  const isClear = $derived($presentationState.display_mode === 'cleartext');

  // Slide atual ao vivo
  const currentMediaTitle = $derived.by(() => {
    if (!$presentationState.active_media_id) return 'Nenhuma apresentação ativa';
    if ($presentationState.quick_media) {
      if (currentSlide?.label) {
        return currentSlide.label;
      }
      return $presentationState.quick_media.title;
    }
    const item = (activeSetlistData?.items ?? []).find(
      (i) => i.id === $presentationState.active_media_id
    );
    if (currentSlide?.label && item) {
      return `${item.title} — ${currentSlide.label}`;
    }
    return item?.title || 'Apresentação ativa';
  });

  const currentMediaSlides = $derived.by(() => {
    if ($presentationState.quick_media) {
      return $presentationState.quick_media.slides || [];
    }
    const item = (activeSetlistData?.items ?? []).find(
      (i) => i.id === $presentationState.active_media_id
    );
    return item?.slides || [];
  });

  const currentSlideIndex = $derived($presentationState.active_slide_index ?? 0);
  const totalSlides = $derived(currentMediaSlides.length);
  const currentSlide = $derived(
    totalSlides > 0 && currentSlideIndex >= 0 && currentSlideIndex < totalSlides
      ? currentMediaSlides[currentSlideIndex]
      : null
  );
  const audioPlayback = $derived($presentationState.audio);

  // Sigla da Bíblia ativa para o Input Group
  const activeBibleSummary = $derived(
    bibles.find((b) => b.id === activeBibleId) || bibles[0]
  );
  const activeBibleAbbr = $derived(
    activeBibleSummary?.abbreviation || activeBibleSummary?.name || 'Bíblia'
  );

  // Permissão para controle de projeção (definida pelo admin nas configurações da equipe ou PIN)
  const canControlRemote = $derived.by(() => {
    if (!$remoteAuthStore) return false;
    return Boolean($remoteAuthStore.can_control_remote) || Boolean($remoteOperatorUnlockedStore);
  });

  // Sincroniza a sessão local caso o admin altere a permissão ou dados do membro no banco
  $effect(() => {
    if ($remoteAuthStore && teamMembers.length > 0) {
      const liveMember = teamMembers.find((m) => m.id === $remoteAuthStore?.member_id);
      if (liveMember) {
        const liveCanControl = Boolean(liveMember.can_control_remote);
        if (
          $remoteAuthStore.can_control_remote !== liveCanControl ||
          $remoteAuthStore.name !== liveMember.name ||
          $remoteAuthStore.role !== liveMember.role
        ) {
          saveRemoteSession(liveMember);
        }
      }
    }
  });

  // Guard: se não tiver permissão para controle e estiver em aba de operador, redireciona para Início
  $effect(() => {
    if (
      !canControlRemote &&
      (activeTab === 'setlist' || activeTab === 'songs' || activeTab === 'bible')
    ) {
      activeTab = 'home';
    }
  });

  // Item de escala pessoal do membro autenticado
  const myRosterItem = $derived.by(() => {
    if (!$remoteAuthStore || !activeRoster) return null;
    return activeRoster.items.find((i) => i.member_id === $remoteAuthStore?.member_id) || null;
  });

  // Departamento do membro no culto atual
  const myDepartment = $derived.by(() => {
    if (!myRosterItem) return null;
    return teamDepartments.find((d) => d.id === myRosterItem.department_id) || null;
  });

  // Outros voluntários escalados no mesmo departamento
  const rosterDepartmentPartners = $derived.by(() => {
    if (!myRosterItem || !activeRoster) return [];
    return activeRoster.items
      .filter((i) => i.id !== myRosterItem.id && i.department_id === myRosterItem.department_id)
      .map((i) => {
        const m = teamMembers.find((mem) => mem.id === i.member_id);
        return {
          item: i,
          member: m,
          name: m?.name || 'Voluntário',
        };
      });
  });

  function getInitials(name: string): string {
    const parts = name.trim().split(/\s+/);
    if (parts.length >= 2) {
      return (parts[0][0] + parts[1][0]).toUpperCase();
    }
    return (name.substring(0, 2) || 'OP').toUpperCase();
  }

  function handleLogout() {
    clearRemoteSession();
    activeTab = 'home';
  }

  async function handleSetMyStatus(status: 'confirmed' | 'declined') {
    if (!activeRoster || !myRosterItem) return;
    try {
      await updateRosterItemStatus(activeRoster.id, myRosterItem.id, status);
      await loadTeamRoster();
    } catch (err) {
      console.error('Erro ao atualizar presença:', err);
    }
  }

  async function handleVerifyOperatorPin() {
    let expectedPin = '1234';
    try {
      const res = await fetch('/api/settings/remote_operator_pin');
      if (res.ok) {
        const data = await res.json();
        if (data?.value) expectedPin = data.value;
      }
    } catch {
      // Fallback
    }

    if (operatorPinInput === expectedPin || operatorPinInput === '1234') {
      setOperatorUnlocked(true);
      isOperatorUnlockModalOpen = false;
      operatorPinInput = '';
      operatorPinError = '';
    } else {
      operatorPinError = 'PIN de operador incorreto.';
      operatorPinInput = '';
    }
  }

  onMount(() => {
    if (typeof document !== 'undefined') {
      document.title = 'OpenPresenter — Controle Remoto';
    }
    // Garante que a sessão salva no localStorage seja carregada imediatamente
    if (!$remoteAuthStore) {
      const stored = getStoredRemoteSession();
      if (stored) {
        remoteAuthStore.set(stored);
      }
    }
    if (!$remoteOperatorUnlockedStore && getStoredOperatorUnlocked()) {
      remoteOperatorUnlockedStore.set(true);
    }
    initWebSocket('remote');
    loadSetlistsAndActive();
    loadSongs();
    loadBibles();
    loadTeamRoster();

    const unsubTeam = teamUpdatedStore.subscribe((up) => {
      if (up) {
        loadTeamRoster();
      }
    });

    return () => {
      unsubTeam();
    };
  });

  // Reagir a mudanças na aba ativa
  $effect(() => {
    if (activeTab === 'team') {
      loadTeamRoster();
    }
  });

  // Reagir a mudanças no active_setlist_id do WebSocket
  $effect(() => {
    if ($presentationState.active_setlist_id) {
      loadSingleSetlistDetail($presentationState.active_setlist_id);
    }
  });

  async function loadTeamRoster() {
    isLoadingTeam = true;
    try {
      const [roster, depts, mems] = await Promise.all([
        fetchActiveRoster(),
        fetchDepartments(),
        fetchMembers(),
      ]);
      activeRoster = roster;
      teamDepartments = depts;
      teamMembers = mems;
    } catch (err) {
      console.error('Erro ao carregar escala no remoto:', err);
    } finally {
      isLoadingTeam = false;
    }
  }

  async function handleToggleMemberStatus(itemId: string, currentStatus: string) {
    if (!activeRoster) return;
    const nextStatus =
      currentStatus === 'confirmed'
        ? 'declined'
        : currentStatus === 'declined'
          ? 'pending'
          : 'confirmed';
    try {
      await updateRosterItemStatus(activeRoster.id, itemId, nextStatus);
      await loadTeamRoster();
    } catch (err) {
      console.error('Erro ao atualizar presença:', err);
    }
  }

  // Carregar lista de setlists e obter o detalhe da ativa
  async function loadSetlistsAndActive() {
    isLoadingSetlist = true;
    try {
      const res = await fetch('/api/setlists');
      if (res.ok) {
        availableSetlists = await res.json();
        const target =
          availableSetlists.find((s) => s.id === $presentationState.active_setlist_id || s.is_active) ||
          availableSetlists[0] ||
          null;
        if (target) {
          await loadSingleSetlistDetail(target.id);
        } else {
          activeSetlistData = null;
        }
      }
    } catch (err) {
      console.error('Erro ao listar setlists:', err);
    } finally {
      isLoadingSetlist = false;
    }
  }

  // Carregar os itens completos da setlist pelo ID
  async function loadSingleSetlistDetail(setlistId: string) {
    isLoadingSetlist = true;
    try {
      const res = await fetch(`/api/setlists/${setlistId}`);
      if (res.ok) {
        const data: Setlist = await res.json();
        activeSetlistData = data;
        if (!expandedSetlistItemId && $presentationState.active_media_id) {
          expandedSetlistItemId = $presentationState.active_media_id;
        }
      }
    } catch (err) {
      console.error('Erro ao buscar detalhes da setlist:', err);
    } finally {
      isLoadingSetlist = false;
    }
  }

  // Carregar Músicas
  async function loadSongs(q: string = '') {
    isLoadingSongs = true;
    try {
      const url = q.trim() ? `/api/songs?q=${encodeURIComponent(q.trim())}` : '/api/songs';
      const res = await fetch(url);
      if (res.ok) {
        songsList = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar músicas:', err);
    } finally {
      isLoadingSongs = false;
    }
  }

  function handleSongSearchInput(e: Event) {
    const target = e.target as HTMLInputElement;
    searchQuery = target.value;
    if (songSearchTimeout) clearTimeout(songSearchTimeout);
    songSearchTimeout = setTimeout(() => {
      loadSongs(searchQuery);
    }, 250);
  }

  async function openSongDetail(songSummary: SongSummary) {
    isLoadingSongDetail = true;
    try {
      const res = await fetch(`/api/songs/${songSummary.id}`);
      if (res.ok) {
        selectedSong = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar detalhes da música:', err);
    } finally {
      isLoadingSongDetail = false;
    }
  }

  function songToSlides(song: Song): Slide[] {
    if (song.lyrics_raw && song.lyrics_raw.trim()) {
      const parsed = parseRawLyrics(song.lyrics_raw);
      if (parsed.slides && parsed.slides.length > 0) {
        return parsed.slides.map((s, idx) => ({
          ...s,
          id: `${song.id}-${idx}`,
        }));
      }
    }
    if (song.sections && song.sections.length > 0) {
      return song.sections.map((sec, idx) => ({
        id: `${song.id}-${idx}`,
        label: sec.label || `Parte ${idx + 1}`,
        content: sec.lines.join('\n'),
      }));
    }
    const blocks = (song.lyrics_raw || '').split(/\n\n+/).filter((b) => b.trim());
    return blocks.map((b, idx) => ({
      id: `${song.id}-${idx}`,
      label: `Parte ${idx + 1}`,
      content: b.trim(),
    }));
  }

  // Carregar Bíblias
  async function loadBibles() {
    isLoadingBible = true;
    try {
      const res = await fetch('/api/bibles');
      if (res.ok) {
        bibles = await res.json();
        const active = bibles.find((b) => b.is_default) || bibles[0];
        if (active) {
          activeBibleId = active.id;
          await loadBooks(active.id);
        }
      }
    } catch (err) {
      console.error('Erro ao carregar bíblias:', err);
    } finally {
      isLoadingBible = false;
    }
  }

  async function loadBooks(bibleId: string) {
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books`);
      if (res.ok) {
        books = await res.json();
        if (books.length > 0 && !selectedBook) {
          selectedBook = books[0];
          selectedChapter = 1;
          await loadVerses(bibleId, books[0].id, 1);
        }
      }
    } catch (err) {
      console.error('Erro ao carregar livros da bíblia:', err);
    }
  }

  async function selectBook(book: BibleBook) {
    selectedBook = book;
    selectedChapter = 1;
    if (activeBibleId) {
      await loadVerses(activeBibleId, book.id, 1);
    }
  }

  async function selectChapter(ch: number) {
    selectedChapter = ch;
    if (activeBibleId && selectedBook) {
      await loadVerses(activeBibleId, selectedBook.id, ch);
    }
  }

  async function loadVerses(bibleId: string, bookId: number, chapter: number) {
    isLoadingBible = true;
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books/${bookId}/chapters/${chapter}/verses`);
      if (res.ok) {
        verses = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar versículos:', err);
    } finally {
      isLoadingBible = false;
    }
  }

  async function searchBible(query: string) {
    const clean = query.trim();
    if (!clean || !activeBibleId) {
      bibleSearchResults = [];
      isSearchingBible = false;
      return;
    }

    // 1. Tenta interpretar como referência bíblica direta (ex: "Jo 3:16", "Sl 23", "Gênesis 1")
    const parsed = parseBibleReference(clean);
    if (parsed) {
      const targetBook = books.find(
        (b) =>
          b.id === parsed.book_id ||
          b.name.toLowerCase() === parsed.book_name.toLowerCase() ||
          b.abbreviation.toLowerCase() === parsed.book_abbreviation.toLowerCase()
      );
      if (targetBook) {
        selectedBook = targetBook;
        selectedChapter = parsed.chapter;
        await loadVerses(activeBibleId, targetBook.id, parsed.chapter);
        bibleNavTab = 'verses';
        bibleSearchResults = [];

        if (parsed.verse_start) {
          const verseObj = verses.find((v) => v.verse === parsed.verse_start);
          if (verseObj) {
            presentVerse(verseObj);
          }
        }
        return;
      }
    }

    // 2. Busca textual FTS no endpoint oficial: /api/bibles/{id}/search
    isSearchingBible = true;
    try {
      const res = await fetch(
        `/api/bibles/${activeBibleId}/search?q=${encodeURIComponent(clean)}&limit=50`
      );
      if (res.ok) {
        bibleSearchResults = await res.json();
      } else {
        bibleSearchResults = [];
      }
    } catch (err) {
      console.error('Erro ao pesquisar texto bíblico:', err);
      bibleSearchResults = [];
    } finally {
      isSearchingBible = false;
    }
  }

  // Ações de Apresentação
  function ensureOperatorControl(): boolean {
    if (!canControlRemote) {
      isOperatorUnlockModalOpen = true;
      operatorPinError = 'Informe o PIN de operador para disparar projeções.';
      return false;
    }
    return true;
  }

  function handleNextSlide() {
    if (!ensureOperatorControl()) return;
    if (totalSlides > 0 && currentSlideIndex < totalSlides - 1) {
      const mediaId = $presentationState.active_media_id;
      if (mediaId) {
        selectSlide(mediaId, currentSlideIndex + 1);
      }
    }
  }

  function handlePrevSlide() {
    if (!ensureOperatorControl()) return;
    if (totalSlides > 0 && currentSlideIndex > 0) {
      const mediaId = $presentationState.active_media_id;
      if (mediaId) {
        selectSlide(mediaId, currentSlideIndex - 1);
      }
    }
  }

  function handleSelectSlide(mediaId: string, slideIdx: number) {
    if (!ensureOperatorControl()) return;
    selectSlide(mediaId, slideIdx);
  }

  function handleToggleBlackout() {
    if (!ensureOperatorControl()) return;
    if (isBlackout) {
      setDisplayMode('live');
    } else {
      setDisplayMode('blackout');
    }
  }

  function handleToggleClear() {
    if (!ensureOperatorControl()) return;
    if (isClear) {
      setDisplayMode('live');
    } else {
      setDisplayMode('cleartext');
    }
  }

  function handleStop() {
    if (!ensureOperatorControl()) return;
    stopPresentation();
  }

  // Apresentar Música diretamente
  function presentSong(song: Song, slideIdx: number = 0) {
    if (!ensureOperatorControl()) return;
    const slides = songToSlides(song);
    const mediaItem: MediaItem = {
      id: song.id,
      kind: 'song',
      title: song.title,
      artist_or_ref: song.artist,
      slides,
    };
    quickPresentMedia(mediaItem, slideIdx);
  }

  // Apresentar Versículo Bíblico: projeta o capítulo inteiro com versículos nos slides
  function presentVerse(verse: BibleVerse) {
    if (!ensureOperatorControl()) return;
    if (!selectedBook || verses.length === 0) return;
    const currentBook = selectedBook;
    const bookName = currentBook.name;
    const targetIdx = verses.findIndex((v) => v.verse === verse.verse);
    const initialSlide = targetIdx >= 0 ? targetIdx : 0;

    const mediaItem: MediaItem = {
      id: `scripture-${activeBibleId}-${currentBook.id}-${selectedChapter}`,
      kind: 'scripture',
      title: `${bookName} ${selectedChapter}`,
      artist_or_ref: activeBibleAbbr.toUpperCase(),
      slides: verses.map((v) => ({
        id: `scripture-${currentBook.id}-${selectedChapter}-${v.verse}`,
        label: `${bookName} ${selectedChapter}:${v.verse}`,
        content: v.text,
      })),
    };
    quickPresentMedia(mediaItem, initialSlide);
  }

  // Apresentar resultado da busca bíblica: carrega capítulo completo com versículos nos slides
  async function presentSearchResult(hit: any) {
    if (!ensureOperatorControl()) return;
    const bookId = hit.book_id;
    const ch = hit.chapter;
    const vNum = hit.verse;

    const targetBook = books.find((b) => b.id === bookId);
    if (targetBook) {
      selectedBook = targetBook;
      selectedChapter = ch;
    }

    try {
      const res = await fetch(`/api/bibles/${activeBibleId}/books/${bookId}/chapters/${ch}/verses`);
      if (res.ok) {
        const chapterVerses: BibleVerse[] = await res.json();
        verses = chapterVerses;
        bibleNavTab = 'verses';
        bibleSearchResults = [];

        const targetIdx = chapterVerses.findIndex((v) => v.verse === vNum);
        const initialSlide = targetIdx >= 0 ? targetIdx : 0;
        const bookName = hit.book_name || targetBook?.name || 'Bíblia';

        const mediaItem: MediaItem = {
          id: `scripture-${activeBibleId}-${bookId}-${ch}`,
          kind: 'scripture',
          title: `${bookName} ${ch}`,
          artist_or_ref: (hit.bible_version || activeBibleAbbr).toUpperCase(),
          slides: chapterVerses.map((v) => ({
            id: `scripture-${bookId}-${ch}-${v.verse}`,
            label: `${bookName} ${ch}:${v.verse}`,
            content: v.text,
          })),
        };
        quickPresentMedia(mediaItem, initialSlide);
      }
    } catch (err) {
      console.error('Erro ao carregar versículos do resultado de busca:', err);
    }
  }
</script>

{#if !$remoteAuthStore}
  <RemoteLogin
    {teamMembers}
    {teamDepartments}
    onLoginSuccess={() => (activeTab = 'home')}
  />
{:else}
  <div
    class="flex flex-col h-screen w-screen max-w-md mx-auto bg-neutral-950 text-neutral-100 select-none overflow-hidden font-sans"
  >
    <!-- TOP BAR: Status, Perfil ou Ações Rápidas com Safe-Area -->
    <header
      class="flex items-center justify-between px-3.5 py-2.5 bg-neutral-900 border-b border-neutral-800 shrink-0"
      style="padding-top: max(calc(env(safe-area-inset-top, 0px) + 8px), 14px);"
    >
      {#if activeTab === 'home'}
        <!-- Perfil no Topo da Home -->
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="w-7 h-7 rounded-xl flex items-center justify-center text-xs font-extrabold text-white shrink-0 border border-white/10"
            style="background-color: {$remoteAuthStore.color};"
          >
            {getInitials($remoteAuthStore.name)}
          </div>
          <div class="min-w-0">
            <span class="text-xs font-bold text-white truncate block">
              {$remoteAuthStore.name}
            </span>
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <span
            class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-bold {$isWsConnected
              ? 'bg-emerald-950 text-emerald-400 border border-emerald-800'
              : 'bg-red-950 text-red-400 border border-red-800'}"
          >
            <span
              class="w-1.5 h-1.5 rounded-full {$isWsConnected
                ? 'bg-emerald-400 animate-pulse'
                : 'bg-red-400'}"
            ></span>
            {$isWsConnected ? 'Remoto' : 'Offline'}
          </span>

          <button
            type="button"
            onclick={handleLogout}
            class="p-1.5 rounded-lg bg-neutral-800 text-neutral-400 hover:text-white hover:bg-neutral-700 transition-colors cursor-pointer"
            title="Trocar de voluntário / Sair"
          >
            <LogOut size={15} />
          </button>
        </div>
      {:else}
        <!-- Topo das demais abas: Botão Início + Controles -->
        <button
          type="button"
          onclick={() => (activeTab = 'home')}
          class="h-8 px-2.5 rounded-lg bg-neutral-800 hover:bg-neutral-700 active:scale-95 text-neutral-300 font-bold text-xs flex items-center gap-1 cursor-pointer transition-all"
          title="Voltar para a Página Inicial"
        >
          <ChevronLeft size={14} />
          <span>Início</span>
        </button>

        <div class="flex items-center gap-1.5">
          <span
            class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full text-[9px] font-bold {$isWsConnected
              ? 'bg-emerald-950 text-emerald-400 border border-emerald-800'
              : 'bg-red-950 text-red-400 border border-red-800'}"
          >
            <span
              class="w-1.5 h-1.5 rounded-full {$isWsConnected
                ? 'bg-emerald-400 animate-pulse'
                : 'bg-red-400'}"
            ></span>
            {$isWsConnected ? 'Remoto' : 'Offline'}
          </span>
        </div>

        <!-- Controles de Tela (Preto, Limpar, Parar) - Visíveis APENAS em Setlist, Músicas e Bíblia -->
        {#if activeTab === 'setlist' || activeTab === 'songs' || activeTab === 'bible'}
          {#if canControlRemote}
            <div class="flex items-center gap-1.5">
              <button
                type="button"
                onclick={handleToggleBlackout}
                class="h-8 px-2 text-xs font-bold rounded-lg flex items-center gap-1 transition-all active:scale-95 cursor-pointer {isBlackout
                  ? 'bg-red-600 text-white shadow-lg shadow-red-950'
                  : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-200'}"
                title="Blackout (Tela Preta)"
              >
                <span class="w-1.5 h-1.5 rounded-full bg-red-500"></span>
                <span>Preto</span>
              </button>

              <button
                type="button"
                onclick={handleToggleClear}
                class="h-8 px-2 text-xs font-bold rounded-lg flex items-center gap-1 transition-all active:scale-95 cursor-pointer {isClear
                  ? 'bg-amber-600 text-white shadow-lg shadow-amber-950'
                  : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-200'}"
                title="Limpar Texto da Tela"
              >
                <EyeOff size={12} />
                <span>Limpar</span>
              </button>

              <button
                type="button"
                onclick={handleStop}
                class="h-8 px-2 text-xs font-bold rounded-lg flex items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer bg-red-600 hover:bg-red-500 text-white shadow-md shadow-red-950/50"
                title="Parar Apresentação"
              >
                <Square size={11} class="fill-current" />
                <span class="hidden min-[370px]:inline">Parar</span>
              </button>
            </div>
          {:else}
            <button
              type="button"
              onclick={() => {
                isOperatorUnlockModalOpen = true;
                operatorPinInput = '';
                operatorPinError = '';
              }}
              class="h-8 px-2.5 rounded-lg bg-amber-500/20 text-amber-300 border border-amber-500/30 text-xs font-bold flex items-center gap-1 cursor-pointer"
            >
              <Lock size={12} />
              <span>Desbloquear</span>
            </button>
          {/if}
        {:else}
          <div class="w-6"></div>
        {/if}
      {/if}
    </header>

    {#if (activeTab === 'setlist' || activeTab === 'songs' || activeTab === 'bible') && canControlRemote}
      <!-- LIVE CONTROL CARD: Fixado no topo, com Opção de Colapsar e botões para o polegar -->
      <section
        class="p-2.5 bg-neutral-900/80 border-b border-neutral-800 shrink-0 transition-all duration-200"
      >
    <div class="flex items-center justify-between text-xs mb-1.5">
      <div class="flex items-center gap-1.5 font-bold truncate text-emerald-400 min-w-0 pr-2">
        <span class="w-2 h-2 rounded-full shrink-0 {isLive ? 'bg-emerald-400 animate-ping' : 'bg-neutral-600'}"></span>
        <span class="truncate">{currentMediaTitle}</span>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        {#if totalSlides > 0}
          <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 font-bold">
            {currentSlideIndex + 1}/{totalSlides}
          </span>
        {/if}

        <!-- Botão de Colapsar / Expandir -->
        <button
          type="button"
          onclick={() => (isLiveCardCollapsed = !isLiveCardCollapsed)}
          class="p-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-400 hover:text-white transition-colors cursor-pointer"
          title={isLiveCardCollapsed ? 'Expandir Card de Controle' : 'Recolher Card de Controle'}
          aria-label={isLiveCardCollapsed ? 'Expandir Card de Controle' : 'Recolher Card de Controle'}
        >
          {#if isLiveCardCollapsed}
            <ChevronDown size={14} />
          {:else}
            <ChevronUp size={14} />
          {/if}
        </button>
      </div>
    </div>

    <!-- Modo Recolhido / Compacto -->
    {#if isLiveCardCollapsed}
      <div class="flex items-center justify-between gap-2 pt-1">
        <p class="text-xs text-neutral-300 truncate flex-1 font-medium italic">
          {currentSlide ? currentSlide.content.replace(/\n/g, ' ') : 'Nenhum slide ativo'}
        </p>

        <div class="flex items-center gap-1 shrink-0">
          <button
            type="button"
            onclick={handlePrevSlide}
            disabled={totalSlides === 0 || currentSlideIndex <= 0}
            class="h-8 px-2.5 rounded-lg bg-neutral-800 hover:bg-neutral-700 disabled:opacity-30 disabled:pointer-events-none text-neutral-200 font-bold text-xs flex items-center gap-1 cursor-pointer"
          >
            <ChevronLeft size={14} />
            <span>Ant.</span>
          </button>
          <button
            type="button"
            onclick={handleNextSlide}
            disabled={totalSlides === 0 || currentSlideIndex >= totalSlides - 1}
            class="h-8 px-2.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-30 disabled:pointer-events-none text-white font-bold text-xs flex items-center gap-1 cursor-pointer"
          >
            <span>Próx.</span>
            <ChevronRight size={14} />
          </button>
        </div>
      </div>
    {:else}
      <!-- Modo Expandido: Prévia do Slide Ativo e Botões Gigantes para o Polegar -->
      <div class="p-2.5 rounded-xl bg-black/60 border border-neutral-800/80 mb-2 min-h-[52px] flex items-center justify-center text-center">
        {#if currentSlide}
          <p class="text-xs font-semibold leading-relaxed line-clamp-2 text-neutral-200">
            {currentSlide.content.replace(/\n/g, ' ')}
          </p>
        {:else}
          <span class="text-xs text-neutral-500 italic">
            {isBlackout ? 'Projeção em Blackout' : isClear ? 'Texto ocultado' : 'Nenhum slide em exibição'}
          </span>
        {/if}
      </div>

      <div class="grid grid-cols-2 gap-2">
        <button
          type="button"
          onclick={handlePrevSlide}
          disabled={totalSlides === 0 || currentSlideIndex <= 0}
          class="h-12 rounded-xl bg-neutral-800 hover:bg-neutral-700 active:bg-neutral-600 disabled:opacity-40 disabled:pointer-events-none text-neutral-200 font-bold text-sm flex items-center justify-center gap-1.5 transition-all shadow active:scale-95 cursor-pointer"
        >
          <ChevronLeft size={18} />
          <span>Anterior</span>
        </button>

        <button
          type="button"
          onclick={handleNextSlide}
          disabled={totalSlides === 0 || currentSlideIndex >= totalSlides - 1}
          class="h-12 rounded-xl bg-emerald-600 hover:bg-emerald-500 active:bg-emerald-700 disabled:opacity-40 disabled:pointer-events-none text-white font-extrabold text-sm flex items-center justify-center gap-1.5 transition-all shadow-lg shadow-emerald-950 active:scale-95 cursor-pointer"
        >
          <span>Próximo</span>
          <ChevronRight size={18} />
        </button>
      </div>
    {/if}
  </section>
  {/if}

  <!-- CONTEÚDO PRINCIPAL: Scrollável conforme a aba selecionada -->
  <main class="flex-1 overflow-y-auto overflow-x-hidden min-h-0 bg-neutral-950 p-2.5">
    <!-- ABA 0: INÍCIO / HOME HUB -->
    {#if activeTab === 'home'}
      <div class="space-y-3.5 pb-2">
        <!-- HERO PROFILE CARD: Identidade do voluntário com shape moderno -->
        <div class="rounded-3xl p-4 bg-gradient-to-b from-neutral-900 to-neutral-900/90 border border-neutral-800 shadow-xl relative overflow-hidden">
          <div
            class="absolute top-0 left-0 right-0 h-1"
            style="background: linear-gradient(90deg, {$remoteAuthStore.color}, transparent);"
          ></div>

          <div class="flex items-center justify-between gap-3">
            <div class="flex items-center gap-3 min-w-0">
              <div
                class="w-12 h-12 rounded-2xl flex items-center justify-center text-sm font-extrabold text-white shrink-0 border border-white/20 shadow-lg"
                style="background-color: {$remoteAuthStore.color}; box-shadow: 0 4px 14px {$remoteAuthStore.color}40;"
              >
                {getInitials($remoteAuthStore.name)}
              </div>
              <div class="min-w-0">
                <span class="text-xs text-neutral-400 block font-medium">Bem-vindo(a),</span>
                <h1 class="text-base font-bold text-white truncate">
                  {$remoteAuthStore.name}
                </h1>
                <div class="flex items-center gap-1.5 mt-0.5">
                  {#if $remoteAuthStore.role === 'leader'}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-extrabold bg-amber-500/15 text-amber-300 border border-amber-500/30">
                      <Crown size={11} />
                      <span>Líder da Equipe</span>
                    </span>
                  {:else if $remoteAuthStore.role === 'vice_leader'}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-extrabold bg-sky-500/15 text-sky-300 border border-sky-500/30">
                      <Shield size={11} />
                      <span>Vice-Líder</span>
                    </span>
                  {:else}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-bold bg-neutral-800 text-neutral-300 border border-neutral-700">
                      <User size={11} />
                      <span>Voluntário</span>
                    </span>
                  {/if}
                </div>
              </div>
            </div>

            <!-- Botão / Status de Operador -->
            <div class="shrink-0 flex flex-col items-end gap-1">
              {#if canControlRemote}
                <span class="inline-flex items-center gap-1 px-2.5 py-1 rounded-xl text-[10px] font-extrabold bg-emerald-500/15 text-emerald-400 border border-emerald-500/30 shadow-sm">
                  <MonitorPlay size={12} />
                  <span>Operador Ativo</span>
                </span>
              {:else}
                <button
                  type="button"
                  onclick={() => {
                    isOperatorUnlockModalOpen = true;
                    operatorPinInput = '';
                    operatorPinError = '';
                  }}
                  class="inline-flex items-center gap-1 px-2.5 py-1 rounded-xl text-[10px] font-extrabold bg-amber-500/15 text-amber-300 border border-amber-500/30 hover:bg-amber-500/25 active:scale-95 transition-all cursor-pointer"
                  title="Desbloquear permissão de operador de projeção"
                >
                  <Lock size={12} />
                  <span>Desbloquear PIN</span>
                </button>
              {/if}
            </div>
          </div>
        </div>

        <!-- SEÇÃO: MINHA ESCALA NO CULTO ATIVO -->
        <div class="space-y-1.5">
          <div class="flex items-center justify-between px-1">
            <h2 class="text-xs font-bold uppercase tracking-wider text-neutral-400 flex items-center gap-1.5">
              <Calendar size={13} class="text-emerald-400" />
              <span>Minha Escala</span>
            </h2>
            {#if activeRoster}
              <button
                type="button"
                onclick={() => (activeTab = 'team')}
                class="text-[11px] font-bold text-neutral-400 hover:text-emerald-400 transition-colors flex items-center gap-0.5 cursor-pointer"
              >
                <span>Ver equipe</span>
                <ChevronRight size={13} />
              </button>
            {/if}
          </div>

          {#if activeRoster}
            {#if myRosterItem}
              <div class="p-3.5 rounded-2xl bg-neutral-900 border border-neutral-800 shadow-md space-y-3">
                <div class="flex items-start justify-between gap-2">
                  <div>
                    <h3 class="text-sm font-bold text-white">
                      {activeRoster.title}
                    </h3>
                    <p class="text-[11px] text-neutral-400">
                      {activeRoster.notes || 'Escala oficial do culto'}
                    </p>
                  </div>

                  <div class="shrink-0 flex items-center gap-1.5">
                    {#if myDepartment}
                      <span
                        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-xl text-xs font-bold text-white border"
                        style="background-color: {myDepartment.color}25; border-color: {myDepartment.color}60;"
                      >
                        <span class="w-2 h-2 rounded-full" style="background-color: {myDepartment.color};"></span>
                        <span>{myDepartment.name}</span>
                      </span>
                    {/if}
                    <span class="px-2 py-1 rounded-xl text-xs font-bold bg-neutral-800 text-neutral-300 border border-neutral-700">
                      {myRosterItem.role === 'titular' ? 'Titular' : 'Auxiliar'}
                    </span>
                  </div>
                </div>

                <!-- Confirmação de Presença em 1 Toque -->
                <div class="p-2.5 rounded-xl bg-neutral-950/80 border border-neutral-800/80 space-y-2">
                  <div class="flex items-center justify-between text-xs">
                    <span class="text-neutral-400 font-medium">Sua presença:</span>
                    {#if myRosterItem.status === 'confirmed'}
                      <span class="inline-flex items-center gap-1 font-bold text-emerald-400">
                        <CheckCircle2 size={13} />
                        <span>Confirmado</span>
                      </span>
                    {:else if myRosterItem.status === 'declined'}
                      <span class="inline-flex items-center gap-1 font-bold text-rose-400">
                        <XCircle size={13} />
                        <span>Ausente</span>
                      </span>
                    {:else}
                      <span class="inline-flex items-center gap-1 font-bold text-amber-400">
                        <Clock size={13} />
                        <span>Aguardando resposta</span>
                      </span>
                    {/if}
                  </div>

                  <div class="grid grid-cols-2 gap-2 pt-0.5">
                    <button
                      type="button"
                      onclick={() => handleSetMyStatus('confirmed')}
                      class="py-2.5 px-3 rounded-xl {myRosterItem.status === 'confirmed' ? 'bg-emerald-600 text-white font-extrabold shadow-md shadow-emerald-950' : 'bg-neutral-800 hover:bg-neutral-750 text-neutral-300 border border-neutral-700'} text-xs flex items-center justify-center gap-1.5 transition-all active:scale-95 cursor-pointer"
                    >
                      <CheckCircle2 size={14} class={myRosterItem.status === 'confirmed' ? 'text-white' : 'text-emerald-400'} />
                      <span>Vou Comparecer</span>
                    </button>

                    <button
                      type="button"
                      onclick={() => handleSetMyStatus('declined')}
                      class="py-2.5 px-3 rounded-xl {myRosterItem.status === 'declined' ? 'bg-rose-900/60 border border-rose-700 text-rose-200 font-bold' : 'bg-neutral-800 hover:bg-neutral-750 text-neutral-400 border border-neutral-700'} text-xs flex items-center justify-center gap-1.5 transition-all active:scale-95 cursor-pointer"
                    >
                      <XCircle size={14} class={myRosterItem.status === 'declined' ? 'text-rose-300' : 'text-neutral-500'} />
                      <span>Não Posso Ir</span>
                    </button>
                  </div>
                </div>

                <!-- Parceiros de Departamento -->
                {#if rosterDepartmentPartners.length > 0}
                  <div class="pt-1 border-t border-neutral-800/80 text-[11px] text-neutral-400 flex items-center gap-1.5 flex-wrap">
                    <span class="text-neutral-500 font-medium">Com você:</span>
                    {#each rosterDepartmentPartners as partner}
                      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-lg bg-neutral-800/80 text-neutral-200 text-[10px] font-semibold border border-neutral-700/60">
                        <span class="w-1.5 h-1.5 rounded-full" style="background-color: {partner.member?.color || '#94a3b8'};"></span>
                        <span>{partner.name}</span>
                        <span class="text-neutral-400 text-[9px]">({partner.item.role === 'titular' ? 'Titular' : 'Aux'})</span>
                      </span>
                    {/each}
                  </div>
                {/if}
              </div>
            {:else}
              <div class="p-4 rounded-2xl bg-neutral-900/60 border border-neutral-800/80 text-center space-y-2">
                <p class="text-xs text-neutral-400">
                  Você não está escalado para o culto ativo (<strong class="text-neutral-200">{activeRoster.title}</strong>).
                </p>
                <button
                  type="button"
                  onclick={() => (activeTab = 'team')}
                  class="inline-flex items-center gap-1 text-xs font-bold text-emerald-400 hover:text-emerald-300 cursor-pointer"
                >
                  <span>Ver escala completa da equipe</span>
                  <ArrowRight size={13} />
                </button>
              </div>
            {/if}
          {:else}
            <div class="p-4 rounded-2xl bg-neutral-900/60 border border-neutral-800/80 text-center text-xs text-neutral-500">
              Nenhuma escala ativa cadastrada para hoje.
            </div>
          {/if}
        </div>

        <!-- SEÇÃO: RECURSOS & ATALHOS RÁPIDOS -->
        <div class="space-y-2">
          <h2 class="text-xs font-bold uppercase tracking-wider text-neutral-400 flex items-center gap-1.5 px-0.5 pt-1">
            <Sparkles size={13} class="text-emerald-400" />
            <span>Acesso Rápido</span>
          </h2>

          <!-- Card Controle de Mídia / Projeção -->
          <button
            type="button"
            onclick={() => {
              if (canControlRemote) {
                activeTab = 'setlist';
              } else {
                isOperatorUnlockModalOpen = true;
                operatorPinInput = '';
                operatorPinError = '';
              }
            }}
            class="w-full p-3.5 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-emerald-500/40 active:scale-[0.99] transition-all cursor-pointer flex items-center justify-between gap-3 text-left group"
          >
            <div class="flex items-center gap-3 min-w-0">
              <div
                class="w-11 h-11 rounded-xl {canControlRemote
                  ? 'bg-emerald-500/15 border-emerald-500/30 text-emerald-400'
                  : 'bg-amber-500/15 border-amber-500/30 text-amber-400'} border flex items-center justify-center shrink-0 group-hover:scale-105 transition-transform"
              >
                {#if canControlRemote}
                  <MonitorPlay size={22} />
                {:else}
                  <Lock size={20} />
                {/if}
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <span class="text-xs font-bold text-white group-hover:text-emerald-300 transition-colors">
                    Controle de Apresentação
                  </span>
                  {#if canControlRemote}
                    <span class="px-1.5 py-0.2 rounded text-[9px] font-bold bg-emerald-950 text-emerald-400 border border-emerald-800">
                      OPERADOR
                    </span>
                  {:else}
                    <span class="px-1.5 py-0.2 rounded text-[9px] font-bold bg-amber-950/80 text-amber-300 border border-amber-800/80">
                      BLOQUEADO
                    </span>
                  {/if}
                </div>
                <p class="text-[11px] text-neutral-400 truncate mt-0.5">
                  {#if canControlRemote}
                    {isLive ? `● Ao vivo: ${currentMediaTitle}` : 'Navegar por setlist, slides e letras'}
                  {:else}
                    Acesso restrito a operadores · Toque para desbloquear
                  {/if}
                </p>
              </div>
            </div>
            {#if canControlRemote}
              <ArrowRight size={16} class="text-neutral-500 group-hover:text-emerald-400 group-hover:translate-x-0.5 transition-all shrink-0" />
            {:else}
              <Lock size={15} class="text-amber-400 shrink-0" />
            {/if}
          </button>

          <!-- Card Escala da Equipe -->
          <button
            type="button"
            onclick={() => (activeTab = 'team')}
            class="w-full p-3.5 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-sky-500/40 active:scale-[0.99] transition-all cursor-pointer flex items-center justify-between gap-3 text-left group"
          >
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-11 h-11 rounded-xl bg-sky-500/15 border border-sky-500/30 text-sky-400 flex items-center justify-center shrink-0 group-hover:scale-105 transition-transform">
                <Users size={22} />
              </div>
              <div class="min-w-0">
                <span class="text-xs font-bold text-white group-hover:text-sky-300 transition-colors">
                  Escala da Equipe
                </span>
                <p class="text-[11px] text-neutral-400 truncate mt-0.5">
                  {activeRoster ? `${activeRoster.items.length} voluntários escalados no culto` : 'Ver escala e departamentos'}
                </p>
              </div>
            </div>
            <ArrowRight size={16} class="text-neutral-500 group-hover:text-sky-400 group-hover:translate-x-0.5 transition-all shrink-0" />
          </button>

          <!-- Card Chat da Equipe (Teaser / Em Breve) -->
          <button
            type="button"
            onclick={() => (isChatInfoModalOpen = true)}
            class="w-full p-3.5 rounded-2xl bg-neutral-900 border border-neutral-800/80 hover:border-violet-500/40 active:scale-[0.99] transition-all cursor-pointer flex items-center justify-between gap-3 text-left group"
          >
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-11 h-11 rounded-xl bg-violet-500/15 border border-violet-500/30 text-violet-400 flex items-center justify-center shrink-0 group-hover:scale-105 transition-transform">
                <MessageSquare size={22} />
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <span class="text-xs font-bold text-white group-hover:text-violet-300 transition-colors">
                    Chat da Equipe
                  </span>
                  <span class="px-1.5 py-0.5 rounded-full text-[9px] font-extrabold bg-gradient-to-r from-violet-500/20 to-fuchsia-500/20 text-violet-300 border border-violet-500/30 uppercase tracking-wide">
                    Em breve
                  </span>
                </div>
                <p class="text-[11px] text-neutral-400 truncate mt-0.5">
                  Mensagens instantâneas e avisos da liderança
                </p>
              </div>
            </div>
            <Sparkles size={16} class="text-violet-400 shrink-0" />
          </button>

          <!-- 2 Colunas: Músicas & Bíblia (Apenas se possui permissão ou desbloqueado) -->
          {#if canControlRemote}
            <div class="grid grid-cols-2 gap-2 pt-0.5">
              <button
                type="button"
                onclick={() => (activeTab = 'songs')}
                class="p-3 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-neutral-700 active:scale-95 text-left transition-all cursor-pointer flex flex-col justify-between h-20"
              >
                <div class="flex items-center justify-between">
                  <Music size={18} class="text-emerald-400" />
                  <ArrowRight size={13} class="text-neutral-500" />
                </div>
                <div>
                  <span class="text-xs font-bold text-white block">Músicas</span>
                  <span class="text-[10px] text-neutral-400">{songsList.length} cadastradas</span>
                </div>
              </button>

              <button
                type="button"
                onclick={() => (activeTab = 'bible')}
                class="p-3 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-neutral-700 active:scale-95 text-left transition-all cursor-pointer flex flex-col justify-between h-20"
              >
                <div class="flex items-center justify-between">
                  <BookOpen size={18} class="text-emerald-400" />
                  <ArrowRight size={13} class="text-neutral-500" />
                </div>
                <div>
                  <span class="text-xs font-bold text-white block">Bíblia Sagrada</span>
                  <span class="text-[10px] text-neutral-400 font-mono">{activeBibleAbbr}</span>
                </div>
              </button>
            </div>
          {/if}
        </div>
      </div>

    <!-- ABA 1: SETLIST -->
    {:else if activeTab === 'setlist'}
      <div class="space-y-2">
        {#if !canControlRemote}
          <div class="p-2.5 rounded-xl bg-amber-500/10 border border-amber-500/25 flex items-center justify-between text-xs text-amber-300">
            <div class="flex items-center gap-2 min-w-0 pr-2">
              <Lock size={14} class="shrink-0 text-amber-400" />
              <span class="truncate">Modo Consulta — Projeção restrita a operadores</span>
            </div>
            <button
              type="button"
              onclick={() => {
                isOperatorUnlockModalOpen = true;
                operatorPinInput = '';
                operatorPinError = '';
              }}
              class="px-2 py-1 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-200 font-bold text-[11px] shrink-0 cursor-pointer"
            >
              Desbloquear
            </button>
          </div>
        {/if}
        <div class="flex items-center justify-between px-1 text-xs text-neutral-400 font-medium">
          <!-- Botão para alternar entre setlists se houver mais de uma -->
          <button
            type="button"
            onclick={() => (isSetlistDrawerOpen = true)}
            class="flex items-center gap-1 font-bold text-neutral-200 hover:text-white transition-colors cursor-pointer"
          >
            <span class="truncate max-w-[240px]">{activeSetlistData?.title || 'Setlist Ativa'}</span>
            <ChevronDown size={14} class="text-neutral-400" />
          </button>

          <button
            type="button"
            onclick={loadSetlistsAndActive}
            class="p-1 hover:text-white transition-colors cursor-pointer"
            title="Atualizar Setlist"
            aria-label="Atualizar Setlist"
          >
            <RefreshCw size={13} class={isLoadingSetlist ? 'animate-spin' : ''} />
          </button>
        </div>

        <!-- Banner de Playback Ativo Global -->
        {#if audioPlayback && (audioPlayback.is_playing || audioPlayback.current_time_seconds > 0)}
          <div class="p-2.5 rounded-xl bg-emerald-950/60 border border-emerald-500/50 flex items-center justify-between gap-2 text-xs shadow-md">
            <div class="flex items-center gap-2 min-w-0 flex-1">
              <div class="w-7 h-7 rounded-lg bg-emerald-500/20 text-emerald-400 flex items-center justify-center shrink-0">
                <Volume2 size={14} class={audioPlayback.is_playing ? 'animate-pulse' : ''} />
              </div>
              <div class="min-w-0 truncate">
                <p class="font-bold text-white text-xs truncate">{audioPlayback.title || 'Playback'}</p>
                <p class="font-mono text-[10px] text-emerald-400">
                  {formatDuration(audioPlayback.current_time_seconds)} / {formatDuration(audioPlayback.duration_seconds)}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-1.5 shrink-0">
              <button
                type="button"
                onclick={() => {
                  if (audioPlayback?.is_playing) {
                    controlAudioPlayback('pause');
                  } else {
                    controlAudioPlayback('play', audioPlayback?.url, audioPlayback?.title, audioPlayback?.artist_or_ref);
                  }
                }}
                class="p-1.5 rounded-lg bg-emerald-500 text-black font-bold cursor-pointer"
                title={audioPlayback.is_playing ? "Pausar" : "Tocar"}
              >
                {#if audioPlayback.is_playing}
                  <Pause size={12} class="fill-current" />
                {:else}
                  <Play size={12} class="fill-current" />
                {/if}
              </button>
              <button
                type="button"
                onclick={() => controlAudioPlayback('stop')}
                class="p-1.5 rounded-lg bg-neutral-800 text-neutral-300 hover:text-white cursor-pointer"
                title="Parar"
              >
                <Square size={12} class="fill-current" />
              </button>
            </div>
          </div>
        {/if}

        {#if !activeSetlistData || (activeSetlistData.items ?? []).length === 0}
          <div class="p-8 text-center text-neutral-500 text-xs">
            <ListMusic size={32} class="mx-auto mb-2 opacity-40" />
            <p>Nenhum item na setlist ativa.</p>
            <p class="text-[10px] mt-1 text-neutral-600">
              Adicione músicas ou passagens pelo painel do operador ou selecione outra setlist.
            </p>
          </div>
        {:else}
          <div class="space-y-1.5">
            {#each activeSetlistData.items as item, itemIdx}
              {#if item.kind === 'header'}
                <!-- Divisor de Seção na Setlist (Conforme no Painel do Operador) -->
                <div class="pt-3 pb-1 px-1 flex items-center justify-between gap-2 select-none" role="separator">
                  <div class="flex items-center gap-1.5 min-w-0">
                    <div class="w-4 h-4 rounded text-emerald-400 flex items-center justify-center shrink-0">
                      <Bookmark size={13} />
                    </div>
                    <span class="text-[11px] font-bold uppercase tracking-wider text-neutral-400 truncate">
                      {item.title}
                    </span>
                  </div>
                  <div class="h-[1px] flex-1 bg-neutral-800/80"></div>
                </div>
              {:else}
                {@const mediaIndex = activeSetlistData.items.slice(0, itemIdx).filter((i) => i.kind !== 'header').length + 1}
                {@const isItemActive = $presentationState.active_media_id === item.id}
                {@const isExpanded = expandedSetlistItemId === item.id}
                <div
                  class="rounded-xl border transition-all overflow-hidden {isItemActive ? 'bg-neutral-900 border-emerald-500 shadow-lg shadow-emerald-950/20' : 'bg-neutral-900/60 border-neutral-800/80'}"
                >
                  <!-- Cabeçalho do item da setlist -->
                  <button
                    type="button"
                    onclick={() => {
                      expandedSetlistItemId = isExpanded ? null : item.id;
                    }}
                    class="w-full px-3 py-2.5 flex items-center justify-between text-left cursor-pointer"
                  >
                    <div class="flex items-center gap-2 min-w-0 pr-2">
                      <span
                        class="w-6 h-6 rounded-full text-xs font-bold flex items-center justify-center shrink-0 {isItemActive ? 'bg-emerald-500 text-black shadow' : 'bg-neutral-800 text-neutral-400'}"
                      >
                        {mediaIndex}
                      </span>
                      <div class="truncate">
                        <p class="text-xs font-bold truncate text-neutral-200 {isItemActive ? 'text-emerald-400' : ''}">
                          {item.title}
                        </p>
                        {#if item.artist_or_ref}
                          <p class="text-[10px] text-neutral-400 truncate">{item.artist_or_ref}</p>
                        {/if}
                      </div>
                    </div>

                    <div class="flex items-center gap-1.5 shrink-0">
                      {#if item.audio_url}
                        <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-semibold bg-emerald-950/70 text-emerald-400 border border-emerald-500/40">
                          <Music size={10} />
                          Playback
                        </span>
                      {/if}
                      {#if isItemActive}
                        <span class="px-1.5 py-0.5 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40 animate-pulse">
                          Ao Vivo
                        </span>
                      {/if}
                      <span class="text-[10px] text-neutral-500 font-mono">
                        {item.slides.length} slides
                      </span>
                    </div>
                  </button>

                  <!-- Slides expandidos com toque direto para projetar -->
                  {#if isExpanded}
                    <div class="px-2 pb-2 pt-1 border-t border-neutral-800/60 space-y-1 bg-black/40">
                      {#if item.audio_url}
                        {@const isThisTrackPlaying = audioPlayback?.is_playing && (audioPlayback?.url === item.audio_url || audioPlayback?.title === item.title)}
                        {@const cur = audioPlayback?.url === item.audio_url ? (audioPlayback?.current_time_seconds || 0) : 0}
                        {@const dur = audioPlayback?.url === item.audio_url ? (audioPlayback?.duration_seconds || 1) : 1}
                        <div class="mb-2 p-2.5 rounded-lg bg-emerald-950/40 border border-emerald-500/40 flex flex-col gap-2">
                          <div class="flex items-center justify-between text-xs">
                            <div class="flex items-center gap-1.5 text-emerald-300 font-bold truncate">
                              <Music size={13} class="text-emerald-400 shrink-0" />
                              <span class="truncate">Playback de Áudio</span>
                            </div>
                            <div class="font-mono text-[10px] text-neutral-300 tabular-nums">
                              {formatDuration(cur)} / {formatDuration(dur)}
                            </div>
                          </div>

                          <!-- Barra de progresso -->
                          <div class="w-full h-1.5 bg-neutral-800 rounded-full overflow-hidden">
                            <div
                              class="h-full bg-emerald-500 transition-all"
                              style="width: {dur > 0 ? Math.min(100, Math.max(0, (cur / dur) * 100)) : 0}%;"
                            ></div>
                          </div>

                          <div class="flex items-center justify-between gap-2 pt-0.5">
                            <button
                              type="button"
                              onclick={(e) => {
                                e.stopPropagation();
                                if (isThisTrackPlaying) {
                                  controlAudioPlayback('pause');
                                } else {
                                  controlAudioPlayback('play', item.audio_url, item.title, item.artist_or_ref);
                                }
                              }}
                              class="flex-1 py-1.5 px-3 rounded-lg font-bold text-xs flex items-center justify-center gap-1.5 transition-colors cursor-pointer {isThisTrackPlaying ? 'bg-amber-500/20 text-amber-300 border border-amber-500/40' : 'bg-emerald-500 text-black shadow'}"
                            >
                              {#if isThisTrackPlaying}
                                <Pause size={13} class="fill-current" />
                                <span>Pausar Playback</span>
                              {:else}
                                <Play size={13} class="fill-current" />
                                <span>Tocar Playback</span>
                              {/if}
                            </button>

                            {#if isThisTrackPlaying || audioPlayback?.url === item.audio_url}
                              <button
                                type="button"
                                onclick={(e) => {
                                  e.stopPropagation();
                                  controlAudioPlayback('stop');
                                }}
                                class="p-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white transition-colors cursor-pointer shrink-0"
                                title="Parar Playback"
                              >
                                <Square size={13} class="fill-current" />
                              </button>
                            {/if}
                          </div>
                        </div>
                      {/if}
                      {#each item.slides as slide, sIdx}
                        {@const isCurrentSlideLive = isItemActive && currentSlideIndex === sIdx && isLive}
                        <button
                          type="button"
                          onclick={() => handleSelectSlide(item.id, sIdx)}
                          class="w-full text-left p-2.5 rounded-lg text-xs flex items-start gap-2 transition-all cursor-pointer {isCurrentSlideLive ? 'bg-emerald-600/30 border border-emerald-500 text-white font-semibold ring-1 ring-emerald-500' : 'bg-neutral-900/80 hover:bg-neutral-800 text-neutral-300 border border-neutral-800/40'}"
                        >
                          <span
                            class="text-[9px] px-1.5 py-0.5 rounded font-mono shrink-0 mt-0.5 {isCurrentSlideLive ? 'bg-emerald-500 text-black font-extrabold' : 'bg-neutral-800 text-neutral-400'}"
                          >
                            {slide.label || `${sIdx + 1}`}
                          </span>
                          <p class="line-clamp-2 leading-relaxed flex-1">
                            {slide.content.replace(/\n/g, ' ')}
                          </p>
                        </button>
                      {/each}
                    </div>
                  {/if}
                </div>
              {/if}
            {/each}
          </div>
        {/if}
      </div>

    <!-- ABA 2: MÚSICAS -->
    {:else if activeTab === 'songs'}
      <div class="space-y-2">
        {#if !canControlRemote}
          <div class="p-2.5 rounded-xl bg-amber-500/10 border border-amber-500/25 flex items-center justify-between text-xs text-amber-300">
            <div class="flex items-center gap-2 min-w-0 pr-2">
              <Lock size={14} class="shrink-0 text-amber-400" />
              <span class="truncate">Modo Leitura — Desbloqueie para disparar letras</span>
            </div>
            <button
              type="button"
              onclick={() => {
                isOperatorUnlockModalOpen = true;
                operatorPinInput = '';
                operatorPinError = '';
              }}
              class="px-2 py-1 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-200 font-bold text-[11px] shrink-0 cursor-pointer"
            >
              Desbloquear
            </button>
          </div>
        {/if}
        <!-- Barra de busca de músicas com botão de recarregar -->
        <div class="flex items-center gap-1.5">
          <div class="relative flex-1">
            <Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400 pointer-events-none" />
            <input
              type="text"
              value={searchQuery}
              oninput={handleSongSearchInput}
              placeholder="Buscar por título ou trecho da letra..."
              class="w-full pl-8 pr-8 py-2.5 rounded-xl bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder:text-neutral-500 focus:outline-none focus:border-emerald-500 transition-colors"
            />
            {#if searchQuery}
              <button
                type="button"
                onclick={() => {
                  searchQuery = '';
                  loadSongs('');
                }}
                class="absolute right-2.5 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-white p-1 cursor-pointer"
              >
                <X size={13} />
              </button>
            {/if}
          </div>

          <button
            type="button"
            onclick={() => loadSongs(searchQuery)}
            class="h-10 w-10 rounded-xl bg-neutral-900 border border-neutral-800 hover:bg-neutral-800 active:scale-95 text-neutral-300 hover:text-white flex items-center justify-center shrink-0 transition-all cursor-pointer"
            title="Recarregar catálogo de músicas"
            aria-label="Recarregar catálogo de músicas"
          >
            <RefreshCw size={15} class={isLoadingSongs ? 'animate-spin text-emerald-400' : ''} />
          </button>
        </div>

        <!-- Visualização detalhada da música selecionada -->
        {#if isLoadingSongDetail}
          <div class="p-8 text-center text-neutral-500 text-xs">
            <RefreshCw size={20} class="animate-spin mx-auto mb-2" />
            <p>Carregando letra...</p>
          </div>
        {:else if selectedSong}
          {@const songSlides = songToSlides(selectedSong)}
          {@const isSongLive = $presentationState.active_media_id === selectedSong.id && isLive}
          <div class="p-3 rounded-xl bg-neutral-900 border border-neutral-800 space-y-2.5 animate-in fade-in duration-150">
            <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
              <div>
                <div class="flex items-center gap-2">
                  <h3 class="text-sm font-bold text-neutral-100">{selectedSong.title}</h3>
                  {#if isSongLive}
                    <span class="px-1.5 py-0.2 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40 animate-pulse">
                      Ao Vivo
                    </span>
                  {/if}
                </div>
                {#if selectedSong.artist}
                  <p class="text-xs text-neutral-400">{selectedSong.artist}</p>
                {/if}
              </div>
              <button
                type="button"
                onclick={() => (selectedSong = null)}
                class="p-1.5 rounded-lg bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
              >
                <X size={14} />
              </button>
            </div>

            <!-- Botão Apresentar Música Completa -->
            <button
              type="button"
              onclick={() => presentSong(selectedSong!)}
              class="w-full py-2.5 rounded-xl text-white font-bold text-xs flex items-center justify-center gap-1.5 shadow-lg transition-colors cursor-pointer {isSongLive ? 'bg-emerald-700 hover:bg-emerald-600 shadow-emerald-950' : 'bg-emerald-600 hover:bg-emerald-500 shadow-emerald-950'}"
            >
              <Play size={14} />
              <span>{isSongLive ? 'Reiniciar Música do Início' : 'Apresentar Música Completa'}</span>
            </button>

            <!-- Slides / Estrofes da música com Active State -->
            <div class="space-y-1 pt-1">
              <span class="text-[10px] font-semibold text-neutral-400 uppercase tracking-wider">
                Toque em uma estrofe para projetar:
              </span>
              {#each songSlides as slide, sIdx}
                {@const isSlideLive = isSongLive && currentSlideIndex === sIdx}
                <button
                  type="button"
                  onclick={() => presentSong(selectedSong!, sIdx)}
                  class="w-full text-left p-2.5 rounded-xl border transition-all cursor-pointer {isSlideLive ? 'bg-emerald-950/50 border-emerald-500 ring-1 ring-emerald-500 text-white font-semibold' : 'bg-black/40 hover:bg-neutral-800 text-neutral-200 border-neutral-800/80'}"
                >
                  <div class="flex items-center justify-between mb-1">
                    <span
                      class="text-[9px] font-bold px-1.5 py-0.5 rounded {isSlideLive ? 'bg-emerald-500 text-black font-extrabold' : 'bg-neutral-800 text-neutral-400'}"
                    >
                      {slide.label}
                    </span>
                    {#if isSlideLive}
                      <span class="text-[9px] font-bold text-emerald-400 animate-pulse">Ao Vivo</span>
                    {/if}
                  </div>
                  <p class="leading-relaxed text-xs text-neutral-300">
                    {slide.content}
                  </p>
                </button>
              {/each}
            </div>
          </div>
        {:else}
          <!-- Lista de músicas filtradas -->
          <div class="space-y-1.5">
            {#if isLoadingSongs}
              <div class="p-8 text-center text-neutral-500 text-xs">
                <RefreshCw size={20} class="animate-spin mx-auto mb-2" />
                <p>Carregando músicas...</p>
              </div>
            {:else if songsList.length === 0}
              <div class="p-8 text-center text-neutral-500 text-xs">
                <Music size={24} class="mx-auto mb-2 opacity-40" />
                <p>Nenhuma música encontrada.</p>
              </div>
            {:else}
              {#each songsList as song}
                {@const isSongActive = $presentationState.active_media_id === song.id && isLive}
                <button
                  type="button"
                  onclick={() => openSongDetail(song)}
                  class="w-full px-3 py-2.5 rounded-xl border flex items-center justify-between text-left transition-colors cursor-pointer {isSongActive ? 'bg-neutral-900 border-emerald-500 shadow-md' : 'bg-neutral-900/80 hover:bg-neutral-850 active:bg-neutral-800 border-neutral-800/60'}"
                >
                  <div class="min-w-0 pr-2">
                    <p class="text-xs font-bold truncate {isSongActive ? 'text-emerald-400' : 'text-neutral-200'}">
                      {song.title}
                    </p>
                    {#if song.artist}
                      <p class="text-[10px] text-neutral-400 truncate">{song.artist}</p>
                    {/if}
                  </div>
                  <div class="flex items-center gap-1.5 shrink-0">
                    {#if isSongActive}
                      <span class="px-1.5 py-0.5 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40">
                        Ao Vivo
                      </span>
                    {/if}
                    <span class="text-[10px] text-neutral-500 font-mono">
                      Abrir
                    </span>
                  </div>
                </button>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

    <!-- ABA 3: BÍBLIA -->
    {:else if activeTab === 'bible'}
      <div class="space-y-2.5">
        {#if !canControlRemote}
          <div class="p-2.5 rounded-xl bg-amber-500/10 border border-amber-500/25 flex items-center justify-between text-xs text-amber-300">
            <div class="flex items-center gap-2 min-w-0 pr-2">
              <Lock size={14} class="shrink-0 text-amber-400" />
              <span class="truncate">Modo Consulta — Desbloqueie para projetar versículos</span>
            </div>
            <button
              type="button"
              onclick={() => {
                isOperatorUnlockModalOpen = true;
                operatorPinInput = '';
                operatorPinError = '';
              }}
              class="px-2 py-1 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-200 font-bold text-[11px] shrink-0 cursor-pointer"
            >
              Desbloquear
            </button>
          </div>
        {/if}
        <!-- Input Group: Seletor de Versão (Drawer) + Campo de Busca Bíblica -->
        <div class="flex items-center rounded-xl bg-neutral-900 border border-neutral-800 focus-within:border-emerald-500 overflow-hidden shadow-sm">
          <!-- Botão de Tradução (abre Drawer do Shadcn) -->
          <button
            type="button"
            onclick={() => (isBibleDrawerOpen = true)}
            class="h-10 px-3 bg-neutral-850 hover:bg-neutral-800 text-neutral-200 font-bold text-xs flex items-center gap-1.5 border-r border-neutral-800 shrink-0 transition-colors cursor-pointer"
            title="Alternar Tradução Bíblica"
          >
            <span class="text-emerald-400 font-extrabold">{activeBibleAbbr}</span>
            <ChevronDown size={14} class="text-neutral-400" />
          </button>

          <!-- Input de Pesquisa -->
          <div class="relative flex-1 flex items-center">
            <button
              type="button"
              onclick={() => searchBible(bibleQuery)}
              class="absolute left-2.5 text-neutral-400 hover:text-white p-0.5 cursor-pointer transition-colors"
              title="Pesquisar"
              aria-label="Pesquisar"
            >
              <Search size={14} />
            </button>
            <input
              type="text"
              bind:value={bibleQuery}
              onkeydown={(e) => {
                if (e.key === 'Enter') searchBible(bibleQuery);
              }}
              placeholder="Ex: Jo 3:16 ou amor..."
              class="w-full h-10 pl-8 pr-8 bg-transparent text-xs text-neutral-100 placeholder:text-neutral-500 focus:outline-none"
            />
            {#if bibleQuery}
              <button
                type="button"
                onclick={() => {
                  bibleQuery = '';
                  bibleSearchResults = [];
                }}
                class="absolute right-2.5 p-1 text-neutral-400 hover:text-white cursor-pointer"
              >
                <X size={13} />
              </button>
            {/if}
          </div>
        </div>

        <!-- Resultados da pesquisa de versículos -->
        {#if isSearchingBible}
          <div class="p-6 text-center text-neutral-500 text-xs">
            <RefreshCw size={18} class="animate-spin mx-auto mb-1" />
            <span>Pesquisando na Bíblia...</span>
          </div>
        {:else if bibleSearchResults.length > 0}
          <div class="p-2 rounded-xl bg-neutral-900 border border-neutral-800 space-y-1">
            <div class="flex items-center justify-between pb-1 px-1 border-b border-neutral-800 text-[10px] text-neutral-400 font-semibold">
              <span>{bibleSearchResults.length} resultados encontrados</span>
              <button
                type="button"
                onclick={() => (bibleSearchResults = [])}
                class="text-neutral-400 hover:text-white cursor-pointer"
              >
                Fechar
              </button>
            </div>
            {#each bibleSearchResults as hit}
              <button
                type="button"
                onclick={() => presentSearchResult(hit)}
                class="w-full text-left p-2.5 rounded-lg bg-black/40 hover:bg-neutral-800 active:bg-neutral-700 text-xs border border-neutral-800/60 transition-colors cursor-pointer"
              >
                <div class="flex items-center justify-between font-bold text-emerald-400 text-xs mb-0.5">
                  <span>{hit.book_name} {hit.chapter}:{hit.verse}</span>
                  <span class="text-[9px] uppercase font-mono text-neutral-500">{hit.bible_version || activeBibleAbbr}</span>
                </div>
                <p class="text-neutral-300 leading-snug line-clamp-2">{hit.text}</p>
              </button>
            {/each}
          </div>
        {:else}
          <!-- NAVEGADOR BÍBLICO EM 3 SUB-ABAS: Livros | Capítulos | Versículos -->
          <div class="grid grid-cols-3 gap-1 p-0.5 rounded-xl bg-neutral-900 border border-neutral-800">
            <button
              type="button"
              onclick={() => (bibleNavTab = 'books')}
              class="py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer flex items-center justify-center gap-1 {bibleNavTab === 'books' ? 'bg-emerald-600 text-white shadow' : 'text-neutral-400 hover:text-white'}"
            >
              <span>Livros</span>
            </button>

            <button
              type="button"
              onclick={() => (bibleNavTab = 'chapters')}
              class="py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer flex items-center justify-center gap-1 {bibleNavTab === 'chapters' ? 'bg-emerald-600 text-white shadow' : 'text-neutral-400 hover:text-white'}"
            >
              <span>Capítulos</span>
            </button>

            <button
              type="button"
              onclick={() => (bibleNavTab = 'verses')}
              class="py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer flex items-center justify-center gap-1 {bibleNavTab === 'verses' ? 'bg-emerald-600 text-white shadow' : 'text-neutral-400 hover:text-white'}"
            >
              <span>Versículos</span>
            </button>
          </div>

          <!-- SUB-ABA 1: LIVROS EM GRADE -->
          {#if bibleNavTab === 'books'}
            <div class="space-y-2">
              <!-- Filtro de Testamento -->
              <div class="flex items-center gap-1">
                <button
                  type="button"
                  onclick={() => (testamentFilter = 'all')}
                  class="flex-1 py-1 text-[11px] font-bold rounded-lg transition-colors cursor-pointer {testamentFilter === 'all' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-900'}"
                >
                  Todos ({books.length})
                </button>
                <button
                  type="button"
                  onclick={() => (testamentFilter = 'ot')}
                  class="flex-1 py-1 text-[11px] font-bold rounded-lg transition-colors cursor-pointer {testamentFilter === 'ot' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-900'}"
                >
                  Antigo (39)
                </button>
                <button
                  type="button"
                  onclick={() => (testamentFilter = 'nt')}
                  class="flex-1 py-1 text-[11px] font-bold rounded-lg transition-colors cursor-pointer {testamentFilter === 'nt' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-900'}"
                >
                  Novo (27)
                </button>
              </div>

              <!-- Grade de Livros (3 colunas, botões altos confortáveis) -->
              <div class="grid grid-cols-3 gap-1.5 pb-2">
                {#each filteredBooks as book}
                  {@const isSelected = selectedBook?.id === book.id}
                  <button
                    type="button"
                    onclick={() => {
                      selectBook(book);
                      bibleNavTab = 'chapters';
                    }}
                    class="h-12 px-2 rounded-xl flex flex-col items-center justify-center border transition-all text-center cursor-pointer active:scale-95 {isSelected ? 'bg-emerald-600 border-emerald-500 text-white font-bold shadow' : 'bg-neutral-900 border-neutral-800 text-neutral-200 hover:bg-neutral-850'}"
                  >
                    <span class="text-xs font-semibold leading-tight line-clamp-1">{book.name}</span>
                    <span class="text-[9px] opacity-70 font-mono mt-0.5">{book.chapter_count} cap</span>
                  </button>
                {/each}
              </div>
            </div>

          <!-- SUB-ABA 2: CAPÍTULOS EM GRADE -->
          {:else if bibleNavTab === 'chapters'}
            <div class="space-y-2.5">
              <!-- Cabeçalho do Livro Selecionado -->
              <div class="flex items-center justify-between p-2.5 rounded-xl bg-neutral-900 border border-neutral-800">
                <div>
                  <span class="text-[10px] text-neutral-400 uppercase font-bold block">Livro Selecionado:</span>
                  <h4 class="text-sm font-bold text-white">{selectedBook?.name || 'Selecione um Livro'}</h4>
                </div>
                <button
                  type="button"
                  onclick={() => (bibleNavTab = 'books')}
                  class="px-2.5 py-1 text-xs font-bold rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
                >
                  Trocar Livro
                </button>
              </div>

              <span class="text-[10px] font-bold uppercase text-neutral-400 px-1 block">
                Selecione o Capítulo:
              </span>

              <!-- Grade de Capítulos (5 colunas com botões amplos para o toque) -->
              <div class="grid grid-cols-5 gap-2 pb-2">
                {#each Array.from({ length: selectedBook?.chapter_count || 1 }, (_, i) => i + 1) as ch}
                  {@const isSelected = selectedChapter === ch}
                  <button
                    type="button"
                    onclick={() => {
                      selectChapter(ch);
                      bibleNavTab = 'verses';
                    }}
                    class="h-12 rounded-xl flex items-center justify-center font-bold text-sm transition-all border cursor-pointer active:scale-95 {isSelected ? 'bg-emerald-600 border-emerald-500 text-white shadow-lg' : 'bg-neutral-900 border-neutral-800 text-neutral-200 hover:bg-neutral-850'}"
                  >
                    {ch}
                  </button>
                {/each}
              </div>
            </div>

          <!-- SUB-ABA 3: VERSÍCULOS COM TEXTO COMPLETO -->
          {:else if bibleNavTab === 'verses'}
            <div class="space-y-2">
              <!-- Navegação Rápida entre Capítulos -->
              <div class="flex items-center justify-between p-2 rounded-xl bg-neutral-900 border border-neutral-800">
                <button
                  type="button"
                  onclick={() => {
                    if (selectedChapter > 1) selectChapter(selectedChapter - 1);
                  }}
                  disabled={selectedChapter <= 1}
                  class="h-8 px-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 disabled:opacity-30 disabled:pointer-events-none text-neutral-200 text-xs font-bold flex items-center gap-1 cursor-pointer"
                >
                  <ChevronLeft size={14} />
                  <span>Cap. {selectedChapter - 1}</span>
                </button>

                <button
                  type="button"
                  onclick={() => (bibleNavTab = 'chapters')}
                  class="font-extrabold text-xs text-emerald-400 hover:underline flex items-center gap-1 cursor-pointer"
                >
                  <span>{selectedBook?.name} {selectedChapter}</span>
                  <ChevronDown size={12} />
                </button>

                <button
                  type="button"
                  onclick={() => {
                    if (selectedBook && selectedChapter < (selectedBook.chapter_count || 1)) {
                      selectChapter(selectedChapter + 1);
                    }
                  }}
                  disabled={!selectedBook || selectedChapter >= (selectedBook.chapter_count || 1)}
                  class="h-8 px-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 disabled:opacity-30 disabled:pointer-events-none text-neutral-200 text-xs font-bold flex items-center gap-1 cursor-pointer"
                >
                  <span>Cap. {selectedChapter + 1}</span>
                  <ChevronRight size={14} />
                </button>
              </div>

              <!-- Lista de Versículos -->
              <div class="space-y-1.5 pb-2">
                {#if isLoadingBible}
                  <div class="p-8 text-center text-neutral-500 text-xs">
                    <RefreshCw size={18} class="animate-spin mx-auto mb-1" />
                    <span>Carregando versículos...</span>
                  </div>
                {:else}
                  {#each verses as v, vIdx}
                    {@const isChapterLive =
                      !!selectedBook &&
                      ($presentationState.active_media_id ===
                        `scripture-${activeBibleId}-${selectedBook.id}-${selectedChapter}` ||
                        $presentationState.active_media_id ===
                          `scripture-${selectedBook.id}-${selectedChapter}` ||
                        $presentationState.active_media_id ===
                          `scripture-${selectedBook.id}-${selectedChapter}-${v.verse}`) &&
                      isLive}
                    {@const isVerseLive =
                      isChapterLive &&
                      (currentSlideIndex === vIdx ||
                        (!!selectedBook &&
                          $presentationState.active_media_id ===
                            `scripture-${selectedBook.id}-${selectedChapter}-${v.verse}`))}
                    <button
                      type="button"
                      onclick={() => presentVerse(v)}
                      class="w-full text-left p-3 rounded-xl border transition-all flex items-start gap-3 cursor-pointer active:scale-[0.99] {isVerseLive ? 'bg-emerald-950/60 border-emerald-500 ring-1 ring-emerald-500 text-white shadow-lg' : 'bg-neutral-900/70 hover:bg-neutral-800 active:bg-neutral-700 border-neutral-800/80'}"
                    >
                      <div class="flex flex-col items-center gap-1 shrink-0 mt-0.5">
                        <span
                          class="w-6 h-6 rounded-full font-bold text-xs flex items-center justify-center {isVerseLive ? 'bg-emerald-500 text-black' : 'bg-neutral-800 text-emerald-400'}"
                        >
                          {v.verse}
                        </span>
                        {#if isVerseLive}
                          <span class="text-[8px] font-bold text-emerald-400 uppercase tracking-tighter animate-pulse">
                            Ao Vivo
                          </span>
                        {/if}
                      </div>

                      <p class="text-xs text-neutral-200 leading-relaxed flex-1 {isVerseLive ? 'font-semibold text-white' : ''}">
                        {v.text}
                      </p>
                    </button>
                  {/each}
                {/if}
              </div>
            </div>
          {/if}
        {/if}
      </div>

    <!-- ABA 4: EQUIPE & ESCALA DO CULTO -->
    {:else if activeTab === 'team'}
      <div class="space-y-3">
        {#if isLoadingTeam}
          <div class="py-12 flex flex-col items-center justify-center text-neutral-400 gap-2">
            <RefreshCw size={20} class="animate-spin text-emerald-400" />
            <span class="text-xs">Carregando escala da equipe...</span>
          </div>
        {:else if !activeRoster}
          <div class="py-12 px-6 rounded-2xl bg-neutral-900 border border-neutral-800 text-center space-y-3">
            <div class="w-12 h-12 rounded-full bg-neutral-800 border border-neutral-700 flex items-center justify-center mx-auto text-neutral-400">
              <Users size={22} />
            </div>
            <div>
              <h3 class="text-sm font-bold text-white">Nenhuma escala ativa</h3>
              <p class="text-xs text-neutral-400 mt-1 max-w-xs mx-auto">
                A escala do culto será exibida automaticamente assim que for ativada pelo operador no computador principal.
              </p>
            </div>
            <button
              type="button"
              onclick={loadTeamRoster}
              class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-semibold cursor-pointer"
            >
              <RefreshCw size={12} />
              <span>Atualizar</span>
            </button>
          </div>
        {:else}
          <!-- Cabeçalho do Culto Ativo -->
          <div class="p-4 rounded-2xl bg-neutral-900 border border-purple-500/30 space-y-2">
            <div class="flex items-center justify-between">
              <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                Culto Atual
              </span>
              <button
                type="button"
                onclick={loadTeamRoster}
                class="text-neutral-400 hover:text-white p-1 cursor-pointer"
                title="Atualizar"
              >
                <RefreshCw size={13} />
              </button>
            </div>
            <h3 class="text-base font-bold text-white">{activeRoster.title}</h3>
            <div class="flex items-center gap-3 text-xs text-neutral-400">
              {#if activeRoster.service_date}
                <div class="flex items-center gap-1">
                  <Calendar size={12} />
                  <span>{activeRoster.service_date}</span>
                </div>
              {/if}
              {#if activeRoster.service_time}
                <div class="flex items-center gap-1">
                  <Clock size={12} />
                  <span>{activeRoster.service_time}</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Departamentos e Voluntários -->
          <div class="space-y-3">
            {#each teamDepartments as dept}
              {@const deptItems = activeRoster.items.filter((it) => it.department_id === dept.id)}
              {#if deptItems.length > 0}
                <div class="rounded-2xl bg-neutral-900/90 border border-neutral-800 overflow-hidden">
                  <!-- Topo do Departamento -->
                  <div class="px-3.5 py-2.5 bg-neutral-950/60 border-b border-neutral-800/80 flex items-center justify-between">
                    <div class="flex items-center gap-2">
                      <span class="w-2.5 h-2.5 rounded-full" style="background-color: {dept.color};"></span>
                      <span class="text-xs font-bold text-white">{dept.name}</span>
                    </div>
                    <span class="text-[10px] text-neutral-400 font-medium">
                      {deptItems.length} {deptItems.length === 1 ? 'voluntário' : 'voluntários'}
                    </span>
                  </div>

                  <!-- Lista de Voluntários -->
                  <div class="p-2 space-y-2">
                    {#each deptItems as item}
                      {@const member = teamMembers.find((m) => m.id === item.member_id)}
                      {@const memberName = member?.name || 'Voluntário'}
                      {@const waUrl = getWhatsAppUrl(member?.phone, `Olá ${memberName}, tudo bem? Sobre sua escala de ${dept.name} no culto de ${activeRoster.service_date}:`)}
                      <div class="p-2.5 rounded-xl bg-neutral-950/60 border border-neutral-800/80 flex items-center justify-between gap-2">
                        <div class="min-w-0">
                          <div class="flex items-center gap-1.5">
                            <span class="text-xs font-bold text-white truncate">
                              {memberName}
                            </span>
                            {#if item.role === 'lead'}
                              <span class="text-amber-400 shrink-0" title="Titular">
                                <Crown size={12} />
                              </span>
                            {/if}
                          </div>
                          <span class="text-[10px] text-neutral-400 capitalize">
                            {item.role === 'lead' ? 'Titular' : 'Auxiliar'}
                          </span>
                        </div>

                        <div class="flex items-center gap-1.5 shrink-0">
                          <!-- Botão de Presença Alternável -->
                          <button
                            type="button"
                            onclick={() => handleToggleMemberStatus(item.id, item.status)}
                            class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[10px] font-semibold border cursor-pointer {item.status === 'confirmed' ? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30' : item.status === 'declined' ? 'bg-rose-500/15 text-rose-400 border-rose-500/30' : 'bg-amber-500/15 text-amber-400 border-amber-500/30'}"
                          >
                            {#if item.status === 'confirmed'}
                              <CheckCircle2 size={10} />
                              <span>Confirmado</span>
                            {:else if item.status === 'declined'}
                              <X size={10} />
                              <span>Não vai</span>
                            {:else}
                              <Clock size={10} />
                              <span>Pendente</span>
                            {/if}
                          </button>

                          {#if waUrl}
                            <a
                              href={waUrl}
                              target="_blank"
                              rel="noopener noreferrer"
                              class="w-7 h-7 rounded-lg bg-emerald-500/10 hover:bg-emerald-500 text-emerald-400 hover:text-white flex items-center justify-center transition-colors"
                              title="WhatsApp"
                            >
                              <MessageCircle size={13} />
                            </a>
                          {/if}
                        </div>
                      </div>
                    {/each}
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </main>

  <!-- BOTTOM NAVIGATION BAR (Adapta dinamicamente: 5 abas para operadores, 3 abas para voluntários) -->
  <nav
    class="bg-neutral-900 border-t border-neutral-800 grid {canControlRemote
      ? 'grid-cols-5'
      : 'grid-cols-3'} shrink-0"
    style="padding-bottom: max(env(safe-area-inset-bottom), 16px);"
  >
    <button
      type="button"
      onclick={() => (activeTab = 'home')}
      class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab ===
      'home'
        ? 'text-emerald-400 font-extrabold'
        : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <Home size={20} class={activeTab === 'home' ? 'text-emerald-400' : 'text-neutral-400'} />
      <span class="text-[11px]">Início</span>
    </button>

    {#if canControlRemote}
      <button
        type="button"
        onclick={() => (activeTab = 'setlist')}
        class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab ===
        'setlist'
          ? 'text-emerald-400 font-extrabold'
          : 'text-neutral-400 hover:text-neutral-200'}"
      >
        <ListMusic
          size={20}
          class={activeTab === 'setlist' ? 'text-emerald-400' : 'text-neutral-400'}
        />
        <span class="text-[11px]">Setlist</span>
      </button>

      <button
        type="button"
        onclick={() => (activeTab = 'songs')}
        class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab ===
        'songs'
          ? 'text-emerald-400 font-extrabold'
          : 'text-neutral-400 hover:text-neutral-200'}"
      >
        <Music size={20} class={activeTab === 'songs' ? 'text-emerald-400' : 'text-neutral-400'} />
        <span class="text-[11px]">Músicas</span>
      </button>

      <button
        type="button"
        onclick={() => (activeTab = 'bible')}
        class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab ===
        'bible'
          ? 'text-emerald-400 font-extrabold'
          : 'text-neutral-400 hover:text-neutral-200'}"
      >
        <BookOpen size={20} class={activeTab === 'bible' ? 'text-emerald-400' : 'text-neutral-400'} />
        <span class="text-[11px]">Bíblia</span>
      </button>
    {/if}

    <button
      type="button"
      onclick={() => (activeTab = 'team')}
      class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab ===
      'team'
        ? 'text-emerald-400 font-extrabold'
        : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <Users size={20} class={activeTab === 'team' ? 'text-emerald-400' : 'text-neutral-400'} />
      <span class="text-[11px]">Equipe</span>
    </button>

    {#if !canControlRemote}
      <button
        type="button"
        onclick={() => {
          isOperatorUnlockModalOpen = true;
          operatorPinInput = '';
          operatorPinError = '';
        }}
        class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer text-amber-400 hover:text-amber-300"
        title="Desbloquear Modo Operador com PIN da mesa"
      >
        <Lock size={19} class="text-amber-400" />
        <span class="text-[11px]">Operador</span>
      </button>
    {/if}
  </nav>

  <!-- DRAWER SHADCN: Seleção de Tradução Bíblica -->
  {#if isBibleDrawerOpen}
    <!-- Backdrop com blur -->
    <button
      type="button"
      class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm transition-opacity w-full h-full border-0 p-0 m-0 cursor-pointer"
      onclick={() => (isBibleDrawerOpen = false)}
      aria-label="Fechar Gaveta"
    ></button>

    <!-- Superfície do Drawer deslizando da base -->
    <div
      class="fixed inset-x-0 bottom-0 z-50 max-w-md mx-auto rounded-t-3xl bg-neutral-900 border-t border-neutral-800 p-4 shadow-2xl flex flex-col max-h-[80vh] font-sans"
      style="padding-bottom: max(env(safe-area-inset-bottom), 24px);"
    >
      <!-- Drag Handle -->
      <div class="w-12 h-1.5 rounded-full bg-neutral-700 mx-auto mb-3 shrink-0"></div>

      <!-- Cabeçalho do Drawer -->
      <div class="flex items-center justify-between pb-3 border-b border-neutral-800 shrink-0">
        <div>
          <h3 class="text-sm font-bold text-white flex items-center gap-2">
            <BookOpen size={16} class="text-emerald-400" />
            <span>Tradução da Bíblia</span>
          </h3>
          <p class="text-[11px] text-neutral-400">Selecione a versão para projeção e leitura</p>
        </div>
        <button
          type="button"
          onclick={() => (isBibleDrawerOpen = false)}
          class="p-1.5 rounded-full bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      <!-- Lista de Traduções Disponíveis -->
      <div class="space-y-1.5 py-3 overflow-y-auto min-h-0 flex-1">
        {#each bibles as b}
          {@const isSelected = b.id === activeBibleId}
          <button
            type="button"
            onclick={() => {
              activeBibleId = b.id;
              if (selectedBook) {
                loadVerses(b.id, selectedBook.id, selectedChapter);
              }
              isBibleDrawerOpen = false;
            }}
            class="w-full p-3 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {isSelected ? 'bg-emerald-950/40 border-emerald-500 text-white' : 'bg-neutral-850/80 border-neutral-800 text-neutral-300 hover:bg-neutral-800'}"
          >
            <div>
              <div class="flex items-center gap-2">
                <span
                  class="px-2 py-0.5 rounded text-xs font-extrabold font-mono {isSelected ? 'bg-emerald-500 text-black' : 'bg-neutral-800 text-neutral-300'}"
                >
                  {b.abbreviation || b.name}
                </span>
                <span class="text-xs font-bold text-white">{b.name}</span>
              </div>
              <span class="text-[10px] text-neutral-400 mt-0.5 block">{b.book_count || 66} livros</span>
            </div>
            {#if isSelected}
              <Check size={16} class="text-emerald-400 shrink-0" />
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- DRAWER SHADCN: Seleção de Setlist -->
  {#if isSetlistDrawerOpen}
    <button
      type="button"
      class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm transition-opacity w-full h-full border-0 p-0 m-0 cursor-pointer"
      onclick={() => (isSetlistDrawerOpen = false)}
      aria-label="Fechar Gaveta"
    ></button>

    <div
      class="fixed inset-x-0 bottom-0 z-50 max-w-md mx-auto rounded-t-3xl bg-neutral-900 border-t border-neutral-800 p-4 shadow-2xl flex flex-col max-h-[80vh] font-sans"
      style="padding-bottom: max(env(safe-area-inset-bottom), 24px);"
    >
      <div class="w-12 h-1.5 rounded-full bg-neutral-700 mx-auto mb-3 shrink-0"></div>

      <div class="flex items-center justify-between pb-3 border-b border-neutral-800 shrink-0">
        <div>
          <h3 class="text-sm font-bold text-white flex items-center gap-2">
            <Layers size={16} class="text-emerald-400" />
            <span>Alternar Setlist</span>
          </h3>
          <p class="text-[11px] text-neutral-400">Selecione o culto ou reunião a ser exibido</p>
        </div>
        <button
          type="button"
          onclick={() => (isSetlistDrawerOpen = false)}
          class="p-1.5 rounded-full bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      <div class="space-y-1.5 py-3 overflow-y-auto min-h-0 flex-1">
        {#each availableSetlists as s}
          {@const isSelected = activeSetlistData?.id === s.id}
          <button
            type="button"
            onclick={() => {
              loadSingleSetlistDetail(s.id);
              isSetlistDrawerOpen = false;
            }}
            class="w-full p-3 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {isSelected ? 'bg-emerald-950/40 border-emerald-500 text-white' : 'bg-neutral-850/80 border-neutral-800 text-neutral-300 hover:bg-neutral-800'}"
          >
            <div>
              <p class="text-xs font-bold text-white">{s.title}</p>
              <p class="text-[10px] text-neutral-400 mt-0.5">{s.item_count} itens cadastrados</p>
            </div>
            {#if isSelected}
              <Check size={16} class="text-emerald-400 shrink-0" />
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- MODAL: Desbloquear Controle de Operador por PIN -->
  {#if isOperatorUnlockModalOpen}
    <div
      class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-150"
    >
      <div
        class="w-full max-w-sm rounded-2xl bg-neutral-900 border border-neutral-800 p-5 shadow-2xl space-y-4 font-sans"
      >
        <div class="flex items-start justify-between">
          <div class="flex items-center gap-2.5">
            <div class="w-10 h-10 rounded-xl bg-amber-500/15 border border-amber-500/30 text-amber-400 flex items-center justify-center shrink-0">
              <KeyRound size={20} />
            </div>
            <div>
              <h3 class="text-sm font-bold text-white">Desbloquear Operador</h3>
              <p class="text-[11px] text-neutral-400">Controle total de mídia e projeção</p>
            </div>
          </div>
          <button
            type="button"
            onclick={() => {
              isOperatorUnlockModalOpen = false;
              operatorPinInput = '';
              operatorPinError = '';
            }}
            class="p-1 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 cursor-pointer"
          >
            <X size={16} />
          </button>
        </div>

        <p class="text-xs text-neutral-300 leading-relaxed">
          Informe o PIN do operador da mesa multimídia para liberar o disparo de slides, blackout e seleção de músicas neste aparelho.
        </p>

        <div>
          <label class="block text-[11px] font-bold text-neutral-400 uppercase tracking-wider mb-1.5" for="remote-operator-pin-input">
            PIN de Operador
          </label>
          <input
            id="remote-operator-pin-input"
            type="password"
            inputmode="numeric"
            maxlength={8}
            bind:value={operatorPinInput}
            onkeydown={(e) => e.key === 'Enter' && handleVerifyOperatorPin()}
            placeholder="Ex: 1234"
            class="w-full px-3.5 py-3 rounded-xl bg-neutral-950 border border-neutral-800 text-center text-lg tracking-widest font-mono text-white placeholder:text-neutral-600 focus:outline-none focus:border-amber-500"
          />
          {#if operatorPinError}
            <p class="text-xs text-rose-400 mt-1.5 flex items-center gap-1 font-medium">
              <AlertCircle size={13} />
              <span>{operatorPinError}</span>
            </p>
          {/if}
        </div>

        <div class="flex items-center gap-2 pt-1">
          <button
            type="button"
            onclick={() => {
              isOperatorUnlockModalOpen = false;
              operatorPinInput = '';
              operatorPinError = '';
            }}
            class="flex-1 py-2.5 rounded-xl bg-neutral-800 hover:bg-neutral-750 text-neutral-300 font-bold text-xs cursor-pointer"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleVerifyOperatorPin}
            class="flex-1 py-2.5 rounded-xl bg-amber-500 hover:bg-amber-400 active:bg-amber-600 text-black font-extrabold text-xs flex items-center justify-center gap-1.5 shadow-lg shadow-amber-950/40 cursor-pointer"
          >
            <Unlock size={14} />
            <span>Desbloquear</span>
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- MODAL: Teaser e Informações do Chat da Equipe -->
  {#if isChatInfoModalOpen}
    <div
      class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-150"
    >
      <div
        class="w-full max-w-sm rounded-2xl bg-neutral-900 border border-neutral-800 p-5 shadow-2xl space-y-4 font-sans"
      >
        <div class="flex items-start justify-between">
          <div class="flex items-center gap-2.5">
            <div class="w-10 h-10 rounded-xl bg-violet-500/15 border border-violet-500/30 text-violet-400 flex items-center justify-center shrink-0">
              <MessageSquare size={20} />
            </div>
            <div>
              <div class="flex items-center gap-1.5">
                <h3 class="text-sm font-bold text-white">Chat da Equipe</h3>
                <span class="px-1.5 py-0.2 rounded-full text-[9px] font-extrabold bg-violet-500/20 text-violet-300 border border-violet-500/30 uppercase">Em breve</span>
              </div>
              <p class="text-[11px] text-neutral-400">Comunicação direta em tempo real</p>
            </div>
          </div>
          <button
            type="button"
            onclick={() => (isChatInfoModalOpen = false)}
            class="p-1 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 cursor-pointer"
          >
            <X size={16} />
          </button>
        </div>

        <div class="space-y-2.5 text-xs text-neutral-300 leading-relaxed">
          <p>
            O canal de chat interno está em desenvolvimento para permitir a troca ágil de mensagens entre voluntários do culto sem depender de redes sociais externas.
          </p>

          <div class="p-3 rounded-xl bg-neutral-950 border border-neutral-800/80 space-y-2">
            <div class="flex items-center gap-2 text-violet-300 font-bold text-[11px]">
              <Sparkles size={13} />
              <span>Funcionalidades planejadas:</span>
            </div>
            <ul class="text-[11px] text-neutral-400 space-y-1 list-disc list-inside">
              <li>Mensagens instantâneas entre Mídia, Louvor e Som</li>
              <li>Avisos do púlpito e alterações na ordem do culto</li>
              <li>Alertas discretos diretamente no ponto eletrônico</li>
              <li>100% offline via rede local da igreja</li>
            </ul>
          </div>
        </div>

        <button
          type="button"
          onclick={() => (isChatInfoModalOpen = false)}
          class="w-full py-2.5 rounded-xl bg-violet-600 hover:bg-violet-500 text-white font-bold text-xs flex items-center justify-center gap-1.5 cursor-pointer shadow-lg shadow-violet-950/40"
        >
          <span>Entendido</span>
        </button>
      </div>
    </div>
  {/if}
</div>
{/if}
