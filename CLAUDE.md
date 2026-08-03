# XzHub AI Coding Guide

## Project Goal

Build a Windows tray dashboard application.

## Architecture Rules

- Use Tauri 2 for desktop integration.
- Use React + TypeScript for UI.
- Use Rust for system-level features.
- Keep monitoring logic outside the frontend.
- Use provider adapters for external services.

## Development Order

1. Desktop shell and tray icon.
2. Floating dashboard window.
3. UI components.
4. SQLite storage.
5. Service providers.
6. Notifications.

## Code Style

Prefer simple, maintainable implementations over premature abstraction.
