<script lang="ts">
  import { onMount } from 'svelte';
  import { AlertTriangle, BookOpen, Check, ListMusic, Loader2, ShieldCheck } from 'lucide-svelte';
  import {
    presentationState,
    activeSetlist,
    isWsConnected,
    teamUpdatedStore,
    initWebSocket,
    closeWebSocket,
    selectSlide,
    stopPresentation,
    toggleBlackout,
    toggleClearText,
  } from '$lib/ws';
  import { initAppTheme, setTheme, themeStore } from '$lib/settings';
  import type {
    MediaItem,
    Setlist,
    SetlistSummary,
    SongSummary,
    BibleVersionSummary,
    BibleBook,
    BibleVerse,
    BibleSearchResult,
    ServiceRoster,
    ServiceRosterItem,
    TeamDepartment,
    TeamMember,
  } from '$lib/types';
  import { parseBibleReference } from '$lib/bibleParser';
  import { parseRawLyrics } from '$lib/songParser';
  import { updateRosterItemStatus, isRosterLeadRole } from '$lib/teams';
  import {
    remoteAuthStore,
    remoteOperatorUnlockedStore,
    getStoredRemoteSession,
    getStoredOperatorUnlocked,
    clearRemoteSession,
    setOperatorUnlocked,
    saveRemoteSession,
  } from '$lib/remoteAuth';
  import {
    fetchJsonWithError,
    getStoredSafeDispatch,
    setStoredSafeDispatch,
    verifyRemoteOperatorPin,
  } from '$lib/remoteData';
  import { notifyError, notifySuccess } from '$lib/toast';
  import RemoteLogin from '$lib/components/RemoteLogin.svelte';
  import RemoteDialog from '$lib/components/remote/RemoteDialog.svelte';
  import RemoteHeader from '$lib/components/remote/RemoteHeader.svelte';
  import RemoteLiveControlCard from '$lib/components/remote/RemoteLiveControlCard.svelte';
  import RemoteBottomNav, { type RemoteTab } from '$lib/components/remote/RemoteBottomNav.svelte';
  import RemoteHomeTab from '$lib/components/remote/RemoteHomeTab.svelte';
  import RemoteSetlistTab from '$lib/components/remote/RemoteSetlistTab.svelte';
  import RemoteSongsTab from '$lib/components/remote/RemoteSongsTab.svelte';
  import RemoteBibleTab from '$lib/components/remote/RemoteBibleTab.svelte';
  import RemoteTeamTab from '$lib/components/remote/RemoteTeamTab.svelte';
  import { Eye, EyeOff } from 'lucide-svelte';

  // --- Navegação e Controles Gerais ---
  let activeTab = $state<RemoteTab>('home');
  let isLiveCardCollapsed = $state(false);
  let safeDispatch = $state(getStoredSafeDispatch());

  // --- Modais / Drawers ---
  let isPinModalOpen = $state(false);
  let operatorPinInput = $state('');
  let operatorPinError = $state('');
  let showOperatorPin = $state(false);
  let isVerifyingOperatorPin = $state(false);
  let isStopConfirmOpen = $state(false);
  let isSetlistSelectorOpen = $state(false);
  let isBibleSelectorOpen = $state(false);

  // --- Dados de Equipe e Escala ---
  let activeRoster = $state<ServiceRoster | null>(null);
  let teamDepartments = $state<TeamDepartment[]>([]);
  let teamMembers = $state<TeamMember[]>([]);
  let teamLoading = $state(false);
  let teamError = $state<string | null>(null);
  let teamAttempted = $state(false);
  let updatingStatusId = $state<string | null>(null);

  // --- Dados de Setlists ---
  let setlistsList = $state<SetlistSummary[]>([]);
  let activeSetlistData = $state<Setlist | null>(null);
  let setlistLoading = $state(false);
  let setlistError = $state<string | null>(null);
  let setlistsAttempted = $state(false);
  let expandedSetlistItemId = $state<string | null>(null);
  let lastSyncedSetlistId: string | undefined = undefined;

  // --- Dados de Músicas ---
  let songs = $state<SongSummary[]>([]);
  let songSearchQuery = $state('');
  let isSearchingSongs = $state(false);
  let songsError = $state<string | null>(null);
  let songsAttempted = $state(false);
  let selectedSong = $state<MediaItem | null>(null);
  let songSearchTimeout: ReturnType<typeof setTimeout> | null = null;
  let songsAbortController: AbortController | null = null;

  // --- Dados da Bíblia ---
  let bibles = $state<BibleVersionSummary[]>([]);
  let selectedBibleId = $state('');
  let books = $state<BibleBook[]>([]);
  let selectedBookId = $state(1);
  let chapters = $state<number[]>([]);
  let selectedChapter = $state(1);
  let verses = $state<BibleVerse[]>([]);
  let testamentFilter = $state<'all' | 'ot' | 'nt'>('all');
  let bibleSearchQuery = $state('');
  let bibleSearchResults = $state<BibleSearchResult[]>([]);
  let isLoadingBible = $state(false);
  let bibleError = $state<string | null>(null);
  let biblesAttempted = $state(false);
  let bibleNavTab = $state<'verses' | 'books' | 'chapters'>('verses');
  let bibleAbortController: AbortController | null = null;

  // --- Tema ---
  const isLightTheme = $derived.by(() => {
    if ($themeStore === 'light') return true;
    if ($themeStore === 'dark') return false;
    if (typeof window !== 'undefined') {
      return !window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    return false;
  });

  function handleToggleTheme() {
    setTheme(isLightTheme ? 'dark' : 'light');
  }

  function handleToggleSafeDispatch() {
    const next = !safeDispatch;
    safeDispatch = next;
    setStoredSafeDispatch(next);
  }

  // --- Estado Derivado de Apresentação ---
  const isLive = $derived(
    Boolean($presentationState.active_media_id) && $presentationState.display_mode === 'live'
  );
  const isBlackout = $derived($presentationState.display_mode === 'blackout');
  const isClear = $derived($presentationState.display_mode === 'cleartext');
  const audioPlayback = $derived($presentationState.audio);

  const currentMedia = $derived.by<MediaItem | undefined>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return undefined;
    if (p.quick_media && p.quick_media.id === p.active_media_id) return p.quick_media;
    return activeSetlistData?.items.find((i) => i.id === p.active_media_id);
  });

  const currentMediaTitle = $derived(currentMedia?.title || 'Nenhuma mídia ativa');
  const currentMediaRef = $derived(currentMedia?.artist_or_ref || '');
  const currentMediaSlides = $derived(currentMedia?.slides || []);
  const currentSlideIndex = $derived($presentationState.active_slide_index ?? 0);
  const totalSlides = $derived(currentMediaSlides.length);
  const currentSlide = $derived(
    currentMediaSlides.length > 0 && currentSlideIndex >= 0 && currentSlideIndex < currentMediaSlides.length
      ? currentMediaSlides[currentSlideIndex]
      : null
  );
  const nextSlide = $derived(
    currentMediaSlides.length > 0 && currentSlideIndex + 1 < currentMediaSlides.length
      ? currentMediaSlides[currentSlideIndex + 1]
      : null
  );

  const activeBibleSummary = $derived(
    bibles.find((b) => b.id === selectedBibleId) || bibles[0]
  );
  const activeBibleAbbr = $derived(activeBibleSummary?.abbreviation || 'Bíblia');
  const activeBibleName = $derived(activeBibleSummary?.name || 'Bíblia Sagrada');
  const selectedBookName = $derived(
    books.find((b) => b.id === selectedBookId)?.name || 'Livro'
  );

  const canControlRemote = $derived.by(() => {
    if (!$remoteAuthStore) return false;
    return Boolean($remoteAuthStore.can_control_remote) || Boolean($remoteOperatorUnlockedStore);
  });

  const myRosterItem = $derived.by(() => {
    if (!$remoteAuthStore || !activeRoster) return null;
    return activeRoster.items.find((i) => i.member_id === $remoteAuthStore?.member_id) || null;
  });

  const myDepartment = $derived.by(() => {
    if (!myRosterItem) return null;
    return teamDepartments.find((d) => d.id === myRosterItem.department_id) || null;
  });

  const rosterDepartmentPartners = $derived.by(() => {
    if (!myRosterItem || !activeRoster) return [];
    const memberById = new Map<string, TeamMember>();
    for (const m of teamMembers) memberById.set(m.id, m);

    return activeRoster.items
      .filter(
        (i) => i.department_id === myRosterItem.department_id && i.member_id !== myRosterItem.member_id
      )
      .map((item) => ({
        ...item,
        member: memberById.get(item.member_id),
      }));
  });

  // --- Efeitos Reativos Seguros (Sem loops de retry nem refetches 1x/s) ---
  // 1. Mantém sessão sincronizada quando teamMembers atualiza; revoga acesso se inativado
  $effect(() => {
    const auth = $remoteAuthStore;
    if (!auth || auth.member_id === 'operator-guest' || teamMembers.length === 0) return;

    const liveMember = teamMembers.find((m) => m.id === auth.member_id);
    if (!liveMember || !liveMember.is_active) {
      resetTransientState();
      clearRemoteSession();
      notifyError('Sessão encerrada', 'Seu cadastro de voluntário foi desativado no painel.');
      return;
    }

    const sameDeps =
      liveMember.department_ids.length === auth.department_ids.length &&
      liveMember.department_ids.every((id, idx) => id === auth.department_ids[idx]);

    if (
      auth.can_control_remote !== Boolean(liveMember.can_control_remote) ||
      auth.name !== liveMember.name ||
      auth.role !== liveMember.role ||
      auth.color !== liveMember.color ||
      !sameDeps
    ) {
      saveRemoteSession(liveMember);
    }
  });

  // 2. Restringe apenas Músicas e Bíblia a operadores; Setlist permanece em Modo Consulta
  $effect(() => {
    if (!$remoteAuthStore) return;
    if (!canControlRemote && (activeTab === 'songs' || activeTab === 'bible')) {
      activeTab = 'home';
    }
  });

  // 3. Carga inicial pós-login (guarded por flags attempted)
  $effect(() => {
    if (!$remoteAuthStore) return;
    if (!teamAttempted) {
      teamAttempted = true;
      void loadTeamRoster();
    }
    if (!setlistsAttempted) {
      setlistsAttempted = true;
      void loadSetlistsAndActive();
    }
  });

  // 4. Carga sob demanda para Músicas e Bíblia quando o operador entra na aba
  $effect(() => {
    if (!$remoteAuthStore || !canControlRemote) return;
    if (activeTab === 'songs' && !songsAttempted) {
      songsAttempted = true;
      void loadSongs('');
    } else if (activeTab === 'bible' && !biblesAttempted) {
      biblesAttempted = true;
      void loadBibles();
    }
  });

  // 5. Atualiza via WebSocket ou quando active_setlist_id mudar de fato (nunca 1x/s)
  $effect(() => {
    if (!$remoteAuthStore) return;
    const activeId = $presentationState.active_setlist_id;
    const wsSetlist = $activeSetlist;

    if (wsSetlist && (!activeSetlistData || wsSetlist.id === activeSetlistData.id)) {
      activeSetlistData = wsSetlist;
    }

    if (activeId && activeId !== lastSyncedSetlistId) {
      lastSyncedSetlistId = activeId;
      void loadSingleSetlistDetail(activeId);
    }
  });

  // 6. Sincronia reativa de equipe via WebSocket
  $effect(() => {
    const update = $teamUpdatedStore;
    if (update && $remoteAuthStore) {
      void loadTeamRoster();
    }
  });

  // --- Lifecycle ---
  onMount(() => {
    initAppTheme();

    if (!$remoteAuthStore) {
      const stored = getStoredRemoteSession();
      if (stored) remoteAuthStore.set(stored);
    }
    if (!$remoteOperatorUnlockedStore) {
      const unlocked = getStoredOperatorUnlocked();
      if (unlocked) remoteOperatorUnlockedStore.set(true);
    }

    initWebSocket('remote');

    return () => {
      if (songSearchTimeout) clearTimeout(songSearchTimeout);
      songsAbortController?.abort();
      bibleAbortController?.abort();
      closeWebSocket();
    };
  });

  function resetTransientState() {
    activeTab = 'home';
    selectedSong = null;
    expandedSetlistItemId = null;
    songSearchQuery = '';
    bibleSearchQuery = '';
    bibleSearchResults = [];
    setlistError = null;
    songsError = null;
    bibleError = null;
    teamError = null;
    teamAttempted = false;
    setlistsAttempted = false;
    songsAttempted = false;
    biblesAttempted = false;
    lastSyncedSetlistId = undefined;
  }

  function handleLogout() {
    resetTransientState();
    clearRemoteSession();
  }

  function handleNavigateTab(tab: RemoteTab) {
    if (!canControlRemote && (tab === 'songs' || tab === 'bible')) {
      openOperatorPinModal();
      return;
    }
    activeTab = tab;
  }

  // --- Autenticação PIN Operador ---
  function openOperatorPinModal() {
    operatorPinInput = '';
    operatorPinError = '';
    showOperatorPin = false;
    isPinModalOpen = true;
  }

  function closeOperatorPinModal() {
    isPinModalOpen = false;
    operatorPinInput = '';
    operatorPinError = '';
  }

  async function handleVerifyOperatorPin(e?: SubmitEvent) {
    if (e) e.preventDefault();
    isVerifyingOperatorPin = true;
    operatorPinError = '';

    const result = await verifyRemoteOperatorPin(operatorPinInput);
    isVerifyingOperatorPin = false;

    if (!result.ok) {
      operatorPinError = result.error || 'PIN incorreto.';
      return;
    }

    setOperatorUnlocked(true);
    closeOperatorPinModal();
    notifySuccess('Controle liberado', 'Modo de operação completo ativo neste dispositivo.');
  }

  // --- Carregamentos de API ---
  async function loadTeamRoster() {
    teamLoading = true;
    teamError = null;
    try {
      const [roster, depts, mems] = await Promise.all([
        fetch('/api/teams/rosters/active').then(async (r) => {
          if (r.status === 404) return null;
          if (!r.ok) throw new Error('Falha ao carregar escala ativa.');
          return (await r.json()) as ServiceRoster | null;
        }),
        fetchJsonWithError<TeamDepartment[]>(
          '/api/teams/departments',
          'Falha ao carregar departamentos.'
        ),
        fetchJsonWithError<TeamMember[]>(
          '/api/teams/members',
          'Falha ao carregar voluntários.'
        ),
      ]);
      activeRoster = roster;
      teamDepartments = depts;
      teamMembers = mems;
    } catch (err) {
      teamError = err instanceof Error ? err.message : 'Erro ao comunicar com a escala da igreja.';
    } finally {
      teamLoading = false;
    }
  }

  async function handleSetMyStatus(status: 'confirmed' | 'declined') {
    if (!activeRoster || !myRosterItem || updatingStatusId) return;
    updatingStatusId = myRosterItem.id;
    const prev = myRosterItem.status;

    activeRoster = {
      ...activeRoster,
      items: activeRoster.items.map((i) => (i.id === myRosterItem.id ? { ...i, status } : i)),
    };

    const ok = await updateRosterItemStatus(activeRoster.id, myRosterItem.id, status);
    updatingStatusId = null;

    if (!ok) {
      activeRoster = {
        ...activeRoster,
        items: activeRoster.items.map((i) => (i.id === myRosterItem.id ? { ...i, status: prev } : i)),
      };
      notifyError('Falha ao confirmar presença', 'Não foi possível salvar sua resposta no servidor.');
      return;
    }

    notifySuccess(
      status === 'confirmed' ? 'Presença confirmada' : 'Ausência registrada',
      status === 'confirmed' ? 'Seu status na escala foi atualizado.' : 'A equipe foi notificada.'
    );
  }

  async function handleToggleMemberStatus(item: ServiceRosterItem) {
    if (!activeRoster || updatingStatusId) return;
    if (item.member_id !== $remoteAuthStore?.member_id && !canControlRemote) return;

    const nextStatus =
      item.status === 'pending' ? 'confirmed' : item.status === 'confirmed' ? 'declined' : 'pending';
    const prev = item.status;

    updatingStatusId = item.id;
    activeRoster = {
      ...activeRoster,
      items: activeRoster.items.map((i) => (i.id === item.id ? { ...i, status: nextStatus } : i)),
    };

    const ok = await updateRosterItemStatus(activeRoster.id, item.id, nextStatus);
    updatingStatusId = null;

    if (!ok) {
      activeRoster = {
        ...activeRoster,
        items: activeRoster.items.map((i) => (i.id === item.id ? { ...i, status: prev } : i)),
      };
      notifyError('Erro na escala', 'Não foi possível atualizar a confirmação no servidor.');
    }
  }

  async function loadSetlistsAndActive() {
    setlistLoading = true;
    setlistError = null;
    try {
      const summaries = await fetchJsonWithError<SetlistSummary[]>(
        '/api/setlists',
        'Não foi possível carregar as setlists.'
      );
      setlistsList = summaries;

      const targetId =
        $presentationState.active_setlist_id ||
        summaries.find((s) => s.is_active)?.id ||
        summaries[0]?.id;

      if (targetId) {
        lastSyncedSetlistId = targetId;
        await loadSingleSetlistDetail(targetId);
      } else {
        activeSetlistData = null;
      }
    } catch (err) {
      setlistError = err instanceof Error ? err.message : 'Erro ao carregar a ordem do culto.';
    } finally {
      setlistLoading = false;
    }
  }

  async function loadSingleSetlistDetail(id: string) {
    try {
      const data = await fetchJsonWithError<Setlist>(
        `/api/setlists/${encodeURIComponent(id)}`,
        'Não foi possível abrir os detalhes da setlist.'
      );
      activeSetlistData = data;
      setlistError = null;
    } catch (err) {
      setlistError = err instanceof Error ? err.message : 'Erro ao carregar itens da setlist.';
    }
  }

  async function selectSetlist(id: string) {
    isSetlistSelectorOpen = false;
    lastSyncedSetlistId = id;
    setlistLoading = true;
    await loadSingleSetlistDetail(id);
    setlistLoading = false;
  }

  async function loadSongs(q = songSearchQuery) {
    songsAbortController?.abort();
    const controller = new AbortController();
    songsAbortController = controller;

    isSearchingSongs = true;
    songsError = null;
    try {
      const url = q.trim() ? `/api/songs?q=${encodeURIComponent(q.trim())}` : '/api/songs';
      songs = await fetchJsonWithError<SongSummary[]>(
        url,
        'Não foi possível consultar o catálogo de louvores.',
        { signal: controller.signal }
      );
    } catch (err) {
      if ((err as Error).name === 'AbortError') return;
      songsError = err instanceof Error ? err.message : 'Falha ao carregar catálogo de músicas.';
    } finally {
      if (songsAbortController === controller) {
        isSearchingSongs = false;
      }
    }
  }

  function handleSongSearchInput(q: string) {
    songSearchQuery = q;
    if (songSearchTimeout) clearTimeout(songSearchTimeout);
    songSearchTimeout = setTimeout(() => {
      void loadSongs(q);
    }, 250);
  }

  async function handleOpenSong(songSummary: SongSummary) {
    songsError = null;
    try {
      selectedSong = await fetchJsonWithError<MediaItem>(
        `/api/songs/${encodeURIComponent(songSummary.id)}`,
        'Não foi possível abrir a letra desta música.'
      );
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Falha ao carregar música.';
      songsError = msg;
      notifyError('Erro ao abrir louvor', msg);
    }
  }

  async function loadBibles() {
    isLoadingBible = true;
    bibleError = null;
    try {
      const data = await fetchJsonWithError<BibleVersionSummary[]>(
        '/api/bibles',
        'Não foi possível carregar as versões da Bíblia.'
      );
      bibles = data;
      if (data.length > 0 && !selectedBibleId) {
        const def = data.find((b) => b.is_default) || data[0];
        selectedBibleId = def.id;
        await loadBooks(def.id);
      }
    } catch (err) {
      bibleError = err instanceof Error ? err.message : 'Erro ao conectar à biblioteca bíblica.';
    } finally {
      isLoadingBible = false;
    }
  }

  async function loadBooks(bibleId: string) {
    try {
      const data = await fetchJsonWithError<BibleBook[]>(
        `/api/bibles/${encodeURIComponent(bibleId)}/books`,
        'Não foi possível listar os livros bíblicos.'
      );
      books = data;
      if (data.length > 0) {
        const book = data.find((b) => b.id === selectedBookId) || data[0];
        await selectBook(book.id, selectedChapter, bibleId, false);
      }
    } catch (err) {
      bibleError = err instanceof Error ? err.message : 'Falha ao carregar livros.';
    }
  }

  async function selectBook(
    bookId: number,
    chapterTarget = 1,
    bibleId = selectedBibleId,
    switchToChapters = true
  ) {
    selectedBookId = bookId;
    const book = books.find((b) => b.id === bookId);
    const totalCh = book?.chapter_count || 1;
    chapters = Array.from({ length: totalCh }, (_, i) => i + 1);
    const safeChapter = Math.min(Math.max(1, chapterTarget), totalCh);
    await selectChapter(safeChapter, bibleId, switchToChapters);
  }

  async function selectChapter(chapter: number, bibleId = selectedBibleId, switchToVerses = true) {
    selectedChapter = chapter;
    if (switchToVerses) bibleNavTab = 'verses';
    await loadVerses(selectedBookId, chapter, bibleId);
  }

  async function loadVerses(bookId: number, chapter: number, bibleId = selectedBibleId): Promise<boolean> {
    if (!bibleId) return false;
    bibleAbortController?.abort();
    const controller = new AbortController();
    bibleAbortController = controller;

    isLoadingBible = true;
    bibleError = null;
    try {
      verses = await fetchJsonWithError<BibleVerse[]>(
        `/api/bibles/${encodeURIComponent(bibleId)}/books/${bookId}/chapters/${chapter}`,
        'Não foi possível carregar os versículos deste capítulo.',
        { signal: controller.signal }
      );
      return true;
    } catch (err) {
      if ((err as Error).name === 'AbortError') return false;
      bibleError = err instanceof Error ? err.message : 'Erro ao carregar capítulo.';
      return false;
    } finally {
      if (bibleAbortController === controller) {
        isLoadingBible = false;
      }
    }
  }

  async function handleBibleSearch(e: SubmitEvent) {
    e.preventDefault();
    const q = bibleSearchQuery.trim();
    if (!q || !selectedBibleId) return;

    const parsed = parseBibleReference(q);
    if (parsed) {
      bibleSearchResults = [];
      bibleNavTab = 'verses';
      const targetBook = books.find((b) => b.id === parsed.book_id) || books[0];
      if (targetBook) {
        selectedBookId = targetBook.id;
        chapters = Array.from({ length: targetBook.chapter_count }, (_, i) => i + 1);
        selectedChapter = parsed.chapter;
        const ok = await loadVerses(targetBook.id, parsed.chapter);
        if (ok && parsed.verse_start && !safeDispatch) {
          const targetVerse = verses.find((v) => v.verse === parsed.verse_start);
          if (targetVerse) await presentVerse(targetVerse);
        }
        return;
      }
    }

    isLoadingBible = true;
    bibleError = null;
    try {
      bibleSearchResults = await fetchJsonWithError<BibleSearchResult[]>(
        `/api/bibles/${encodeURIComponent(selectedBibleId)}/search?q=${encodeURIComponent(q)}&limit=30`,
        'Falha ao pesquisar na Bíblia.'
      );
    } catch (err) {
      bibleError = err instanceof Error ? err.message : 'Erro ao buscar texto bíblico.';
    } finally {
      isLoadingBible = false;
    }
  }

  // --- Comandos ao Vivo ---
  function ensureOperator(): boolean {
    if (canControlRemote) return true;
    openOperatorPinModal();
    return false;
  }

  function handlePrevSlide() {
    if (!ensureOperator() || !currentMedia) return;
    if (currentSlideIndex > 0) {
      selectSlide(currentMedia.id, currentSlideIndex - 1);
    }
  }

  function handleNextSlide() {
    if (!ensureOperator() || !currentMedia) return;
    if (currentSlideIndex + 1 < totalSlides) {
      selectSlide(currentMedia.id, currentSlideIndex + 1);
    }
  }

  async function presentSetlistItemSlide(item: MediaItem, slideIndex: number) {
    if (!ensureOperator()) return;
    try {
      const res = await fetch('/api/presentation/quick', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ media: item, slide_index: slideIndex }),
      });
      if (!res.ok) throw new Error();
    } catch {
      selectSlide(item.id, slideIndex);
    }
  }

  async function presentSong(song: MediaItem, slideIndex = 0) {
    if (!ensureOperator()) return;
    let slides = song.slides || [];
    const rawLyrics = (song as unknown as { lyrics?: string }).lyrics;
    if (slides.length === 0 && rawLyrics) {
      slides = parseRawLyrics(rawLyrics).slides.map((s, idx) => ({
        ...s,
        id: `${song.id}-s-${idx}`,
      }));
    }

    const mediaToSend: MediaItem = { ...song, kind: 'song', slides };
    try {
      const res = await fetch('/api/presentation/quick', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ media: mediaToSend, slide_index: slideIndex }),
      });
      if (!res.ok) throw new Error();
    } catch {
      notifyError('Falha ao projetar música', 'Verifique se o computador principal está conectado.');
    }
  }

  async function presentVerse(verseObj: BibleVerse) {
    if (!ensureOperator()) return;
    const book = books.find((b) => b.id === selectedBookId);
    const title = `${book?.name || 'Bíblia'} ${selectedChapter}`;
    const verAbbr = activeBibleSummary?.abbreviation || '';
    const refText = `${title} • ${verAbbr}`;

    const slides = verses.map((v) => ({
      id: `v-${v.chapter}-${v.verse}`,
      label: `v.${v.verse}`,
      content: `${v.text}\n— ${book?.abbreviation || ''} ${v.chapter}:${v.verse}${verAbbr ? ` (${verAbbr})` : ''}`,
    }));

    const mediaToSend: MediaItem = {
      id: `bible-remote-${selectedBookId}-${selectedChapter}`,
      title,
      artist_or_ref: refText,
      kind: 'scripture',
      slides,
      align_h: 'center',
      align_v: 'center',
    };

    const slideIdx = Math.max(0, verses.findIndex((v) => v.verse === verseObj.verse));
    try {
      const res = await fetch('/api/presentation/quick', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ media: mediaToSend, slide_index: slideIdx }),
      });
      if (!res.ok) throw new Error();
    } catch {
      notifyError('Falha ao projetar versículo', 'Verifique a conexão com a mesa.');
    }
  }

  async function presentSearchResult(resObj: BibleSearchResult) {
    if (!ensureOperator()) return;
    selectedBookId = resObj.book_id;
    const book = books.find((b) => b.id === resObj.book_id);
    chapters = Array.from({ length: book?.chapter_count || 50 }, (_, i) => i + 1);
    selectedChapter = resObj.chapter;

    const ok = await loadVerses(resObj.book_id, resObj.chapter);
    if (!ok) return;

    const match = verses.find((v) => v.verse === resObj.verse);
    if (match) {
      await presentVerse(match);
    }
  }
