# Roadmap — Lethal Company CLI

> Tarefas pequenas, ordenadas por dependência e por progressão pedagógica de Rust.
> Marque `[x]` ao concluir. Cada tarefa tem **objetivo**, **conceitos de Rust** e **critério de aceite**.

Convenção:
- 🦀 = conceito novo de Rust introduzido
- 🎯 = critério de aceite (DoD)
- 📚 = leitura sugerida do Rust Book
- 🧪 = tarefa de teste (sempre roda com `cargo test`)

**Política de testes**: toda função pura ganha teste unitário. Lógica que depende de aleatoriedade é testada com `StdRng::seed_from_u64` (RNG injetado). I/O é testado com diretório temporário. Doc-tests entram em pelo menos um método público para você conhecer a feature.
📚 Cap. 11 — Writing Automated Tests.

> ⚠️ **Edition 2024**: este projeto usa a edition mais nova. O Rust Book ainda referencia 2021 em vários capítulos. Atenção a:
> - novas keywords reservadas (`gen`, `try`) — evite nomes de variáveis/funções como `gen_event` (use `generate_event`)
> - mudanças no prelúdio (`Future`, `IntoFuture` agora estão lá)
> - regras de captura de closures e `if let` chains podem diferir do Book
> Quando uma diferença aparecer, o Claude vai sinalizar.

---

## Fase 0 — Fundação

### [x] T0.1 — Estrutura de módulos inicial (`lib.rs` + `main.rs`)
- **Objetivo**: organizar `src/` em módulos vazios que vão ser preenchidos depois.
- 🦀 `mod`, `pub`, árvore de módulos, `main.rs` vs `lib.rs`.
- 🎯 `cargo build` compila com `lib.rs` declarando módulos `commands`, `state`, `ui` (mesmo vazios) e `main.rs` apenas chamando uma função pública da lib.
- 💡 **Por que `lib.rs` desde já**: doc-tests (T16.4) **só rodam em código de biblioteca**. Separar lógica em `lib.rs` desde o início evita refactor depois e é o padrão idiomático para projetos com binário + lógica testável.
- 📚 Cap. 7 — Managing Growing Projects; Cap. 12.3 — Refactoring to Improve Modularity.

### [x] T0.2 — Hello, employee
- **Objetivo**: imprimir uma mensagem de boas-vindas no `main`.
- 🦀 `println!`, macros vs funções, string literais (`&'static str`).
- 🎯 `cargo run` imprime uma linha estilo corporativo.

### [x] T0.3 — `rustfmt` + `clippy`
- **Objetivo**: rodar formatador e linter desde o primeiro código, antes de criar maus hábitos.
- 🦀 ferramental do toolchain Rust, anatomia de um warning de clippy.
- 🎯 `cargo fmt --check` passa; `cargo clippy -- -D warnings` passa sem warnings.
- 💡 acostume-se a rodar os dois antes de pedir review. Vão aparecer naturalmente warnings pedagógicos (ex.: `needless_return`, `redundant_clone`).

---

## Fase 1 — Boot e REPL básico (features 1, 2)

### [x] T1.1 — Splash screen
- **Objetivo**: imprimir nome da corporação, versão e mensagem inicial.
- 🦀 constantes (`const`), `env!("CARGO_PKG_VERSION")`, formatação com `{}`.
- 🎯 ao iniciar, aparece bloco fixo com 4–5 linhas.

### [x] T1.2 — Loop de prompt (REPL)
- **Objetivo**: ler linha do `stdin`, ecoar de volta, repetir até o usuário digitar `exit`.
- 🦀 `std::io::{self, BufRead, Write}`, `stdout().flush()`, `String::trim`, `loop`, `break`.
- 🎯 prompt `> ` aparece, qualquer texto é ecoado, `exit` encerra.
- 📚 Cap. 2 — Programming a Guessing Game (similaridade de I/O).

### [x] T1.3 — Comando `exit` e `clear`
- **Objetivo**: tratar dois comandos hardcoded antes de generalizar.
- 🦀 `match` em `&str`.
- 🎯 `exit` sai, `clear` limpa tela (pode ser `print!("\x1B[2J\x1B[H")`).

---

## Fase 2 — Sistema de comandos (feature 3)

