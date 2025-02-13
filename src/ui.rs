use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::Padding,
        canvas::{Canvas, Painter},
        Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap,
    },
    Frame,
};

use crate::app::{App, CurrentScreen, IntroScreenOptions};
use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use std::rc::Rc;

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    // setting up base layout and boxes
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(5), Constraint::Min(10)].as_ref())
        .split(frame.area());

    let title_text = "As you might know, I have a question I've been meaning to ask you.";
    let title = Paragraph::new(Text::from(vec![
        match app.current_screen {
            CurrentScreen::IntroScreen => Line::from(title_text).white(),
            _ => Line::from(""),
        },
        match app.current_screen {
            CurrentScreen::WillYouBeMyValentine => {
                Line::from("I LOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOVE YOUUUU")
            }
            CurrentScreen::IntroScreen => Line::from("Press any key to continue".magenta()),
            CurrentScreen::YesValentine => Line::from("Please say yes"),
            CurrentScreen::NotValentine => Line::from("I'm gonna kill you"),
            CurrentScreen::DateInfo => Line::from("Bag secured!!! B)"),
        },
    ]))
    .style(Style::default().fg(Color::White))
    .on_black()
    .centered()
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("To My Valentine...")
            .title_style(Style::new().magenta())
            .borders(Borders::ALL)
            .border_style(Style::new().magenta()),
    );
    frame.render_widget(title, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(50), Constraint::Max(50)].as_ref())
        .split(chunks[1]);

    match app.current_screen {
        CurrentScreen::WillYouBeMyValentine => render_valentine_screen(frame, app, main_chunks),
        CurrentScreen::IntroScreen => render_intro_screens(frame, app, main_chunks),
        CurrentScreen::YesValentine => render_yes_screen(frame, app, main_chunks),
        CurrentScreen::NotValentine => render_no_screen(frame, app, main_chunks),
        CurrentScreen::DateInfo => render_dateinfo_screen(frame, app, main_chunks),

        _ => {}
    }
}

