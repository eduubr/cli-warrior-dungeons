// use std::fs::File;
// use std::io::{BufReader, BufRead, Error};


// use std::path::Path;
// use std::io::Read;
use crossterm::{
	cursor,
	event::{poll, read, Event, KeyCode, KeyEvent},
	execute, queue,
	style::{Color, Print, ResetColor, SetForegroundColor, SetAttribute},
	terminal::{
		disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
		EnterAlternateScreen, LeaveAlternateScreen,
	},
	Result,
};
use std::io::{self, Write};
use std::time::Duration;

// use std::fs::File;
// use std::io::{self, BufRead};
use std::path::Path;

mod game;


fn main() -> Result<()> {
    // let path = "logo.txt";
    let mut selector = 0;
    check_save();
    let mut stdout = io::stdout();
    enable_raw_mode()?;
	execute!(
		stdout,
		EnterAlternateScreen,
		SetForegroundColor(Color::Magenta),
		DisableLineWrap,
		cursor::Hide
	)?;
    
	loop {
		let mut n = 0;
        queue!(stdout, Clear(ClearType::All),cursor::MoveTo(0, (1) as u16),SetForegroundColor(Color::Magenta))?;
        for line in game::LOGO.split('\n') {
			queue!(stdout, Print(line), cursor::MoveToNextLine(1))?;
		}
		
		// queue!(stdout, Print(game::LOGO), cursor::MoveToNextLine(1))?;
        while n < 10 {
			queue!(stdout, Print("\n"), cursor::MoveToNextLine(1))?;
			n += 1;
		}


		// for line in game::INITIAL_MENU.split('\n') {
		// 	queue!(stdout, Print(line), cursor::MoveToNextLine(1))?;
		// }
		// let i = 0;
		// while let Some(line) = game.row_as_string(i) {
		// 	queue!(stdout, MoveTo(0, i as u16), Print(line))?;
		// 	i += 1;
		// }
		
		queue!(stdout,ResetColor)?;
		
		for options in game::SELECT_OPTION.iter().enumerate() {
			let (index,option) = options;
			if index == selector {
				queue!(
					stdout,
					SetAttribute(game::OPTION_SELECTED),
					Print(format!("{} ", option)),
					ResetColor,
					cursor::MoveToNextLine(2)
				)?;
				continue;
			}
			queue!(stdout,Print(option),cursor::MoveToNextLine(2))?;
		}

		// queue!(
		// 	stdout,
		// 	cursor::MoveTo(0, (1) as u16)
		// 	// Print("Press Esc to exit...")
		// )?;

		stdout.flush()?;
		if poll(Duration::from_millis(500))? {
			if let Event::Key(KeyEvent { code, .. }) = read()? {
				match code {
					KeyCode::Esc => {
						break;
					},
					KeyCode::Up => {
						if selector == 0 {
							selector = 4;
						} else {
							selector -= 1;
						}
					},
					KeyCode::Down => {
						if selector == 4 {
							selector = 0;
						} else {
							selector += 1;
						}
					},
					_ => {}
				}
			}
		}
	}
	execute!(
		stdout,
		ResetColor,
		EnableLineWrap,
		cursor::Show,
		LeaveAlternateScreen
	)?;
	disable_raw_mode()?;
	Ok(())
}

fn check_save() {
    let g_save: bool = path_exist("./save.txt");
    let logo: bool = path_exist("./logo.txt");
    if !(g_save && logo) {
        println!("algum arquivo esta faltando!");
    }
}

fn path_exist(path: &str) -> bool {
    Path::new(path).exists()

    //     println!("o arquivo/pasta existe!");
    // }else {
    //     println!("o arquivo/pasta nao existe!");
    // }
}

// fn run_logo(path: &str) {
//     let mut input = std::fs::File::open(path).unwrap();
//     let mut contents = String::new();
//     input.read_to_string(&mut contents).unwrap();
//     print!("{}", contents);
// }
