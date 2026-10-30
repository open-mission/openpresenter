export interface BackgroundPreset {
  id: string;
  name: string;
  category: 'gradients' | 'mesh' | 'animated' | 'local' | 'custom';
  style: string;
  animationClass?: string;
  textColor?: string;
  description?: string;
  isCustom?: boolean;
}

export interface ResolvedBackground {
  isCss: boolean;
  style: string;
  className: string;
  isImage: boolean;
  isVideo: boolean;
  url: string;
  textColor: string;
}

export interface BackgroundStylesMap {
  defaultBackgroundId?: string;
  customPresets: BackgroundPreset[];
  overrides: Record<string, { textColor?: string; style?: string; animationClass?: string }>;
}

export const BACKGROUND_PRESETS: BackgroundPreset[] = [
  // --- GRADIENTES LINEARES ---
  {
    id: 'grad-obsidian',
    name: 'Obsidiana Escuro',
    category: 'gradients',
    style: 'linear-gradient(135deg, #09090b 0%, #18181b 50%, #09090b 100%)',
    description: 'Tons neutros escuros discretos para alta legibilidade.',
  },
  {
    id: 'grad-indigo',
    name: 'Índigo Meia-Noite',
    category: 'gradients',
    style: 'linear-gradient(135deg, #0f172a 0%, #1e1b4b 60%, #020617 100%)',
    description: 'Azul profundo e sóbrio para cultos e momentos solenes.',
  },
  {
    id: 'grad-purple',
    name: 'Crepúsculo Roxo',
    category: 'gradients',
    style: 'linear-gradient(135deg, #18052e 0%, #3b0764 50%, #090114 100%)',
    description: 'Gradiente roxo imperial para louvor de adoração.',
  },
  {
    id: 'grad-emerald',
    name: 'Esmeralda Noturno',
    category: 'gradients',
    style: 'linear-gradient(135deg, #022c22 0%, #064e3b 50%, #021a14 100%)',
    description: 'Tons verdes nobres e serenos de tranquilidade.',
  },
  {
    id: 'grad-royal-blue',
    name: 'Azul Real',
    category: 'gradients',
    style: 'linear-gradient(135deg, #082f49 0%, #0369a1 50%, #0c1929 100%)',
    description: 'Vibrante e majestoso com contraste nítido.',
  },
  {
    id: 'grad-crimson',
    name: 'Rubi & Carmesim',
    category: 'gradients',
    style: 'linear-gradient(135deg, #1a050d 0%, #4c0519 50%, #0a0105 100%)',
    description: 'Destaque de tom vinho profundo e caloroso.',
  },
  {
    id: 'grad-gold',
    name: 'Dourado & Carvão',
    category: 'gradients',
    style: 'linear-gradient(135deg, #1c1917 0%, #451a03 50%, #0c0a09 100%)',
    description: 'Tons terrosos e dourados escuros de reverência.',
  },
  {
    id: 'grad-cyber-ocean',
    name: 'Oceano Noturno',
    category: 'gradients',
    style: 'linear-gradient(135deg, #042f2e 0%, #115e59 50%, #081e24 100%)',
    description: 'Ciano e turquesa profundos em harmonia suave.',
  },

  // --- MESH GRADIENTS MULTICAMADAS ---
  {
    id: 'mesh-worship',
    name: 'Nebulosa Worship',
    category: 'mesh',
    style:
      'radial-gradient(at 0% 0%, hsla(253,16%,7%,1) 0, transparent 50%), radial-gradient(at 50% 0%, hsla(225,39%,25%,1) 0, transparent 50%), radial-gradient(at 100% 0%, hsla(339,49%,20%,1) 0, transparent 50%), radial-gradient(at 50% 100%, hsla(242,50%,15%,1) 0, transparent 50%), #050508',
    description: 'Composição fluida de roxos e azuis para celebração.',
  },
  {
    id: 'mesh-cosmic',
    name: 'Horizonte Cósmico',
    category: 'mesh',
    style:
      'radial-gradient(at 80% 20%, hsla(242,47%,20%,1) 0, transparent 50%), radial-gradient(at 20% 80%, hsla(189,45%,15%,1) 0, transparent 50%), radial-gradient(at 50% 50%, hsla(263,40%,12%,1) 0, transparent 60%), #040711',
    description: 'Profundidade espacial com nuances de ciano e violeta.',
  },
  {
    id: 'mesh-aurora-glow',
    name: 'Aurora Glow',
    category: 'mesh',
    style:
      'radial-gradient(at 15% 15%, hsla(160,50%,15%,1) 0, transparent 50%), radial-gradient(at 85% 85%, hsla(190,60%,14%,1) 0, transparent 50%), radial-gradient(at 50% 30%, hsla(215,50%,12%,1) 0, transparent 55%), #02110c',
    description: 'Luzes do norte em tons esmeralda difusos.',
  },
  {
    id: 'mesh-celestial',
    name: 'Veludo Celestial',
    category: 'mesh',
    style:
      'radial-gradient(at 0% 100%, hsla(280,45%,16%,1) 0, transparent 50%), radial-gradient(at 100% 0%, hsla(210,50%,18%,1) 0, transparent 50%), radial-gradient(at 50% 50%, hsla(260,35%,10%,1) 0, transparent 50%), #090614',
    description: 'Suavidade aveludada com contraste escuro refinado.',
  },
  {
    id: 'mesh-eclipse',
    name: 'Solar Eclipse',
    category: 'mesh',
    style:
      'radial-gradient(at 50% 0%, hsla(25,60%,18%,1) 0, transparent 50%), radial-gradient(at 100% 100%, hsla(350,50%,15%,1) 0, transparent 50%), radial-gradient(at 0% 50%, hsla(270,30%,10%,1) 0, transparent 50%), #0d070a',
    description: 'Dourado e bordô com calor ambiente e elegância.',
  },
  {
    id: 'mesh-deep-waters',
    name: 'Águas Profundas',
    category: 'mesh',
    style:
      'radial-gradient(at 30% 70%, hsla(195,70%,14%,1) 0, transparent 50%), radial-gradient(at 70% 30%, hsla(230,50%,18%,1) 0, transparent 50%), radial-gradient(at 50% 10%, hsla(210,60%,12%,1) 0, transparent 50%), #030a13',
    description: 'Camadas náuticas de azul intenso para momentos de oração.',
  },

  // --- GRADIENTES ANIMADOS CSS ---
  {
    id: 'anim-aurora-borealis',
    name: 'Aurora Boreal Viva',
    category: 'animated',
    style:
      'linear-gradient(135deg, #022c22 0%, #0e7490 35%, #1e1b4b 70%, #064e3b 100%)',
    animationClass: 'animate-bg-aurora',
    description: 'Transição suave contínua simulando auroras polares.',
  },
  {
    id: 'anim-nebula-shift',
    name: 'Nebulosa em Movimento',
    category: 'animated',
    style:
      'radial-gradient(circle at 40% 40%, #3b0764 0%, #1e1b4b 40%, #030712 80%)',
    animationClass: 'animate-bg-nebula',
    description: 'Expansão e rotação sutil de nebulosa profunda.',
  },
  {
    id: 'anim-ocean-flow',
    name: 'Fluxo Oceânico',
    category: 'animated',
    style:
      'linear-gradient(270deg, #0369a1 0%, #1d4ed8 30%, #4338ca 70%, #0c4a6e 100%)',
    animationClass: 'animate-bg-flow',
    description: 'Ondas cromáticas graduais de azuis e roxos.',
  },
  {
    id: 'anim-celestial-dusk',
    name: 'Crepúsculo Animado',
    category: 'animated',
    style:
      'linear-gradient(135deg, #4c0519 0%, #581c87 35%, #1e1b4b 70%, #311042 100%)',
    animationClass: 'animate-bg-aurora',
    description: 'Bordô e violeta fluindo em harmonia orgânica.',
  },
  {
    id: 'anim-cosmic-pulse',
    name: 'Pulso Cósmico',
    category: 'animated',
    style:
      'radial-gradient(circle at 50% 50%, #1e1b4b 0%, #0f172a 45%, #020617 90%)',
    animationClass: 'animate-bg-pulse',
    description: 'Variação luminosa imperceptível para manter a tela viva.',
  },
  {
    id: 'anim-dawn-light',
    name: 'Alvorecer Vivo',
    category: 'animated',
    style:
      'linear-gradient(120deg, #1e1b4b 0%, #701a75 35%, #451a03 70%, #172554 100%)',
    animationClass: 'animate-bg-flow',
    description: 'Transição suave de alvorecer cósmico.',
  },
];