fn render_valentine_screen(frame: &mut Frame, app: &mut App, main_chunks: Rc<[Rect]>) {
    let my_string = match app.offset {
        0 => "My Valentine My Lovely Lady My Love <3 ".repeat(500),
        1 => "My Lovely Lady My Love <3 ".repeat(500),
        2 => "My Love <3 ".repeat(500),
        3 => " <3 ".repeat(900),
        _ => "My Valentine My Lovely Lady My Love <3 ".repeat(500),
    };
    let valentines_background = Paragraph::new(Text::from(my_string))
        .wrap(Wrap { trim: false })
        .style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .centered()
        .block(Block::default().borders(Borders::ALL).title(""))
        .alignment(Alignment::Center);
    frame.render_widget(valentines_background, main_chunks[0]);

    let area = frame.area();
    let popup_area = Rect {
        x: area.width / 3,
        y: area.height / 5,
        width: area.width / 4,
        height: area.height / 3,
    };

    frame.render_widget(Clear, popup_area);
    let wybmv = Paragraph::new(Text::from(vec![
        Line::from("Will ".magenta().bold()),
        Line::from("You ".magenta()).bold(),
        Line::from("Be ".magenta()).bold(),
        Line::from("My ".magenta()).bold(),
        Line::from(
            "Valentine?"
                .magenta()
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ),
        Line::from(""),
        Line::from("please type y/n"),
    ]))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center)
    .style(Style::new().magenta().on_black())
    .centered()
    .block(
        Block::new()
            .title("To my pretty lady:")
            .title_style(Style::new().white().bold())
            .borders(Borders::ALL)
            .padding(Padding::new(0, 0, popup_area.height / 4, 0))
            .border_style(Style::new().white()),
    );
    frame.render_widget(wybmv, popup_area);

    app.playground = main_chunks[1];
    frame.render_widget(app.pong_canvas(app.playground), app.playground);
}
fn render_intro_screens(frame: &mut Frame, app: &App, main_chunks: Rc<[Rect]>) {
    match app.intro_screen {
        IntroScreenOptions::Screen1 => {}
        IntroScreenOptions::Screen2 => {
            let next_text = Paragraph::new(Text::from(vec![
                Line::from("I'm sorry that this took longer than expected...".white()),
                Line::from("But this took a little while to make...".white()),
            ]))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .centered()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .padding(Padding::new(0, 0, frame.area().height / 4, 0))
                    .title_style(Style::new().magenta())
                    .borders(Borders::ALL)
                    .border_style(Style::new().magenta()),
            );
            frame.render_widget(next_text, main_chunks[0]);
        }
        IntroScreenOptions::Screen3 => {
            let next_text2 = Paragraph::new(Text::from(vec![
                Line::from("I'm sorry that this took longer than expected...".white()),
                Line::from("But this took a little while to make...".white()),
            ]))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .centered()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .padding(Padding::new(0, 0, frame.area().height / 4, 0))
                    .title_style(Style::new().magenta())
                    .borders(Borders::ALL)
                    .border_style(Style::new().magenta()),
            );
            frame.render_widget(next_text2, main_chunks[0]);

            let next_text = Paragraph::new(Text::from(vec![
                Line::from("You've waited long enough my pretty lady".white()),
                Line::from("So I have a question for you..."),
            ]))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .centered()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .padding(Padding::new(0, 0, frame.area().height / 4, 0))
                    .title_style(Style::new().magenta())
                    .borders(Borders::ALL)
                    .border_style(Style::new().magenta()),
            );

            frame.render_widget(next_text, main_chunks[1])
        }
    }
}
fn render_yes_screen(frame: &mut Frame, app: &App, main_chunks: Rc<[Rect]>) {
    let my_string = match app.offset {
        0 => "My Valentine My Lovely Lady My Love <3 ".repeat(300),
        1 => "My Lovely Lady My Love <3 ".repeat(300),
        2 => "My Love <3 ".repeat(300),
        3 => " <3 ".repeat(500),
        _ => "My Valentine My Lovely Lady My Love <3 ".repeat(300),
    };
    let valentines_background = Paragraph::new(Text::from(my_string))
        .wrap(Wrap { trim: false })
        .style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .centered()
        .block(Block::default().borders(Borders::ALL).title(""))
        .alignment(Alignment::Center);
    frame.render_widget(valentines_background, main_chunks[0]);

    frame.render_widget(app.pong_canvas(app.playground), app.playground);

    let area = frame.area();
    let popup_area = Rect {
        x: area.width / 3,
        y: area.height / 5,
        width: area.width / 4,
        height: area.height / 3,
    };
    frame.render_widget(Clear, popup_area);
    let wybmv = Paragraph::new(Text::from(vec![
        Line::from("How amazing! ".green()),
        Line::from("I must be the luckiest guy ever! ".green()),
        Line::from(""),
        Line::from("Would you like to know our Valentines plans? ".green()),
        Line::from(""),
        Line::from("please enter y/n ".green()),
    ]))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center)
    .centered()
    .on_black()
    .block(
        Block::new()
            .title("Lucky meeeee")
            .title_style(Style::new().white().bold())
            .borders(Borders::ALL)
            .padding(Padding::new(0, 0, popup_area.height / 4, 0))
            .border_style({
                match app.offset % 2 {
                    0 => Style::new().gray(),
                    1 => Style::new().green(),
                    _ => Style::new().gray(),
                }
            }),
    );
    frame.render_widget(wybmv, popup_area);
}
fn render_no_screen(frame: &mut Frame, app: &App, main_chunks: Rc<[Rect]>) {
    let my_string = match app.offset {
        0 => "My Valentine My Lovely Lady My Love <3 ".repeat(300),
        1 => "My Lovely Lady My Love <3 ".repeat(300),
        2 => "My Love <3 ".repeat(300),
        3 => " <3 ".repeat(500),
        _ => "My Valentine My Lovely Lady My Love <3 ".repeat(300),
    };
    let valentines_background = Paragraph::new(Text::from(my_string))
        .wrap(Wrap { trim: false })
        .style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .centered()
        .block(Block::default().borders(Borders::ALL).title(""))
        .alignment(Alignment::Center);
    frame.render_widget(valentines_background, main_chunks[0]);

    frame.render_widget(app.pong_canvas(app.playground), app.playground);
    let area = frame.area();

    let popup_area = Rect {
        x: area.width / 3,
        y: area.height / 5,
        width: area.width / 4,
        height: area.height / 3,
    };
    frame.render_widget(Clear, popup_area);
    let wybmv = Paragraph::new(Text::from(vec![
        Line::from("THAT ".red()),
        Line::from("WAS ".red()),
        Line::from("THE ".red()),
        Line::from("WRONG ".red()),
        Line::from("ANSWER!!!!".red().add_modifier(Modifier::BOLD)),
        Line::from(""),
        Line::from("press any key to continue").red(),
    ]))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center)
    .centered()
    .on_black()
    .block(
        Block::new()
            .title("FUCK YOUUUUU")
            .title_style(Style::new().red().bold())
            .borders(Borders::ALL)
            .padding(Padding::new(0, 0, popup_area.height / 4, 0))
            .border_style({
                match app.offset % 2 {
                    0 => Style::new().gray(),
                    1 => Style::new().red(),
                    _ => Style::new().gray(),
                }
            }),
    );
    frame.render_widget(wybmv, popup_area);
}
fn render_dateinfo_screen(frame: &mut Frame, app: &App, main_chunks: Rc<[Rect]>) {
    let my_string = match app.offset {
        0 => "My Valentine My Lovely Lady My Love <3 ".repeat(300),
        1 => "My Lovely Lady My Love <3 ".repeat(300),
        2 => "My Love <3 ".repeat(300),
        3 => " <3 ".repeat(500),
        _ => "My Valentine My Lovely Lady My Love <3 ".repeat(300),
    };
    let valentines_background = Paragraph::new(Text::from(my_string))
        .wrap(Wrap { trim: false })
        .style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .centered()
        .block(Block::default().borders(Borders::ALL).title(""))
        .alignment(Alignment::Center);
    frame.render_widget(valentines_background, main_chunks[0]);

    frame.render_widget(app.pong_canvas(app.playground), app.playground);
    let area = frame.area();
    let target_time = Local.with_ymd_and_hms(2025, 2, 15, 16, 0, 0).unwrap();
    let now = Local::now();
    let remaining_time = if now < target_time {
        target_time - now
    } else {
        Duration::zero()
    };

    let hours = remaining_time.num_hours();
    let minutes = remaining_time.num_minutes() % 60;
    let seconds = remaining_time.num_seconds() % 60;

    let mut middle_string = "";
    if hours < 1 {
        middle_string = "Hurry up cracker!!!";
    } else if hours < 40 && hours > 16 {
        middle_string = "Happy Valentines Day!";
    } else if hours <= 16 && hours > 1 {
        middle_string = "Date today!";
    }

    let countdown_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Countdown To Our Date...")
        .style(Style::default().fg(Color::Red));

    let time_popup_area = Rect {
        x: area.width / 4,
        y: area.height / 4,
        width: area.width / 6,
        height: area.height / 6,
    };
    let time = Local::now().format("%H:%M:%S").to_string();
    let time_popup = Paragraph::new(Text::from(vec![
        Line::from(format!("We have {} until our date!", countdown_str)),
        Line::from(middle_string),
        Line::from(format!("Current Time: {}", time)),
    ]))
    .wrap(Wrap { trim: true })
    .on_black()
    .alignment(Alignment::Center)
    .centered()
    .block(
        Block::new()
            .title("Countdown")
            .title_style(Style::new().white().bold())
            .borders(Borders::ALL)
            .border_style(Style::new().white()),
    );

    let popup_area = center(
        frame.area(),
        Constraint::Percentage(50),
        Constraint::Length(25),
    );
    frame.render_widget(Clear, popup_area);
    frame.render_widget(time_popup, popup_area);
    let wybmv = Paragraph::new(Text::from(vec![
        Line::from("Our Date: ".red().bold()),
        Line::from("Location: J Wilson's ".red()),
        Line::from("Time: 4:00 PM ".red()),
        Line::from("Attire: Something sexy ".red()),
        Line::from(" ".red()),
        Line::from("Excited to see you my pretty lady! ".white()),
        Line::from(middle_string.white()),
        Line::from("Press \'q\' to quit!".white()),
    ]))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center)
    .centered()
    .block(
        Block::new()
            .title("Our Date:")
            .title_style(Style::new().white().bold())
            .borders(Borders::ALL)
            .padding(Padding::new(0, 0, popup_area.height / 4, 0))
            .border_style(Style::new().white()),
    );
    frame.render_widget(wybmv, popup_area);
}
