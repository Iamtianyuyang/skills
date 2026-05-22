# skills

A terminal UI for managing personal knowledge snippets — prompts, notes, commands, or anything you want to recall fast.

Built with [ratatui](https://github.com/ratatui-org/ratatui) (Rust TUI framework).

## What it does

Organizes Markdown entries into named categories. Browse, read with rendered Markdown or raw text, scroll, and add/delete — all from the terminal.

```
┌── 分类 ────────────┐ ┌── nvidia ──────────────────────────────────┐
│                    │ │                                            │
│ ▶ agents           │ │ ▶ prof                                     │
│   nvidia           │ │                                            │
│   test             │ │                                            │
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

### Browse

| Key | Action |
|-----|--------|
| `j` / `k` or ↑↓ | Navigate list |
| `l` / → / Tab / Enter | Focus entries panel |
| `h` / ← | Back to categories |
| `Enter` (on entry) | View full content |
| `a` | Add new entry (3-step flow) |
| `d` → `y` | Delete selected entry |
| `q` / Esc | Quit |

### View

| Key | Action |
|-----|--------|
| `j` / `k` or ↑↓ | Scroll one line |
| `d` / `u` | Scroll 10 lines down / up |
| `g` / `G` | Jump to top / bottom |
| `Tab` | Toggle raw ↔ rendered Markdown |
| `q` / Esc | Back to browse |

## Adding an entry

Press `a` from the main view to start a 3-step flow:

1. **Content** — paste or type Markdown (multi-line). Press `Ctrl+D` to proceed.
2. **Category** — choose an existing category or create a new one.
3. **Name** — give the entry a short name. Press `Enter` to save.

Entries are saved as `<name>.md` under `~/skills/data/<category>/`.

## Data

All entries are stored as Markdown files under `~/skills/data/`:

```
~/skills/data/
├── agents/
│   └── opencode_role.md
└── nvidia/
    └── prof.md
```

The `data/` directory is excluded from git — entries stay local only.
Edit files directly with any text editor if needed.

## Markdown rendering

The built-in renderer supports:

- `#` `##` `###` `####` headings (coloured)
- `**bold**`, `*italic*`, `` `inline code` ``
- `- ` / `* ` bullet lists
- ` ``` ` fenced code blocks
- `> ` blockquotes
- `---` horizontal rules
