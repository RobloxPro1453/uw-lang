//! `uw browse`: a terminal UI over the Native atlas. Type to search (part of a name or in-game
//! text), the right side explains the selected thing and lists who uses it; Enter jumps to a
//! user, Esc goes back. State and key handling live in [`App`] so tests drive it without a
//! terminal.

use std::collections::BTreeMap;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::model::{entry_hint, entry_label, short, Detail, Hit, Model, Target, UseSite};

const RESULT_LIMIT: usize = 300;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Results,
    Uses,
}

/// A row of the "used by" list: a group header or a place.
#[derive(Debug, Clone)]
pub enum UseRow {
    Header(String, usize),
    Site(UseSite),
}

/// What the lower right list shows: who mentions the current target, or what it mentions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListMode {
    UsedBy,
    LeadsTo,
}

pub struct App<'a> {
    model: &'a Model,
    pub query: String,
    pub hits: Vec<Hit>,
    pub total: usize,
    pub results: ListState,
    pub focus: Focus,
    pub current: Option<Target>,
    pub detail: Option<Detail>,
    pub uses: Vec<UseRow>,
    pub use_state: ListState,
    pub list_mode: ListMode,
    /// Where Enter came from: (target, selected use row) to go back to.
    pub history: Vec<(Target, Option<usize>)>,
    pub quit: bool,
}

