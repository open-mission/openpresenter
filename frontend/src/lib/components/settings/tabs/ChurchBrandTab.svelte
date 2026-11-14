<script lang="ts">
  import { Church, Upload, Check, AlertCircle } from 'lucide-svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import SettingsToggle from '../SettingsToggle.svelte';
  import SettingsRangeField from '../SettingsRangeField.svelte';
  import { readFileAsDataUrl, readHttpError } from '$lib/media';
  import { notifyError, notifySuccess } from '$lib/toast';
  import type { ChurchBrandSettings } from '$lib/types';
  import type { SettingsConfirmRequest } from '../types';

  interface Props {
    localBrand: ChurchBrandSettings;
    onUpdateField: <K extends keyof ChurchBrandSettings>(key: K, val: ChurchBrandSettings[K]) => void;
    onSaveBrandNow: (next: ChurchBrandSettings) => Promise<void> | void;
    onRequestConfirm: (req: SettingsConfirmRequest) => void;
  }

  let { localBrand, onUpdateField, onSaveBrandNow, onRequestConfirm }: Props = $props();

  type LogoVariant = 'primary' | 'dark' | 'light' | 'full';

  let uploadStatus = $state<Partial<Record<LogoVariant, { ok: boolean; msg: string }>>>({});

  const logoVariants: Array<{ key: LogoVariant; label: string; desc: string; previewSurface: string }> = [
    { key: 'primary', label: 'Logo Principal', desc: 'Símbolo ou ícone padrão', previewSurface: 'bg-muted/60' },
    { key: 'dark', label: 'Versão Escura', desc: 'Para fundos muito claros', previewSurface: 'bg-zinc-100' },
    { key: 'light', label: 'Versão Clara / Branca', desc: 'Para fundos escuros e projeção', previewSurface: 'bg-zinc-900' },
    { key: 'full', label: 'Logo Completo (Horizontal)', desc: 'Símbolo com nome por extenso', previewSurface: 'bg-muted/60' },
  ];

  function getLogoUrl(variant: LogoVariant): string {
    if (variant === 'primary') return localBrand.logo_primary || '';
    if (variant === 'dark') return localBrand.logo_dark || '';
    if (variant === 'light') return localBrand.logo_light || '';
    return localBrand.logo_full || '';
  }

  function logoKey(variant: LogoVariant): 'logo_primary' | 'logo_dark' | 'logo_light' | 'logo_full' {
    return `logo_${variant}`;
  }

  async function handleLogoUpload(variant: LogoVariant, e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    uploadStatus = { ...uploadStatus, [variant]: { ok: true, msg: 'Enviando...' } };

    try {
      const dataUrl = await readFileAsDataUrl(file);
      const commaIdx = dataUrl.indexOf(',');
      const base64 = commaIdx >= 0 ? dataUrl.slice(commaIdx + 1) : dataUrl;

      const res = await fetch('/api/church/upload-logo', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          filename: file.name,
          content_base64: base64,
        }),
      });

      if (!res.ok) {
        throw new Error(await readHttpError(res, 'Falha no upload do logotipo'));
      }

      const data = await res.json();
      const updated: ChurchBrandSettings = {
        ...localBrand,
        [logoKey(variant)]: data.url,
      };
      await onSaveBrandNow(updated);

      uploadStatus = { ...uploadStatus, [variant]: { ok: true, msg: 'Salvo!' } };
      notifySuccess('Logotipo atualizado');
      setTimeout(() => {
        uploadStatus = { ...uploadStatus, [variant]: undefined };
      }, 3000);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Falha no upload';
      uploadStatus = { ...uploadStatus, [variant]: { ok: false, msg: `Erro: ${msg}` } };
      notifyError('Erro ao enviar logotipo', msg);
    } finally {
      input.value = '';
    }
  }

  function requestRemoveLogo(variant: LogoVariant, label: string) {
    onRequestConfirm({
      title: 'Remover Logotipo',
      description: `Deseja remover "${label}" das configurações de marca da igreja?`,
      confirmText: 'Remover Logo',
      variant: 'danger',
      onConfirm: () => {
        onUpdateField(logoKey(variant), '');
        notifySuccess('Logotipo removido', label);
      },
    });
  }
