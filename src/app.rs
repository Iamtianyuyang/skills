use crate::data;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{Input, TextArea};

pub enum Mode {
    Browse,
    Viewing { content: String, rendered: bool, scroll: u16 },
    AddStep1,
    AddStep2 { cat_typing: bool },
    AddStep3,
    ConfirmDelete,
}

pub struct App {
    pub mode: Mode,

    // browse state
    pub categories: Vec<String>,
    pub cat_idx: usize,
    pub entries: Vec<String>,
    pub entry_idx: usize,
    pub focus_right: bool,

    // add flow state
    pub textarea: TextArea<'static>,
    pub add_content: String,
    pub add_cat_list: Vec<String>,
    pub add_cat_idx: usize,
    pub add_cat_input: String,
    pub add_category: String,
    pub add_name: String,

    // delete
    pub del_target: Option<(String, String)>,

    // transient status message
    pub message: Option<String>,

    // updated each frame by draw_view so G can scroll to exact bottom
    pub view_height: u16,
}

impl App {
    pub fn new() -> Result<Self> {
        let categories = data::categories();
        let entries = categories.first().map(|c| data::entries(c)).unwrap_or_default();
        Ok(Self {
            mode: Mode::Browse,
            categories,
            cat_idx: 0,
            entries,
            entry_idx: 0,
            focus_right: false,
            textarea: TextArea::default(),
            add_content: String::new(),
            add_cat_list: vec![],
            add_cat_idx: 0,
            add_cat_input: String::new(),
            add_category: String::new(),
            add_name: String::new(),
            del_target: None,
            message: None,
            view_height: 0,
        })
    }

    pub fn selected_category(&self) -> Option<&str> {
        self.categories.get(self.cat_idx).map(String::as_str)
    }

    pub fn selected_entry(&self) -> Option<&str> {
        self.entries.get(self.entry_idx).map(String::as_str)
    }

    fn refresh_categories(&mut self) {
        self.categories = data::categories();
        self.cat_idx = self.cat_idx.min(self.categories.len().saturating_sub(1));
        self.refresh_entries();
    }

    fn refresh_entries(&mut self) {
        self.entries = self
            .selected_category()
            .map(data::entries)
            .unwrap_or_default();
        self.entry_idx = self.entry_idx.min(self.entries.len().saturating_sub(1));
    }

    /// Returns true if the app should quit.
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<bool> {
        self.message = None;
        match self.mode {
            Mode::Browse => self.on_browse(key),
            Mode::Viewing { .. } => self.on_view(key),
            Mode::AddStep1 => self.on_add_step1(key),
            Mode::AddStep2 { .. } => self.on_add_step2(key),
            Mode::AddStep3 => self.on_add_step3(key),
            Mode::ConfirmDelete => self.on_confirm_delete(key),
        }
    }

    fn on_browse(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            KeyCode::Char('a') => {
                self.textarea = TextArea::default();
                self.mode = Mode::AddStep1;
            }
            KeyCode::Char('d') => {
                if let (Some(cat), Some(entry)) =
                    (self.selected_category(), self.selected_entry())
                {
                    self.del_target = Some((cat.to_string(), entry.to_string()));
                    self.mode = Mode::ConfirmDelete;
                }
            }
            KeyCode::Enter => {
                if self.focus_right {
                    if let (Some(cat), Some(entry)) =
                        (self.selected_category(), self.selected_entry())
                    {
                        let content = data::read_entry(cat, entry)?;
                        self.mode = Mode::Viewing { content, rendered: true, scroll: 0 };
                    }
                } else if !self.entries.is_empty() {
                    self.focus_right = true;
                }
            }
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                if !self.entries.is_empty() {
                    self.focus_right = true;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.focus_right = false;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.focus_right {
                    if self.entry_idx > 0 {
                        self.entry_idx -= 1;
                    }
                } else {
                    if self.cat_idx > 0 {
                        self.cat_idx -= 1;
                    }
                    self.refresh_entries();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.focus_right {
                    if self.entry_idx + 1 < self.entries.len() {
                        self.entry_idx += 1;
                    }
                } else {
                    if self.cat_idx + 1 < self.categories.len() {
                        self.cat_idx += 1;
                    }
                    self.refresh_entries();
                }
            }
            _ => {}
        }
        Ok(false)
    }

