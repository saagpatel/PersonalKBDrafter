# PersonalKBDrafter

[![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Turn Jira tickets into Confluence articles automatically — with a local LLM that never phones home.

PersonalKBDrafter is a Tauri desktop app that automates knowledge base documentation. Connect to Jira, pull a ticket, and a local Ollama model drafts a structured KB article — Problem, Solution, Prerequisites, and all — then publishes it directly to Confluence after a security scan for sensitive data.

## Features

- **AI drafting** — local Ollama model generates structured KB articles from Jira ticket summaries, descriptions, and comments
- **Security scanning** — detects API keys, passwords, tokens, and PII before anything is published
- **Quality scoring** — automated assessment of completeness, clarity, structure, and detail level
- **Live markdown preview** — review and iterate before pushing to Confluence
- **Direct publishing** — formatted output goes straight to Confluence with status tracking
- **Customizable templates** — adjust article structure to match your team's standards

## Quick Start

### Prerequisites
- Node.js 18+, Rust toolchain
- [Ollama](https://ollama.com/) installed and running locally
- Jira and Confluence credentials

### Installation
```bash
git clone https://github.com/saagpatel/PersonalKBDrafter.git
cd PersonalKBDrafter
npm install
```

### Usage
```bash
npm run tauri dev
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop runtime | Tauri 2 |
| Language | TypeScript + Rust |
| AI inference | Ollama (local, any compatible model) |
| Integrations | Jira REST API, Confluence REST API |
| UI | React + Tailwind CSS |

## License

MIT
