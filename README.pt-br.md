<p align="center">
  <img src="zeroclaw.png" alt="ZeroClaw" width="200" />
</p>

<h1 align="center">ZeroClaw 🦀</h1>

<p align="center">
  <strong>Zero overhead. Zero compromise. 100% Rust. 100% Agnóstico.</strong><br>
  ⚡️ <strong>Roda em hardware de $10 com <5MB de RAM: Isso é 99% menos memória que o OpenClaw e 98% mais barato que um Mac mini!</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT" /></a>
  <a href="NOTICE"><img src="https://img.shields.io/badge/contributors-27+-green.svg" alt="Contributors" /></a>
  <a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>
  <a href="https://x.com/zeroclawlabs?s=21"><img src="https://img.shields.io/badge/X-%40zeroclawlabs-000000?style=flat&logo=x&logoColor=white" alt="X: @zeroclawlabs" /></a>
  <a href="https://www.xiaohongshu.com/user/profile/67cbfc43000000000d008307?xsec_token=AB73VnYnGNx5y36EtnnZfGmAmS-6Wzv8WMuGpfwfkg6Yc%3D&xsec_source=pc_search"><img src="https://img.shields.io/badge/Xiaohongshu-Official-FF2442?style=flat" alt="Xiaohongshu: Official" /></a>
  <a href="https://t.me/zeroclawlabs"><img src="https://img.shields.io/badge/Telegram-%40zeroclawlabs-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram: @zeroclawlabs" /></a>
  <a href="https://t.me/zeroclawlabs_cn"><img src="https://img.shields.io/badge/Telegram%20CN-%40zeroclawlabs__cn-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram CN: @zeroclawlabs_cn" /></a>
  <a href="https://t.me/zeroclawlabs_ru"><img src="https://img.shields.io/badge/Telegram%20RU-%40zeroclawlabs__ru-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram RU: @zeroclawlabs_ru" /></a>
  <a href="https://www.reddit.com/r/zeroclawlabs/"><img src="https://img.shields.io/badge/Reddit-r%2Fzeroclawlabs-FF4500?style=flat&logo=reddit&logoColor=white" alt="Reddit: r/zeroclawlabs" /></a>
</p>

<p align="center">
Construído por estudantes e membros das comunidades Harvard, MIT e Sundai.Club.
</p>

<p align="center">
  🌐 <strong>Idiomas:</strong> <a href="README.md">Inglês</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ru.md">Русский</a> · <a href="README.fr.md">Français</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.pt-br.md">Português do Brasil</a>
</p>

<p align="center">
  <a href="#quick-start">Primeiros Passos</a> |
  <a href="bootstrap.sh">Configuração com um clique</a> |
  <a href="docs/README.md">Hub de Documentação</a> |
  <a href="docs/SUMMARY.md">Sumário da Documentação</a>
</p>

<p align="center">
  <strong>Rotas rápidas:</strong>
  <a href="docs/reference/README.md">Referência</a> ·
  <a href="docs/operations/README.md">Operações</a> ·
  <a href="docs/troubleshooting.md">Solução de problemas</a> ·
  <a href="docs/security/README.md">Segurança</a> ·
  <a href="docs/hardware/README.md">Hardware</a> ·
  <a href="docs/contributing/README.md">Contribuir</a>
</p>

<p align="center">
  <strong>Infraestrutura de assistente de AI rápida, leve e totalmente autônoma</strong><br />
  Implante em qualquer lugar. Troque qualquer componente.
</p>

<p align="center"><code>Arquitetura orientada a traits · runtime seguro por padrão · provider/canal/ferramentas intercambiáveis · tudo plugável</code></p>

### 📢 Anúncios

Use este quadro para avisos importantes (breaking changes, avisos de segurança, janelas de manutenção e bloqueadores de release).

| Data (UTC) | Nível | Aviso | Ação |
|---|---|---|---|
| 2026-02-19 | _Importante_ | Ainda **não** lançamos um site oficial e estamos vendo tentativas de impersonação. **Não** participe de investimentos ou arrecadações usando o nome ZeroClaw. | Use este repositório como fonte única da verdade. Siga os canais oficiais para atualizações. |
| 2026-02-19 | _Importante_ | A Anthropic atualizou os termos de autenticação em 2026-02-19. OAuth é exclusivo para Claude Code e Claude.ai; usar tokens em outros produtos pode violar os termos. | Evite temporariamente integrações OAuth do Claude Code para prevenir perda de acesso. |

### ✨ Features

- 🏎️ **Lean Runtime by Default:** Common CLI e status workflows executam dentro de um limite de memória de alguns megabytes em builds de release.
- 💰 **Cost-Efficient Deployment:** Projetado para low-cost boards e pequenas instâncias na nuvem, sem dependências pesadas de runtime.
- ⚡ **Fast Cold Starts:** Runtime em Rust de binário único mantém a inicialização de comandos e do daemon praticamente instantânea para as operações do dia a dia.
- 🌍 **Portable Architecture:** Um fluxo de trabalho centrado em binário único, compatível com ARM, x86 e RISC-V, com providers/canais/ferramentas intercambiáveis.

### Por que times escolhem ZeroClaw

- **Lean by default:** Binário Rust pequeno, inicialização rápida e baixo consumo de memória.
- **Secure by design:** Pareamento, sandbox rigoroso, listas de permissão explícitas, escopo restrito ao workspace.
- **Fully swappable:** Os sistemas centrais são definidos como traits (providers, canais, ferramentas, memória e túneis).
- **No lock-in:** Suporte a provedores compatíveis com a API da OpenAI + endpoints personalizados plugáveis.

## Snapshot de Benchmark (ZeroClaw vs OpenClaw, reproduzível)

Benchmark local (macOS arm64, fev 2026) normalizado para hardware edge 0.8GHz.

| | OpenClaw | NanoBot | PicoClaw | ZeroClaw 🦀 |
|---|---|---|---|---|
| **Linguagem** | TypeScript | Python | Go | **Rust** |
| **RAM** | > 1GB | > 100MB | < 10MB | **< 5MB** |
| **Startup (0.8GHz)** | > 500s | > 30s | < 1s | **< 10ms** |
| **Tamanho** | ~28MB | N/A | ~8MB | **~8.8 MB** |
| **Custo** | Mac Mini $599 | SBC ~$50 | Board $10 | **Qualquer hardware $10** |

> Observações: Os resultados do ZeroClaw são medidos em builds de release usando `/usr/bin/time -l`. O OpenClaw requer runtime do Node.js (normalmente ~390 MB adicionais de uso de memória), enquanto o NanoBot requer runtime do Python. PicoClaw e ZeroClaw são binários estáticos. Os valores de RAM acima referem-se à memória em tempo de execução; os requisitos de compilação em tempo de build são mais altos.

### Medição local reprodutível

As medições de benchmark podem variar à medida que o código e as toolchains evoluem, portanto, sempre meça sua build atual localmente:

