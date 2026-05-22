use crate::app::{App, Mode};
use crate::md;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let is_browse = matches!(app.mode, Mode::Browse | Mode::ConfirmDelete);
    let is_viewing = matches!(app.mode, Mode::Viewing { .. });
    let is_step1 = matches!(app.mode, Mode::AddStep1);
    let is_step2 = matches!(app.mode, Mode::AddStep2 { .. });
    let is_step3 = matches!(app.mode, Mode::AddStep3);
    let is_confirm = matches!(app.mode, Mode::ConfirmDelete);

    if is_browse { draw_browse(f, app); }
    if is_viewing { draw_view(f, app); }
    if is_step1 { draw_step1(f, app); }
    if is_step2 { draw_step2(f, app); }
    if is_step3 { draw_step3(f, app); }
    if is_confirm { draw_confirm(f, app); }

    if let Some(ref msg) = app.message.clone() {
        draw_toast(f, msg);
    }
}

// ── Browse ────────────────────────────────────────────────────────────────────

fn draw_browse(f: &mut Frame, app: &App) {
    let area = f.area();
    let [main, bar] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(28), Constraint::Percentage(72)]).areas(main);

    // categories
    let cat_items: Vec<ListItem> = app
        .categories
        .iter()
        .map(|c| ListItem::new(format!("  {c}")))
        .collect();
    let mut cat_state = ListState::default();
    if !app.categories.is_empty() {
        cat_state.select(Some(app.cat_idx));
    }
    let cat_border = if app.focus_right {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Cyan)
    };
    let cat_list = List::new(cat_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 分类 ")
                .border_style(cat_border),
        )
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(cat_list, left, &mut cat_state);

    // entries
    let entry_items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|e| ListItem::new(format!("  {e}")))
        .collect();
    let mut entry_state = ListState::default();
    if !app.entries.is_empty() {
        entry_state.select(Some(app.entry_idx));
    }
    let entry_border = if app.focus_right {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let entry_title = app
        .selected_category()
        .map(|c| format!(" {c} "))
        .unwrap_or_else(|| " 条目 ".into());
    let entry_list = List::new(entry_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(entry_title)
                .border_style(entry_border),
        )
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(entry_list, right, &mut entry_state);

    // status bar
    let hint = if app.focus_right {
        " Enter:查看  a:添加  d:删除  h/←:分类  j/k:导航  q:退出"
    } else {
        " Enter/l:选条目  a:添加  j/k:导航  q:退出"
    };
    f.render_widget(
        Paragraph::new(hint).style(Style::default().fg(Color::DarkGray)),
        bar,
    );
}

// ── View ──────────────────────────────────────────────────────────────────────

fn draw_view(f: &mut Frame, app: &App) {
    let area = f.area();
    let [main, bar] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

    let (content, rendered, scroll) = match &app.mode {
        Mode::Viewing { content, rendered, scroll } => (content.as_str(), *rendered, *scroll),
        _ => ("", false, 0),
    };
    let mode_tag = if rendered { "MD" } else { "原始" };
    let title = match (app.selected_category(), app.selected_entry()) {
        (Some(cat), Some(entry)) => format!(" {cat}/{entry}  [{mode_tag}] "),
        _ => format!(" 查看  [{mode_tag}] "),
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::Cyan));

    if rendered {
        let text = Text::from(md::render(content));
        f.render_widget(
            Paragraph::new(text)
                .block(block)
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0)),
            main,
        );
    } else {
        f.render_widget(
            Paragraph::new(content)
                .block(block)
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0)),
            main,
        );
    }

    f.render_widget(
        Paragraph::new(" j/k:滚动  d/u:翻页  g/G:首/尾  Tab:切换渲染  y:复制  q:返回")
            .style(Style::default().fg(Color::DarkGray)),
        bar,
    );
}

// ── Add step 1: content ───────────────────────────────────────────────────────