impl<'a> App<'a> {
    pub fn new(model: &'a Model, query: &str) -> App<'a> {
        let mut app = App {
            model,
            query: query.to_owned(),
            hits: Vec::new(),
            total: 0,
            results: ListState::default(),
            focus: Focus::Results,
            current: None,
            detail: None,
            uses: Vec::new(),
            use_state: ListState::default(),
            list_mode: ListMode::UsedBy,
            history: Vec::new(),
            quit: false,
        };
        app.refresh_search();
        app
    }

    fn refresh_search(&mut self) {
        self.hits = self.model.search(&self.query, RESULT_LIMIT);
        self.total = if self.hits.len() < RESULT_LIMIT { self.hits.len() } else { self.model.count(&self.query) };
        self.results.select(if self.hits.is_empty() { None } else { Some(0) });
        self.history.clear();
        self.follow_selection();
    }

    fn follow_selection(&mut self) {
        let target = self.results.selected().and_then(|i| self.hits.get(i)).map(|h| h.target.clone());
        self.show(target, None);
    }

    fn show(&mut self, target: Option<Target>, use_sel: Option<usize>) {
        self.detail = target.as_ref().map(|t| self.model.describe(t));
        self.current = target;
        let rows = match (&self.current, self.list_mode) {
            (Some(current), ListMode::LeadsTo) => self.leads_to_rows(current),
            _ => self.used_by_rows(),
        };
        self.uses = rows;
        let first_site = self.uses.iter().position(|r| matches!(r, UseRow::Site(_)));
        let picked = use_sel.filter(|&i| matches!(self.uses.get(i), Some(UseRow::Site(_))));
        self.use_state.select(picked.or(first_site));
    }

    /// Who mentions the current target, as `Detail` already grouped it.
    fn used_by_rows(&self) -> Vec<UseRow> {
        let mut rows = Vec::new();
        if let Some(detail) = &self.detail {
            for group in &detail.uses {
                rows.push(UseRow::Header(group.table.clone(), group.sites.len()));
                rows.extend(group.sites.iter().cloned().map(UseRow::Site));
            }
        }
        rows
    }

    /// What the current target mentions: one row per named entry, grouped by table. Globals are
    /// left out: they have no place in the kit that a list row could point at.
    fn leads_to_rows(&self, current: &Target) -> Vec<UseRow> {
        // table -> index -> op -> count, as `Model::group` does for mentions.
        let mut by_table: BTreeMap<&str, BTreeMap<usize, BTreeMap<&str, usize>>> = BTreeMap::new();
        for (to, op) in self.model.outgoing(current) {
            let Target::Entry { table, index } = to else { continue };
            *by_table
                .entry(table.as_str())
                .or_default()
                .entry(*index)
                .or_default()
                .entry(op.as_deref().unwrap_or("data"))
                .or_default() += 1;
        }
        let mut rows = Vec::new();
        for (table, sites) in by_table {
            let t = &self.model.atlas.tables[table];
            rows.push(UseRow::Header(table.to_owned(), sites.len()));
            for (index, ops) in sites {
                let entry = &t.entries[index];
                rows.push(UseRow::Site(UseSite {
                    target: Target::Entry { table: table.to_owned(), index },
                    label: entry_label(&self.model.atlas, table, entry),
                    hint: entry_hint(table, entry),
                    place: match entry.line {
                        Some(line) => format!("{}.py:{line}", t.module),
                        None => format!("{}.py", t.module),
                    },
                    ops: ops.into_iter().map(|(op, n)| (op.to_owned(), n)).collect(),
                }));
            }
        }
        rows
    }

    fn toggle_list_mode(&mut self) {
        self.list_mode = match self.list_mode {
            ListMode::UsedBy => ListMode::LeadsTo,
            ListMode::LeadsTo => ListMode::UsedBy,
        };
        let current = self.current.clone();
        self.show(current, None);
    }

    fn has_sites(&self) -> bool {
        self.uses.iter().any(|r| matches!(r, UseRow::Site(_)))
    }

    fn move_results(&mut self, delta: isize) {
        if self.hits.is_empty() {
            return;
        }
        let cur = self.results.selected().unwrap_or(0) as isize;
        let next = (cur + delta).clamp(0, self.hits.len() as isize - 1) as usize;
        if Some(next) != self.results.selected() {
            self.results.select(Some(next));
            self.history.clear();
            self.follow_selection();
        }
    }

    fn move_uses(&mut self, delta: isize) {
        let sites: Vec<usize> = self
            .uses
            .iter()
            .enumerate()
            .filter(|(_, r)| matches!(r, UseRow::Site(_)))
            .map(|(i, _)| i)
            .collect();
        if sites.is_empty() {
            return;
        }
        let cur = self.use_state.selected().and_then(|s| sites.iter().position(|&i| i == s)).unwrap_or(0) as isize;
        let next = (cur + delta).clamp(0, sites.len() as isize - 1) as usize;
        self.use_state.select(Some(sites[next]));
    }

    fn open_selected_use(&mut self) {
        let Some(UseRow::Site(site)) = self.use_state.selected().and_then(|i| self.uses.get(i)).cloned() else {
            return;
        };
        if let Some(current) = self.current.clone() {
            self.history.push((current, self.use_state.selected()));
        }
        self.show(Some(site.target), None);
        if !self.has_sites() {
            self.use_state.select(None);
        }
    }

    fn back(&mut self) -> bool {
        match self.history.pop() {
            Some((target, sel)) => {
                self.show(Some(target), sel);
                true
            }
            None => false,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }
        match self.focus {
            Focus::Results => match key.code {
                // Only a toggle while nothing is typed; otherwise `g` is part of the search.
                KeyCode::Char('g') if self.query.is_empty() => self.toggle_list_mode(),
                KeyCode::Char(c) => {
                    self.query.push(c);
                    self.refresh_search();
                }
                KeyCode::Backspace => {
                    if self.query.pop().is_some() {
                        self.refresh_search();
                    }
                }
                KeyCode::Up => self.move_results(-1),
                KeyCode::Down => self.move_results(1),
                KeyCode::PageUp => self.move_results(-10),
                KeyCode::PageDown => self.move_results(10),
                KeyCode::Enter | KeyCode::Right | KeyCode::Tab => {
                    if self.has_sites() {
                        self.focus = Focus::Uses;
                    }
                }
                KeyCode::Esc => {
                    if self.query.is_empty() {
                        self.quit = true;
                    } else {
                        self.query.clear();
                        self.refresh_search();
                    }
                }
                _ => {}
            },
            Focus::Uses => match key.code {
                KeyCode::Up => self.move_uses(-1),
                KeyCode::Down => self.move_uses(1),
                KeyCode::PageUp => self.move_uses(-10),
                KeyCode::PageDown => self.move_uses(10),
                KeyCode::Enter | KeyCode::Right => self.open_selected_use(),
                KeyCode::Esc | KeyCode::Left | KeyCode::Backspace => {
                    if !self.back() {
                        self.focus = Focus::Results;
                    } else if !self.has_sites() {
                        self.focus = Focus::Results;
                    }
                }
                KeyCode::Tab => self.focus = Focus::Results,
                KeyCode::Char('g') => self.toggle_list_mode(),
                KeyCode::Char(c) => {
                    self.focus = Focus::Results;
                    self.query.push(c);
                    self.refresh_search();
                }
                _ => {}
            },
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(1)])
            .split(frame.area());
        self.draw_search(frame, outer[0]);
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
            .split(outer[1]);
        self.draw_results(frame, body[0]);
        self.draw_detail(frame, body[1]);
        self.draw_help(frame, outer[2]);
    }