    fn on_view(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.mode = Mode::Browse,
            KeyCode::Tab => {
                if let Mode::Viewing { ref mut rendered, ref mut scroll, .. } = self.mode {
                    *rendered = !*rendered;
                    *scroll = 0;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Mode::Viewing { ref mut scroll, .. } = self.mode {
                    *scroll = scroll.saturating_add(1);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if let Mode::Viewing { ref mut scroll, .. } = self.mode {
                    *scroll = scroll.saturating_sub(1);
                }
            }
            KeyCode::Char('d') => {
                if let Mode::Viewing { ref mut scroll, .. } = self.mode {
                    *scroll = scroll.saturating_add(10);
                }
            }
            KeyCode::Char('u') => {
                if let Mode::Viewing { ref mut scroll, .. } = self.mode {
                    *scroll = scroll.saturating_sub(10);
                }
            }
            KeyCode::Char('g') => {
                if let Mode::Viewing { ref mut scroll, .. } = self.mode {
                    *scroll = 0;
                }
            }
            KeyCode::Char('G') => {
                if let Mode::Viewing { ref content, ref rendered, ref mut scroll, .. } = self.mode {
                    let total_lines = if *rendered {
                        crate::md::render(content).len() as u16
                    } else {
                        content.lines().count() as u16
                    };
                    *scroll = total_lines.saturating_sub(self.view_height);
                }
            }
_ => {}
        }
        Ok(false)
    }

    fn on_add_step1(&mut self, key: KeyEvent) -> Result<bool> {
        match (key.code, key.modifiers) {
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                let content = self.textarea.lines().join("\n");
                if content.trim().is_empty() {
                    return Ok(false);
                }
                self.add_content = content;
                self.add_cat_list = data::categories();
                self.add_cat_idx = 0;
                self.add_cat_input = String::new();
                self.mode = Mode::AddStep2 { cat_typing: false };
            }
            (KeyCode::Esc, _) => self.mode = Mode::Browse,
            _ => {
                self.textarea.input(Input::from(key));
            }
        }
        Ok(false)
    }

    fn on_add_step2(&mut self, key: KeyEvent) -> Result<bool> {
        let cat_typing = matches!(self.mode, Mode::AddStep2 { cat_typing: true });

        if cat_typing {
            match key.code {
                KeyCode::Enter => {
                    let name = self.add_cat_input.trim().to_string();
                    if !name.is_empty() {
                        self.add_category = name;
                        self.add_name = String::new();
                        self.mode = Mode::AddStep3;
                    }
                }
                KeyCode::Esc => self.mode = Mode::AddStep2 { cat_typing: false },
                KeyCode::Backspace => {
                    self.add_cat_input.pop();
                }
                KeyCode::Char(c) => self.add_cat_input.push(c),
                _ => {}
            }
        } else {
            let total = self.add_cat_list.len() + 1; // last item = "新建分类"
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.add_cat_idx > 0 {
                        self.add_cat_idx -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.add_cat_idx + 1 < total {
                        self.add_cat_idx += 1;
                    }
                }
                KeyCode::Enter => {
                    if self.add_cat_idx == self.add_cat_list.len() {
                        self.add_cat_input = String::new();
                        self.mode = Mode::AddStep2 { cat_typing: true };
                    } else {
                        self.add_category = self.add_cat_list[self.add_cat_idx].clone();
                        self.add_name = String::new();
                        self.mode = Mode::AddStep3;
                    }
                }
                KeyCode::Esc => self.mode = Mode::Browse,
                _ => {}
            }
        }
        Ok(false)
    }

    fn on_add_step3(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Enter => {
                let name = self.add_name.trim().to_string();
                if !name.is_empty() {
                    data::save_entry(&self.add_category, &name, &self.add_content)?;
                    let saved_cat = self.add_category.clone();
                    self.refresh_categories();
                    if let Some(idx) = self.categories.iter().position(|c| c == &saved_cat) {
                        self.cat_idx = idx;
                        self.refresh_entries();
                        if let Some(eidx) = self.entries.iter().position(|e| e == &name) {
                            self.entry_idx = eidx;
                        }
                        self.focus_right = true;
                    }
                    self.message = Some(format!("✓ 已保存: {}/{}", saved_cat, name));
                    self.mode = Mode::Browse;
                }
            }
            KeyCode::Esc => self.mode = Mode::Browse,
            KeyCode::Backspace => {
                self.add_name.pop();
            }
            KeyCode::Char(c) => self.add_name.push(c),
            _ => {}
        }
        Ok(false)
    }

    fn on_confirm_delete(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some((cat, name)) = self.del_target.take() {
                    data::delete_entry(&cat, &name)?;
                    self.refresh_categories();
                    self.message = Some("✓ 已删除".into());
                }
                self.mode = Mode::Browse;
            }
            _ => {
                self.del_target = None;
                self.mode = Mode::Browse;
            }
        }
        Ok(false)
    }
}
