# dumb-manager

An ultra-lightweight, high-speed, local-first daily life tracker for Linux (designed for KDE Plasma / Wayland).

## Features

- **Notes & Interactive Checklists:** Full Markdown editor with instant live preview and interactive checkboxes (`- [ ]` / `- [x]`), split/edit/preview modes, search filtering, and automatic persistence.
- **"Days Since" Streaks:** Track habits or behaviors to avoid (fast food, smoking, unnecessary shopping). Features prominent real-time counters, quick resets with optional notes/reasons, and comprehensive history tracking with interval duration calculations.
- **Quantitative Habits & Monthly Heatmap:** Set daily targets (e.g. 8 glasses of water, 3 tooth brushings), increment/decrement counts on the fly, and audit consistency with a month-by-month calendar heatmap and compliance percentages.
- **Local-First & Private:** Embedded SQLite database located at `$XDG_DATA_HOME/dumb-manager/dumb.db` (fallback `~/.local/share/dumb-manager/dumb.db`). No telemetry, no cloud dependencies.
- **Lightweight & Fast:** Cold start < 1s, minimal memory footprint (< 80 MB RAM), and Wayland-native rendering via WebKit2GTK.

## Tech Stack

- **Backend:** Tauri 2.0 (Rust) + SQLite (`rusqlite` bundled) + `chrono` + `uuid` + `dirs`
- **Frontend:** Vue 3 (Composition API) + Vite + TypeScript + Tailwind CSS (`@tailwindcss/typography`) + Lucide Icons + `dayjs` + `marked` + `DOMPurify`

## Getting Started

### Prerequisites

- Node.js & pnpm
- Rust toolchain (`cargo`, `rustc`)
- WebKit2GTK development libraries (`webkit2gtk-4.1` on Arch / EndeavourOS)

### Development

Run the Tauri development application:

```bash
pnpm tauri dev
```

Or run frontend only in a browser:

```bash
pnpm dev
```

### Build

Compile the native desktop binary:

```bash
pnpm tauri build
```
