<script lang="ts">
  import { Video, ExternalLink, Copy, Check, Upload, Trash2 } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import SettingsToggle from '../SettingsToggle.svelte';
  import SettingsRangeField from '../SettingsRangeField.svelte';
  import { readFileAsBase64, readHttpError } from '$lib/media';
  import { notifySuccess, notifyError } from '$lib/toast';
  import type { LowerThirdSettings } from '$lib/types';
  import type { SettingsConfirmRequest } from '../types';

  interface Props {
    localLowerThird: LowerThirdSettings;
    onUpdateLowerThird: (next: LowerThirdSettings) => void;
    onSaveLowerThirdNow: (next: LowerThirdSettings) => Promise<void> | void;
    onRequestConfirm: (req: SettingsConfirmRequest) => void;
  }

  let { localLowerThird, onUpdateLowerThird, onSaveLowerThirdNow, onRequestConfirm }: Props =
    $props();

  let ltPreviewMode = $state<'lyrics' | 'scripture' | 'image'>('lyrics');
  let settingsLtBgFileInputEl = $state<HTMLInputElement | null>(null);
  let isUploadingLtBg = $state(false);
  let isCopiedUrl = $state(false);

  const originUrl = $derived(
    typeof window !== 'undefined' ? window.location.origin : 'http://localhost:18080'
  );
  const fullLtUrl = $derived(`${originUrl}/lower-third`);

  function patchLowerThird(patch: Partial<LowerThirdSettings>) {
    onUpdateLowerThird({ ...localLowerThird, ...patch });
  }

  async function copyLowerThirdUrl() {
    if (typeof navigator === 'undefined') return;
    try {
      await navigator.clipboard.writeText(fullLtUrl);
      isCopiedUrl = true;
      notifySuccess('URL copiada para a área de transferência');
      setTimeout(() => (isCopiedUrl = false), 2000);
    } catch {
      notifyError('Não foi possível copiar a URL');
    }
  }

  async function handleLtBgUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    isUploadingLtBg = true;

    try {
      const b64 = await readFileAsBase64(file);
      const res = await fetch('/api/media/upload', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          filename: file.name,
          content_base64: b64,
        }),
      });
      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha ao enviar imagem de fundo'));
      }
      const data = await res.json();
      const updated: LowerThirdSettings = {
        ...localLowerThird,
        box_bg_type: 'image',
        box_bg_image_url: data.url,
      };
      await onSaveLowerThirdNow(updated);
      notifySuccess('Fundo do Lower Third atualizado');
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Erro no upload da imagem';
      notifyError('Falha ao enviar imagem', msg);
    } finally {
      isUploadingLtBg = false;
      input.value = '';
    }
  }

  function requestRemoveBgImage() {
    onRequestConfirm({
      title: 'Remover Imagem de Fundo',
      description: 'Deseja remover a imagem personalizada da caixa do Lower Third?',
      confirmText: 'Remover Imagem',
      variant: 'danger',
      onConfirm: () => {
        patchLowerThird({ box_bg_image_url: '' });
      },
    });
  }
</script>