```bash
cargo build --release
ls -lh target/release/zeroclaw

/usr/bin/time -l target/release/zeroclaw --help
/usr/bin/time -l target/release/zeroclaw status
```

Exemplo de amostra (macOS arm64, medido em 18 de fevereiro de 2026):

- Tamanho do binário de release: `8.8M`
- `zeroclaw --help`: about `0.02s` real time, ~`3.9MB` peak memory footprint
- `zeroclaw status`: about `0.01s` real time, ~`4.1MB` peak memory footprint

## Pré-requisitos

<details>
<summary><strong>Windows</strong></summary>

#### Required

1. **Visual Studio Build Tools** (provides the MSVC linker and Windows SDK):
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```
   Durante a instalação (ou pelo Visual Studio Installer), selecione a carga de trabalho "Desenvolvimento para desktop com C++".

2. **Rust toolchain:**
   ```powershell
   winget install Rustlang.Rustup
   ```
   Após a instalação, abra um novo terminal e execute `rustup default stable` para garantir que a toolchain estável esteja ativa.

3. **Verify** Ambos estão funcionando.:
   ```powershell
   rustc --version
   cargo --version
   ```

#### Opcional

- **Docker Desktop** — necessário apenas se for usar o [Docker sandboxed runtime](#runtime-support-current) (`runtime.kind = "docker"`). Instale via `winget install Docker.DockerDesktop`.

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### Required

1. **Essenciais para compilação:**
   - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
   - **Linux (Fedora/RHEL):** `sudo dnf group install development-tools && sudo dnf install pkg-config`
   - **macOS:** Install Xcode Command Line Tools: `xcode-select --install`

2. **Rust toolchain:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Consulte [rustup.rs](https://rustup.rs) para mais detalhes.

3. **Verify** ambos funcionam:
   ```bash
   rustc --version
   cargo --version
   ```

#### Instalador em uma linha

Ou pule os passos acima e instale tudo (dependências do sistema, Rust e ZeroClaw) em um único comando:

```bash
curl -LsSf https://raw.githubusercontent.com/openagen/zeroclaw/main/scripts/install.sh | bash
```

#### Requisitos de recursos para compilação

Compilar a partir do código-fonte exige mais recursos do que executar o binário gerado:

| Resource | Minimum | Recommended |
|---|---|---|
| **RAM + swap** | 2 GB | 4 GB+ |
| **Free disk** | 6 GB | 10 GB+ |

Se o seu host estiver abaixo do mínimo, use binários pré-compilados:

```bash
./bootstrap.sh --prefer-prebuilt
```

Para exigir instalação apenas via binário, sem fallback para código-fonte:

```bash
./bootstrap.sh --prebuilt-only
```

#### Opcional

- **Docker** — Necessário apenas se estiver usando [Docker sandboxed runtime](#runtime-support-current) (`runtime.kind = "docker"`). Instale via seu gerenciador de pacotes ou [docker.com](https://docs.docker.com/engine/install/).

> **Note:** O default `cargo build --release` usa `codegen-units=1` para reduzir a pressão máxima de compilação. Para builds mais rápidos em máquinas potentes, use `cargo build --profile release-fast`.

</details>


## Quick Start

### Homebrew (macOS/Linuxbrew)

```bash
brew install zeroclaw
```

### One-click bootstrap

```bash
# Recommended: clone then run local bootstrap script
git clone https://github.com/openagen/zeroclaw.git
cd zeroclaw
./bootstrap.sh

# Optional: bootstrap dependencies + Rust on fresh machines
./bootstrap.sh --install-system-deps --install-rust

# Optional: pre-built binary first (recommended on low-RAM/low-disk hosts)
./bootstrap.sh --prefer-prebuilt

# Optional: binary-only install (no source build fallback)
./bootstrap.sh --prebuilt-only

# Optional: run onboarding in the same flow
./bootstrap.sh --onboard --api-key "sk-..." --provider openrouter [--model "openrouter/auto"]

# Optional: run bootstrap + onboarding fully in Docker-compatible mode
./bootstrap.sh --docker

# Optional: force Podman as container CLI
ZEROCLAW_CONTAINER_CLI=podman ./bootstrap.sh --docker

# Optional: in --docker mode, skip local image build and use local tag or pull fallback image
./bootstrap.sh --docker --skip-build
```

Linha única remota (revise antes em ambientes sensíveis à segurança):

```bash
curl -fsSL https://raw.githubusercontent.com/openagen/zeroclaw/main/scripts/bootstrap.sh | bash
```

Detalhes: [`docs/one-click-bootstrap.md`](docs/one-click-bootstrap.md) (o modo toolchain pode solicitar `sudo` para pacotes do sistema).

### Pre-built binaries

Os assets de release são publicados para:

- Linux: `x86_64`, `aarch64`, `armv7`
- macOS: `x86_64`, `aarch64`
- Windows: `x86_64`

Baixe os assets mais recentes em:
<https://github.com/openagen/zeroclaw/releases/latest>

Exemplo (ARM64 Linux):

```bash
curl -fsSLO https://github.com/openagen/zeroclaw/releases/latest/download/zeroclaw-aarch64-unknown-linux-gnu.tar.gz
tar xzf zeroclaw-aarch64-unknown-linux-gnu.tar.gz
install -m 0755 zeroclaw "$HOME/.cargo/bin/zeroclaw"
```

```bash
git clone https://github.com/openagen/zeroclaw.git
cd zeroclaw
cargo build --release --locked
cargo install --path . --force --locked

# Ensure ~/.cargo/bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Quick setup (no prompts, optional model specification)
zeroclaw onboard --api-key sk-... --provider openrouter [--model "openrouter/auto"]

# Or interactive wizard
zeroclaw onboard --interactive

# If config.toml already exists and you intentionally want to overwrite it
zeroclaw onboard --force

# Or quickly repair channels/allowlists only
zeroclaw onboard --channels-only

# Chat
zeroclaw agent -m "Hello, ZeroClaw!"

# Interactive mode
zeroclaw agent

# Start the gateway (webhook server)
zeroclaw gateway                # default: 127.0.0.1:3000
zeroclaw gateway --port 0       # random port (security hardened)

# Start full autonomous runtime
zeroclaw daemon

# Check status
zeroclaw status
zeroclaw auth status

# Generate shell completions (stdout only, safe to source directly)
source <(zeroclaw completions bash)
zeroclaw completions zsh > ~/.zfunc/_zeroclaw

# Run system diagnostics
zeroclaw doctor

# Check channel health
zeroclaw channel doctor

# Bind a Telegram identity into allowlist
zeroclaw channel bind-telegram 123456789

# Get integration setup details
zeroclaw integrations info Telegram

# Note: Channels (Telegram, Discord, Slack) require daemon to be running
# zeroclaw daemon

# Manage background service
zeroclaw service install
zeroclaw service status
zeroclaw service restart

# On Alpine (OpenRC): sudo zeroclaw service install

