# Especificação: Ajustes de Layout e Proporções do Preset Modern Grid no Monitor de Palco

## 1. Intenção

Refinar o layout visual do preset "Grid Moderno Pro" no Monitor de Palco (`/stage`) com base no feedback de design, alinhando a tipografia do próximo slide com o slide atual, equilibrando a coluna lateral em 50%/50% em repouso e adotando a proporção 60%/20%/20% com tipografia gigante para avisos ao palco.

## 2. Escopo

1. **Equiparação de Tipografia entre Slide Atual e Próximo Slide**:
   - No preset `modern-grid`, o próximo slide utilizava `text-2xl sm:text-3xl md:text-4xl`, ficando visualmente menor do que o slide atual (`text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black`).
   - Ajustar o tamanho de fonte do próximo slide para a mesma escala tipográfica do slide atual (`text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black`).
2. **Distribuição Equilibrada da Coluna Lateral em Repouso (50% / 50%)**:
   - Quando não há mensagem de aviso ao vivo, o card de Horário e o card de Cronômetro devem ocupar exatamente 50% da altura da coluna lateral cada um (`flex: 50` / `flex: 50`), centralizados e proporcionais.
3. **Distribuição com Aviso ao Palco (60% / 20% / 20%) e Tipografia Gigante**:
   - Quando há um aviso de palco ativo (`stageMessage`):
     - Card de Aviso: **60%** da altura (`flex: 60`), com tipografia de alta visibilidade (`text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-black`) para permitir leitura instantânea à distância (15 a 20 metros).
     - Card de Horário: **20%** da altura (`flex: 20`).
     - Card de Cronômetro: **20%** da altura (`flex: 20`).
4. **Sincronização no `StagePreviewBox`**:
   - Atualizar o componente de preview em miniatura para refletir fielmente as mesmas proporções de 50%/50% e 60%/20%/20%.

## 3. Critérios de Aceite

- [x] O próximo slide no `modern-grid` possui font-size idêntico ao slide atual.
- [x] Na ausência de avisos, a coluna lateral divide a altura igualmente (50% Horário, 50% Cronômetro).
- [x] Na presença de avisos, a coluna lateral divide a altura em 60% Aviso, 20% Horário e 20% Cronômetro.
- [x] O texto do aviso tem tamanho expressivo e legível para o palco.
- [x] Os previews no Admin e no modal refletem a mesma distribuição.
- [x] Portões de qualidade passam: `cargo fmt --all`, `cargo clippy`, `cargo test`, `bun run check` e `bun run build`.
