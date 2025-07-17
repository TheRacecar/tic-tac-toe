use crate::game::{BoardCoordinate, Game, GameResult, PlayerSymbol};

pub fn minimax(game: &mut Game, is_maximizing: bool) -> (i32, Option<BoardCoordinate>) {
    if let Some(result) = game.game_over() {
        return match result {
            GameResult::Win(winner) => {
                if winner == PlayerSymbol::Nought {
                    (1, None) // AI wins
                } else {
                    (-1, None) // Human wins
                }
            }
            GameResult::Draw => (0, None), // Draw
            GameResult::ViewingPosition => (0, None), // Viewing position
        };
    }

    let mut best_score = if is_maximizing { i32::MIN } else { i32::MAX };
    let mut best_move = None;

    for (x, y) in game.get_available_moves() {
        game.board_layout[y][x] = Some(if is_maximizing { PlayerSymbol::Nought } else { PlayerSymbol::Cross });

        let (score, _) = minimax(game, !is_maximizing);

        game.board_layout[y][x] = None; // undo move

        if is_maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some((x, y));
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some((x, y));
            }
        }
    }

    (best_score, best_move)
}