### [x] T2.1 — Enum `Command`
- **Objetivo**: modelar comandos como enum com variantes (`Help`, `Exit`, `Clear`, `Unknown(String)`).
- 🦀 `enum`, variantes com dados, `derive(Debug)`.
- 🎯 enum compila e pode ser usada com `match` exaustivo.
- 📚 Cap. 6 — Enums and Pattern Matching.

### [x] T2.2 — Parser `&str -> Command`
- **Objetivo**: função pura que converte uma linha em `Command`.
- 🦀 `str::split_whitespace`, `match`, retorno por valor (ownership).
- 🎯 `parse("help")` → `Command::Help`; `parse("xyz")` → `Command::Unknown(...)`.

### [x] T2.3 — Dispatcher
- **Objetivo**: função que recebe `Command` e executa o comportamento.
- 🦀 `match` exaustivo, exhaustiveness checking do compilador.
- 🎯 cada variante imprime algo distinto; `Unknown` mostra mensagem corporativa de erro.

### [x] T2.4 — Comando `help`
- **Objetivo**: listar comandos disponíveis.
- 🦀 arrays `[&str; N]` ou `&[&str]`, iteração com `for`.
- 🎯 `help` imprime lista alinhada de comandos com 1 linha de descrição cada.

### [x] 🧪 T2.5 — Primeiros testes unitários do parser
- **Objetivo**: cobrir o parser `&str -> Command` com `cargo test`.
- 🦀 `#[cfg(test)] mod tests`, `#[test]`, `assert_eq!`, `assert!(matches!(...))`, derive `PartialEq` em `Command`.
- 🎯 ao menos 4 casos: `help`, `exit`, com espaços extras, e `Unknown` para input inválido. `cargo test` passa verde.
- 📚 Cap. 11.1 — How to Write Tests.

### [x] T2.6 — Comandos com argumentos
- **Objetivo**: evoluir o `enum Command` para suportar argumentos (ex.: `Command::Buy(String)`, `Command::Route(String)`), preparando o terreno para `buy`, `route` e `travel`.
- 🦀 variantes de enum com dados associados, refactor guiado pelo compilador (exhaustiveness checking força atualizar todos os `match`), `String` vs `&str` na escolha de ownership da variante.
- 🎯 parser passa a reconhecer `buy lantern` → `Command::Buy("lantern".into())`; argumentos faltando viram `Command::Unknown(...)` ou variante específica de erro; testes de T2.5 atualizados para cobrir o caso com argumento.
- 💡 **Por que agora**: introduzir args aqui evita um refactor desconfortável na Fase 6, quando o foco já é `Result` + `?`. Aprende-se uma coisa de cada vez.

---

## Fase 3 — Estado global (feature 10)

> ℹ️ A feature 10 pede estado **persistente durante a execução** — ou seja, vive enquanto o processo está rodando. Persistência em disco (save/load entre sessões) é separada e fica para a Fase 16.

### [x] T3.1 — Struct `GameState`
- **Objetivo**: criar struct com campos básicos (créditos, dia, planeta atual como `Option<String>`).
- 🦀 `struct`, `Option<T>`, `Default`, métodos com `impl`.
- 🎯 `GameState::new()` retorna estado inicial.
- 📚 Cap. 5 — Using Structs.

### [x] T3.2 — Passar estado mutável ao dispatcher
- **Objetivo**: dispatcher recebe `&mut GameState` para que comandos alterem estado.
- 🦀 referência mutável `&mut`, regras de borrow (uma só por vez).
- 🎯 um comando dummy (ex.: `tick`) incrementa `dia`.
- 📚 Cap. 4 — Understanding Ownership.

### [x] T3.3 — Comando `status`
- **Objetivo**: imprimir o estado atual.
- 🦀 `Display` trait (opcional aqui) ou `Debug`, `{:?}`.
- 🎯 `status` mostra créditos, dia, planeta atual.

---

## Fase 4 — Créditos (feature 7)

### [x] T4.1 — Métodos `add_credits` / `spend_credits`
- **Objetivo**: encapsular mutação no `impl GameState`.
- 🦀 métodos `&mut self`, retorno de `Result<T, E>`.
- 🎯 `spend_credits(50)` retorna `Err` se saldo insuficiente.
- 📚 Cap. 9 — Error Handling.

### [x] T4.2 — Erro customizado `WalletError`
- **Objetivo**: enum de erro próprio.
- 🦀 `enum` para erro, `impl Display`, `impl std::error::Error` (opcional), `From` (opcional).
- 🎯 mensagem de erro corporativa quando saldo insuficiente.