<div class="space-y-6">
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-3 border-b border-border">
    <div>
      <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
        <Video size={16} class="text-primary shrink-0" />
        <span>Saída Lower Third Transparente para OBS Studio / vMix</span>
      </h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Projeta letras e passagens bíblicas no terço inferior da tela com fundo transparente (canal alfa nativo).
      </p>
    </div>

    <a
      href="/lower-third"
      target="_blank"
      rel="noreferrer"
      class="min-h-8 inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-secondary hover:bg-secondary/80 text-xs font-semibold text-secondary-foreground border border-border transition-colors shrink-0 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    >
      <ExternalLink size={13} />
      <span>Abrir /lower-third</span>
    </a>
  </div>

  <!-- URL e Instruções Rápidas para OBS -->
  <div class="p-3.5 rounded-xl bg-card border border-border space-y-2">
    <label for="lt-browser-source-url" class="text-xs font-bold text-foreground block">
      URL da Fonte de Navegador (Browser Source):
    </label>
    <div class="flex items-center gap-2">
      <input
        id="lt-browser-source-url"
        type="text"
        readonly
        value={fullLtUrl}
        class="flex-1 h-8 px-3 rounded-lg bg-background border border-border text-xs font-mono text-emerald-700 dark:text-emerald-400 select-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      />
      <Button variant="outline" size="sm" onclick={copyLowerThirdUrl} class="shrink-0 gap-1.5 text-xs">
        {#if isCopiedUrl}
          <Check size={13} class="text-emerald-700 dark:text-emerald-400" />
          <span>Copiado</span>
        {:else}
          <Copy size={13} />
          <span>Copiar</span>
        {/if}
      </Button>
    </div>
    <p class="text-[11px] text-muted-foreground">
      No OBS Studio, adicione uma fonte <strong>"Navegador" (Browser Source)</strong> com resolução <strong>1920x1080</strong> e marque <em>"Desligar fonte quando invisível"</em>.
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
    <!-- Coluna de Controles -->
    <div class="lg:col-span-7 space-y-5">
      <div class="p-4 rounded-xl bg-card border border-border flex items-center justify-between gap-4">
        <div>
          <span class="text-xs font-bold text-foreground block">Ativar Saída Lower Third</span>
          <p class="text-[11px] text-muted-foreground">
            Quando desativado, o texto é ocultado automaticamente da transmissão.
          </p>
        </div>
        <SettingsToggle
          checked={localLowerThird.enabled}
          ariaLabel="Ativar Saída Lower Third"
          onToggle={() => patchLowerThird({ enabled: !localLowerThird.enabled })}
        />
      </div>

      <!-- Estilo Visual -->
      <div class="space-y-2">
        <span class="text-xs font-bold text-foreground block">Estilo Visual do Lower Third:</span>
        <div
          class="grid grid-cols-2 sm:grid-cols-4 gap-2"
          role="radiogroup"
          aria-label="Estilo visual do Lower Third"
        >
          {#each [
            { id: 'gradient', title: 'Gradiente Central', sub: 'Suave e dinâmico' },
            { id: 'bar', title: 'Barra Inferior', sub: 'Estilo transmissão' },
            { id: 'floating', title: 'Caixa Flutuante', sub: 'Vidro translúcido' },
            { id: 'minimal', title: 'Minimalista', sub: 'Apenas tipografia' },
          ] as st}
            {@const active = localLowerThird.theme === st.id}
            <button
              type="button"
              role="radio"
              aria-checked={active}
              onclick={() => patchLowerThird({ theme: st.id as LowerThirdSettings['theme'] })}
              class="p-2.5 rounded-xl border text-left transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                ? 'border-primary bg-primary/10 text-foreground font-bold'
                : 'border-border bg-card text-muted-foreground hover:text-foreground'}"
            >
              <span class="text-xs block">{st.title}</span>
              <span class="text-[11px] opacity-75 font-normal">{st.sub}</span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Cores e Composição de Fundo -->
      <div class="p-4 rounded-xl bg-card border border-border space-y-4">
        <h4 class="text-xs font-bold text-foreground">Cores & Composição de Fundo</h4>

        <div class="space-y-1.5">
          <span class="text-xs font-semibold text-foreground block">Tipo de Fundo da Caixa:</span>
          <div class="grid grid-cols-2 gap-2" role="radiogroup" aria-label="Tipo de fundo da caixa">
            <button
              type="button"
              role="radio"
              aria-checked={(localLowerThird.box_bg_type || 'color') === 'color'}
              onclick={() => patchLowerThird({ box_bg_type: 'color' })}
              class="min-h-8 p-2 rounded-lg border text-xs font-semibold transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localLowerThird.box_bg_type || 'color') === 'color'
                ? 'border-primary bg-primary/15 text-primary'
                : 'border-border bg-background text-muted-foreground hover:text-foreground'}"
            >
              Cor & Gradiente
            </button>
            <button
              type="button"
              role="radio"
              aria-checked={localLowerThird.box_bg_type === 'image'}
              onclick={() => patchLowerThird({ box_bg_type: 'image' })}
              class="min-h-8 p-2 rounded-lg border text-xs font-semibold transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {localLowerThird.box_bg_type === 'image'
                ? 'border-primary bg-primary/15 text-primary'
                : 'border-border bg-background text-muted-foreground hover:text-foreground'}"
            >
              Imagem Personalizada (PNG/JPG)
            </button>
          </div>
        </div>

        {#if localLowerThird.box_bg_type === 'image'}
          <div class="p-3 rounded-lg border border-border bg-background/70 space-y-3">
            <div class="flex items-center justify-between gap-3">
              <div class="flex items-center gap-2 min-w-0">
                {#if localLowerThird.box_bg_image_url}
                  <img
                    src={localLowerThird.box_bg_image_url}
                    alt="Fundo Lower Third"
                    class="w-12 h-8 object-cover rounded border border-border shrink-0"
                  />
                  <span class="text-xs text-foreground truncate max-w-xs">
                    {localLowerThird.box_bg_image_url.split('/').pop()}
                  </span>
                {:else}
                  <div class="w-12 h-8 rounded border border-dashed border-border flex items-center justify-center text-muted-foreground text-[11px] shrink-0">
                    Vazio
                  </div>
                  <span class="text-xs text-muted-foreground">Nenhuma imagem selecionada</span>
                {/if}
              </div>

              <div class="flex items-center gap-1.5 shrink-0">
                <input
                  type="file"
                  accept="image/png,image/jpeg,image/webp,image/svg+xml"
                  bind:this={settingsLtBgFileInputEl}
                  onchange={handleLtBgUpload}
                  class="hidden"
                />
                <Button
                  variant="outline"
                  size="sm"
                  class="h-7 text-xs gap-1"
                  onclick={() => settingsLtBgFileInputEl?.click()}
                  disabled={isUploadingLtBg}
                >
                  <Upload size={12} />
                  <span>
                    {isUploadingLtBg
                      ? 'Enviando...'
                      : localLowerThird.box_bg_image_url
                        ? 'Alterar'
                        : 'Enviar'}
                  </span>
                </Button>
                {#if localLowerThird.box_bg_image_url}
                  <button
                    type="button"
                    onclick={requestRemoveBgImage}
                    aria-label="Remover imagem de fundo"
                    class="w-7 h-7 rounded-md hover:bg-red-500/10 text-muted-foreground hover:text-red-700 dark:hover:text-red-400 flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    title="Remover imagem de fundo"
                  >
                    <Trash2 size={14} />
                  </button>
                {/if}
              </div>
            </div>

            <SettingsRangeField
              id="lt-box-radius"
              label="Arredondamento das Bordas"
              value={localLowerThird.box_border_radius ?? 16}
              min={0}
              max={48}
              step={2}
              unit="px"
              onchange={(v) => patchLowerThird({ box_border_radius: v })}
            />

            <div class="flex items-center justify-between pt-2 border-t border-border gap-4">
              <div>
                <span class="text-xs font-semibold text-foreground block">
                  Suavizar Bordas com Fade (Feather)
                </span>
                <span class="text-[11px] text-muted-foreground">
                  Aplica desvanecimento radial suave nas extremidades da caixa.
                </span>
              </div>
              <SettingsToggle
                checked={Boolean(localLowerThird.box_edge_fade)}
                ariaLabel="Suavizar Bordas com Fade"
                onToggle={() => patchLowerThird({ box_edge_fade: !localLowerThird.box_edge_fade })}
              />
            </div>

            {#if localLowerThird.box_edge_fade}
              <SettingsRangeField
                id="lt-edge-fade-amount"
                label="Intensidade do Fade"
                value={localLowerThird.box_edge_fade_amount ?? 20}
                min={5}
                max={40}
                step={5}
                unit="%"
                onchange={(v) => patchLowerThird({ box_edge_fade_amount: v })}
              />
            {/if}
          </div>
        {:else}
          <div class="flex items-center justify-between gap-4">
            <div>
              <span class="text-xs font-semibold text-foreground block">
                Vincular Cor ao Fundo Público Ativo
              </span>
              <span class="text-[11px] text-muted-foreground">
                Adapta automaticamente o gradiente à cor do tema de projeção.
              </span>
            </div>
            <SettingsToggle
              checked={localLowerThird.use_public_bg}
              ariaLabel="Vincular Cor ao Fundo Público Ativo"
              onToggle={() => patchLowerThird({ use_public_bg: !localLowerThird.use_public_bg })}
            />
          </div>

          {#if !localLowerThird.use_public_bg}
            <div class="flex items-center justify-between pt-2 border-t border-border gap-2">
              <label for="lt-bg-color-hex" class="text-xs text-foreground">
                Cor Base do Gradiente / Barra:
              </label>
              <div class="flex items-center gap-2">
                <input
                  type="color"
                  aria-label="Seletor de cor base do Lower Third"
                  value={localLowerThird.bg_color}
                  oninput={(e) => patchLowerThird({ bg_color: (e.target as HTMLInputElement).value })}
                  class="w-8 h-8 rounded border border-border bg-background p-0.5 cursor-pointer"
                />
                <input
                  id="lt-bg-color-hex"
                  type="text"
                  value={localLowerThird.bg_color}
                  oninput={(e) => patchLowerThird({ bg_color: (e.target as HTMLInputElement).value })}
                  class="w-24 h-8 px-2 rounded-md bg-background border border-border text-xs font-mono uppercase text-center text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                />
              </div>
            </div>
          {/if}
        {/if}

        <SettingsRangeField
          id="lt-bg-opacity"
          label="Opacidade do Fundo"
          value={localLowerThird.bg_opacity}
          min={10}
          max={100}
          step={5}
          unit="%"
          onchange={(v) => patchLowerThird({ bg_opacity: v })}
        />

        <SettingsRangeField
          id="lt-margin-bottom"
          label="Distância da Margem Inferior"
          value={localLowerThird.margin_bottom}
          min={16}
          max={80}
          step={4}
          unit="px"
          onchange={(v) => patchLowerThird({ margin_bottom: v })}
        />

        <SettingsRangeField
          id="lt-max-width"
          label="Aproveitamento Máximo de Largura"
          value={localLowerThird.max_width_percent}
          min={50}
          max={95}
          step={5}
          unit="%"
          onchange={(v) => patchLowerThird({ max_width_percent: v })}
        />
      </div>

      <!-- Tipografia e Sombra -->
      <div class="p-4 rounded-xl bg-card border border-border space-y-4">
        <h4 class="text-xs font-bold text-foreground">Tipografia & Contraste para Transmissão</h4>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
          <SettingsRangeField
            id="lt-song-font-size"
            label="Fonte de Músicas"
            value={localLowerThird.font_size}
            min={18}
            max={60}
            step={2}
            unit="px"
            onchange={(v) => patchLowerThird({ font_size: v })}
          />

          <SettingsRangeField
            id="lt-scripture-font-size"
            label="Fonte de Versículos"
            value={localLowerThird.scripture_font_size ?? 28}
            min={16}
            max={56}
            step={2}
            unit="px"
            onchange={(v) => patchLowerThird({ scripture_font_size: v })}
          />
        </div>

        <div class="grid grid-cols-2 gap-3 pt-1">
          <div>
            <label for="lt-text-align" class="text-xs text-foreground block mb-1">Alinhamento:</label>
            <select
              id="lt-text-align"
              value={localLowerThird.text_align}
              onchange={(e) =>
                patchLowerThird({
                  text_align: (e.target as HTMLSelectElement).value as LowerThirdSettings['text_align'],
                })}
              class="w-full h-8 px-2.5 rounded-lg bg-background border border-border text-xs text-foreground font-semibold focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="left">Esquerda</option>
              <option value="center">Centralizado</option>
              <option value="right">Direita</option>
            </select>
          </div>

          <div>
            <label for="lt-font-weight" class="text-xs text-foreground block mb-1">
              Peso da Fonte:
            </label>
            <select
              id="lt-font-weight"
              value={localLowerThird.font_weight}
              onchange={(e) =>
                patchLowerThird({
                  font_weight: (e.target as HTMLSelectElement).value as LowerThirdSettings['font_weight'],
                })}
              class="w-full h-8 px-2.5 rounded-lg bg-background border border-border text-xs text-foreground font-semibold focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="normal">Normal (400)</option>
              <option value="semibold">Semibold (600)</option>
              <option value="bold">Negrito (700)</option>
              <option value="extrabold">Extra Negrito (800)</option>
            </select>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-3 pt-1">
          <div>
            <label for="lt-shadow-strength" class="text-xs text-foreground block mb-1">
              Força da Sombra do Texto:
            </label>
            <select
              id="lt-shadow-strength"
              value={localLowerThird.shadow_strength}
              onchange={(e) =>
                patchLowerThird({
                  shadow_strength: (e.target as HTMLSelectElement)
                    .value as LowerThirdSettings['shadow_strength'],
                })}
              class="w-full h-8 px-2.5 rounded-lg bg-background border border-border text-xs text-foreground font-semibold focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="none">Nenhuma</option>
              <option value="subtle">Suave</option>
              <option value="medium">Média</option>
              <option value="strong">Forte (Alto Contraste)</option>
            </select>
          </div>

          <div>
            <label for="lt-animation" class="text-xs text-foreground block mb-1">
              Animação de Transição:
            </label>
            <select
              id="lt-animation"
              value={localLowerThird.animation}
              onchange={(e) =>
                patchLowerThird({
                  animation: (e.target as HTMLSelectElement)
                    .value as LowerThirdSettings['animation'],
                })}
              class="w-full h-8 px-2.5 rounded-lg bg-background border border-border text-xs text-foreground font-semibold focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="fade">Fade Suave</option>
              <option value="slide">Deslize Vertical (Slide)</option>
              <option value="zoom">Zoom Leve</option>
              <option value="none">Instantânea</option>
            </select>
          </div>
        </div>

        <div class="flex items-center justify-between pt-2 border-t border-border gap-4">
          <div>
            <span class="text-xs font-semibold text-foreground block">
              Exibir Referência Bíblica / Título
            </span>
            <span class="text-[11px] text-muted-foreground">
              Mostra etiqueta identificadora no topo do Lower Third.
            </span>
          </div>
          <SettingsToggle
            checked={localLowerThird.show_reference}
            ariaLabel="Exibir Referência Bíblica ou Título"
            onToggle={() => patchLowerThird({ show_reference: !localLowerThird.show_reference })}
          />
        </div>

        <!-- Exibição de Imagens no Lower Third -->
        <div class="pt-3 border-t border-border space-y-3">
          <div>
            <span class="text-xs font-bold text-foreground block">
              Exibição de Imagens no Lower Third
            </span>
            <span class="text-[11px] text-muted-foreground">
              Define como imagens projetadas serão exibidas na saída Lower Third (OBS).
            </span>
          </div>

          <div class="space-y-1.5">
            <span class="text-[11px] font-medium text-muted-foreground block">
              Modo de Exibição de Imagens:
            </span>
            <div class="grid grid-cols-3 gap-1.5" role="radiogroup" aria-label="Modo de exibição de imagens">
              <button
                type="button"
                role="radio"
                aria-checked={(localLowerThird.image_display_mode ?? 'fullscreen') === 'fullscreen'}
                onclick={() => patchLowerThird({ image_display_mode: 'fullscreen' })}
                class="min-h-7 px-2.5 py-1.5 rounded-lg text-[11px] font-semibold border transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localLowerThird.image_display_mode ??
                  'fullscreen') === 'fullscreen'
                  ? 'bg-primary text-primary-foreground border-primary'
                  : 'bg-background text-muted-foreground border-border hover:text-foreground'}"
              >
                Tela Cheia
              </button>
              <button
                type="button"
                role="radio"
                aria-checked={(localLowerThird.image_display_mode ?? 'fullscreen') === 'lower_third'}
                onclick={() => patchLowerThird({ image_display_mode: 'lower_third' })}
                class="min-h-7 px-2.5 py-1.5 rounded-lg text-[11px] font-semibold border transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localLowerThird.image_display_mode ??
                  'fullscreen') === 'lower_third'
                  ? 'bg-primary text-primary-foreground border-primary'
                  : 'bg-background text-muted-foreground border-border hover:text-foreground'}"
              >
                Banner
              </button>
              <button
                type="button"
                role="radio"
                aria-checked={(localLowerThird.image_display_mode ?? 'fullscreen') === 'hide'}
                onclick={() => patchLowerThird({ image_display_mode: 'hide' })}
                class="min-h-7 px-2.5 py-1.5 rounded-lg text-[11px] font-semibold border transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localLowerThird.image_display_mode ??
                  'fullscreen') === 'hide'
                  ? 'bg-red-600 text-white border-red-500'
                  : 'bg-background text-muted-foreground border-border hover:text-foreground'}"
              >
                Ocultar
              </button>
            </div>
            <p class="text-[11px] text-muted-foreground leading-relaxed">
              {#if (localLowerThird.image_display_mode ?? 'fullscreen') === 'fullscreen'}
                A imagem projetada será exibida em tela cheia sobre a transmissão, ideal para avisos, cartazes e flyers.
              {:else if (localLowerThird.image_display_mode ?? 'fullscreen') === 'lower_third'}
                A imagem será exibida como banner no terço inferior da tela, ajustada dentro da barra do Lower Third.
              {:else}
                Imagens projetadas não serão exibidas na saída Lower Third.
              {/if}
            </p>
          </div>

          {#if (localLowerThird.image_display_mode ?? 'fullscreen') !== 'hide'}
            <div class="space-y-1.5">
              <span class="text-[11px] font-medium text-muted-foreground block">
                Ajuste da Imagem:
              </span>
              <div class="inline-flex rounded-lg bg-muted/50 border border-border p-0.5">
                <button
                  type="button"
                  aria-pressed={(localLowerThird.image_fit ?? 'contain') === 'contain'}
                  onclick={() => patchLowerThird({ image_fit: 'contain' })}
                  class="min-h-7 px-3 py-1 text-[11px] font-semibold rounded transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localLowerThird.image_fit ??
                    'contain') === 'contain'
                    ? 'bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:text-foreground'}"
                >
                  Conter (Contain)
                </button>
                <button
                  type="button"
                  aria-pressed={(localLowerThird.image_fit ?? 'contain') === 'cover'}
                  onclick={() => patchLowerThird({ image_fit: 'cover' })}
                  class="min-h-7 px-3 py-1 text-[11px] font-semibold rounded transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localLowerThird.image_fit ??
                    'contain') === 'cover'
                    ? 'bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:text-foreground'}"
                >
                  Cobrir (Cover)
                </button>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Coluna de Prévia Interativa 16:9 -->
    <div class="lg:col-span-5 space-y-3">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <span class="text-xs font-bold text-foreground">Prévia 16:9 (Simulação OBS):</span>
        <div
          class="inline-flex rounded-lg bg-muted/50 border border-border p-0.5"
          role="group"
          aria-label="Conteúdo da prévia Lower Third"
        >
          <button
            type="button"
            aria-pressed={ltPreviewMode === 'lyrics'}
            onclick={() => (ltPreviewMode = 'lyrics')}
            class="min-h-7 px-2.5 py-0.5 text-[11px] font-semibold rounded cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {ltPreviewMode ===
            'lyrics'
              ? 'bg-primary text-primary-foreground'
              : 'text-muted-foreground hover:text-foreground'}"
          >
            Louvor
          </button>
          <button
            type="button"
            aria-pressed={ltPreviewMode === 'scripture'}
            onclick={() => (ltPreviewMode = 'scripture')}
            class="min-h-7 px-2.5 py-0.5 text-[11px] font-semibold rounded cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {ltPreviewMode ===
            'scripture'
              ? 'bg-primary text-primary-foreground'
              : 'text-muted-foreground hover:text-foreground'}"
          >
            Bíblia
          </button>
          <button
            type="button"
            aria-pressed={ltPreviewMode === 'image'}
            onclick={() => (ltPreviewMode = 'image')}
            class="min-h-7 px-2.5 py-0.5 text-[11px] font-semibold rounded cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {ltPreviewMode ===
            'image'
              ? 'bg-primary text-primary-foreground'
              : 'text-muted-foreground hover:text-foreground'}"
          >
            Imagem
          </button>
        </div>
      </div>

      <!-- Canvas 16:9 de Câmera OBS -->
      <div class="relative w-full aspect-video rounded-xl overflow-hidden border border-border shadow-2xl flex flex-col justify-end bg-gradient-to-tr from-slate-900 via-indigo-950 to-zinc-900">
        <div
          aria-hidden="true"
          class="absolute inset-0 pointer-events-none"
          style="background: radial-gradient(circle at center, transparent 0%, rgba(0, 0, 0, 0.2) 55%, rgba(0, 0, 0, 0.6) 100%);"
        ></div>

        <div class="absolute top-2 left-2 px-2 py-0.5 rounded bg-red-600/90 text-white font-mono text-[11px] font-bold uppercase tracking-wider flex items-center gap-1.5 z-10">
          <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
          <span>LIVE OBS</span>
        </div>

        <div
          class="w-full flex flex-col items-center justify-end z-10 p-3 transition-all"
          style="padding-bottom: {Math.max(8, Math.round((localLowerThird.margin_bottom ?? 36) * 0.35))}px;"
        >
          <div class="w-full transition-all" style="max-width: {localLowerThird.max_width_percent ?? 85}%;">
            {#if localLowerThird.enabled}
              {#if ltPreviewMode === 'image'}
                {#if (localLowerThird.image_display_mode ?? 'fullscreen') === 'hide'}
                  <div class="p-3 text-center text-[11px] text-zinc-300 italic">
                    Imagens ocultas no Lower Third.
                  </div>
                {:else if (localLowerThird.image_display_mode ?? 'fullscreen') === 'fullscreen'}
                  <div class="absolute inset-0 z-20 flex items-center justify-center bg-black/35">
                    <div class="w-3/4 h-3/4 bg-gradient-to-br from-blue-900/70 via-purple-900/70 to-pink-900/70 rounded-lg border border-white/25 flex items-center justify-center">
                      <span class="text-xs text-white font-bold">Imagem em Tela Cheia</span>
                    </div>
                  </div>
                {:else}
                  <div class="max-h-10 overflow-hidden rounded-lg border border-white/25 bg-black/70 flex items-center justify-center mx-auto w-2/3">
                    <span class="text-[11px] text-white font-bold py-1">Banner Imagem</span>
                  </div>
                {/if}
              {:else}
                {@const effectivePreviewFontSize =
                  ltPreviewMode === 'scripture'
                    ? Math.max(11, Math.round((localLowerThird.scripture_font_size ?? 28) * 0.42))
                    : Math.max(11, Math.round((localLowerThird.font_size ?? 32) * 0.42))}
                {@const previewHasImage =
                  localLowerThird.box_bg_type === 'image' && !!localLowerThird.box_bg_image_url}
                {@const previewBorderRadius = localLowerThird.box_border_radius ?? 16}
                {@const previewFadeMask =
                  localLowerThird.box_edge_fade ?? false
                    ? `radial-gradient(ellipse at center, black ${Math.max(
                        10,
                        100 - (localLowerThird.box_edge_fade_amount ?? 30) * 1.8
                      )}%, transparent 100%)`
                    : undefined}

                {#if localLowerThird.theme === 'gradient'}
                  <div
                    class="px-4 py-2.5 text-center flex flex-col items-center justify-center overflow-hidden transition-all"
                    style="background: radial-gradient(ellipse at center, rgba(0,0,0,{(localLowerThird.bg_opacity ?? 75) / 100}) 0%, rgba(0,0,0,{((localLowerThird.bg_opacity ?? 75) / 100) * 0.75}) 40%, rgba(0,0,0,{((localLowerThird.bg_opacity ?? 75) / 100) * 0.2}) 70%, transparent 100%); -webkit-mask-image: radial-gradient(ellipse at center, black 55%, transparent 100%); mask-image: radial-gradient(ellipse at center, black 55%, transparent 100%);"
                  >
                    {#if localLowerThird.show_reference}
                      <span class="px-2 py-0.5 rounded-full text-[11px] font-bold uppercase tracking-wider mb-1 bg-black/65 border border-white/25 text-white shadow">
                        {ltPreviewMode === 'scripture' ? 'João 3:16 (ARA)' : 'Porque Ele Vive'}
                      </span>
                    {/if}
                    <p
                      class="leading-tight {localLowerThird.text_align === 'left'
                        ? 'text-left'
                        : localLowerThird.text_align === 'right'
                          ? 'text-right'
                          : 'text-center'}"
                      style="font-size: {effectivePreviewFontSize}px; font-weight: {localLowerThird.font_weight === 'extrabold'
                        ? '800'
                        : localLowerThird.font_weight === 'bold'
                          ? '700'
                          : localLowerThird.font_weight === 'semibold'
                            ? '600'
                            : '400'}; color: {localLowerThird.text_color || '#ffffff'}; text-shadow: 0 1px 3px rgba(0,0,0,0.9);"
                    >
                      {ltPreviewMode === 'scripture'
                        ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                        : 'Porque Ele vive, eu posso crer no amanhã'}
                    </p>
                  </div>
                {:else if localLowerThird.theme === 'bar'}
                  <div class="w-full rounded-lg bg-zinc-950/85 shadow-lg overflow-hidden flex items-stretch">
                    <span aria-hidden="true" class="w-1 bg-primary shrink-0"></span>
                    <div class="flex-1 px-3.5 py-2 text-left">
                      {#if localLowerThird.show_reference}
                        <span class="text-[11px] font-bold uppercase text-emerald-400 block">
                          {ltPreviewMode === 'scripture' ? 'João 3:16 (ARA)' : 'Porque Ele Vive'}
                        </span>
                      {/if}
                      <p
                        class="leading-tight {localLowerThird.text_align === 'left'
                          ? 'text-left'
                          : localLowerThird.text_align === 'right'
                            ? 'text-right'
                            : 'text-center'}"
                        style="font-size: {effectivePreviewFontSize}px; font-weight: {localLowerThird.font_weight === 'extrabold'
                          ? '800'
                          : localLowerThird.font_weight === 'bold'
                            ? '700'
                            : localLowerThird.font_weight === 'semibold'
                              ? '600'
                              : '400'}; color: {localLowerThird.text_color || '#ffffff'};"
                      >
                        {ltPreviewMode === 'scripture'
                          ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                          : 'Porque Ele vive, eu posso crer no amanhã'}
                      </p>
                    </div>
                  </div>
                {:else if localLowerThird.theme === 'floating'}
                  <div
                    class="px-4 py-2 border border-white/20 shadow-lg backdrop-blur-sm relative overflow-hidden transition-all {localLowerThird.text_align === 'left'
                      ? 'text-left mr-auto'
                      : localLowerThird.text_align === 'right'
                        ? 'text-right ml-auto'
                        : 'text-center mx-auto'}"
                    style="border-radius: {previewBorderRadius}px; {previewFadeMask ? `-webkit-mask-image: ${previewFadeMask}; mask-image: ${previewFadeMask};` : ''} background-color: {previewHasImage ? 'transparent' : 'rgba(0,0,0,0.72)'};"
                  >
                    {#if previewHasImage}
                      <img
                        src={localLowerThird.box_bg_image_url}
                        alt=""
                        class="absolute inset-0 w-full h-full object-cover pointer-events-none -z-10"
                        style="opacity: {(localLowerThird.bg_opacity ?? 75) / 100}; border-radius: {previewBorderRadius}px;"
                      />
                    {/if}
                    {#if localLowerThird.show_reference}
                      <span class="text-[11px] font-bold uppercase text-white/85 block mb-0.5">
                        {ltPreviewMode === 'scripture' ? 'João 3:16' : 'Porque Ele Vive'}
                      </span>
                    {/if}
                    <p
                      class="leading-tight {localLowerThird.text_align === 'left'
                        ? 'text-left'
                        : localLowerThird.text_align === 'right'
                          ? 'text-right'
                          : 'text-center'}"
                      style="font-size: {effectivePreviewFontSize}px; font-weight: {localLowerThird.font_weight === 'extrabold'
                        ? '800'
                        : localLowerThird.font_weight === 'bold'
                          ? '700'
                          : localLowerThird.font_weight === 'semibold'
                            ? '600'
                            : '400'}; color: {localLowerThird.text_color || '#ffffff'};"
                    >
                      {ltPreviewMode === 'scripture'
                        ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                        : 'Porque Ele vive, eu posso crer no amanhã'}
                    </p>
                  </div>
                {:else}
                  <div class="text-center w-full">
                    {#if localLowerThird.show_reference}
                      <span class="text-[11px] font-bold uppercase text-amber-300 block mb-0.5 drop-shadow">
                        {ltPreviewMode === 'scripture' ? 'João 3:16' : 'Porque Ele Vive'}
                      </span>
                    {/if}
                    <p
                      class="leading-tight drop-shadow-lg {localLowerThird.text_align === 'left'
                        ? 'text-left'
                        : localLowerThird.text_align === 'right'
                          ? 'text-right'
                          : 'text-center'}"
                      style="font-size: {effectivePreviewFontSize}px; font-weight: {localLowerThird.font_weight === 'extrabold'
                        ? '800'
                        : localLowerThird.font_weight === 'bold'
                          ? '700'
                          : localLowerThird.font_weight === 'semibold'
                            ? '600'
                            : '400'}; color: {localLowerThird.text_color || '#ffffff'}; text-shadow: 0 2px 6px rgba(0,0,0,1);"
                    >
                      {ltPreviewMode === 'scripture'
                        ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                        : 'Porque Ele vive, eu posso crer no amanhã'}
                    </p>
                  </div>
                {/if}
              {/if}
            {:else}
              <div class="p-3 text-center text-[11px] text-zinc-300 italic">
                Lower Third desativado.
              </div>
            {/if}
          </div>
        </div>
      </div>

      <p class="text-[11px] text-muted-foreground">
        A visualização acima simula como o texto aparecerá no OBS Studio sobreposto ao feed da câmera de transmissão.
      </p>
    </div>
  </div>
</div>