    fn draw_search(&self, frame: &mut Frame, area: Rect) {
        let count = if self.query.is_empty() {
            String::new()
        } else {
            format!("  {} found", self.total)
        };
        let line = Line::from(vec![
            Span::styled(" search ", Style::default().fg(Color::Black).bg(Color::Cyan)),
            Span::raw(" "),
            Span::styled(self.query.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::styled("▏", Style::default().fg(Color::Cyan)),
            Span::styled(count, Style::default().fg(Color::DarkGray)),
        ]);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" uw browse · Native ");
        frame.render_widget(Paragraph::new(line).block(block), area);
    }

    fn panel(title: String, focused: bool) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if focused { Style::default().fg(Color::Cyan) } else { Style::default().fg(Color::DarkGray) })
            .title(title)
    }

    fn draw_results(&mut self, frame: &mut Frame, area: Rect) {
        let block = Self::panel(" results ".into(), self.focus == Focus::Results);
        if self.hits.is_empty() {
            let text = if self.query.is_empty() {
                "Type to search.\n\nA name or part of it:\n  praven, town_6, merchant\nIn-game text:\n  Calradia\nA global, an operation:\n  $g_player_luck, call_script"
            } else {
                "Nothing matches.\n\nTry a shorter part of the name\nor a word from the in-game text."
            };
            frame.render_widget(
                Paragraph::new(text).style(Style::default().fg(Color::DarkGray)).block(block),
                area,
            );
            return;
        }
        let width = area.width.saturating_sub(4) as usize;
        let items: Vec<ListItem> = self
            .hits
            .iter()
            .map(|hit| {
                let mut spans = vec![Span::styled(hit.label.clone(), Style::default().add_modifier(Modifier::BOLD))];
                let used = hit.label.chars().count();
                if let Some(hint) = &hit.hint {
                    if used + 4 < width {
                        spans.push(Span::styled(
                            format!("  {}", short(hint, width - used - 2)),
                            Style::default().fg(Color::Yellow),
                        ));
                    }
                }
                let kind = Line::from(Span::styled(format!("  {}", hit.kind), Style::default().fg(Color::DarkGray)));
                ListItem::new(vec![Line::from(spans), kind])
            })
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Rgb(40, 60, 90)))
            .highlight_symbol("› ");
        frame.render_stateful_widget(list, area, &mut self.results);
    }

    fn draw_detail(&mut self, frame: &mut Frame, area: Rect) {
        let Some(detail) = self.detail.clone() else {
            let text = "Pick something on the left to see what it is,\nwhere Native defines it and who uses it.";
            frame.render_widget(
                Paragraph::new(text)
                    .style(Style::default().fg(Color::DarkGray))
                    .block(Self::panel(" details ".into(), false)),
                area,
            );
            return;
        };
        let mut lines = Vec::new();
        if !self.history.is_empty() {
            let mut trail: Vec<String> = self.history.iter().map(|(t, _)| self.model.label(t)).collect();
            trail.push(detail.label.clone());
            lines.push(Line::from(Span::styled(
                short(&trail.join(" › "), area.width.saturating_sub(4) as usize),
                Style::default().fg(Color::DarkGray),
            )));
        }
        lines.push(Line::from(Span::styled(
            detail.label.clone(),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(vec![Span::styled("what     ", Style::default().fg(Color::DarkGray)), Span::raw(detail.kind.clone())]));
        if let Some(defined) = &detail.defined {
            lines.push(Line::from(vec![
                Span::styled("defined  ", Style::default().fg(Color::DarkGray)),
                Span::styled(defined.clone(), Style::default().fg(Color::Green)),
            ]));
        }
        for note in &detail.notes {
            lines.push(Line::from(vec![Span::styled("note     ", Style::default().fg(Color::DarkGray)), Span::raw(note.clone())]));
        }
        if let Some(hint) = &detail.hint {
            // Last, so a long in-game text never pushes the facts above out of view.
            lines.push(Line::from(Span::styled(format!("“{}”", short(hint, 240)), Style::default().fg(Color::Yellow))));
        }
        if let Some(tr) = &detail.tr {
            // The official Turkish text, right under the English one.
            lines.push(Line::from(vec![
                Span::styled("tr       ", Style::default().fg(Color::DarkGray)),
                Span::styled(short(tr, 240), Style::default().fg(Color::Yellow)),
            ]));
        }
        let header_height = (lines.len() as u16 + 4).min(area.height / 2).max(4);
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(header_height), Constraint::Min(3)])
            .split(area);
        frame.render_widget(
            Paragraph::new(lines).wrap(Wrap { trim: false }).block(Self::panel(" details ".into(), false)),
            parts[0],
        );

        let sites = self.uses.iter().filter(|r| matches!(r, UseRow::Site(_))).count();
        let title = match (self.list_mode, sites) {
            (ListMode::UsedBy, 0) => " used by: nothing in Native ".to_owned(),
            (ListMode::UsedBy, sites) => format!(
                " used by {sites} {} ({} mentions) ",
                if sites == 1 { "place" } else { "places" },
                detail.total_mentions
            ),
            (ListMode::LeadsTo, 0) => " leads to: nothing in Native ".to_owned(),
            (ListMode::LeadsTo, sites) => format!(" leads to {sites} {} ", if sites == 1 { "place" } else { "places" }),
        };
        let block = Self::panel(title, self.focus == Focus::Uses);
        if sites == 0 {
            let text = match self.list_mode {
                ListMode::LeadsTo => "Nothing in Native is written here by name.\n(Data written as Python numbers can't be traced.)",
                ListMode::UsedBy => match self.current {
                    Some(Target::Op(_)) => "Operations are listed with their usage above.",
                    _ => "No operation in Native mentions this by name.\n(Data written as Python numbers can't be traced.)",
                },
            };
            frame.render_widget(Paragraph::new(text).style(Style::default().fg(Color::DarkGray)).block(block), parts[1]);
            return;
        }
        let width = parts[1].width.saturating_sub(4) as usize;
        let items: Vec<ListItem> = self
            .uses
            .iter()
            .map(|row| match row {
                UseRow::Header(table, n) => ListItem::new(Line::from(Span::styled(
                    format!("{table} ({n})"),
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                ))),
                UseRow::Site(site) => {
                    let first = Line::from(vec![
                        Span::styled(format!("  {}", site.label), Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(format!("  {}", site.place), Style::default().fg(Color::Green)),
                    ]);
                    let mut second = vec![Span::styled(format!("    {}", site.ops_text()), Style::default().fg(Color::Gray))];
                    if let Some(hint) = &site.hint {
                        let used = site.ops_text().chars().count() + 4;
                        if used + 6 < width {
                            second.push(Span::styled(
                                format!("  “{}”", short(hint, width - used - 4)),
                                Style::default().fg(Color::Yellow),
                            ));
                        }
                    }
                    ListItem::new(vec![first, Line::from(second)])
                }
            })
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Rgb(40, 60, 90)))
            .highlight_symbol("› ");
        let mut state = self.use_state.clone();
        if self.focus != Focus::Uses {
            // Keep the scroll position but do not show a cursor when the list is not active.
            *state.offset_mut() = self.use_state.offset();
            state.select(None);
        }
        frame.render_stateful_widget(list, parts[1], &mut state);
        if self.focus == Focus::Uses {
            self.use_state = state;
        }
    }

    fn draw_help(&self, frame: &mut Frame, area: Rect) {
        let toggle = match self.list_mode {
            ListMode::UsedBy => ("g", "leads to"),
            ListMode::LeadsTo => ("g", "used by"),
        };
        let keys: &[(&str, &str)] = match self.focus {
            Focus::Results if self.query.is_empty() => {
                &[("type", "search"), ("↑↓", "pick"), ("Enter", "go to its users"), toggle, ("Esc", "clear / quit"), ("Ctrl+C", "quit")]
            }
            Focus::Results => &[("type", "search"), ("↑↓", "pick"), ("Enter", "go to its users"), ("Esc", "clear / quit"), ("Ctrl+C", "quit")],
            Focus::Uses => &[("↑↓", "pick a user"), ("Enter", "open it"), ("Esc", "back"), ("Tab", "results"), toggle, ("type", "new search")],
        };
        let mut spans = Vec::new();
        for (key, what) in keys {
            spans.push(Span::styled(format!(" {key} "), Style::default().fg(Color::Black).bg(Color::DarkGray)));
            spans.push(Span::styled(format!(" {what}  "), Style::default().fg(Color::Gray)));
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }
}