### [x] T4.3 — Comando `credits`
- **Objetivo**: ler/exibir saldo.
- 🎯 `credits` imprime "BALANCE: 100 CR".

### [x] 🧪 T4.4 — Testes de `WalletError`
- **Objetivo**: cobrir caminhos `Ok` / `Err` de `add_credits` e `spend_credits`.
- 🦀 `assert!(matches!(result, Err(WalletError::InsufficientFunds { .. })))`, testes que cobrem overflow se aplicável.
- 🎯 testes para: gasto válido, gasto sem saldo, soma normal, e (opcional) overflow.
- 📚 Cap. 11.1 — assert macros.

---

## Fase 5 — Inventário (feature 8)

### [x] T5.1 — Struct `Item` e enum `ItemKind`
- **Objetivo**: modelar itens (lanterna, pá, etc.).
- 🦀 derive (`Clone`, `Debug`, `PartialEq`), variantes de enum sem dado.
- 🎯 lista com 5 itens iniciais possíveis.

### [ ] T5.2 — Inventário com `Vec<Item>`
- **Objetivo**: campo `inventory: Vec<Item>` em `GameState`.
- 🦀 `Vec`, `push`, `iter`, `len`, capacidade fixa via constante.
- 🎯 `inventory` aceita até N itens; tentativas além retornam erro.
- 📚 Cap. 8 — Common Collections.

### [ ] T5.3 — Comando `inventory`
- **Objetivo**: listar itens agrupados por tipo.
- 🦀 `HashMap<ItemKind, u32>` para contar, ou `iter().fold`.
- 🎯 saída do tipo `LANTERN x2 / SHOVEL x1`.

### [ ] 🧪 T5.4 — Testes de capacidade do inventário
- **Objetivo**: garantir que o limite máximo é respeitado.
- 🦀 `#[should_panic]` (se a regra usar panic) ou `Result` + `matches!`, helper de setup em testes.
- 🎯 testes: adicionar até o limite, ultrapassar limite → erro, contagem por tipo correta.

---

## Fase 6 — Loja (feature 6)

### [ ] T6.1 — Catálogo da loja
- **Objetivo**: lista estática de itens com preço.
- 🦀 `const` arrays vs `static`, tuplas ou structs.
- 🎯 `store` lista nome + preço.

### [ ] T6.2 — Comando `buy <item>`
- **Objetivo**: parse do argumento, débito de crédito, push no inventário.
- 🦀 parser que recebe `Vec<&str>` ou `&[&str]`, propagação de `Result` com `?`.
- 🎯 `buy lantern` debita e adiciona; sem saldo → erro; item inválido → erro.

### [ ] 🧪 T6.3 — Testes de integração de `buy`
- **Objetivo**: testar a interação entre carteira e inventário (não mais função pura).
- 🦀 enum de erro composto (ex.: `BuyError::Wallet(WalletError) | BuyError::UnknownItem | BuyError::InventoryFull`), `From` impls para usar `?`.
- 🎯 testes: compra ok, sem saldo, item inexistente, inventário cheio.

---

## Fase 7 — Planetas e navegação (features 4, 9)

### [ ] T7.1 — Struct `Planet`
- **Objetivo**: modelar planeta (nome, perigo, clima, custo de viagem).
- 🦀 `enum DangerLevel { Low, Medium, High }`, `enum Weather { ... }`.
- 🎯 catálogo com 4–6 planetas.

### [ ] T7.2 — Comando `planets`
- **Objetivo**: listar planetas com info resumida.
- 🦀 iteradores `.iter().map().collect()`, formatação com `{:>10}`.
- 🎯 saída em colunas alinhadas.

### [ ] T7.3 — Comando `route <planet>`
- **Objetivo**: selecionar destino sem viajar ainda.
- 🦀 busca em `Vec<Planet>` com `iter().find(|p| p.name == name)`, retorno `Option<&Planet>`.
- 🎯 destino guardado em `GameState`.

### [ ] T7.4 — Comando `travel`
- **Objetivo**: confirmar viagem (debita custo, troca planeta atual, avança dia).
- 🎯 viajar funciona; sem destino setado ou sem créditos → erro.

---

## Fase 8 — Escaneamento (feature 5)

### [ ] T8.1 — Adicionar crate `rand`
- 🦀 `Cargo.toml`, semver, features.
- 🎯 `cargo build` baixa a crate.

