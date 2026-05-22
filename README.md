# skills

A terminal UI for managing personal knowledge snippets — prompts, notes, role definitions, references, or anything you want to recall fast.

Built with [ratatui](https://github.com/ratatui-org/ratatui) (Rust TUI framework).

## What it does

Organizes any piece of text into named entries inside custom categories.  
Open the TUI, browse your library, view and copy entries to clipboard in seconds.

```
┌── 分类 ────────────┐ ┌── agents ──────────────────────────────────┐
│                    │ │                                            │
│ ▶ agents           │ │ ▶ claude-roles                            │
│   prompts          │ │   sisyphus                                │
│   references       │ │                                            │
│                    │ │                                            │
└────────────────────┘ └────────────────────────────────────────────┘
 Enter/l:选条目  a:添加  j/k:导航  q:退出
```

## Install

**Requirements:** Rust toolchain ([rustup.rs](https://rustup.rs))

```bash
git clone https://github.com/Iamtianyuyang/skills.git
cd skills
cargo build --release
ln -sf "$PWD/target/release/skills" ~/.local/bin/skills
```

## Usage

```bash
skills   # open TUI
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `k` or ↑↓ | Navigate list |
| `l` / → / Tab / Enter | Focus entries panel |
| `h` / ← | Back to categories |
| `Enter` (on entry) | View full content |
| `y` | Copy content to clipboard |
| `a` | Add new entry (3-step flow) |
| `d` → `y` | Delete selected entry |
| `q` / Esc | Quit / go back |

## Adding an entry

Press `a` from the main view to start a 3-step flow:

1. **Content** — paste or type anything (multi-line). Press `Ctrl+D` to proceed.
2. **Category** — choose an existing category or create a new one.
3. **Name** — give the entry a short name. Press `Enter` to save.

## Data

All entries are stored as plain text files under `~/skills/`:

```
~/skills/
├── agents/
│   ├── claude-roles
│   └── sisyphus
├── prompts/
│   └── debug-pro
└── references/
    └── api-endpoints
```

Edit files directly with any text editor if needed.
