use core::fmt;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::{Backend, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, StatefulWidget, Widget},
};
use std::io;

pub struct App<'a> {
    pub stateful_staging: StatefulStagingList<'a>,
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        App {
            stateful_staging: StatefulStagingList::new(),
        }
    }
    pub fn add_list(&mut self, status: StageStatus) {
        self.stateful_staging
            .staging
            .push(StagingFile::new("abc.rs", status))
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('p') => self.add_list(StageStatus::Staging),
                        KeyCode::Char('o') => self.add_list(StageStatus::Staged),
                        KeyCode::Char('k') => self
                            .stateful_staging
                            .change_staging_area(StageStatus::Staging),
                        KeyCode::Char('l') => self
                            .stateful_staging
                            .change_staging_area(StageStatus::Staged),
                        KeyCode::Down => self.stateful_staging.next(),
                        KeyCode::Up => self.stateful_staging.previous(),
                        KeyCode::Enter => self.stateful_staging.stage_file(),
                        //Future keys
                        _ => match key {
                            KeyEvent{modifiers: KeyModifiers::ALT, code: KeyCode::Char('h'), kind, state} => println!("fetcH"),
                            _ => {}

                        }
                    }
                }
            }
        }
    }
    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn render_staging_area(&mut self, staging_layout: Rect, buf: &mut Buffer) {
        let staging_area =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(staging_layout);

        let staging_block = Block::default().borders(Borders::ALL).title(" Staging ").title_alignment(ratatui::layout::Alignment::Center);
        let staging_list = List::new(
            self.stateful_staging
                .staging
                .iter()
                .filter(|x| x.status == StageStatus::Staging)
                .map(|x| x.file),
        )
        .block(staging_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Blue),
        );
        let staged_block = Block::default().borders(Borders::ALL).title(" Staged ").title_alignment(ratatui::layout::Alignment::Center);;
        let staged_list = List::new(
            self.stateful_staging
                .staging
                .iter()
                .filter(|x| x.status == StageStatus::Staged)
                .map(|x| x.file),
        )
        .block(staged_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Blue),
        );
        StatefulWidget::render(
            staging_list,
            staging_area[0],
            buf,
            &mut self.stateful_staging.state,
        );
        StatefulWidget::render(
            staged_list,
            staging_area[1],
            buf,
            &mut self.stateful_staging.staged_state,
        );
    }

    fn render_changes_area(&self, changes_layout: Rect, buf: &mut Buffer) {
        let changes_area = Layout::vertical([Constraint::Percentage(100)]).split(changes_layout);

        let changes_block = Block::default().borders(Borders::ALL);
        Widget::render(changes_block, changes_area[0], buf)
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);
        self.render_staging_area(main_layout[0], buf);
        self.render_changes_area(main_layout[1], buf);
    }
}

pub struct StatefulStagingList<'a> {
    pub state: ListState,
    pub staged_state: ListState,
    pub staging: Vec<StagingFile<'a>>,
    last_selected: Option<usize>,
    using_block: StageStatus,
}

impl StatefulStagingList<'_> {
    fn new<'a>() -> StatefulStagingList<'a> {
        StatefulStagingList {
            state: ListState::default(),
            staged_state: ListState::default(),
            last_selected: None,
            staging: vec![
                StagingFile::new("file_path", StageStatus::Staging),
                StagingFile::new("file_path_two", StageStatus::Staging),
                StagingFile::new("file_path_teste", StageStatus::Staged),
                StagingFile::new("file_path_staged", StageStatus::Staged),
            ],
            using_block: StageStatus::Staging,
        }
    }
    fn previous(&mut self) {
        let i: usize = match if self.using_block == StageStatus::Staging {
            self.state.selected()
        } else {
            self.staged_state.selected()
        } {
            Some(i) => {
                let len_block_list = self
                    .staging
                    .iter()
                    .filter(|x| x.status == self.using_block)
                    .count();
                if i == 0 {
                    len_block_list - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        if self.using_block == StageStatus::Staging {
            self.state.select(Some(i))
        } else {
            self.staged_state.select(Some(i))
        }
    }
    fn next(&mut self) {
        let i: usize = match if self.using_block == StageStatus::Staging {
            self.state.selected()
        } else {
            self.staged_state.selected()
        } {
            Some(i) => {
                let len_block_list = self
                    .staging
                    .iter()
                    .filter(|x| x.status == self.using_block)
                    .count();
                let mut index = i;
                if index >= len_block_list - 1 {
                    index = 0;
                } else {
                    index += 1;
                }
                index
            }
            None => self.last_selected.unwrap_or(0),
        };
        if self.using_block == StageStatus::Staging {
            self.state.select(Some(i))
        } else {
            self.staged_state.select(Some(i))
        }
    }
    fn stage_file(&mut self) {
        let i: usize = if self.using_block == StageStatus::Staging {
            self.state.selected().unwrap()
        } else {
            self.staged_state.selected().unwrap()
        };
        if self.staging[i].status == StageStatus::Staged {
            self.staging[i].status = StageStatus::Staging
        } else {
            self.staging[i].status = StageStatus::Staged
        }
    }
    fn change_staging_area(&mut self, stage: StageStatus) {
        if stage == StageStatus::Staging {
            self.staged_state.select(None)
        } else {
            self.state.select(None)
        }
        self.using_block = stage
    }
}

pub struct StagingFile<'a> {
    pub file: &'a str,
    pub status: StageStatus,
}

impl StagingFile<'_> {
    pub fn new<'a>(file_path: &'a str, stage_status: StageStatus) -> StagingFile<'a> {
        StagingFile {
            file: file_path,
            status: stage_status,
        }
    }
    pub fn get_file(&self) -> &str {
        self.file
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StageStatus {
    Staging,
    Staged,
}

impl fmt::Display for StageStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StageStatus::Staged => write!(f, "staged"),
            StageStatus::Staging => write!(f, "staging"),
        }
    }
}
