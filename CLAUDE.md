# CLAUDE.md — AnotAI

Instruções para o Claude Code trabalhar neste repositório. Leia este arquivo inteiro antes de qualquer tarefa.

## O projeto

AnotAI é um app desktop **open source e local-first** para estudantes:

1. captura o áudio do sistema durante uma aula (Windows, loopback WASAPI);
2. transcreve localmente com whisper.cpp;
3. salva a transcrição, e nas versões seguintes gera anotações com IA.

Stack: Rust (Cargo workspace) + Tauri 2 + React/TypeScript. Dados locais em SQLite. O servidor Axum de sincronização é opcional e só entra na v0.4.

O produto é Windows-first, mas o desenvolvimento segue um **modelo híbrido**:

* **WSL2 (Ubuntu):** todo o código, testes, clippy e CI dos crates. É onde o Claude Code trabalha.
* **Windows nativo (um segundo clone, só para validar):** o que depende de áudio real ou do app Tauri: captura WASAPI (B03), testes com áudio real (B05, B09), F01–F04 e R01.

Regras do modelo híbrido:
* Código específico de Windows fica atrás de `#[cfg(windows)]`. Todo crate compila e passa nos testes no Linux.
* Quando um critério de aceite só puder ser verificado no Windows, não finja que validou: garanta que compila (`cargo check --target x86_64-pc-windows-msvc` quando possível, ou o job `windows-latest` do CI), escreva no comentário do card um roteiro exato de validação manual e marque o PR como **pendente de validação no Windows**.

## Fonte de verdade: o Jira

O trabalho está no Jira, projeto **AN**, site `tgops.atlassian.net`, acessado pelo MCP da Atlassian.

* A versão atual é a **v0.1**, épico **AN-105**. Ele tem a ordem de execução e o critério de pronto da versão.
* Cada card é autossuficiente: objetivo, o que fazer, fora de escopo, critérios de aceite, como validar e dependências.
* Próximo card a executar:
  ```
  project = AN AND labels = "v0.1" AND issuetype != Epic AND status = "A fazer" ORDER BY summary ASC
  ```
  Pegue o primeiro resultado cujas dependências ("Depende de") estejam em **Em análise** ou **Feito**.
* Os cards sem a etiqueta `v0.1` (backlog antigo, `v0.2`+, `v0.4`, `descartado-oss`) são só referência. **Não execute.**

## Fluxo de trabalho por card

1. **Ler:** o card inteiro e o épico AN-105. Confirme as dependências. Se faltar algo, comente no card e pare.
2. **Começar:** mova o card para **Fazendo** e crie a branch a partir de `main` atualizada: `an-<número>-<slug-curto>` (ex.: `an-109-workspace-setup`).
3. **Planejar:** antes de codar, liste em poucas linhas os arquivos que vai criar ou alterar. Se o card for grande, divida em commits lógicos.
4. **Implementar:** só o que está em "O que fazer". Nada de "Fora de escopo", nada de cards futuros.
5. **Validar:** rode os comandos de "Como validar" e o checklist abaixo. Todos passam antes de abrir o PR.
6. **Entregar:**
   * commits no padrão Conventional Commits, em inglês, terminando com a chave do card: `feat(audio): capture system output via WASAPI loopback (AN-111)`;
   * abra o PR com `gh pr create`, título igual ao card e corpo com o checklist do template e `Jira: AN-<número>`;
   * comente no card: link do PR, resumo do que foi feito, saída resumida da validação e decisões tomadas;
   * mova o card para **Em análise**.
7. **Parar:** um card por sessão, salvo instrução em contrário. Reporte e espere o Matheus.

Nunca mova cards para **Feito**, nunca faça merge e nunca dê push direto na `main`. Isso é do Matheus.