fn draw_step1(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let [header, body, bar] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);

    f.render_widget(
        Paragraph::new(" ● 步骤 1/3 — 粘贴或输入内容")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        header,
    );

    app.textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(&app.textarea, body);

    f.render_widget(
        Paragraph::new(" Ctrl+D:下一步  Esc:取消")
            .style(Style::default().fg(Color::DarkGray)),
        bar,
    );
}

// ── Add step 2: category ─────────────────────────────────────────────────────

fn draw_step2(f: &mut Frame, app: &App) {
    let cat_typing = matches!(app.mode, Mode::AddStep2 { cat_typing: true });
    let area = centered_rect(52, 60, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ● 步骤 2/3 — 选择分类 ")
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    f.render_widget(block, area);

    if cat_typing {
        let [label, input, _, hint] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);

        f.render_widget(
            Paragraph::new(" 新分类名:").style(Style::default().fg(Color::Yellow)),
            label,
        );
        f.render_widget(
            Paragraph::new(format!("> {}█", app.add_cat_input))
                .style(Style::default().fg(Color::White)),
            input,
        );
        f.render_widget(
            Paragraph::new(" Enter:确认  Esc:返回列表")
                .style(Style::default().fg(Color::DarkGray)),
            hint,
        );
    } else {
        let [list_area, hint] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);

        let mut items: Vec<ListItem> = app
            .add_cat_list
            .iter()
            .map(|c| ListItem::new(format!("  {c}")))
            .collect();
        items.push(
            ListItem::new("  ＋ 新建分类").style(Style::default().fg(Color::Green)),
        );

        let mut state = ListState::default();
        state.select(Some(app.add_cat_idx));

        let list = List::new(items)
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");
        f.render_stateful_widget(list, list_area, &mut state);

        f.render_widget(
            Paragraph::new(" j/k:导航  Enter:选择  Esc:取消")
                .style(Style::default().fg(Color::DarkGray)),
            hint,
        );
    }
}

// ── Add step 3: name ─────────────────────────────────────────────────────────

fn draw_step3(f: &mut Frame, app: &App) {
    let area = centered_rect(52, 22, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ● 步骤 3/3 — 起个名字 ")
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let [label, input, _, hint] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(inner);

    f.render_widget(
        Paragraph::new(format!(" 分类: {}", app.add_category))
            .style(Style::default().fg(Color::DarkGray)),
        label,
    );
    f.render_widget(
        Paragraph::new(format!("> {}█", app.add_name))
            .style(Style::default().fg(Color::White)),
        input,
    );
    f.render_widget(
        Paragraph::new(" Enter:保存  Esc:取消")
            .style(Style::default().fg(Color::DarkGray)),
        hint,
    );
}

// ── Confirm delete ────────────────────────────────────────────────────────────

fn draw_confirm(f: &mut Frame, app: &App) {
    let area = centered_rect(44, 24, f.area());
    f.render_widget(Clear, area);

    let target = app
        .del_target
        .as_ref()
        .map(|(c, n)| format!("{c}/{n}"))
        .unwrap_or_default();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 确认删除 ")
        .border_style(Style::default().fg(Color::Red));
    let inner = block.inner(area);
    f.render_widget(block, area);

    f.render_widget(
        Paragraph::new(format!("\n  {target}\n\n  y:确认    其他键:取消"))
            .style(Style::default().fg(Color::White)),
        inner,
    );
}

// ── Toast ─────────────────────────────────────────────────────────────────────

fn draw_toast(f: &mut Frame, msg: &str) {
    let area = f.area();
    let w = (msg.chars().count() as u16 + 4).min(area.width);
    let toast_area = Rect::new(
        area.width.saturating_sub(w),
        area.height.saturating_sub(2),
        w,
        1,
    );
    f.render_widget(
        Paragraph::new(format!(" {msg} "))
            .style(Style::default().fg(Color::Black).bg(Color::Green)),
        toast_area,
    );
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn centered_rect(w: u16, h: u16, area: Rect) -> Rect {
    let [_, vert, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(h),
        Constraint::Fill(1),
    ])
    .areas(area);
    let [_, horiz, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(w),
        Constraint::Fill(1),
    ])
    .areas(vert);
    horiz
}