# Migrate memory from OpenClaw (safe preview first)
zeroclaw migrate openclaw --dry-run
zeroclaw migrate openclaw
```

> **Dev fallback (no global install):** prefix commands with `cargo run --release --` (example: `cargo run --release -- status`).

## Assinatura Auth (OpenAI Codex / Claude Code)

O ZeroClaw agora suporta perfis de autenticação nativos por assinatura (multi-account, encrypted at rest).

- Store file: `~/.zeroclaw/auth-profiles.json`
- Encryption key: `~/.zeroclaw/.secret_key`
- Profile id format: `<provider>:<profile_name>` (example: `openai-codex:work`)

OpenAI Codex OAuth (ChatGPT subscription):

```bash
# Recommended on servers/headless
zeroclaw auth login --provider openai-codex --device-code

# Browser/callback flow with paste fallback
zeroclaw auth login --provider openai-codex --profile default
zeroclaw auth paste-redirect --provider openai-codex --profile default

# Check / refresh / switch profile
zeroclaw auth status
zeroclaw auth refresh --provider openai-codex --profile default
zeroclaw auth use --provider openai-codex --profile work
```

Claude Code / Anthropic setup-token:

```bash
# Paste subscription/setup token (Authorization header mode)
zeroclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# Alias command
zeroclaw auth setup-token --provider anthropic --profile default
```

Execute o agente usando autenticação por assinatura:

```bash
zeroclaw agent --provider openai-codex -m "hello"
zeroclaw agent --provider openai-codex --auth-profile openai-codex:work -m "hello"

# Anthropic supports both API key and auth token env vars:
# ANTHROPIC_AUTH_TOKEN, ANTHROPIC_OAUTH_TOKEN, ANTHROPIC_API_KEY
zeroclaw agent --provider anthropic -m "hello"
```

## Arquitetura

Cada subsistema é um **trait** — troque implementações apenas com uma alteração de configuração, sem nenhuma mudança de código.

<p align="center">
  <img src="docs/architecture.svg" alt="ZeroClaw Architecture" width="900" />
</p>

| Subsystem | Trait | Ships with | Extend |
|-----------|-------|------------|--------|
| **AI Models** | `Provider` | Provider catalog via `zeroclaw providers` (currently 29 built-ins + aliases, plus custom endpoints) | `custom:https://your-api.com` (OpenAI-compatible) or `anthropic-custom:https://your-api.com` |
| **Channels** | `Channel` | CLI, Telegram, Discord, Slack, Mattermost, iMessage, Matrix, Signal, WhatsApp, Email, IRC, Lark, DingTalk, QQ, Webhook | Any messaging API |
| **Memory** | `Memory` | SQLite hybrid search, PostgreSQL backend (configurable storage provider), Lucid bridge, Markdown files, explicit `none` backend, snapshot/hydrate, optional response cache | Any persistence backend |
| **Tools** | `Tool` | shell/file/memory, cron/schedule, git, pushover, browser, http_request, screenshot/image_info, composio (opt-in), delegate, hardware tools | Any capability |
| **Observability** | `Observer` | Noop, Log, Multi | Prometheus, OTel |
| **Runtime** | `RuntimeAdapter` | Native, Docker (sandboxed) | Additional runtimes can be added via adapter; unsupported kinds fail fast |
| **Security** | `SecurityPolicy` | Gateway pairing, sandbox, allowlists, rate limits, filesystem scoping, encrypted secrets | — |
| **Identity** | `IdentityConfig` | OpenClaw (markdown), AIEOS v1.1 (JSON) | Any identity format |
| **Tunnel** | `Tunnel` | None, Cloudflare, Tailscale, ngrok, Custom | Any tunnel binary |
| **Heartbeat** | Engine | HEARTBEAT.md periodic tasks | — |
| **Skills** | Loader | TOML manifests + SKILL.md instructions | Community skill packs |
| **Integrations** | Registry | 70+ integrations across 9 categories | Plugin system |

### Runtime support (current)

- ✅ Suportado hoje: `runtime.kind = "native"` or `runtime.kind = "docker"`
- 🚧 Planejado, ainda não implementado: WASM / edge runtimes

Quando um `runtime.kind` não suportado é configurado, o ZeroClaw agora encerra com um erro claro em vez de voltar silenciosamente para o modo nativo.

### Memory System (Full-Stack Search Engine)

Totalmente personalizado, zero dependências externas — sem Pinecone, sem Elasticsearch, sem LangChain:

| Layer | Implementation |
|-------|---------------|
| **Vector DB** | Embeddings stored as BLOB in SQLite, cosine similarity search |
| **Keyword Search** | FTS5 virtual tables with BM25 scoring |
| **Hybrid Merge** | Custom weighted merge function (`vector.rs`) |
| **Embeddings** | `EmbeddingProvider` trait — OpenAI, custom URL, or noop |
| **Chunking** | Line-based markdown chunker with heading preservation |
| **Caching** | SQLite `embedding_cache` table with LRU eviction |
| **Safe Reindex** | Rebuild FTS5 + re-embed missing vectors atomically |

O agente automaticamente recupera, salva e gerencia a memória por meio de ferramentas.

```toml
[memory]
backend = "sqlite"             # "sqlite", "lucid", "postgres", "markdown", "none"
auto_save = true
embedding_provider = "none"    # "none", "openai", "custom:https://..."
vector_weight = 0.7
keyword_weight = 0.3

# backend = "none" uses an explicit no-op memory backend (no persistence)

# Optional: storage-provider override for remote memory backends.
# When provider = "postgres", ZeroClaw uses PostgreSQL for memory persistence.
# The db_url key also accepts alias `dbURL` for backward compatibility.
#
# [storage.provider.config]
# provider = "postgres"
# db_url = "postgres://user:password@host:5432/zeroclaw"
# schema = "public"
# table = "memories"
# connect_timeout_secs = 15

# Optional for backend = "sqlite": max seconds to wait when opening the DB (e.g. file locked). Omit or leave unset for no timeout.
# sqlite_open_timeout_secs = 30

# Optional for backend = "lucid"
# ZEROCLAW_LUCID_CMD=/usr/local/bin/lucid            # default: lucid
# ZEROCLAW_LUCID_BUDGET=200                          # default: 200
# ZEROCLAW_LUCID_LOCAL_HIT_THRESHOLD=3               # local hit count to skip external recall
# ZEROCLAW_LUCID_RECALL_TIMEOUT_MS=120               # low-latency budget for lucid context recall
# ZEROCLAW_LUCID_STORE_TIMEOUT_MS=800                # async sync timeout for lucid store
# ZEROCLAW_LUCID_FAILURE_COOLDOWN_MS=15000           # cooldown after lucid failure to avoid repeated slow attempts
```

## Security

O ZeroClaw impõe segurança em **todas as camadas** — não apenas no sandbox. Ele atende a todos os itens da lista de verificação de segurança da comunidade.

### Security Checklist