## Checklist antes de abrir o PR

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# a partir do F01, também:
cd apps/desktop && pnpm lint && pnpm typecheck && pnpm test
```

* `CHANGELOG.md` atualizado em **Unreleased** quando a mudança for visível ao usuário.
* Tipos Rust compartilhados com a UI regenerados (`cargo test -p anotai-core`) e commitados.
* Cache do SQLx atualizado (`cargo sqlx prepare --workspace`) quando mudar alguma query.

## Arquitetura e regras de código

**Estrutura**
```
crates/core        anotai-core        tipos de domínio, máquina de estados. Sem I/O, sem async.
crates/audio       anotai-audio       captura, normalização (16 kHz mono), janelas
crates/transcribe  anotai-transcribe  trait TranscriptionEngine, whisper-rs, modelos
crates/session     anotai-session     orquestra captura → VAD → transcrição → eventos
crates/storage     anotai-storage     SQLite (SQLx) e exportação Markdown
apps/cli           anotai             CLI de ponta a ponta
apps/desktop       Tauri 2 + React    UI (a partir do F01)
bench/             bench              benchmarks de WER/RTF/memória
docs/adr/          decisões de arquitetura
```

**Dependências**
* Direção: `apps` → `session` → `audio`/`transcribe`/`storage` → `core`. Os crates em `crates/` nunca dependem de Tauri nem da UI.
* Versões no `[workspace.dependencies]` da raiz. Os crates usam `dep.workspace = true`.
* **Antes de usar a API de qualquer crate externo** (whisper-rs, cpal, rubato, sqlx, tauri, ts-rs…), confira a documentação da versão atual no docs.rs. Essas APIs mudam entre versões; não escreva de memória.
* Prefira crates estáveis e muito usados. Dependência nova e relevante leva uma linha de justificativa no PR.

**Concorrência**
* A inferência do Whisper e o trabalho pesado de CPU rodam numa `std::thread` dedicada ou em `spawn_blocking`, nunca no runtime async.
* O callback de áudio nunca bloqueia: usa canal limitado e descarta com contador quando cheio.
* Todo buffer e canal é **limitado**.
* Nunca mantenha `Mutex`/`RwLock` através de `.await` nem durante a inferência.

**Erros e tipos**
* Bibliotecas usam `thiserror`. `anyhow` só nas bordas (CLI, comandos Tauri).
* Sem `unwrap()`/`expect()` fora de testes, exceto invariantes documentadas.
* Tipos consumidos pela UI: `serde` com `rename_all = "camelCase"` e exportação via `ts-rs`. Entradas com `deny_unknown_fields`.

**Testes**
* Todo critério de aceite verificável vira teste automatizado.
* `FakeSource` e `MockEngine` existem para testar sem hardware nem modelo. Testes que precisam de modelo real ficam com `#[ignore]` e uma instrução de como rodar.

**Idioma**
* Código, comentários, commits, PRs e documentação do repositório: **inglês**. O `README.pt-BR.md` é a exceção.
* Comentários no Jira: **português**.

## Regras de privacidade (inegociáveis)

1. Áudio bruto **nunca** é gravado em disco nem enviado pela rede. Só existe em buffers limitados em memória.
2. A captura só começa por ação explícita do usuário, e a UI mostra um indicador durante toda a sessão.
3. Logs, telemetria e erros nunca contêm áudio, texto transcrito, anotações ou chaves de API (nada de transcrição em `info` ou acima).
4. Na v0.1 nada sai da máquina, exceto o download dos modelos do Whisper.
5. Testes e benchmarks usam só dados sintéticos ou datasets abertos, nunca gravações de aulas reais.

Se um card parecer exigir violar alguma dessas regras, pare e comente no card.

## Quando parar e perguntar

Pare, comente no card e espere o Matheus quando:
* uma dependência do card não estiver pronta;
* um critério de aceite for impossível ou contraditório;
* a documentação atual de um crate contradisser o card (ex.: API ou recurso que não existe mais). Proponha a alternativa no comentário;
* a mudança exigir decisão de arquitetura não prevista. Nesse caso, proponha um ADR em `docs/adr/`.

Não invente requisitos e não amplie o escopo para "deixar pronto" o próximo card.
