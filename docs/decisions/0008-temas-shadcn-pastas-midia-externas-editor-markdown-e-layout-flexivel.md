# ADR 0008: Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Layout Flexível

- **Status:** Aceito
- **Data:** 2026-10-29
- **Decisores:** Antigravity & Usuário
- **Substitui:** -
- **Substituído por:** -

---

## Contexto

Com a expansão do aplicativo para apresentações ao vivo completas, foram levantados requisitos ergonômicos e funcionais essenciais:
1. Customização visual profissional baseada no ecossistema Shadcn UI (Base Colors para tons de superfície e Theme Accents para cores de destaque primárias).
2. Flexibilidade espacial na tela do operador (Admin) permitindo colapsar os painéis laterais de Setlist e Preview para maximizar o foco nos slides, e um painel inferior (Bottom Panel) acoplado para acesso ágil à biblioteca de músicas e galeria de imagens.
3. Criação ágil de textos e avisos rápidos com a mesma riqueza de recursos do editor de músicas (divisão em slides, prévia 16:9 em tempo real).
4. Suporte a formatação rica Markdown com typeset (negrito, itálico, sublinhado, highlight) e controle de alinhamento vertical e horizontal dos slides.
5. Integração com o sistema de arquivos local: suporte a pastas do sistema organizadas por categoria (`images`, `videos`, `fonts`, `wallpapers`, `db`) e mapeamento de pastas externas do usuário sem duplicação de dados (zero-copy).

---

## Decisões

1. **Separação de Paletas: Base Color vs Theme:**
   - Adotar a especificação de dois eixos de cores do Shadcn UI:
     - `Base Color`: determina as variáveis de tons neutros (`--background`, `--card`, `--border`, `--muted`, `--foreground`) para os estilos `neutral`, `stone`, `zinc`, `mauve`, `olive`, `mist`, `taupe`.
     - `Theme (Accent)`: determina a variável `--primary` e complementos para `neutral`, `amber`, `blue`, `cyan`, `emerald`, `fuchsia`, `green`, `indigo`, `lime`, `orange`, `pink`, `purple`, `rose`, `violet`, `yellow`.
2. **Layout Operacional de Sidebars Colapsáveis e Bottom Panel:**
   - As colunas esquerda (Setlist) e direita (Previews) passam a ser retráteis via botões dedicados de recolhimento e no cabeçalho.
   - O painel inferior conta com três estados: colapsado em barra de rodapé, aberto com redimensionamento de altura por arraste do mouse, e modo maximizado (100% de tela).
3. **Editor Unificado de Slides (Música e Texto):**
   - Generalizar o editor em modal de 95% para suportar tanto `Song` quanto `Text` (apresentação rápida), incluindo barra de ferramentas typeset com inserção de Markdown (`**`, `*`, `<u>`, `==`) e seletores de alinhamento (`align_h`, `align_v`).
4. **Armazenamento de Pastas Externas Mapeadas:**
   - Mapear diretórios do usuário na tabela SQLite `media_folders`. O backend lê arquivos diretamente do disco (`zero-copy`) e os expõe via endpoints seguros de streaming de mídia.

---

## Consequências

- **Positivas:**
  - Interface moderna, altamente configurável e confortável para diferentes ambientes de iluminação em cabines de transmissão e templos.
  - Maior produtividade do operador com reordenação por arrastar e soltar e acesso simultâneo a mídias e músicas sem sair da tela principal.
  - Leitura rápida de milhares de imagens locais de eventos sem consumir espaço em disco duplicado.
- **Negativas / Mitigações:**
  - Acesso a arquivos do disco em pastas externas requer validação de segurança para impedir Directory Traversal. Mitigado com conferência canônica dos caminhos autorizados em `media_folders`.