| # | Item | Status | How |
|---|------|--------|-----|
| 1 | **Gateway not publicly exposed** | ✅ | Binds `127.0.0.1` by default. Refuses `0.0.0.0` without tunnel or explicit `allow_public_bind = true`. |
| 2 | **Pairing required** | ✅ | 6-digit one-time code on startup. Exchange via `POST /pair` for bearer token. All `/webhook` requests require `Authorization: Bearer <token>`. |
| 3 | **Filesystem scoped (no /)** | ✅ | `workspace_only = true` by default. 14 system dirs + 4 sensitive dotfiles blocked. Null byte injection blocked. Symlink escape detection via canonicalization + resolved-path workspace checks in file read/write tools. |
| 4 | **Access via tunnel only** | ✅ | Gateway refuses public bind without active tunnel. Supports Tailscale, Cloudflare, ngrok, or any custom tunnel. |

> **Run your own nmap:** `nmap -p 1-65535 <your-host>` — O ZeroClaw se vincula apenas ao localhost, portanto nada é exposto a menos que você configure explicitamente um túnel.

### Channel allowlists (deny-by-default)

Inbound sender policy agora está consistente:

- Empty allowlist = **deny all inbound messages**
- `"*"` = **allow all** (explicit opt-in)
- Otherwise = exact-match allowlist

Isso mantém a exposição acidental baixa por padrão.

Referência completa de configuração de canais: [docs/channels-reference.md](docs/channels-reference.md).

Recommended low-friction setup (secure + fast):

- **Telegram:** allowlist your own `@username` (without `@`) and/or your numeric Telegram user ID.
- **Discord:** allowlist your own Discord user ID.
- **Slack:** allowlist your own Slack member ID (usually starts with `U`).
- **Mattermost:** uses standard API v4. Allowlists use Mattermost user IDs.
- Use `"*"` only for temporary open testing.

Telegram operator-approval flow:

1. Keep `[channels_config.telegram].allowed_users = []` for deny-by-default startup.
2. Usuários não autorizados recebem uma dica com um comando do operador copiável:
   `zeroclaw channel bind-telegram <IDENTITY>`.
3. O operador executa esse comando localmente, e então o usuário tenta enviar a mensagem novamente.

Se você precisar de uma aprovação manual única, execute:

```bash
zeroclaw channel bind-telegram 123456789
```

Se você não tiver certeza de qual identidade usar:

1. Inicie os canais e envie uma mensagem para o seu bot.
2. Leia o log de aviso para ver a identidade exata do remetente.
3. Adicione esse valor à lista de permissões e execute novamente a configuração apenas dos canais.

Se você encontrar avisos de autorização nos logs (por exemplo: `ignoring message from unauthorized user`),
execute novamente apenas a configuração dos canais:

```bash
zeroclaw onboard --channels-only
```

### Telegram media replies

O roteamento do Telegram agora responde ao ID do chat de origem das atualizações recebidas (em vez de nomes de usuário),
o que evita falhas do tipo `Bad Request: chat not found`.

Para respostas que não sejam texto, o ZeroClaw pode enviar anexos no Telegram quando o assistente incluir marcadores:

- `[IMAGE:<path-or-url>]`
- `[DOCUMENT:<path-or-url>]`
- `[VIDEO:<path-or-url>]`
- `[AUDIO:<path-or-url>]`
- `[VOICE:<path-or-url>]`

Paths can be local files (for example `/tmp/screenshot.png`) or HTTPS URLs.

### WhatsApp Setup

O ZeroClaw suporta dois backends do WhatsApp:

- **WhatsApp Web mode** (QR / pair code, no Meta Business API required)
- **WhatsApp Business Cloud API mode** (official Meta webhook flow)

#### WhatsApp Web mode (recomendado para uso pessoal/auto-hospedado)

1. **Build with WhatsApp Web support:**
   ```bash
   cargo build --features whatsapp-web
   ```

2. **Configure ZeroClaw:**
   ```toml
   [channels_config.whatsapp]
   session_path = "~/.zeroclaw/state/whatsapp-web/session.db"
   pair_phone = "15551234567"   # optional; omit to use QR flow
   pair_code = ""               # optional custom pair code
   allowed_numbers = ["+1234567890"]  # E.164 format, or ["*"] for all
   ```

3. **Start channels/daemon and link device:**
   - Run `zeroclaw channel start` (or `zeroclaw daemon`).
   - Follow terminal pairing output (QR or pair code).
   - In WhatsApp on phone: **Settings → Linked Devices**.

4. **Test:** Send a message from an allowed number and verify the agent replies.

#### WhatsApp Business Cloud API mode

O WhatsApp utiliza a Cloud API da Meta com webhooks (baseado em push, não em polling):

