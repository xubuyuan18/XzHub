# XzHub Architecture

## Overview

XzHub is designed as a lightweight Windows desktop application.

```
Windows
  |
  +-- Tauri Application
        |
        +-- React UI
        |     - Dashboard
        |     - Settings
        |     - Charts
        |
        +-- Rust Core
              - Tray management
              - Scheduler
              - Data collection
              - SQLite storage
```

## Design Principles

1. UI only displays data.
2. Background services handle monitoring tasks.
3. All monitoring sources use provider adapters.

## Providers

Examples:

- HTTP endpoint
- JSON API
- VPS Agent
- Ping monitor
- Custom scripts