</script>

<div class="space-y-6 w-full max-w-4xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Identidade Visual da Igreja</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Faça o upload dos logotipos oficiais da sua igreja para sobreposição nas projeções públicas e transmissões.
    </p>
  </div>

  <!-- Nome da Igreja -->
  <div class="p-3.5 rounded-xl border border-border bg-card space-y-2">
    <label for="church-name-input" class="block text-xs font-semibold text-foreground">
      Nome da Congregação / Ministério
    </label>
    <input
      id="church-name-input"
      type="text"
      value={localBrand.name}
      oninput={(e) => onUpdateField('name', (e.target as HTMLInputElement).value)}
      placeholder="Ex: Igreja Batista Central"
      class="w-full h-8 px-2.5 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    />
  </div>

  <!-- Cards de Upload dos 4 formatos de Logotipo -->
  <div class="space-y-2.5">
    <span class="block text-xs font-semibold text-foreground">Arquivos de Logotipo</span>
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      {#each logoVariants as variant}
        {@const url = getLogoUrl(variant.key)}
        {@const st = uploadStatus[variant.key]}
        <div class="p-3.5 rounded-xl border border-border bg-card flex flex-col justify-between gap-3">
          <div class="flex items-center justify-between gap-2">
            <div>
              <div class="text-xs font-semibold text-foreground">{variant.label}</div>
              <div class="text-[11px] text-muted-foreground">{variant.desc}</div>
            </div>
            {#if url}
              <Badge
                variant="outline"
                class="text-[11px] text-emerald-700 dark:text-emerald-400 border-emerald-500/35 shrink-0"
              >
                Ativo
              </Badge>
            {/if}
          </div>

          <!-- Preview com fundo adequado para contraste da variação -->
          <div
            class="h-20 w-full rounded-lg border border-border flex items-center justify-center p-2 relative overflow-hidden {variant.previewSurface}"
          >
            {#if url}
              <img src={url} alt={variant.label} class="max-h-full max-w-full object-contain" />
            {:else}
              <div class="text-[11px] text-muted-foreground flex flex-col items-center gap-1">
                <Church size={18} class="opacity-60" />
                <span>Sem arquivo</span>
              </div>
            {/if}
          </div>

          <div class="flex items-center justify-between gap-2 pt-1">
            <label class="cursor-pointer inline-flex">
              <input
                type="file"
                accept="image/*"
                class="sr-only"
                onchange={(e) => handleLogoUpload(variant.key, e)}
              />
              <span class="min-h-7 px-2.5 py-1 text-xs rounded-md bg-secondary hover:bg-secondary/80 text-secondary-foreground font-medium inline-flex items-center gap-1.5 transition-colors border border-border">
                <Upload size={12} />
                <span>{url ? 'Substituir...' : 'Enviar...'}</span>
              </span>
            </label>

            {#if url}
              <button
                type="button"
                onclick={() => requestRemoveLogo(variant.key, variant.label)}
                class="min-h-7 px-2 rounded-md text-xs text-muted-foreground hover:text-red-700 dark:hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              >
                Remover
              </button>
            {/if}
          </div>

          {#if st}
            <div
              role={st.ok ? 'status' : 'alert'}
              class="text-[11px] flex items-center gap-1.5 {st.ok
                ? 'text-emerald-700 dark:text-emerald-400'
                : 'text-red-700 dark:text-red-300 font-medium'}"
            >
              {#if st.ok}
                <Check size={12} class="shrink-0" />
              {:else}
                <AlertCircle size={12} class="shrink-0" />
              {/if}
              <span>{st.msg}</span>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>

  <!-- Configuração de Exibição no Público -->
  <div class="pt-4 border-t border-border space-y-4">
    <div>
      <h4 class="text-xs font-semibold text-foreground">Sobreposição na Tela Pública</h4>
      <p class="text-[11px] text-muted-foreground mt-0.5">
        Defina se e onde o logotipo oficial da igreja ficará fixado durante a projeção.
      </p>
    </div>

    <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
      <div>
        <div class="text-xs font-semibold text-foreground">Exibir Logotipo na Tela Pública</div>
        <p class="text-[11px] text-muted-foreground mt-0.5">
          Mantém o logo visível sobre os slides e vídeos da projeção.
        </p>
      </div>
      <SettingsToggle
        checked={localBrand.show_logo_on_public}
        ariaLabel="Alternar exibição de logotipo na tela pública"
        onToggle={() => onUpdateField('show_logo_on_public', !localBrand.show_logo_on_public)}
      />
    </div>

    {#if localBrand.show_logo_on_public}
      <div class="space-y-1.5">
        <span class="text-xs font-medium text-foreground block">Variante a Exibir</span>
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2" role="radiogroup" aria-label="Variante do logotipo">
          {#each [
            { key: 'primary', label: 'Principal' },
            { key: 'light', label: 'Clara / Branca' },
            { key: 'dark', label: 'Escura' },
            { key: 'full', label: 'Completa' },
          ] as opt}
            {@const active = localBrand.logo_variant === opt.key}
            <button
              type="button"
              role="radio"
              aria-checked={active}
              onclick={() => onUpdateField('logo_variant', opt.key as ChurchBrandSettings['logo_variant'])}
              class="min-h-8 p-2 rounded-lg border text-xs font-medium text-center transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                ? 'border-primary bg-primary/10 text-primary font-bold'
                : 'border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
            >
              {opt.label}
            </button>
          {/each}
        </div>
      </div>

      <div class="space-y-1.5">
        <span class="text-xs font-medium text-foreground block">Posição no Slide</span>
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2" role="radiogroup" aria-label="Posição do logotipo no slide">
          {#each [
            { key: 'top-left', label: 'Superior Esquerdo' },
            { key: 'top-right', label: 'Superior Direito' },
            { key: 'bottom-left', label: 'Inferior Esquerdo' },
            { key: 'bottom-right', label: 'Inferior Direito' },
          ] as pos}
            {@const active = localBrand.logo_position === pos.key}
            <button
              type="button"
              role="radio"
              aria-checked={active}
              onclick={() => onUpdateField('logo_position', pos.key as ChurchBrandSettings['logo_position'])}
              class="min-h-8 p-2.5 rounded-lg border text-xs font-medium text-center transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                ? 'border-primary bg-primary/10 text-primary font-bold'
                : 'border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
            >
              {pos.label}
            </button>
          {/each}
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
        <SettingsRangeField
          id="brand-logo-size"
          label="Tamanho Máximo"
          value={localBrand.logo_size}
          min={32}
          max={240}
          step={4}
          unit="px"
          onchange={(v) => onUpdateField('logo_size', v)}
        />

        <SettingsRangeField
          id="brand-logo-opacity"
          label="Opacidade"
          value={localBrand.logo_opacity}
          min={10}
          max={100}
          step={5}
          unit="%"
          onchange={(v) => onUpdateField('logo_opacity', v)}
        />
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
        <SettingsRangeField
          id="brand-logo-margin-h"
          label="Margem Horizontal"
          value={localBrand.logo_margin_horizontal ?? 24}
          min={0}
          max={160}
          step={4}
          unit="px"
          onchange={(v) => onUpdateField('logo_margin_horizontal', v)}
        />

        <SettingsRangeField
          id="brand-logo-margin-v"
          label="Margem Vertical"
          value={localBrand.logo_margin_vertical ?? 24}
          min={0}
          max={160}
          step={4}
          unit="px"
          onchange={(v) => onUpdateField('logo_margin_vertical', v)}
        />
      </div>
    {/if}
  </div>
</div>