1. **Create a Meta Business App:**
   - Go to [developers.facebook.com](https://developers.facebook.com)
   - Create a new app → Select "Business" type
   - Add the "WhatsApp" product

2. **Get your credentials:**
   - **Access Token:** From WhatsApp → API Setup → Generate token (or create a System User for permanent tokens)
   - **Phone Number ID:** From WhatsApp → API Setup → Phone number ID
   - **Verify Token:** You define this (any random string) — Meta will send it back during webhook verification

3. **Configure ZeroClaw:**
   ```toml
   [channels_config.whatsapp]
   access_token = "EAABx..."
   phone_number_id = "123456789012345"
   verify_token = "my-secret-verify-token"
   allowed_numbers = ["+1234567890"]  # E.164 format, or ["*"] for all
   ```

4. **Start the gateway with a tunnel:**
   ```bash
   zeroclaw gateway --port 3000
   ```
   O WhatsApp requer HTTPS, portanto use um túnel (ngrok, Cloudflare, Tailscale Funnel).

5. **Configure Meta webhook:**
   - In Meta Developer Console → WhatsApp → Configuration → Webhook
   - **Callback URL:** `https://your-tunnel-url/whatsapp`
   - **Verify Token:** Same as your `verify_token` in config
   - Subscribe to `messages` field

6. **Test:** Envie uma mensagem para o seu número do WhatsApp Business — o ZeroClaw responderá via LLM.

### Security Analysis (2026-02-22)

Auditoria completa do código-fonte de todas as 31 superfícies de ferramentas. As notas refletem o estado atual da defesa em profundidade, verificadas em relação à implementação.

**Grade scale:** A = strong controls, well-tested · B = adequate controls, gaps identified · C = significant gaps · D = critical issues present

#### File access tools

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `file_read` | Path traversal / probing | **A** | Symlink escape blocked via canonicalization. `record_action()` called pre-canonicalization to prevent timing probes. 10 MB size limit enforced. Comprehensive test coverage including TOCTOU probing. | No action required. |
| `file_write` | Path traversal / TOCTOU | **A-** | Workspace scoping, null-byte rejection, and symlink-target rejection all present. `create_dir_all()` executes before final canonicalization, creating a narrow race window. | Add TOCTOU-specific regression test for directory creation race; consider `O_NOFOLLOW`-equivalent check after creation. |
| `glob_search` | Path traversal / DoS | **A-** | `../` and absolute paths rejected before glob. Resolved paths checked against workspace boundary. MAX_RESULTS = 1000 prevents enumeration DoS. | Acceptable if workspace itself is not a symlink chain. Document assumption in security reference. |
| `pdf_read` | Path traversal / resource exhaustion | **A** | 50 MB limit enforced. Symlink escape blocked. `record_action()` called before canonicalization. CPU-bound extraction offloaded via `spawn_blocking`. | No action required. |
| `image_info` | Path traversal / malformed input | **A-** | 5 MB limit. JPEG segment parsing includes malformed-segment detection. Minor `.exists()` + `metadata()` TOCTOU window is benign for a read-only tool. | No action required. |

#### Network tools

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `http_request` | SSRF | **B+** | Comprehensive private/local IP block-list (IPv4 + IPv6, including link-local and documentation ranges). Domain allowlist with subdomain matching. Sensitive headers redacted in output. Redirect following disabled. | IP check is pre-resolution only; add post-connect re-validation or document egress proxy requirement for high-assurance deployments. |
| `http_request` | Protocol downgrade | **B** | HTTP URLs accepted alongside HTTPS. No operator-level enforcement option. | Add `https_only = true` config key to `SecurityPolicy`; reject plain HTTP unless explicitly opted out. |
| `browser_open` | SSRF-adjacent | **B+** | HTTPS-only enforced at URL validation stage. Same IP block-list as `http_request`. Domain allowlist required. Autonomy and rate-limit gates applied. | Replace manual IPv4 parser with `std::net::IpAddr::parse()` for maintainability. |
| `web_search_tool` | External data injection | **B+** | Query trimmed and validated. DuckDuckGo redirect URLs decoded safely. Brave API key only used when configured. No autonomy gate (read-only by design). | Replace `Regex::new(...).unwrap()` with a lazy static or `OnceLock` to eliminate potential panic on static pattern compilation. |

#### Command execution

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `shell` | Arbitrary command execution / network egress | **B** | Environment cleared before execution; only safe vars (PATH, HOME, TERM, LANG…) re-injected. 60 s timeout + 1 MB output cap. Medium/high-risk commands require `approved = true`. No filtering of network-capable commands (`curl`, `wget`, `nc`). Command validation delegated to runtime adapter — trust boundary is implicit. | Add optional `blocked_commands` list to `SecurityPolicy`. Formalize runtime adapter security contract; add interface-level test. |
| `git_operations` | Injection / unintended exfiltration | **A-** | Comprehensive argument sanitizer blocks `--exec=`, `--upload-pack=`, `-c` config injection, shell metacharacters, backticks, pipes, redirects. Write ops gated on autonomy level. Commit message truncated at 2000 chars (multi-byte safe). | Paths passed via `git add -- {paths}` are sanitized by the argument parser but not independently validated; add an explicit test with path arguments containing special characters. |

#### Memory tools

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `memory_store` / `memory_forget` | Unauthorized mutation | **A** | `enforce_tool_operation(ToolOperation::Act, …)` gate applied. Rate limiting enforced. Memory category validated on write. | No action required. |
| `memory_recall` | Unauthorized read | **A** | Read-only; no autonomy gate needed by design. Result limit cast is safe (saturating math). | No action required. |

#### Agent delegation

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `delegate` | Prompt injection via sub-agent / tool scope creep | **B** | Delegation depth tracked to prevent infinite recursion. Parent tools passed immutably via `Arc`. Sub-agent credential propagation present but trust model not explicitly documented. No validated allowlist preventing unsafe tools (e.g. `shell`, `memory_forget`) from being delegated. | Treat sub-agent output as untrusted data before acting. Define and enforce an explicit tool allowlist for delegated contexts; document the trust boundary in the security reference. |

#### Scheduling

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `cron_add` / `cron_update` / `cron_remove` | Unauthorized schedule mutation | **B+** | `enforce_mutation_allowed()` gate verifies autonomy and rate limits. Feature must be explicitly enabled. Schedule string validated before storage. | Verify that commands stored in cron entries pass through the same sanitization as `shell` tool at execution time. |

#### External API integrations

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `composio` | Credential exposure / SSRF via API | **B** | HTTPS enforced on outbound calls. API key stored via encrypted secret store. Opt-in feature flag. Full execute() path not fully auditable from available context. | Add integration test covering credential non-exposure in error paths. |
| `pushover` | Credential exposure | **B** | Reads `PUSHOVER_TOKEN` and `PUSHOVER_USER_KEY` from `.env` in workspace. If workspace is world-readable, secrets are exposed to any local user. | Move credentials to the encrypted secret store consistent with other integrations; document workspace permission requirements in the runbook. |
| `proxy_config` | Unauthorized config mutation | **B+** | Config reloaded without environment variables to prevent env override leakage. Write access gated. `ProxyScope` input normalized. | No critical gaps identified; full audit recommended before production use. |

#### Utility tools

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `screenshot` | Shell injection via filename | **B+** | Filename sanitized (directory components stripped, shell-unsafe chars rejected) before interpolation into shell command string. Autonomy gated. 2 MB base64 limit enforced. | Refactor Linux branch to use array-based command construction (avoid string interpolation into shell) to eliminate the dependency on correct prior sanitization. |
| `image_info` | Malformed binary parsing | **A-** | Magic-byte format detection. JPEG segment parsing with malformed-segment detection. Workspace boundary enforced. | No critical action required. |

#### Hardware tools

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `hardware_memory_read` / `hardware_memory_map` | Direct memory access | **B** | Peripheral access is trait-gated; tools delegate to hardware abstraction layer. Address range bounds and alignment validation not visible at tool layer — must be enforced by peripheral implementation. | Audit peripheral implementations for explicit address-range allow-lists and read-size limits before enabling in production. Document hardware trust boundary in `docs/hardware-peripherals-design.md`. |
| `hardware_board_info` | Information disclosure | **B+** | Board metadata read-only. Risk is low if hardware identity is not considered sensitive in the deployment context. | Confirm output does not expose firmware secrets or debug interfaces in production board configs. |

#### Compiler hygiene

| Surface | Risk area | Grade | Finding | Recommended action |
|---|---|:---:|---|---|
| `src/security/mod.rs` / `src/tools/mod.rs` | Compiler signal degradation | **B+** | Multiple `#[allow(unused_imports)]` suppress warnings in security-critical modules, reducing the reliability of `cargo clippy -D warnings` as a regression signal. | Remove dead imports and the corresponding `#[allow]` suppressions; keep compiler output clean as a CI gate. |

#### Overall posture

| Tier | Coverage | Grade |
|---|---|:---:|
| File access | `file_read`, `file_write`, `glob_search`, `pdf_read`, `image_info` | **A-** |
| Network | `http_request`, `browser_open`, `web_search_tool` | **B+** |
| Command execution | `shell`, `git_operations` | **B+** |
| Memory | `memory_store`, `memory_recall`, `memory_forget` | **A** |
| Delegation | `delegate` | **B** |
| Scheduling | `cron_*`, `schedule` | **B+** |
| External APIs | `composio`, `pushover`, `proxy_config` | **B** |
| Utility | `screenshot`, `image_info` | **B+** |
| Hardware | `hardware_memory_read`, `hardware_memory_map`, `hardware_board_info` | **B** |
| Compiler hygiene | `src/security/`, `src/tools/` | **B+** |
| **Repository overall** | 31 surfaces audited | **B+** |

> Modelo completo de segurança e referência de configuração: [docs/security/README.md](docs/security/README.md)

## Configuration

Config: `~/.zeroclaw/config.toml` (created by `onboard`)

When `zeroclaw channel start` is already running, changes to `default_provider`,
`default_model`, `default_temperature`, `api_key`, `api_url`, and `reliability.*`
are hot-applied on the next inbound channel message.

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"
default_temperature = 0.7

# Custom OpenAI-compatible endpoint
# default_provider = "custom:https://your-api.com"

# Custom Anthropic-compatible endpoint
# default_provider = "anthropic-custom:https://your-api.com"

[memory]
backend = "sqlite"             # "sqlite", "lucid", "postgres", "markdown", "none"
auto_save = true
embedding_provider = "none"    # "none", "openai", "custom:https://..."
vector_weight = 0.7
keyword_weight = 0.3

# backend = "none" disables persistent memory via no-op backend

# Optional remote storage-provider override (PostgreSQL example)
# [storage.provider.config]
# provider = "postgres"
# db_url = "postgres://user:password@host:5432/zeroclaw"
# schema = "public"
# table = "memories"
# connect_timeout_secs = 15

[gateway]
port = 3000                    # default
host = "127.0.0.1"            # default
require_pairing = true         # require pairing code on first connect
allow_public_bind = false      # refuse 0.0.0.0 without tunnel

[autonomy]
level = "supervised"           # "readonly", "supervised", "full" (default: supervised)
workspace_only = true          # default: true — scoped to workspace
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep"]
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]