/**
 * Retorna todos os presets combinando os nativos com os customizados e sobrescritas de configuração.
 */
export function getAllBackgrounds(stylesMap?: BackgroundStylesMap | null): BackgroundPreset[] {
  const overrides = stylesMap?.overrides || {};
  const custom = stylesMap?.customPresets || [];

  const base = BACKGROUND_PRESETS.map((p) => {
    const ov = overrides[p.id];
    if (ov) {
      return {
        ...p,
        ...ov,
      };
    }
    return p;
  });

  return [...base, ...custom];
}

/**
 * Converte um preset em string serializável para persistência em wallpaper_path.
 */
export function encodeBackground(preset: BackgroundPreset): string {
  const anim = preset.animationClass || 'none';
  const textCol = preset.textColor || '#ffffff';
  return `css:${anim}|${preset.style}|${textCol}`;
}

/**
 * Resolve qualquer valor armazenado em wallpaper_path (seja url de imagem ou css).
 */
export function resolveBackground(
  wallpaperPath?: string,
  stylesMap?: BackgroundStylesMap | null
): ResolvedBackground {
  if (!wallpaperPath || wallpaperPath.trim() === '') {
    return {
      isCss: false,
      style: '',
      className: '',
      isImage: false,
      isVideo: false,
      url: '',
      textColor: '#ffffff',
    };
  }

  const trimmed = wallpaperPath.trim();

  // Formato com prefixo css:
  if (trimmed.startsWith('css:')) {
    const raw = trimmed.slice(4);
    const parts = raw.split('|');
    let anim = '';
    let style = '';
    let textColor = '#ffffff';

    if (parts.length >= 3) {
      anim = parts[0].trim();
      textColor = parts[parts.length - 1].trim();
      style = parts.slice(1, parts.length - 1).join('|').trim();
    } else if (parts.length === 2) {
      anim = parts[0].trim();
      style = parts[1].trim();
    } else {
      style = raw.trim();
    }

    // Se o textColor não foi explicitado na string e temos stylesMap, tenta buscar em preset conhecido
    if ((!textColor || textColor === '#ffffff') && stylesMap) {
      const all = getAllBackgrounds(stylesMap);
      const match = all.find((p) => p.style.trim() === style);
      if (match?.textColor) {
        textColor = match.textColor;
      }
    }

    return {
      isCss: true,
      style: `background: ${style};`,
      className: anim === 'none' ? '' : anim,
      isImage: false,
      isVideo: false,
      url: '',
      textColor: textColor || '#ffffff',
    };
  }

  // Gradientes brutos inline
  if (
    trimmed.startsWith('linear-gradient') ||
    trimmed.startsWith('radial-gradient') ||
    trimmed.startsWith('conic-gradient')
  ) {
    let textColor = '#ffffff';
    if (stylesMap) {
      const all = getAllBackgrounds(stylesMap);
      const match = all.find((p) => p.style.trim() === trimmed);
      if (match?.textColor) {
        textColor = match.textColor;
      }
    }

    return {
      isCss: true,
      style: `background: ${trimmed};`,
      className: '',
      isImage: false,
      isVideo: false,
      url: '',
      textColor,
    };
  }

  // Imagem ou Vídeo normal de arquivo ou URL
  const lower = trimmed.toLowerCase();
  const isVideo =
    lower.endsWith('.mp4') ||
    lower.endsWith('.webm') ||
    lower.endsWith('.mov') ||
    lower.endsWith('.mkv');

  return {
    isCss: false,
    style: '',
    className: '',
    isImage: !isVideo,
    isVideo,
    url: trimmed,
    textColor: '#ffffff',
  };
}
