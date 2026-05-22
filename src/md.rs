use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn render(content: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut in_code_block = false;

    for raw in content.lines() {
        if raw.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            lines.push(Line::from(Span::styled(
                format!("  {raw}"),
                Style::default().fg(Color::Yellow),
            )));
            continue;
        }

        if let Some(rest) = raw.strip_prefix("# ") {
            lines.push(Line::from(Span::styled(
                rest.to_owned(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )));
        } else if let Some(rest) = raw.strip_prefix("## ") {
            lines.push(Line::from(Span::styled(
                rest.to_owned(),
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            )));
        } else if let Some(rest) = raw.strip_prefix("### ") {
            lines.push(Line::from(Span::styled(
                rest.to_owned(),
                Style::default().add_modifier(Modifier::BOLD),
            )));
        } else if let Some(rest) = raw.strip_prefix("#### ") {
            lines.push(Line::from(Span::styled(
                rest.to_owned(),
                Style::default().fg(Color::Magenta),
            )));
        } else if raw.starts_with("- ") || raw.starts_with("* ") {
            let mut spans = vec![Span::raw("  • ")];
            spans.extend(inline(&raw[2..]));
            lines.push(Line::from(spans));
        } else if raw.starts_with("> ") {
            lines.push(Line::from(Span::styled(
                format!("▌ {}", &raw[2..]),
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            )));
        } else if raw == "---" || raw == "***" || raw == "___" {
            lines.push(Line::from(Span::styled(
                "─".repeat(60),
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            lines.push(Line::from(inline(raw)));
        }
    }
    lines
}

fn inline(s: &str) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut buf = String::new();

    while i < chars.len() {
        // **bold**
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if !buf.is_empty() {
                spans.push(Span::raw(buf.clone()));
                buf.clear();
            }
            i += 2;
            let mut inner = String::new();
            while i < chars.len() {
                if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
                    i += 2;
                    break;
                }
                inner.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(inner, Style::default().add_modifier(Modifier::BOLD)));
        }
        // *italic*
        else if chars[i] == '*' {
            if !buf.is_empty() {
                spans.push(Span::raw(buf.clone()));
                buf.clear();
            }
            i += 1;
            let mut inner = String::new();
            while i < chars.len() && chars[i] != '*' {
                inner.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
            spans.push(Span::styled(inner, Style::default().add_modifier(Modifier::ITALIC)));
        }
        // `code`
        else if chars[i] == '`' {
            if !buf.is_empty() {
                spans.push(Span::raw(buf.clone()));
                buf.clear();
            }
            i += 1;
            let mut inner = String::new();
            while i < chars.len() && chars[i] != '`' {
                inner.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
            spans.push(Span::styled(inner, Style::default().fg(Color::Yellow)));
        } else {
            buf.push(chars[i]);
            i += 1;
        }
    }
    if !buf.is_empty() {
        spans.push(Span::raw(buf));
    }
    if spans.is_empty() {
        spans.push(Span::raw(String::new()));
    }
    spans
}