### [ ] T8.2 — Comando `scan` (com RNG injetado)
- **Objetivo**: gerar relatório aleatório do planeta atual, **recebendo** o RNG como parâmetro genérico.
- 🦀 `rand::Rng` como **trait bound** (`fn scan<R: Rng>(rng: &mut R, ...)`), `thread_rng` em produção, `StdRng::seed_from_u64` em teste.
- 🎯 cada `scan` traz métricas variáveis; ~10% chance de "INTERFERENCE — DATA CORRUPTED".
- 💡 **Por que isso importa para teste**: receber `&mut R` em vez de chamar `thread_rng()` direto torna a função determinística sob seed. Esse é o padrão idiomático em Rust para lógica testável.

### [ ] 🧪 T8.3 — Testes determinísticos com seed
- **Objetivo**: usar `StdRng::seed_from_u64` para travar o resultado e fazer asserts.
- 🦀 `rand::SeedableRng`, `rand::rngs::StdRng`, snapshot simples da saída.
- 🎯 com seed fixo, o relatório de `scan` é byte-a-byte igual entre execuções; teste valida isso.

---

## Fase 9 — Histórico de comandos (feature 11)

### [ ] T9.1 — `VecDeque<String>` com tamanho máximo
- 🦀 `std::collections::VecDeque`, `push_back` + `pop_front`.
- 🎯 últimos 20 comandos guardados.

### [ ] T9.2 — Comando `history`
- 🎯 imprime os comandos numerados.

### [ ] 🧪 T9.3 — Testes do buffer circular de histórico
- **Objetivo**: validar que o histórico nunca passa do tamanho máximo.
- 🦀 testes em loop, `assert_eq!` com slice esperado.
- 🎯 inserir N+5 comandos resulta em apenas os últimos N guardados, em ordem correta.

---

## Fase 10 — Logs corporativos (feature 12)

> 💡 **Refactor oportunista**: ao concluir esta fase, vale voltar em pontos anteriores onde já existia mensagem de erro ou aviso (ex.: `Unknown` em T2.3, `WalletError` em T4.2, falha de prazo em T11.4) e migrá-los para os novos helpers. Não é tarefa formal, mas é um exercício prático de localizar call sites com `grep`/IDE e aplicar mudança consistente.

### [ ] T10.1 — Adicionar crate `crossterm` (ou `colored`)
- 🎯 build OK.

### [ ] T10.2 — Funções `alert!`, `warn!`, `transmission!`
- **Objetivo**: helpers que imprimem com cor + tag (`[ALERT]`, `[WARN]`, `[TX LOST]`).
- 🦀 macros declarativas (`macro_rules!`) opcional, ou funções comuns.
- 🎯 mensagens coloridas no terminal.

---

## Fase 11 — Tempo e quotas (features 16, 17)

### [ ] T11.1 — Avanço de dia ao viajar/dormir
- 🦀 mutação controlada de campos.
- 🎯 `dia` cresce; preços/clima podem variar por dia.

### [ ] T11.2 — Struct `Quota` (meta + prazo)
- **Objetivo**: modelar a quota como dado, sem lógica ainda.
- 🦀 `struct` com campos (`target: u32`, `deadline_day: u32`, `delivered: u32`), método `progress() -> f32`.
- 🎯 `Quota` integrada em `GameState`; `progress()` retorna 0.0..=1.0.

### [ ] T11.3 — Comando `quota`
- **Objetivo**: exibir progresso, prazo restante e status (em dia / atrasado / cumprida).
- 🦀 `enum QuotaStatus { OnTrack, Behind, Met, Failed }`, `match` para formatar saída.
- 🎯 `quota` mostra algo como `QUOTA: 340/500 CR — 2 DAYS REMAINING — STATUS: BEHIND`.

### [ ] T11.4 — Lógica de penalidade ao falhar prazo
- **Objetivo**: ao avançar dia além de `deadline_day` sem cumprir, aplicar penalidade narrativa (mensagem + reset/multa).
- 🦀 mutação de estado disparada por evento de tempo, `Result` para sinalizar fim de jogo (opcional).
- 🎯 ultrapassar prazo sem entregar → mensagem corporativa + reset da quota com novo prazo (ou game over, decisão de design).

### [ ] 🧪 T11.5 — Testes de quota
- **Objetivo**: cobrir cumprimento, falha por prazo, progresso parcial e penalidade.
- 🦀 fixtures em testes, helper `fn make_state(...) -> GameState`.
- 🎯 quatro casos: meta atingida, atraso (status Behind), prazo expirado (penalidade aplicada), progresso parcial.

