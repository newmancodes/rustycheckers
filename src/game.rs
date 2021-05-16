use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialise_pieces();
        engine
    }

    pub fn initialise_pieces(&mut self) {
        [1, 3 , 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::White));
            });
        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
            })
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();

        if !legal_moves.contains(mv) {
            return Err(());
        }

        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap();
        let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }

        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();

        Ok(MoveResult {
            mv: mv.clone(),
            crowned: crowned,
        })
    }

    pub fn current_turn(&self) -> PieceColor {
        self.current_turn
    }

    pub fn get_piece(&self, loc: Coordinate) -> Result<Option<GamePiece>, ()> {
        let Coordinate(x, y) = loc;
        if x > 7 || y > 7 {
            Err(())
        } else {
            Ok(self.board[x][y])
        }
    }

    fn midpiece_coordinate(&self, fx: usize, fy: usize, tx: usize, ty: usize) -> Option<Coordinate> {
        if fx + 2 == tx {
            if fy + 2 == ty {
                return Some(Coordinate(fx + 1, fy + 1));
            } else if fy - 2 == ty {
                return Some(Coordinate(fx + 1, fy - 1));
            }
        } else if fx - 2 == tx {
            if fy + 2 == ty {
                return Some(Coordinate(fx - 1, fy + 1));
            } else if fy - 2 == ty {
                return Some(Coordinate(fx - 1, fy - 1));
            }
        }

        None
    }

    fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }

        moves
    }

    fn valid_moves_from(&mut self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, & t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone()
                }).collect::<Vec<Move>>();
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            moves.append(&mut jumps);
            moves
        } else {
            Vec::new()
        }
    }

    fn valid_move(&mut self, p: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        let source_piece = self.get_piece(*from);
        let target_piece = self.get_piece(*to);
        source_piece.is_ok() && source_piece.unwrap().is_some() && source_piece.unwrap().unwrap().color == self.current_turn && target_piece.is_ok() && !target_piece.unwrap().is_some()
    }

    fn valid_jump(&mut self, p: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        let Coordinate(fx, fy) = *from;
        let Coordinate(tx, ty) = *to;
        if let Some(midpiece) = self.midpiece_coordinate(fx, fy, tx, ty) {
            if let Some(jumped_piece) = self.get_piece(midpiece).unwrap() {
                return p.color != jumped_piece.color;
            }
        }

        false
    }
    
    fn should_crown(&self, p: GamePiece, loc: Coordinate) -> bool {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            match (p.color, y) {
                (PieceColor::White, 7) => return true,
                (PieceColor::Black, 0) => return true,
                _ => return false,
            }
        }

        false
    }

    fn crown_piece(&mut self, loc: Coordinate) {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            self.board[x][y] = Some(GamePiece::crowned(p));
        }
    }

    fn advance_turn(&mut self) {
        self.move_count = self.move_count + 1;
        match self.current_turn {
            PieceColor::White => self.current_turn = PieceColor::Black,
            PieceColor::Black => self.current_turn = PieceColor::White,
        }
    }
}