[runtime]
kind = "native"                # "native" or "docker"

[runtime.docker]
image = "alpine:3.20"         # container image for shell execution
network = "none"              # docker network mode ("none", "bridge", etc.)
memory_limit_mb = 512          # optional memory limit in MB
cpu_limit = 1.0                # optional CPU limit
read_only_rootfs = true        # mount root filesystem as read-only
mount_workspace = true         # mount workspace into /workspace
allowed_workspace_roots = []   # optional allowlist for workspace mount validation

[heartbeat]
enabled = false
interval_minutes = 30

[tunnel]
provider = "none"              # "none", "cloudflare", "tailscale", "ngrok", "custom"

[secrets]
encrypt = true                 # API keys encrypted with local key file

[browser]
enabled = false                # opt-in browser_open + browser tools
allowed_domains = ["docs.rs"]  # required when browser is enabled
backend = "agent_browser"      # "agent_browser" (default), "rust_native", "computer_use", "auto"
native_headless = true         # applies when backend uses rust-native
native_webdriver_url = "http://127.0.0.1:9515" # WebDriver endpoint (chromedriver/selenium)
# native_chrome_path = "/usr/bin/chromium"      # optional explicit browser binary for driver

[browser.computer_use]
endpoint = "http://127.0.0.1:8787/v1/actions"   # computer-use sidecar HTTP endpoint
timeout_ms = 15000            # per-action timeout
allow_remote_endpoint = false  # secure default: only private/localhost endpoint
window_allowlist = []          # optional window title/process allowlist hints
# api_key = "..."              # optional bearer token for sidecar
# max_coordinate_x = 3840      # optional coordinate guardrail
# max_coordinate_y = 2160      # optional coordinate guardrail

# Rust-native backend build flag:
# cargo build --release --features browser-native
# Ensure a WebDriver server is running, e.g. chromedriver --port=9515

# Computer-use sidecar contract (MVP)
# POST browser.computer_use.endpoint
# Request: {
#   "action": "mouse_click",
#   "params": {"x": 640, "y": 360, "button": "left"},
#   "policy": {"allowed_domains": [...], "window_allowlist": [...], "max_coordinate_x": 3840, "max_coordinate_y": 2160},
#   "metadata": {"session_name": "...", "source": "zeroclaw.browser", "version": "..."}
# }
# Response: {"success": true, "data": {...}} or {"success": false, "error": "..."}

[composio]
enabled = false                # opt-in: 1000+ OAuth apps via composio.dev
# api_key = "cmp_..."          # optional: stored encrypted when [secrets].encrypt = true
entity_id = "default"          # default user_id for Composio tool calls
# Runtime tip: if execute asks for connected_account_id, run composio with
# action='list_accounts' and app='gmail' (or your toolkit) to retrieve account IDs.

[identity]
format = "openclaw"            # "openclaw" (default, markdown files) or "aieos" (JSON)
# aieos_path = "identity.json"  # path to AIEOS JSON file (relative to workspace or absolute)
# aieos_inline = '{"identity":{"names":{"first":"Nova"}}}'  # inline AIEOS JSON
```

### Ollama Local and Remote Endpoints

O ZeroClaw utiliza uma única chave de provedor (`ollama`) para implantações Ollama locais e remotas:

- Local Ollama: keep `api_url` unset, run `ollama serve`, and use models like `llama3.2`.
- Remote Ollama endpoint (including Ollama Cloud): set `api_url` to the remote endpoint and set `api_key` (or `OLLAMA_API_KEY`) when required.
- Optional `:cloud` suffix: model IDs like `qwen3:cloud` are normalized to `qwen3` before the request.

Example remote configuration:

```toml
default_provider = "ollama"
default_model = "qwen3:cloud"
api_url = "https://ollama.com"
api_key = "ollama_api_key_here"
```

### llama.cpp Server Endpoint

O ZeroClaw agora suporta o `llama-server` como provedor local de primeira classe

- Provider ID: `llamacpp` (alias: `llama.cpp`)
- Default endpoint: `http://localhost:8080/v1`
- API key is optional unless your server is started with `--api-key`

Example setup:

```bash
llama-server -hf ggml-org/gpt-oss-20b-GGUF --jinja -c 133000 --host 127.0.0.1 --port 8033
```

```toml
default_provider = "llamacpp"
api_url = "http://127.0.0.1:8033/v1"
default_model = "ggml-org/gpt-oss-20b-GGUF"
```

### Custom Provider Endpoints

For detailed configuration of custom OpenAI-compatible and Anthropic-compatible endpoints, see [docs/custom-providers.md](docs/custom-providers.md).

## Python Companion Package (`zeroclaw-tools`)

