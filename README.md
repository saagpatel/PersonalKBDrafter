# PersonalKBDrafter

An AI-powered desktop application that automatically drafts knowledge base articles from Jira tickets and publishes them to Confluence.

## What It Is

PersonalKBDrafter is a Tauri-based desktop app that streamlines the creation of technical documentation. It connects to your Jira instance, pulls ticket information, uses a local AI model to draft comprehensive KB articles, and publishes them directly to Confluence—all while ensuring quality and security.

## Why You'd Want This

**Stop wasting time on documentation boilerplate.** If you're an IT support engineer, DevOps professional, or technical writer who:

- Constantly documents solutions to recurring problems
- Maintains a knowledge base in Confluence
- Works with Jira tickets daily
- Wants consistent, high-quality documentation
- Needs to catch sensitive data before it goes public

Then this tool automates the tedious parts while keeping you in control.

## Key Features

### 🎯 **Smart Drafting**

- Fetches Jira ticket details (summary, description, comments)
- Uses local Ollama AI models to generate structured KB articles
- Follows customizable templates for consistency
- Automatically structures content (Problem, Solution, Prerequisites, etc.)

### 🔒 **Security First**

- Scans for sensitive data (API keys, passwords, tokens, PII) before publishing
- Flags potential security issues for review
- Keeps your LLM completely local—no cloud API calls

### ✅ **Quality Scoring**

- Automated quality assessment of drafted articles
- Scores based on completeness, clarity, structure, and detail
- Ensures documentation meets your standards

### 📝 **Full Workflow**

- Save drafts and iterate before publishing
- Live markdown preview
- Direct Confluence publishing with proper formatting
- Tracks publication status and links

### ⚙️ **Flexible Setup**

- Works with any Ollama-compatible model
- Customizable article templates
- Supports Jira and Confluence Data Center style PAT workflows
- Stores Jira and Confluence tokens in the OS keychain

## What You'd Use It For

**Common use cases:**

1. **Incident Documentation** - Document how you solved a production issue for future reference
2. **How-To Guides** - Turn detailed Jira tickets into step-by-step documentation
3. **Troubleshooting Articles** - Create searchable solutions for common problems
4. **Runbook Generation** - Document operational procedures from tickets
5. **Post-Mortem Reports** - Draft incident post-mortems with consistent structure

## How to Use It

### Prerequisites

- **Ollama** running locally with a model installed (e.g., `llama3.2:3b`, `mistral`)
- **Jira Data Center** account with API access (Personal Access Token)
- **Confluence Data Center** account with API access (Personal Access Token)
- macOS, Windows, or Linux

### Installation