---

## Fase 12 — Eventos aleatórios (feature 13)

### [ ] T12.1 — Enum `Event` e gerador
- 🦀 trait objects `Box<dyn Fn(&mut GameState)>` (opcional, avançado), ou `match` simples.
- 🎯 ao avançar dia, ~30% chance de evento (tempestade, interferência, criatura detectada).

### [ ] 🧪 T12.2 — Testes do gerador de eventos com seed
- **Objetivo**: reaproveitar o padrão do RNG injetado para validar distribuição.
- 🦀 mesmo padrão de T8.3.
- 🎯 com seed fixo, sequência de eventos é reproduzível e validada.

---

## Fase 13 — Bestiário (feature 14)

### [ ] T13.1 — Catálogo de criaturas (hardcoded)
- **Objetivo**: catálogo de criaturas em código, comando `bestiary` que lista todas.
- 🦀 `const`/`static` arrays de structs, `enum CreatureBehavior { Passive, Hunter, Ambusher, ... }`.
- 🎯 comando `bestiary` lista criaturas com perigo e comportamento.
- 💡 **Por que não JSON aqui**: `serde` + `serde_json` só entram na Fase 16 (persistência). Mantemos hardcoded para não introduzir uma crate fora de ordem. Se quiser migrar para JSON depois da Fase 16, é uma boa tarefa de refactor opcional.

---

## Fase 14 — Radar (feature 15)

### [ ] T14.1 — Comando `radar`
- 🦀 grid 2D com `Vec<Vec<char>>` ou string-art, formatação.
- 🎯 mostra mapa textual com pontos próximos no planeta atual.

---

## Fase 15 — Seeds (feature 18)

> ℹ️ O mecanismo de seed (`StdRng::seed_from_u64`) já entrou na Fase 8 para tornar `scan` testável. Aqui o foco é **expor a seed ao usuário** para que mundos/eventos sejam reproduzíveis fora dos testes.

### [ ] T15.1 — Centralizar o RNG no `GameState`
- **Objetivo**: parar de criar RNG ad-hoc em cada comando; passar a guardar `StdRng` dentro do `GameState` para que toda aleatoriedade da sessão derive da mesma seed.
- 🦀 campo de struct com `StdRng`, repassar `&mut self.rng` para funções que precisam, evitar `thread_rng` em código de produção do app.
- 🎯 `scan` e o gerador de eventos (T12.1) consomem o mesmo RNG do estado.

### [ ] T15.2 — Flag de CLI `--seed <u64>`
- **Objetivo**: aceitar uma seed na linha de comando ao iniciar o app; sem a flag, usar uma seed aleatória mas **logar qual foi sorteada** para o usuário poder reproduzir.
- 🦀 parsing simples de `std::env::args` (ainda sem `clap` — manter o escopo de crates), `u64::from_str`, `Result` na inicialização.
- 🎯 `cargo run -- --seed 42` produz a mesma sequência de scans/eventos entre execuções; sem flag, o splash mostra `SEED: 17392...`.

---

## Fase 16 — Persistência (extra MVP)

### [ ] T16.1 — Adicionar `serde` + `serde_json`
- 🦀 derive `Serialize` / `Deserialize`.
- 🎯 build OK com features.

### [ ] T16.2 — Comandos `save` / `load`
- 🦀 `std::fs`, `Result`, propagação com `?`.
- 🎯 grava `save.json`, carrega ao iniciar se existir.

### [ ] 🧪 T16.3 — Testes de `save` / `load` com diretório temporário
- **Objetivo**: I/O testável sem sujar o disco do projeto.
- 🦀 `std::env::temp_dir()` ou crate `tempfile`, função que recebe `&Path` (em vez de hardcoded).
- 🎯 round-trip: `save → load → estado idêntico` (use `PartialEq` em `GameState`).