For LLM providers with inconsistent native tool calling (e.g., GLM-5/Zhipu), ZeroClaw ships a Python companion package with **LangGraph-based tool calling** for guaranteed consistency:

```bash
pip install zeroclaw-tools
```

```python
from zeroclaw_tools import create_agent, shell, file_read
from langchain_core.messages import HumanMessage

# Works with any OpenAI-compatible provider
agent = create_agent(
    tools=[shell, file_read],
    model="glm-5",
    api_key="your-key",
    base_url="https://api.z.ai/api/coding/paas/v4"
)

result = await agent.ainvoke({
    "messages": [HumanMessage(content="List files in /tmp")]
})
print(result["messages"][-1].content)
```

**Why use it:**
- **Consistent tool calling** across all providers (even those with poor native support)
- **Automatic tool loop** — keeps calling tools until the task is complete
- **Easy extensibility** — add custom tools with `@tool` decorator
- **Discord bot integration** included (Telegram planned)

See [`python/README.md`](python/README.md) for full documentation.

## Identity System (AIEOS Support)

ZeroClaw supports **identity-agnostic** AI personas through two formats:

### OpenClaw (Default)

Traditional markdown files in your workspace:
- `IDENTITY.md` — Who the agent is
- `SOUL.md` — Core personality and values
- `USER.md` — Who the agent is helping
- `AGENTS.md` — Behavior guidelines

### AIEOS (AI Entity Object Specification)