1. **Download the latest release** from the releases page
2. **Install Ollama** from [ollama.com](https://ollama.com)
3. **Pull a model**: `ollama pull llama3.2:3b`
4. **Launch PersonalKBDrafter**

### Setup

1. **Configure Settings** (⚙️ icon):
   - Ollama URL (default: `http://localhost:11434`)
   - Model name (e.g., `llama3.2:3b`)
   - Jira base URL and PAT
   - Confluence base URL and PAT

2. **Test Connections** - Verify all services are reachable

### Basic Workflow

1. **Search Jira** - Find the ticket you want to document
2. **Select Template** - Choose a KB article structure (or use default)
3. **Generate Draft** - AI drafts the article from ticket content
4. **Review & Edit** - Refine the generated content
5. **Check Quality** - Review the automated quality score
6. **Scan for Secrets** - Ensure no sensitive data is present
7. **Save Draft** - Store locally for later or publish immediately
8. **Publish to Confluence** - One-click publishing with proper formatting

### Keyboard Shortcuts

- `Cmd/Ctrl + S` - Save draft
- `Cmd/Ctrl + Shift + P` - Toggle preview
- `Cmd/Ctrl + D` - Show drafts list

## Technology Stack

- **Frontend**: React + TypeScript + Vite + TailwindCSS
- **Backend**: Rust + Tauri
- **Database**: SQLite (local storage)
- **AI**: Ollama (local LLM inference)
- **Integrations**: Jira REST API, Confluence REST API
- **State Management**: Zustand
- **Markdown**: CodeMirror 6

## Architecture

```
┌─────────────────┐
│   React UI      │  User interacts with forms, previews
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Tauri Bridge   │  IPC commands to Rust backend
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────────┐
│              Rust Backend                   │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐ │
│  │ Jira API │  │ Ollama   │  │ Confluence│ │
│  │ Service  │  │ Service  │  │ Service   │ │
│  └──────────┘  └──────────┘  └───────────┘ │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐ │
│  │ Quality  │  │ Sensitive│  │  SQLite   │ │
│  │ Scorer   │  │ Data Scan│  │  Database │ │
│  └──────────┘  └──────────┘  └───────────┘ │
└─────────────────────────────────────────────┘
```

## Development

### Requirements

- Rust 1.70+
- Node.js 18+
- Ollama running locally

### Setup

```bash
# Check local prerequisites
npm run check:prereqs

# Install frontend dependencies
npm ci

# Normal dev mode (fast rebuilds, more disk usage over time)
npm run tauri dev

# Lean dev mode (temporary build caches, auto-clean on exit)
npm run dev:lean

# Build for production
npm run tauri build

# Run Rust tests
cd src-tauri && cargo test

# Run the local verification bundle
bash .codex/scripts/run_verify_commands.sh
```

### Normal Dev vs Lean Dev

- `npm run tauri dev`
  - Uses default local caches in the repo.
  - Faster subsequent startup/rebuilds.
  - Grows disk usage over time (`src-tauri/target`, `node_modules/.vite`).

- `npm run dev:lean`
  - Starts the same Tauri + Vite development flow.
  - Redirects heavy build caches to a temporary location.
  - Automatically removes temporary caches when the process exits.
  - Uses less persistent disk space, but cold starts are slower.

### Cleanup Commands

```bash
# Remove heavy build artifacts only (keep dependencies for speed)
npm run clean:heavy

# Remove all reproducible local caches, including dependencies
npm run clean:full
```

Disk/speed tradeoff in plain language:

- Keep `node_modules` if you want reasonable startup speed day-to-day.
- Use `clean:heavy` regularly to reclaim most build bloat safely.
- Use `clean:full` only when you want a full local reset and accept a slower next install/build.

### Project Structure

```
PersonalKBDrafter/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/              # Custom React hooks
│   ├── stores/             # Zustand state stores
│   └── lib/                # Utilities (markdown parser, etc.)
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── services/       # Business logic (Jira, Ollama, Confluence)
│   │   ├── db/             # SQLite database layer
│   │   └── models/         # Data structures
│   └── migrations/         # Database migrations
└── README.md
```

## Security & Privacy

- **Local-first**: All AI processing happens on your machine
- **No telemetry**: Zero data collection or phone-home
- **Credential storage**: Jira/Confluence tokens stored in OS keychain
- **Sensitive data detection**: Regex-based scanning for common secrets
- **Manual review**: You approve everything before it goes to Confluence

## Current Validation Scope

- Local launch is validated with `npm`, Vite, and Tauri 2 on macOS.
- Jira parsing supports plain-text fields and Atlassian Document Format (ADF) content.
- Jira PATs and Confluence PATs are stored in the OS keychain, and both service base URLs persist in the local SQLite app database.
- End-to-end live smoke testing still requires a safe Jira test ticket, a non-production Confluence space, and valid service credentials on the machine running the app.

## Roadmap

- [ ] Multi-language template support
- [ ] Batch article generation from JQL queries
- [ ] Confluence page hierarchy support
- [ ] Export to multiple formats (PDF, Markdown, HTML)
- [ ] Custom quality scoring rules
- [ ] Plugin system for custom integrations

## Contributing

Contributions welcome! This is a personal project but PRs for bug fixes and features are appreciated.

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see LICENSE file for details

## Support

Found a bug? Have a feature request? [Open an issue](https://github.com/samueladad75-byte/PersonalKBDrafter/issues)

---

Built with ❤️ for IT support engineers who are tired of writing the same documentation over and over.