### [ ] 🧪 T16.4 — Doc-test em um método público
- **Objetivo**: conhecer a feature exclusiva do Rust de testes em comentários `///`.
- 🦀 doc-comments com bloco ` ``` ` rodam como teste; `cargo test` os executa.
- 🎯 ao menos um método de `GameState` (ex.: `add_credits`) tem exemplo na doc que o `cargo test` valida.
- 📚 Cap. 14.2 — Making Useful Documentation Comments.

---

## Fase 17 — TUI completa (feature 19)

> ⚠️ **Maior salto de complexidade do roadmap.** `ratatui` traz lifetimes em widgets, event loop com `event::poll`, e raw mode do terminal. Para reduzir risco, a fase tem **dois marcos**:
> - **Marco 1 — TUI mínima viável** (T17.1 → T17.3): TUI abre, desenha um placeholder, fecha em `q`. O REPL antigo continua intocado em paralelo.
> - **Marco 2 — Migração completa** (T17.4 → T17.6): só começa depois do Marco 1 estar estável. Cada comando do REPL é portado um a um.
>
> Não tente pular o Marco 1. Validar terminal raw mode + redraw antes de mexer em estado é o que evita debug brutal depois.

### [ ] T17.1 — Adicionar `ratatui`
- 🎯 dependência instalada.

### [ ] T17.2 — Layout com painéis
- 🦀 closures, lifetimes em widgets.
- 🎯 painéis: status, log, prompt, mapa.

### [ ] T17.3 — Event loop básico da TUI
- **Objetivo**: substituir o `loop` do REPL por um event loop que faz tick + redraw.
- 🦀 `terminal.draw(|f| ...)`, `event::poll`, `event::read`, `Duration` para frame budget.
- 🎯 TUI abre, mostra placeholder, e fecha ao apertar `q` ou `Ctrl+C`.

### [ ] T17.4 — Renderização do estado atual
- **Objetivo**: desenhar o `GameState` nos painéis (status, log, mapa).
- 🦀 widgets `Block`, `Paragraph`, `List`; closures que capturam `&GameState`; lifetimes em widgets.
- 🎯 alterar o estado em testes manuais reflete na tela em tempo real.

### [ ] T17.5 — Input handling no painel de prompt
- **Objetivo**: capturar digitação tecla-a-tecla e montar a string do comando.
- 🦀 `KeyEvent`, `KeyCode`, buffer mutável de input, `match` em variantes.
- 🎯 digitar comando + Enter dispara o dispatcher existente.

### [ ] T17.6 — Substituição comando-a-comando (REPL → TUI)
- **Objetivo**: garantir que cada comando do REPL antigo continua funcionando no novo loop.
- 🦀 enum-based state machine para modos (`Normal`, `Editing`, `Help`...).
- 🎯 todas as features anteriores (`status`, `buy`, `scan`, etc.) funcionam dentro da TUI.

---

## Fase 18 — Polimento (features 20, 21, 22, 23, 24, 25)

### [ ] T18.1 — Digitação simulada (feature 21)
- 🦀 `std::thread::sleep`, `Duration`, `print!` + flush.

### [ ] T18.2 — Pseudo filesystem + arquivos internos (features 22, 25)
- **Objetivo**: simular um filesystem corporativo em memória (`HashMap<PathBuf, FakeFile>`) **e** popular com conteúdo narrativo (logs, mensagens, emails da corporação). Comandos `ls`, `cat`, `cd` operam sobre essa estrutura.
- 🦀 `HashMap<PathBuf, FakeFile>`, `enum FakeFile { Dir(Vec<PathBuf>), Text(String) }`, recursão controlada para `cd ..`/paths absolutos vs relativos.
- 🎯 `ls /logs`, `cat /mail/inbox/2024-07-12.txt`, `cd /reports` funcionam; conteúdo dos arquivos transmite o tom corporativo/hostil definido nas regras implícitas do escopo.
- 💡 **Por que juntar 22 + 25**: a feature 22 (conteúdo dos arquivos) só faz sentido com a estrutura da feature 25. Separar em duas tarefas duplicaria o modelo de dados.

### [ ] T18.3 — Sistema de autenticação (feature 23)
- 🦀 enum de role + checagem antes de comandos restritos.

### [ ] T18.4 — Rede simulada (feature 24)
- 🎯 mensagens estilo `CONNECTING... / SIGNAL LOST` em momentos chave.

### [ ] T18.5 — Sons (feature 20)
- 🦀 crate opcional (`rodio`) — alinhar antes.

---

## Como usar este roadmap

1. Abra uma sessão. O Claude lê este arquivo e pergunta a próxima tarefa.
2. Antes de implementar uma tarefa, peça ao Claude para explicar os conceitos novos da seção 🦀.
3. Implemente.
4. Cole o código no chat e peça review.
5. Marque `[x]` na tarefa concluída.
6. Próxima.
