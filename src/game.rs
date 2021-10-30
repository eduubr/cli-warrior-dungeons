// use std::{fmt,path::Path,fs::metadata};

// pub mod teste{

//     fn path_exist(path: &str) {
//         if Path::new(path).exists() {
//             println!("o arquivo/pasta existe!");
//         }else {
//             println!("o arquivo/pasta existe!");
//         }
//     }
// }

// pub fn exists(&self) -> bool {
//     metadata(self).is_ok()
// }


// use std::fmt;

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum Cell {
// 	Dead = 0,
// 	Alive = 1,
// }

// pub struct Universe {
// 	width: u32,
// 	height: u32,
// 	cells: Vec<Cell>,
// }

// impl Universe {

//     pub fn run_logo() {
//         let filename = "./logo.txt";
//         // Open the file in read-only mode.
//         match File::open(filename) {
//             // The file is open (no error).
//             Ok(mut file) => {
//                 let mut content = String::new();
//                 // Read all the file content into a variable (ignoring the result of theoperation).
//                 file.read_to_string(&mut content).unwrap();
//                 println!("{}", content);
//                 // The file is automatically closed when is goes out of scope.
//             },
//             // Error handling.
//             Err(error) => {
//                 println!("Error opening file {}: {}", filename, error);
//             },
//         }
//     }

	// pub fn new(width: u32, height: u32) -> Universe {
	// 	Universe {
	// 		width: width,
	// 		height: height,
	// 		cells: vec![Cell::Dead; (width * height) as usize],
	// 	}
	// }

	// pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
	// 	for (row, col) in cells.iter().cloned() {
	// 		let idx = self.get_index(row, col);
	// 		self.cells[idx] = Cell::Alive;
	// 	}
	// }

	// pub fn tick(&mut self) {
	// 	let mut next = self.cells.clone();
	// 	for row in 0..self.height {
	// 		for col in 0..self.width {
	// 			let idx = self.get_index(row, col);
	// 			let cell = self.cells[idx];
	// 			let live_neighbours = self.live_neighbour_count(row, col);
	// 			next[idx] = match (cell, live_neighbours) {
	// 				(Cell::Alive, x) if x < 2 => Cell::Dead,
	// 				(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
	// 				(Cell::Alive, x) if x > 3 => Cell::Dead,
	// 				(Cell::Dead, 3) => Cell::Alive,
	// 				(otherwise, _) => otherwise,
	// 			};
	// 		}
	// 	}
	// 	self.cells = next;
	// }

	// pub fn row_as_string(&self, row: u32) -> Option<String> {
	// 	if row < self.height {
	// 		let mut row_string = String::new();
	// 		let start = self.get_index(row, 0);
	// 		let end = self.get_index(row, self.width);
	// 		let line = &self.cells[start..end];
	// 		for &cell in line {
	// 			let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
	// 			row_string.push(symbol);
	// 		}
	// 		Some(row_string)
	// 	} else {
	// 		None
	// 	}
	// }

	// fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
	// 	let mut count = 0;
	// 	for delta_row in [self.height - 1, 0, 1].iter().cloned() {
	// 		for delta_col in [self.width - 1, 0, 1].iter().cloned() {
	// 			if delta_row == 0 && delta_col == 0 {
	// 				continue;
	// 			}

	// 			let neighbour_row = (row + delta_row) % self.height;
	// 			let neighbour_col = (column + delta_col) % self.width;
	// 			let idx = self.get_index(neighbour_row, neighbour_col);
	// 			count += self.cells[idx] as u8;
	// 		}
	// 	}

	// 	count
	// }
	// fn get_index(&self, row: u32, column: u32) -> usize {
	// 	(row * self.width + column) as usize
	// }
// }

// impl fmt::Display for Universe {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		for line in self.cells.as_slice().chunks(self.width as usize) {
// 			for &cell in line {
// 				let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
// 				write!(f, "{}", symbol)?;
// 			}
// 			write!(f, "\n")?;
// 		}

// 		Ok(())
// 	}
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn get_index() {
// 		let sut = Universe::new(4, 4);

// 		assert_eq!(sut.get_index(1, 2), 6);
// 	}

	// #[test]
	// fn set_cells() {
	// 	let mut sut = Universe::new(1, 2);
	// 	sut.set_cells(&[(0, 1)]);
	// 	assert_eq!(sut.cells, vec![Cell::Dead, Cell::Alive]);
	// }

	// #[test]
	// fn tick() {
	// 	let mut sut = Universe::new(6, 6);
	// 	sut.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
	// 	let mut expected_universe = Universe::new(6, 6);
	// 	expected_universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
	// 	sut.tick();
	// 	assert_eq!(&sut.cells, &expected_universe.cells);
	// }
// }


// use std::fs::File;
// use std::io::Read;
// pub fn run_logo() {
// 	let filename = "./logo.txt";
// 	// Open the file in read-only mode.
// 	match File::open(filename) {
// 		// The file is open (no error).
// 		Ok(mut file) => {
// 			let mut content = String::new();
// 			// Read all the file content into a variable (ignoring the result of theoperation).
// 			file.read_to_string(&mut content).unwrap();
// 			println!("{}", content);
// 			// The file is automatically closed when is goes out of scope.
// 		},
// 		// Error handling.
// 		Err(error) => {
// 			println!("Error opening file {}: {}", filename, error);
// 		},
// 	}
// }
use crossterm::style;
pub const LOGO: &str = r#"
.::    .   .::::::.    :::::::..  :::::::..   :::    ...    :::::::..       :::::::-.   ...    ::::::.    :::.  .,-:::::/ .,::::::     ...   :::.    :::. .::::::. 
';;,  ;;  ;;;' ;;`;;   ;;;;``;;;; ;;;;``;;;;  ;;; .;;;;;;;. ;;;;``;;;;       ;;,   `';, ;;     ;;;`;;;;,  `;;;,;;-'````'  ;;;;''''  .;;;;;;;.`;;;;,  `;;;;;;`    ` 
 '[[, [[, [[' ,[[ '[[,  [[[,/[[['  [[[,/[[['  [[[,[[     \[[,[[[,/[[['       `[[     [[[['     [[[  [[[[[. '[[[[[   [[[[[[/[[cccc  ,[[     \[[,[[[[[. '[['[==/[[[[,
   Y$c$$$c$P c$$$cc$$$c $$$$$$c    $$$$$$c    $$$$$$,     $$$$$$$$$c          $$,    $$$$      $$$  $$$ "Y$c$$"$$c.    "$$ $$""""  $$$,     $$$$$$ "Y$c$$  '''    $
   "88"888   888   888,888b "88bo,888b "88bo,888"888,_ _,88P888b "88bo,      888_,o8P'88    .d888  888    Y88 `Y8bo,,,o88o888oo,__"888,_ _,88P888    Y88 88b    dP
    "M "M"   YMM   ""` MMMM   "W" MMMM   "W" MMM  "YMMMMMP" MMMM   "W"       MMMMP"`   "YmmMMMM""  MMM     YM   `'YMUP"YMM""""YUMMM "YMMMMMP" MMM     YM  "YMmMY" 
																			 _    ____ 
																		  __ __ / |  |__  |
																		  \ V / | | _  / / 
																		   \_/  |_|(_)/_/ "#;

pub const INITIAL_MENU: &str = "INICIAR JOGO \n\nCARREGAR SAVE \n\nSALVAR E SAIR\n\nAJUSTES \n\nAJUDA";

// pub const SELECTOR_ATRIBUTES: [(style::Attribute);5] = [
// 	(style::Attribute::Reverse),
// 	(style::Attribute::Reverse),
// 	(style::Attribute::Reverse),
// 	(style::Attribute::Reverse),
// 	(style::Attribute::Reverse)
// ];

pub const SELECT_OPTION: [&str;5] = [
	" INICIAR JOGO ",
	" CARREGAR SAVE ",
	" SALVAR E SAIR ",
	" AJUSTES ",
	" AJUDA "
];

pub const OPTION_SELECTED: style::Attribute = style::Attribute::Reverse;

// pub const LOGO: &str = r#"
// .::    .   .::::::.    :::::::..  :::::::..   :::    ...    :::::::..       :::::::-.   ...    ::::::.    :::.  .,-:::::/ .,::::::     ...   :::.    :::. .::::::. 
// ';;,  ;;  ;;;' ;;`;;   ;;;;``;;;; ;;;;``;;;;  ;;; .;;;;;;;. ;;;;``;;;;       ;;,   `';, ;;     ;;;`;;;;,  `;;;,;;-'````'  ;;;;''''  .;;;;;;;.`;;;;,  `;;;;;;`    ` 
//  '[[, [[, [[' ,[[ '[[,  [[[,/[[['  [[[,/[[['  [[[,[[     \\[[,[[[,/[[['       `[[     [[[['     [[[  [[[[[. '[[[[[   [[[[[[/[[cccc  ,[[     \\[[,[[[[[. '[['[==/[[[[,
//    Y$c$$$c$P c$$$cc$$$c $$$$$$c    $$$$$$c    $$$$$$,     $$$$$$$$$c          $$,    $$$$      $$$  $$$ \"Y$c$$\"$$c.    \"$$ $$\"\"\"\"  $$$,     $$$$$$ \"Y$c$$  '''    $
// 	\"88\"888   888   888,888b \"88bo,888b \"88bo,888\"888,_ _,88P888b \"88bo,      888_,o8P'88    .d888  888    Y88 `Y8bo,,,o88o888oo,__\"888,_ _,88P888    Y88 88b    dP
// 		 \"M \"M\"   YMM   \"\"` MMMM   \"W\" MMMM   \"W\" MMM  \"YMMMMMP\" MMMM   \"W\"       MMMMP\"`   \"YmmMMMM\"\"  MMM     YM   `'YMUP\"YMM\"\"\"\"YUMMM \"YMMMMMP\" MMM     YM  \"YMmMY\" 
// 																					 _    ____ 
// 																			  __ __ / |  |__  |
// 																			  \\ V / | | _  / / 
// 																				   \\_/  |_|(_)/_/ "#;
// pub fn run_logo() {
	
// 	for line in MENU.split('\n') {
// 		queue!(w, Print(line), cursor::MoveToNextLine(1))?;
// 	}

// }