</script>

{#if !$remoteAuthStore}
  <RemoteLogin onLoginSuccess={() => {
    teamAttempted = false;
    setlistsAttempted = false;
  }} />
{:else}
  <div class="h-dvh w-full overflow-hidden flex flex-col bg-background text-foreground">
    <RemoteHeader
      session={$remoteAuthStore}
      isConnected={$isWsConnected}
      {canControlRemote}
      {isBlackout}
      {isClear}
      {isLightTheme}
      onToggleTheme={handleToggleTheme}
      onLogout={handleLogout}
      onToggleBlackout={() => ensureOperator() && toggleBlackout()}
      onToggleClearText={() => ensureOperator() && toggleClearText()}
      onRequestStop={() => {
        if (ensureOperator()) isStopConfirmOpen = true;
      }}
    />

    <main class="flex-1 min-h-0 overflow-y-auto px-3 sm:px-5 py-3">
      <div class="max-w-md md:max-w-4xl lg:max-w-5xl mx-auto space-y-3">
        <RemoteLiveControlCard
          {canControlRemote}
          {isLive}
          {isBlackout}
          {isClear}
          {currentMediaTitle}
          {currentMediaRef}
          {currentSlide}
          {nextSlide}
          {currentSlideIndex}
          {totalSlides}
          timer={$presentationState.timer}
          isCollapsed={isLiveCardCollapsed}
          {safeDispatch}
          onToggleCollapse={() => (isLiveCardCollapsed = !isLiveCardCollapsed)}
          onToggleSafeDispatch={handleToggleSafeDispatch}
          onPrevSlide={handlePrevSlide}
          onNextSlide={handleNextSlide}
        />

        {#if activeTab === 'home'}
          <RemoteHomeTab
            session={$remoteAuthStore}
            {canControlRemote}
            operatorUnlocked={$remoteOperatorUnlockedStore}
            {activeRoster}
            {myRosterItem}
            {myDepartment}
            partners={rosterDepartmentPartners}
            {teamLoading}
            {teamError}
            {updatingStatusId}
            songsCount={songs.length}
            setlistItemsCount={activeSetlistData?.items.length || 0}
            onUpdateMyStatus={handleSetMyStatus}
            onRetryTeam={() => void loadTeamRoster()}
            onNavigateTab={handleNavigateTab}
            onOpenPinModal={openOperatorPinModal}
            onLockOperatorPin={() => {
              setOperatorUnlocked(false);
              notifySuccess('Controle bloqueado', 'Modo Consulta restaurado.');
            }}
          />
        {:else if activeTab === 'setlist'}
          <RemoteSetlistTab
            {canControlRemote}
            {safeDispatch}
            {activeSetlistData}
            activeMediaId={$presentationState.active_media_id}
            activeSlideIndex={$presentationState.active_slide_index}
            {audioPlayback}
            {expandedSetlistItemId}
            isLoading={setlistLoading}
            error={setlistError}
            onOpenSetlistSelector={() => (isSetlistSelectorOpen = true)}
            onRefresh={() => void loadSetlistsAndActive()}
            onToggleExpandItem={(id) =>
              (expandedSetlistItemId = expandedSetlistItemId === id ? null : id)}
            onProjectSlide={(item, idx) => void presentSetlistItemSlide(item, idx)}
            onOpenPinModal={openOperatorPinModal}
          />
        {:else if activeTab === 'songs'}
          <RemoteSongsTab
            {songs}
            {songSearchQuery}
            {isSearchingSongs}
            {songsError}
            {selectedSong}
            activeMediaId={$presentationState.active_media_id}
            activeSlideIndex={$presentationState.active_slide_index}
            {audioPlayback}
            {safeDispatch}
            onSearchInput={handleSongSearchInput}
            onRetrySongs={() => void loadSongs(songSearchQuery)}
            onOpenSong={(summary) => void handleOpenSong(summary)}
            onCloseSong={() => (selectedSong = null)}
            onProjectSongSlide={(song, idx) => void presentSong(song, idx)}
          />
        {:else if activeTab === 'bible'}
          <RemoteBibleTab
            {activeBibleAbbr}
            {activeBibleName}
            {books}
            {selectedBookId}
            {selectedBookName}
            {chapters}
            {selectedChapter}
            {verses}
            {testamentFilter}
            {bibleSearchQuery}
            {bibleSearchResults}
            {bibleNavTab}
            {isLoadingBible}
            {bibleError}
            activeMediaId={$presentationState.active_media_id}
            activeSlideIndex={$presentationState.active_slide_index}
            {safeDispatch}
            onOpenVersionSelector={() => (isBibleSelectorOpen = true)}
            onSearchQueryChange={(q) => (bibleSearchQuery = q)}
            onClearSearch={() => {
              bibleSearchQuery = '';
              bibleSearchResults = [];
            }}
            onSubmitSearch={handleBibleSearch}
            onSetNavTab={(tab) => (bibleNavTab = tab)}
            onSetTestamentFilter={(f) => (testamentFilter = f)}
            onSelectBook={(id) => void selectBook(id)}
            onSelectChapter={(ch) => void selectChapter(ch)}
            onProjectVerse={(v) => void presentVerse(v)}
            onProjectSearchResult={(res) => void presentSearchResult(res)}
            onRetryBible={() => void loadBibles()}
          />
        {:else if activeTab === 'team'}
          <RemoteTeamTab
            session={$remoteAuthStore}
            {canControlRemote}
            {activeRoster}
            {teamDepartments}
            {teamMembers}
            {teamLoading}
            {teamError}
            {updatingStatusId}
            onRefresh={() => void loadTeamRoster()}
            onToggleMemberStatus={(item) => void handleToggleMemberStatus(item)}
          />
        {/if}
      </div>
    </main>

    <RemoteBottomNav
      {activeTab}
      {canControlRemote}
      onSelectTab={handleNavigateTab}
      onOpenPinModal={openOperatorPinModal}
    />
  </div>

  <!-- Sheet 1: Seletor de Setlists -->
  <RemoteDialog
    open={isSetlistSelectorOpen}
    variant="sheet"
    title="Escolher Setlist"
    closeLabel="Fechar seletor de setlist"
    maxWidthClass="max-w-md md:max-w-lg"
    onclose={() => (isSetlistSelectorOpen = false)}
  >
    <div class="p-3 space-y-1.5">
      {#if setlistsList.length === 0}
        <p class="py-8 text-center text-xs text-muted-foreground">
          Nenhuma setlist salva no computador principal.
        </p>
      {:else}
        {#each setlistsList as s (s.id)}
          {@const selected = activeSetlistData?.id === s.id}
          <button
            type="button"
            onclick={() => void selectSetlist(s.id)}
            class="w-full min-h-[56px] p-3 rounded-xl border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {selected
              ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500 text-foreground'
              : 'bg-background hover:bg-muted border-border text-foreground'}"
          >
            <span class="flex items-center gap-3 min-w-0">
              <ListMusic size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
              <span class="min-w-0">
                <span class="block text-sm font-bold truncate">{s.title}</span>
                <span class="block text-xs text-muted-foreground">
                  {s.item_count} {s.item_count === 1 ? 'item' : 'itens'}
                  {s.is_active ? ' • Ativa no culto' : ''}
                </span>
              </span>
            </span>
            {#if selected}
              <Check size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
            {/if}
          </button>
        {/each}
      {/if}
    </div>
  </RemoteDialog>

  <!-- Sheet 2: Seletor de Versão da Bíblia -->
  <RemoteDialog
    open={isBibleSelectorOpen}
    variant="sheet"
    title="Versão da Bíblia"
    closeLabel="Fechar seletor de versão bíblica"
    maxWidthClass="max-w-md md:max-w-lg"
    onclose={() => (isBibleSelectorOpen = false)}
  >
    <div class="p-3 space-y-1.5">
      {#if bibles.length === 0}
        <p class="py-8 text-center text-xs text-muted-foreground">
          Nenhuma versão bíblica encontrada no servidor.
        </p>
      {:else}
        {#each bibles as b (b.id)}
          {@const selected = selectedBibleId === b.id}
          <button
            type="button"
            onclick={() => {
              selectedBibleId = b.id;
              isBibleSelectorOpen = false;
              void loadBooks(b.id);
            }}
            class="w-full min-h-[56px] p-3 rounded-xl border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {selected
              ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500 text-foreground'
              : 'bg-background hover:bg-muted border-border text-foreground'}"
          >
            <span class="flex items-center gap-3 min-w-0">
              <BookOpen size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
              <span class="min-w-0">
                <span class="block text-sm font-bold truncate">{b.name}</span>
                <span class="block text-xs text-muted-foreground">{b.abbreviation}</span>
              </span>
            </span>
            {#if selected}
              <Check size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
            {/if}
          </button>
        {/each}
      {/if}
    </div>
  </RemoteDialog>

  <!-- Dialog 3: Desbloqueio de PIN de Operador -->
  <RemoteDialog
    open={isPinModalOpen}
    title="Liberar Controle ao Vivo"
    description="Insira o PIN de operador configurado no OpenPresenter para enviar slides e controlar a projeção neste aparelho."
    closeLabel="Fechar modal de PIN"
    onclose={closeOperatorPinModal}
  >
    <form onsubmit={handleVerifyOperatorPin} class="p-4 space-y-4">
      <div class="flex items-center gap-2 text-xs text-emerald-800 dark:text-emerald-300 bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 rounded-xl p-3">
        <ShieldCheck size={16} class="shrink-0" aria-hidden="true" />
        <span>Solicite o PIN ao líder de mídia ou operador titular da mesa.</span>
      </div>

      <div class="space-y-1.5">
        <label for="remote-unlock-pin" class="block text-xs font-semibold text-foreground">
          PIN do Operador
        </label>
        <div class="relative">
          <input
            id="remote-unlock-pin"
            data-autofocus
            type={showOperatorPin ? 'text' : 'password'}
            inputmode="numeric"
            bind:value={operatorPinInput}
            placeholder="Digite o PIN numérico"
            autocomplete="one-time-code"
            class="w-full h-12 bg-background border border-border rounded-xl px-4 pr-12 text-center text-base sm:text-lg tracking-widest text-foreground font-mono placeholder:text-xs placeholder:tracking-normal focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring"
          />
          <button
            type="button"
            onclick={() => (showOperatorPin = !showOperatorPin)}
            aria-label={showOperatorPin ? 'Ocultar PIN de operador' : 'Mostrar PIN de operador'}
            class="h-10 w-10 rounded-lg absolute right-1 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
          >
            {#if showOperatorPin}
              <EyeOff size={18} aria-hidden="true" />
            {:else}
              <Eye size={18} aria-hidden="true" />
            {/if}
          </button>
        </div>

        {#if operatorPinError}
          <p
            role="alert"
            class="text-xs text-red-700 dark:text-red-300 flex items-center gap-1.5 pt-1 font-medium"
          >
            <AlertTriangle size={14} aria-hidden="true" />
            <span>{operatorPinError}</span>
          </p>
        {/if}
      </div>

      <div class="flex items-center gap-2 pt-1">
        <button
          type="button"
          onclick={closeOperatorPinModal}
          class="flex-1 min-h-11 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold transition-colors cursor-pointer"
        >
          Cancelar
        </button>
        <button
          type="submit"
          disabled={isVerifyingOperatorPin}
          class="flex-1 min-h-11 rounded-xl bg-emerald-700 hover:bg-emerald-800 disabled:opacity-50 text-white text-xs font-bold flex items-center justify-center gap-2 transition-colors cursor-pointer"
        >
          {#if isVerifyingOperatorPin}
            <Loader2 size={15} class="animate-spin" aria-hidden="true" />
          {/if}
          <span>Desbloquear</span>
        </button>
      </div>
    </form>
  </RemoteDialog>

  <!-- Dialog 4: Confirmação para Parar Projeção -->
  <RemoteDialog
    open={isStopConfirmOpen}
    role="alertdialog"
    title="Parar projeção ao vivo?"
    description="Esta ação remove imediatamente o slide atual da tela pública da igreja."
    closeLabel="Fechar confirmação de parada"
    onclose={() => (isStopConfirmOpen = false)}
  >
    <div class="p-4 flex items-center gap-2">
      <button
        type="button"
        data-autofocus
        onclick={() => (isStopConfirmOpen = false)}
        class="flex-1 min-h-11 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold transition-colors cursor-pointer"
      >
        Cancelar
      </button>
      <button
        type="button"
        onclick={() => {
          isStopConfirmOpen = false;
          stopPresentation();
        }}
        class="flex-1 min-h-11 rounded-xl bg-red-700 hover:bg-red-800 text-white text-xs font-bold transition-colors cursor-pointer"
      >
        Parar Projeção
      </button>
    </div>
  </RemoteDialog>
{/if}
