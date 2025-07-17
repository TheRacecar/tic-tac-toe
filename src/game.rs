use std::fmt;

use colored::{ Color, ColoredString, Colorize };

pub type BoardCoordinate = (usize, usize);

pub fn parse_output(input: String) -> Result<BoardCoordinate, ()> {
    let mut ret = false;

    let split = input
        .split(",")
        .map(|x| x.trim())
        .map(|x| str::parse::<usize>(x))
        .map(|x| {
            match x {
                Ok(x) => x,
                Err(_) => {
                    ret = true;
                    return 0;
                }
            }
        })
        .collect::<Vec<usize>>();

    if ret || split.len() < 2 {
        return Err(());
    }

    if split[0] > 2 || split[1] > 2 {
        return Err(());
    }

    return Ok((split[0], split[1]));
}

#[derive(Clone, Copy)]
pub enum GameResult {
    Win(PlayerSymbol),
    Draw,
    ViewingPosition,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GameResult::Win(symbol) => format!("Player {} wins", symbol.display()),
            GameResult::Draw => "Draw".bold().green().to_string(),
            GameResult::ViewingPosition => "Viewing Position".bold().magenta().to_string(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerSymbol {
    Nought,
    Cross,
}

impl PlayerSymbol {
    fn display(&self) -> ColoredString {
        String::from(match self {
            PlayerSymbol::Nought => "O",
            PlayerSymbol::Cross => "X",
        }).color(match self {
            PlayerSymbol::Nought => Color::Blue,
            PlayerSymbol::Cross => Color::Red,
        })
    }

    fn switch(&self) -> Self {
        match self {
            Self::Nought => Self::Cross,
            Self::Cross => Self::Nought,
        }
    }
}

impl ToString for PlayerSymbol {
    fn to_string(&self) -> String {
        String::from(match self {
            PlayerSymbol::Nought => "O",
            PlayerSymbol::Cross => "X",
        })
    }
}

#[derive(Clone)]
pub struct Game {
    current_player: PlayerSymbol,
    pub board_layout: [[Option<PlayerSymbol>; 3]; 3],
    last_played: BoardCoordinate,
}

impl Game {
    pub const fn new() -> Self {
        Self {
            current_player: PlayerSymbol::Cross,
            board_layout: [
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ],
            last_played: (0, 0),
        }
    }

    pub fn add_symbol(&mut self, x: usize, y: usize) {
        if let Some(_) = self.board_layout[y][x] {
            println!("There is already a symbol in that square");
            return;
        }

        self.board_layout[y][x] = Some(self.current_player);
        self.current_player = self.current_player.switch();
        self.last_played = (x, y);
    }

    pub fn get_current_player(&self) -> PlayerSymbol {
        self.current_player
    }

    fn display_symbol(&self, x: usize, y: usize) -> ColoredString {
        if let Some(symbol) = self.board_layout[y][x] {
            let formatted = symbol.to_string().color(match symbol {
                PlayerSymbol::Nought => Color::Blue,
                PlayerSymbol::Cross => Color::Red,
            });

            if self.last_played == (x, y) {
                formatted.bold()
            } else {
                formatted
            }
        } else {
            String::from(" ").into()
        }
    }

    fn get_position_id(&self) -> String {
        let mut id = String::new();

        for i in self.board_layout {
            for j in i {
                match j {
                    Some(symbol) => id.push_str(&symbol.to_string()),
                    None => id.push('N'),
                }
            }
        }

        id.push_str(&format!("/{}{}", self.last_played.0, self.last_played.1));

        return id;
    }

    pub fn set_position_id(&mut self, position_id: String) -> Result<(), ()> {
        let split = position_id.split('/')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        
        if split.len() != 2 {
            println!("Invalid position ID");
            return Err(());
        }

        let pos_id = split[0].chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.into_iter().collect::<String>())
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        
        let mut ret = false;
        let last_move = split[1].chars()
            .map(|c| 
                str::parse::<usize>(&c.to_string())
                    .unwrap_or_else(|_| {
                        ret = true;
                        return 0;
                    })
            )
            .collect::<Vec<usize>>();
        
        if ret || last_move.len() != 2 {
            println!("Invalid second sector format");
            return Err(());
        }

        for i in 0..3 {
            for j in 0..3 {
                self.board_layout[i][j] = match pos_id[i][j] {
                    'N' => None,
                    'X' => Some(PlayerSymbol::Cross),
                    'O' => Some(PlayerSymbol::Nought),
                    _ => {
                        println!("Invalid position ID");
                        return Err(());
                    }
                }
            }
        }

        self.last_played = (last_move[0], last_move[1]);

        return Ok(());
    }

    pub fn game_over(&self) -> Option<GameResult> {
        for i in 0..3 {
            if
                let (Some(a), Some(b), Some(c)) = (
                    self.board_layout[0][i],
                    self.board_layout[1][i],
                    self.board_layout[2][i],
                )
            {
                if a == b && b == c {
                    return Some(GameResult::Win(a));
                }
            }

            if
                let (Some(a), Some(b), Some(c)) = (
                    self.board_layout[i][0],
                    self.board_layout[i][1],
                    self.board_layout[i][2],
                )
            {
                if a == b && b == c {
                    return Some(GameResult::Win(a));
                }
            }
        }

        if
            let (Some(a), Some(b), Some(c)) = (
                self.board_layout[0][0],
                self.board_layout[1][1],
                self.board_layout[2][2],
            )
        {
            if a == b && b == c {
                return Some(GameResult::Win(a));
            }
        }

        if
            let (Some(a), Some(b), Some(c)) = (
                self.board_layout[0][2],
                self.board_layout[1][1],
                self.board_layout[2][0],
            )
        {
            if a == b && b == c {
                return Some(GameResult::Win(a));
            }
        }

        let mut full = true;
        for i in 0..3 {
            for j in 0..3 {
                if let None = self.board_layout[i][j] {
                    full = false;
                }
            }
        }

        if full {
            return Some(GameResult::Draw);
        }

        return None;
    }

    pub fn get_board_formatted(&self) -> String {
        format!(
            r#"
    {} │ {} │ {}       {} {}
    ──┼───┼──
    {} │ {} │ {}       {} {}
    ──┼───┼──       Current player: {}
    {} │ {} │ {}       Move {}: "#,
            self.display_symbol(0, 0),
            self.display_symbol(1, 0),
            self.display_symbol(2, 0),
            "Tic Tac Toe    Position ID:".bold(),
            self.get_position_id().cyan().bold(),
            self.display_symbol(0, 1),
            self.display_symbol(1, 1),
            self.display_symbol(2, 1),
            "Turn",
            "0".yellow().bold(),
            self.current_player.display().bold(),
            self.display_symbol(0, 2),
            self.display_symbol(1, 2),
            self.display_symbol(2, 2),
            format!("({}, {})", "x".green(), "y".purple()).bold()
        )
    }

    pub fn get_game_over_formatted(&self, result: GameResult) -> String {
        format!(
            r#"
{} │ {} │ {}       {} {}
──┼───┼──
{} │ {} │ {}       Game {}
──┼───┼──       {}
{} │ {} │ {}"#,
            self.display_symbol(0, 0),
            self.display_symbol(1, 0),
            self.display_symbol(2, 0),
            "Tic Tac Toe    Position ID:".bold(),
            self.get_position_id().cyan().bold(),
            self.display_symbol(0, 1),
            self.display_symbol(1, 1),
            self.display_symbol(2, 1),
            "OVER".yellow().bold(),
            result,
            self.display_symbol(0, 2),
            self.display_symbol(1, 2),
            self.display_symbol(2, 2)
        )
    }

    pub fn get_available_moves(&self) -> Vec<BoardCoordinate> {
        let mut moves = Vec::new();
        for y in 0..3 {
            for x in 0..3 {
                if self.board_layout[y][x].is_none() {
                    moves.push((x, y));
                }
            }
        }
        moves
    }
}