/// Runs the UI until the user quits.
pub fn run(model: &Model, query: &str) -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(model, query);
    let result = (|| -> std::io::Result<()> {
        while !app.quit {
            terminal.draw(|frame| app.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                app.on_key(key);
            }
        }
        Ok(())
    })();
    ratatui::restore();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn model() -> Model {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_refs.json");
        Model::new(uw_atlas::Atlas::from_path(path).expect("fixture"))
    }

    fn press(app: &mut App, code: KeyCode) {
        app.on_key(KeyEvent::new(code, KeyModifiers::NONE));
    }

    fn typed(app: &mut App, text: &str) {
        for c in text.chars() {
            press(app, KeyCode::Char(c));
        }
    }

    fn screen(app: &mut App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(120, 30)).unwrap();
        terminal.draw(|frame| app.draw(frame)).unwrap();
        let buffer = terminal.backend().buffer().clone();
        (0..buffer.area.height)
            .map(|y| (0..buffer.area.width).map(|x| buffer[(x, y)].symbol()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn typing_searches_and_selection_shows_details() {
        let model = model();
        let mut app = App::new(&model, "");
        assert!(app.hits.is_empty());
        assert!(screen(&mut app).contains("Type to search."));
        typed(&mut app, "start");
        assert_eq!(app.hits[0].label, "menu_start");
        assert_eq!(app.current, Some(Target::Entry { table: "menus".into(), index: 0 }));
        let s = screen(&mut app);
        assert!(s.contains("used by 3 places (3 mentions)"), "{s}");
        assert!(s.contains("module_game_menus.py:10"), "{s}");
        press(&mut app, KeyCode::Backspace);
        assert_eq!(app.query, "star");
    }

    #[test]
    fn enter_follows_a_user_and_esc_comes_back() {
        let model = model();
        let mut app = App::new(&model, "mnu_start");
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.focus, Focus::Uses);
        // The first site is selected, not the group header.
        let first = app.use_state.selected().unwrap();
        assert!(matches!(app.uses[first], UseRow::Site(_)));
        press(&mut app, KeyCode::Down);
        let UseRow::Site(site) = app.uses[app.use_state.selected().unwrap()].clone() else { panic!() };
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current.as_ref(), Some(&site.target));
        assert_eq!(app.history.len(), 1);
        assert!(screen(&mut app).contains("menu_start ›"), "breadcrumb shows where we came from");
        press(&mut app, KeyCode::Esc);
        assert_eq!(app.current, Some(Target::Entry { table: "menus".into(), index: 0 }));
        assert_eq!(app.use_state.selected(), Some(first + 1), "back restores the picked row");
        press(&mut app, KeyCode::Esc);
        assert_eq!(app.focus, Focus::Results);
        press(&mut app, KeyCode::Esc);
        assert!(app.query.is_empty() && !app.quit);
        press(&mut app, KeyCode::Esc);
        assert!(app.quit);
    }

    #[test]
    fn nothing_found_and_no_users_are_said_plainly() {
        let model = model();
        let mut app = App::new(&model, "zzzz");
        assert!(screen(&mut app).contains("Nothing matches."));
        let mut app = App::new(&model, "game_start");
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.focus, Focus::Results, "nothing to walk into");
        assert!(screen(&mut app).contains("used by: nothing in Native"));
    }

    #[test]
    fn key_release_events_are_ignored() {
        let model = model();
        let mut app = App::new(&model, "");
        let mut release = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        release.kind = KeyEventKind::Release;
        app.on_key(release);
        assert!(app.query.is_empty());
    }

    fn labels(app: &App) -> Vec<String> {
        app.uses
            .iter()
            .filter_map(|r| match r {
                UseRow::Site(site) => Some(site.label.clone()),
                UseRow::Header(..) => None,
            })
            .collect()
    }

    fn headers(app: &App) -> Vec<(String, usize)> {
        app.uses
            .iter()
            .filter_map(|r| match r {
                UseRow::Header(table, n) => Some((table.clone(), *n)),
                UseRow::Site(_) => None,
            })
            .collect()
    }

    #[test]
    fn g_switches_to_leads_to_and_enter_follows_a_target() {
        let model = model();
        let mut app = App::new(&model, "mnu_start");
        assert_eq!(app.current, Some(Target::Entry { table: "menus".into(), index: 0 }));
        // Enter opens the first place that mentions menu_start: the game_start script.
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.focus, Focus::Uses);
        let UseRow::Site(site) = app.uses[app.use_state.selected().unwrap()].clone() else { panic!() };
        assert_eq!(site.target, Target::Entry { table: "scripts".into(), index: 0 });
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current, Some(Target::Entry { table: "scripts".into(), index: 0 }));
        assert!(screen(&mut app).contains("leads to"), "the help bar names the other mode");

        press(&mut app, KeyCode::Char('g'));
        assert_eq!(app.list_mode, ListMode::LeadsTo);
        assert_eq!(
            app.current,
            Some(Target::Entry { table: "scripts".into(), index: 0 }),
            "the toggle keeps the target"
        );
        assert_eq!(headers(&app), vec![("menus".to_owned(), 2), ("scripts".to_owned(), 1)]);
        assert_eq!(labels(&app), vec!["menu_start", "menu_camp#1", "script_helper"], "$g_x is no place");
        let UseRow::Site(camp) = app.uses[2].clone() else { panic!() };
        assert_eq!(camp.place, "module_game_menus.py:20", "the place is where the target is defined");
        assert_eq!(camp.ops_text(), "jump_to_menu");
        assert_eq!(camp.hint.as_deref(), Some("A"));
        let s = screen(&mut app);
        assert!(s.contains("leads to 3 places"), "{s}");
        assert!(s.contains("used by"), "the help bar offers the other mode: {s}");

        // Enter follows one of the targets; the mode stays.
        let first = app.use_state.selected().unwrap();
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current, Some(Target::Entry { table: "menus".into(), index: 0 }));
        assert_eq!(app.history.len(), 2);
        assert_eq!(app.list_mode, ListMode::LeadsTo, "the mode stays when jumping");
        assert!(screen(&mut app).contains("leads to: nothing in Native"));
        press(&mut app, KeyCode::Esc);
        assert_eq!(app.current, Some(Target::Entry { table: "scripts".into(), index: 0 }));
        assert_eq!(app.list_mode, ListMode::LeadsTo, "the mode stays on the way back");
        assert_eq!(app.use_state.selected(), Some(first), "back restores the picked row");

        press(&mut app, KeyCode::Char('g'));
        assert_eq!(app.list_mode, ListMode::UsedBy);
        assert!(labels(&app).is_empty(), "nothing mentions a hook: the used by list is empty again");
        assert!(screen(&mut app).contains("used by: nothing in Native"));
    }

    #[test]
    fn g_types_into_a_non_empty_search_and_toggles_on_an_empty_one() {
        let model = model();
        let mut app = App::new(&model, "game");
        press(&mut app, KeyCode::Char('g'));
        assert_eq!(app.query, "gameg");
        assert_eq!(app.list_mode, ListMode::UsedBy);
        let mut app = App::new(&model, "");
        press(&mut app, KeyCode::Char('g'));
        assert_eq!(app.list_mode, ListMode::LeadsTo);
        assert!(app.query.is_empty());
    }

    #[test]
    fn detail_shows_the_turkish_text_when_it_is_loaded() {
        let tr = crate::tr::load_dir(std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/tr"
        )))
        .expect("fixture folder");
        let loaded = model().with_tr(tr);
        let mut app = App::new(&loaded, "menu_start");
        assert_eq!(app.current.as_ref().map(|t| loaded.label(t)).as_deref(), Some("menu_start"));
        assert!(screen(&mut app).contains("Üveykent ülkesi"));
        // Without the translation the pane is English only, as before.
        let plain = model();
        let mut app = App::new(&plain, "menu_start");
        assert!(!screen(&mut app).contains("Üveykent"));
    }
}

#[cfg(test)]
mod real {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    /// Renders the real atlas (when present) so a person can look at it:
    /// `cargo test -p uw_cli real_screens -- --nocapture`.
    #[test]
    fn real_screens() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let model = Model::new(uw_atlas::Atlas::from_path(path).unwrap());
        let mut app = App::new(&model, "praven");
        let mut terminal = Terminal::new(TestBackend::new(130, 34)).unwrap();
        for step in 0..2 {
            if step == 1 {
                app.on_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
                app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
            }
            terminal.draw(|frame| app.draw(frame)).unwrap();
            let buffer = terminal.backend().buffer().clone();
            let text: Vec<String> = (0..buffer.area.height)
                .map(|y| (0..buffer.area.width).map(|x| buffer[(x, y)].symbol()).collect())
                .collect();
            println!("{}\n", text.join("\n"));
        }
        // Down picked the second result; Enter moved into its "used by" list.
        assert_eq!(app.focus, Focus::Uses);
        assert_eq!(app.current.as_ref().map(|t| model.label(t)).as_deref(), Some("str_journey_to_praven"));
    }
}