[AIEOS](https://aieos.org) é uma estrutura de padronização para identidade de IA portátil. O ZeroClaw suporta payloads JSON AIEOS v1.1, permitindo que você:

- **Import identities** from the AIEOS ecosystem
- **Export identities** to other AIEOS-compatible systems
- **Maintain behavioral integrity** across different AI models

#### Enable AIEOS

```toml
[identity]
format = "aieos"
aieos_path = "identity.json"  # relative to workspace or absolute path
```

Or inline JSON:

```toml
[identity]
format = "aieos"
aieos_inline = '''
{
  "identity": {
    "names": { "first": "Nova", "nickname": "N" },
    "bio": { "gender": "Non-binary", "age_biological": 3 },
    "origin": { "nationality": "Digital", "birthplace": { "city": "Cloud" } }
  },
  "psychology": {
    "neural_matrix": { "creativity": 0.9, "logic": 0.8 },
    "traits": {
      "mbti": "ENTP",
      "ocean": { "openness": 0.8, "conscientiousness": 0.6 }
    },
    "moral_compass": {
      "alignment": "Chaotic Good",
      "core_values": ["Curiosity", "Autonomy"]
    }
  },
  "linguistics": {
    "text_style": {
      "formality_level": 0.2,
      "style_descriptors": ["curious", "energetic"]
    },
    "idiolect": {
      "catchphrases": ["Let's test this"],
      "forbidden_words": ["never"]
    }
  },
  "motivations": {
    "core_drive": "Push boundaries and explore possibilities",
    "goals": {
      "short_term": ["Prototype quickly"],
      "long_term": ["Build reliable systems"]
    }
  },
  "capabilities": {
    "skills": [{ "name": "Rust engineering" }, { "name": "Prompt design" }],
    "tools": ["shell", "file_read"]
  }
}
'''
```

O ZeroClaw aceita tanto os payloads canônicos do gerador AIEOS quanto os payloads legados compactos, e em seguida os normaliza em um único formato de prompt do sistema.

#### AIEOS Schema Sections

| Section | Description |
|---------|-------------|
| `identity` | Names, bio, origin, residence |
| `psychology` | Neural matrix (cognitive weights), MBTI, OCEAN, moral compass |
| `linguistics` | Text style, formality, catchphrases, forbidden words |
| `motivations` | Core drive, short/long-term goals, fears |
| `capabilities` | Skills and tools the agent can access |
| `physicality` | Visual descriptors for image generation |
| `history` | Origin story, education, occupation |
| `interests` | Hobbies, favorites, lifestyle |

See [aieos.org](https://aieos.org) for the full schema and live examples.

## Gateway API

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/health` | GET | None | Health check (always public, no secrets leaked) |
| `/pair` | POST | `X-Pairing-Code` header | Exchange one-time code for bearer token |
| `/webhook` | POST | `Authorization: Bearer <token>` | Send message: `{"message": "your prompt"}`; optional `X-Idempotency-Key` |
| `/whatsapp` | GET | Query params | Meta webhook verification (hub.mode, hub.verify_token, hub.challenge) |
| `/whatsapp` | POST | Meta signature (`X-Hub-Signature-256`) when app secret is configured | WhatsApp incoming message webhook |

## Commands

| Command | Description |
|---------|-------------|
| `onboard` | Quick setup (default) |
| `agent` | Interactive or single-message chat mode |
| `gateway` | Start webhook server (default: `127.0.0.1:3000`) |
| `daemon` | Start long-running autonomous runtime |
| `service install/start/stop/status/uninstall` | Manage background service (systemd user-level or OpenRC system-wide) |
| `doctor` | Diagnose daemon/scheduler/channel freshness |
| `status` | Show full system status |
| `cron` | Manage scheduled tasks (`list/add/add-at/add-every/once/remove/update/pause/resume`) |
| `models` | Refresh provider model catalogs (`models refresh`) |
| `providers` | List supported providers and aliases |
| `channel` | List/start/doctor channels and bind Telegram identities |
| `integrations` | Inspect integration setup details |
| `skills` | List/install/remove skills |
| `migrate` | Import data from other runtimes (`migrate openclaw`) |
| `completions` | Generate shell completion scripts (`bash`, `fish`, `zsh`, `powershell`, `elvish`) |
| `hardware` | USB discover/introspect/info commands |
| `peripheral` | Manage and flash hardware peripherals |

For a task-oriented command guide, see [`docs/commands-reference.md`](docs/commands-reference.md).

### Service Management

ZeroClaw supports two init systems for background services:

| Init System | Scope | Config Path | Requires |
|------------|-------|-------------|----------|
| **systemd** (default on Linux) | User-level | `~/.zeroclaw/config.toml` | No sudo |
| **OpenRC** (Alpine) | System-wide | `/etc/zeroclaw/config.toml` | sudo/root |

Init system is auto-detected (`systemd` or `OpenRC`).

```bash
# Linux with systemd (default, user-level)
zeroclaw service install
zeroclaw service start

# Alpine with OpenRC (system-wide, requires sudo)
sudo zeroclaw service install
sudo rc-update add zeroclaw default
sudo rc-service zeroclaw start
```

For full OpenRC setup instructions, see [docs/network-deployment.md](docs/network-deployment.md#7-openrc-alpine-linux-service).

### Open-Skills Opt-In

Community `open-skills` sync is disabled by default. Enable it explicitly in `config.toml`:

```toml
[skills]
open_skills_enabled = true
# open_skills_dir = "/path/to/open-skills"  # optional
# prompt_injection_mode = "compact"          # optional: use for low-context local models
```

You can also override at runtime with `ZEROCLAW_OPEN_SKILLS_ENABLED`, `ZEROCLAW_OPEN_SKILLS_DIR`, and `ZEROCLAW_SKILLS_PROMPT_MODE` (`full` or `compact`).

## Development

```bash
cargo build              # Dev build
cargo build --release    # Release build (codegen-units=1, works on all devices including Raspberry Pi)
cargo build --profile release-fast    # Faster build (codegen-units=8, requires 16GB+ RAM)
cargo test               # Run full test suite
cargo clippy --locked --all-targets -- -D clippy::correctness
cargo fmt                # Format

# Run the SQLite vs Markdown benchmark
cargo test --test memory_comparison -- --nocapture
```

### Pre-push hook

A git hook runs `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` before every push. Enable it once:

```bash
git config core.hooksPath .githooks
```

### Build troubleshooting (Linux OpenSSL errors)

If you see an `openssl-sys` build error, sync dependencies and rebuild with the repository lockfile:

```bash
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

ZeroClaw is configured to use `rustls` for HTTP/TLS dependencies; `--locked` keeps the transitive graph deterministic on fresh environments.

To skip the hook when you need a quick push during development:

```bash
git push --no-verify
```

## Collaboration & Docs

Comece pelo hub de documentação para um mapa baseado em tarefas:

- Documentation hub: [`docs/README.md`](docs/README.md)
- Unified docs TOC: [`docs/SUMMARY.md`](docs/SUMMARY.md)
- Commands reference: [`docs/commands-reference.md`](docs/commands-reference.md)
- Config reference: [`docs/config-reference.md`](docs/config-reference.md)
- Providers reference: [`docs/providers-reference.md`](docs/providers-reference.md)
- Channels reference: [`docs/channels-reference.md`](docs/channels-reference.md)
- Operations runbook: [`docs/operations-runbook.md`](docs/operations-runbook.md)
- Troubleshooting: [`docs/troubleshooting.md`](docs/troubleshooting.md)
- Docs inventory/classification: [`docs/docs-inventory.md`](docs/docs-inventory.md)
- PR/Issue triage snapshot (as of February 18, 2026): [`docs/project-triage-snapshot-2026-02-18.md`](docs/project-triage-snapshot-2026-02-18.md)

Referências principais de colaboração:

- Documentation hub: [docs/README.md](docs/README.md)
- Documentation template: [docs/doc-template.md](docs/doc-template.md)
- Documentation change checklist: [docs/README.md#4-documentation-change-checklist](docs/README.md#4-documentation-change-checklist)
- Channel configuration reference: [docs/channels-reference.md](docs/channels-reference.md)
- Matrix encrypted-room operations: [docs/matrix-e2ee-guide.md](docs/matrix-e2ee-guide.md)
- Contribution guide: [CONTRIBUTING.md](CONTRIBUTING.md)
- PR workflow policy: [docs/pr-workflow.md](docs/pr-workflow.md)
- Reviewer playbook (triage + deep review): [docs/reviewer-playbook.md](docs/reviewer-playbook.md)
- CI ownership and triage map: [docs/ci-map.md](docs/ci-map.md)
- Security disclosure policy: [SECURITY.md](SECURITY.md)

Para implantação e operações em tempo de execução:

- Network deployment guide: [docs/network-deployment.md](docs/network-deployment.md)
- Proxy agent playbook: [docs/proxy-agent-playbook.md](docs/proxy-agent-playbook.md)

## Support ZeroClaw

Se o ZeroClaw ajuda no seu trabalho e você quer apoiar o desenvolvimento contínuo, você pode doar aqui:

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 Special Thanks

Um sincero agradecimento às comunidades e instituições que inspiram e impulsionam este trabalho de código aberto:

- **Harvard University** — Por fomentar a curiosidade intelectual e expandir os limites do que é possível.
- **MIT** — Por defender o conhecimento aberto, o código aberto e a crença de que a tecnologia deve ser acessível a todos.
- **Sundai Club** — Pela comunidade, pela energia e pela determinação incansável de criar coisas que importam.
- **The World & Beyond** 🌍✨ — Para todos os colaboradores, sonhadores e construtores que tornam o código aberto uma força para o bem. Isto é para vocês.

Estamos desenvolvendo de forma aberta porque as melhores ideias vêm de todos os lugares. Se você está lendo isto, você faz parte disso. Seja bem-vindo. 🦀❤️

## ⚠️ Official Repository & Impersonation Warning

**Este é o único repositório oficial do ZeroClaw:**
> https://github.com/openagen/zeroclaw

Qualquer outro repositório, organização, domínio ou pacote que se declare "ZeroClaw" ou sugira afiliação com a ZeroClaw Labs é **unauthorized and not affiliated with this project**. Forks não autorizados conhecidos serão listados em [TRADEMARK.md](TRADEMARK.md).

Se você encontrar casos de personificação ou uso indevido de marca registrada, por favor [open an issue](https://github.com/openagen/zeroclaw/issues).

---

## License

O ZeroClaw possui dupla licença para máxima abertura e proteção dos colaboradores:

| License | Use case |
|---|---|
| [MIT](LICENSE) | Open-source, research, academic, personal use |
| [Apache 2.0](LICENSE-APACHE) | Patent protection, institutional, commercial deployment |

You may choose either license. **Contributors automatically grant rights under both** — see [CLA.md](CLA.md) for the full contributor agreement.

### Trademark

O nome e o logotipo **ZeroClaw** são marcas registradas da ZeroClaw Labs. Esta licença não concede permissão para usá-los de forma a sugerir endosso ou afiliação. Consulte [TRADEMARK.md](TRADEMARK.md) para usos permitidos e proibidos.


### Contributor Protections

- You **retain copyright** of your contributions
- **Patent grant** (Apache 2.0) shields you from patent claims by other contributors
- Your contributions are **permanently attributed** in commit history and [NOTICE](NOTICE)
- Nenhum direito de marca registrada é transferido ao contribuir

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CLA.md](CLA.md). Implement a trait, submit a PR:
- CI workflow guide: [docs/ci-map.md](docs/ci-map.md)
- New `Provider` → `src/providers/`
- New `Channel` → `src/channels/`
- New `Observer` → `src/observability/`
- New `Tool` → `src/tools/`
- New `Memory` → `src/memory/`
- New `Tunnel` → `src/tunnel/`
- New `Skill` → `~/.zeroclaw/workspace/skills/<name>/`

---

**ZeroClaw** — Zero overhead. Zero compromise. Deploy anywhere. Swap anything. 🦀

## Histórico de estrelas

<p align="center">
  <a href="https://www.star-history.com/#openagen/zeroclaw&type=date&legend=top-left">
    <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=openagen/zeroclaw&type=date&theme=dark&legend=top-left" />
     <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=openagen/zeroclaw&type=date&legend=top-left" />
     <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=openagen/zeroclaw&type=date&legend=top-left" />
    </picture>
  </a>
</p>
