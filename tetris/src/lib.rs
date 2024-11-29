use random::Randomizer;
use rotate::Rotate;

pub mod random;
pub mod rotate;

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Piece {
    pub fn color(&self) -> (u8, u8, u8) {
        match &self {
            // I: Cyan
            Piece::I => (0, 255, 255),
            // J: Blue
            Piece::J => (0, 0, 255),
            // L: Orange
            Piece::L => (255, 127, 0),
            // O: Yellow
            Piece::O => (255, 255, 0),
            // S: Green
            Piece::S => (0, 255, 0),
            // T: Purple
            Piece::T => (255, 0, 255),
            // Z: Red
            Piece::Z => (255, 0, 0),
        }
    }

    pub fn spawn(self) -> CurrentPiece {
        CurrentPiece {
            piece: self,
            x: match self {
                Piece::I => 3,
                Piece::J => 3,
                Piece::L => 3,
                Piece::O => 4,
                Piece::S => 3,
                Piece::T => 3,
                Piece::Z => 3,
            },
            y: 20,
            rotation: Rotation::Rotate0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Rotation {
    #[default]
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Rotation {
    pub fn right(&self) -> Rotation {
        match self {
            Rotation::Rotate0 => Rotation::Rotate90,
            Rotation::Rotate90 => Rotation::Rotate180,
            Rotation::Rotate180 => Rotation::Rotate270,
            Rotation::Rotate270 => Rotation::Rotate90,
        }
    }

    pub fn left(&self) -> Rotation {
        match self {
            Rotation::Rotate0 => Rotation::Rotate270,
            Rotation::Rotate90 => Rotation::Rotate0,
            Rotation::Rotate180 => Rotation::Rotate90,
            Rotation::Rotate270 => Rotation::Rotate180,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentPiece {
    piece: Piece,
    x: u32,
    y: u32,
    rotation: Rotation,
}

impl CurrentPiece {
    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn mask(&self) -> [u16; 4] {
        match self.piece {
            Piece::I => match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => [0, 0, 0, 0b1111 << self.x],
                Rotation::Rotate90 | Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x;
                    [piece_mask, piece_mask, piece_mask, piece_mask]
                }
            },
            Piece::J => match self.rotation {
                Rotation::Rotate0 => [0, 0, 0b1 << self.x, 0b111 << self.x],
                Rotation::Rotate90 => {
                    let piece_mask = 0b1 << self.x;
                    [0, 0b11 << self.x, piece_mask, piece_mask]
                }
                Rotation::Rotate180 => [0, 0, 0b111 << self.x, 0b1 << self.x + 2],
                Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x + 1;
                    [0, piece_mask, piece_mask, 0b11 << self.x]
                }
            },
            Piece::L => match self.rotation {
                Rotation::Rotate0 => [0, 0, 0b1 << self.x + 2, 0b111 << self.x],
                Rotation::Rotate90 => {
                    let piece_mask = 0b1 << self.x;
                    [0, piece_mask, piece_mask, 0b11 << self.x]
                }
                Rotation::Rotate180 => [0, 0, 0b111 << self.x, 0b1 << self.x],
                Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x + 1;
                    [0, 0b11 << self.x, piece_mask, piece_mask]
                }
            },
            Piece::O => {
                let piece_mask = 0b11 << self.x;
                [0, 0, piece_mask, piece_mask]
            }
            Piece::S => match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => {
                    [0, 0, 0b11 << self.x + 1, 0b11 << self.x]
                }
                Rotation::Rotate90 | Rotation::Rotate270 => {
                    [0, 0b1 << self.x, 0b11 << self.x, 0b1 << self.x + 1]
                }
            },
            Piece::T => match self.rotation {
                Rotation::Rotate0 => [0, 0, 0b1 << self.x + 1, 0b111 << self.x],
                Rotation::Rotate90 => {
                    let piece_mask = 0b1 << self.x;
                    [0, piece_mask, 0b11 << self.x, piece_mask]
                }
                Rotation::Rotate180 => [0, 0, 0b111 << self.x, 0b1 << self.x + 1],
                Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x + 1;
                    [0, piece_mask, 0b11 << self.x, piece_mask]
                }
            },
            Piece::Z => match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => {
                    [0, 0, 0b11 << self.x, 0b11 << self.x + 1]
                }
                Rotation::Rotate90 | Rotation::Rotate270 => {
                    [0, 0b1 << self.x + 1, 0b11 << self.x, 0b1 << self.x]
                }
            },
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        self.piece.color()
    }

    pub fn collides(&self, playfield: &PlayfieldMask) -> bool {
        if self.y >= 40 - 4 {
            false
        } else {
            let mask = self.mask();
            for i in 0..4 {
                if mask[i] & playfield[(self.y as usize) + i] != 0 {
                    return false;
                }
            }
            true
        }
    }
}

pub type PlayfieldMask = [u16; 40];

#[derive(Debug, Clone)]
pub struct Ruleset {}

#[derive(Debug, Clone)]
pub struct Game<RNG, ROT>
where
    RNG: Randomizer,
    ROT: Rotate,
{
    current_piece: CurrentPiece,
    ghost_piece: CurrentPiece,
    next_pieces: [Piece; 6],
    playfield_mask: PlayfieldMask,
    playfield_colors: [[(u8, u8, u8); 10]; 40],
    randomizer: RNG,
    rotation: ROT,
    ruleset: Ruleset,
    held_piece: Option<Piece>,
}

impl<RNG: Randomizer, ROT: Rotate> Game<RNG, ROT> {
    pub fn new(mut rng: RNG, rot: ROT) -> Game<RNG, ROT> {
        let piece = rng.get_next_piece().spawn();
        let mut g = Game {
            current_piece: piece.clone(),
            ghost_piece: piece,
            next_pieces: [rng.get_next_piece(), rng.get_next_piece(), rng.get_next_piece(), rng.get_next_piece(), rng.get_next_piece(), rng.get_next_piece()],
            // Make it so outside the playfeild x >= 10 is masked as something there
            playfield_mask: [0b0000000000111111; 40],
            playfield_colors: [[(0, 0, 0); 10]; 40],
            randomizer: rng,
            rotation: rot,
            ruleset: Ruleset {  },
            held_piece: None,
        };

        g.update_ghost();

        g
    }

    pub fn current_piece(&self) -> &CurrentPiece {
        &self.current_piece
    }

    pub fn ghost_piece(&self) -> &CurrentPiece {
        &self.current_piece
    }

    pub fn board(&self) -> &[[(u8, u8, u8); 10]; 40] {
        &self.playfield_colors
    }

    fn update_ghost(&mut self) {
        self.ghost_piece = self.current_piece.clone();
        for i in self.current_piece.y..=0 {
            self.ghost_piece.y = i;
            if self.ghost_piece.collides(&self.playfield_mask) {
                self.ghost_piece.y -= 1;
                break;
            }
        }
    }

    fn get_next_piece(&mut self) -> Piece {
        let next = self.next_pieces[0];
        for i in 0..self.next_pieces.len() - 1 {
            self.next_pieces[i] = self.next_pieces[i + 1];
        }

        self.next_pieces[self.next_pieces.len() - 1] = self.randomizer.get_next_piece();

        // next
        Piece::Z
    }

    pub fn hold(&mut self) {
        let new_piece = if let Some(held) = self.held_piece {
            held.spawn()
        } else {
            self.get_next_piece().spawn()
        };

        self.held_piece = Some(self.current_piece.piece);
        self.current_piece = new_piece;
        self.update_ghost();
    }

    pub fn move_left(&mut self) {
        if self.current_piece.x < 10 {
            self.current_piece.x += 1;
            if self.current_piece.collides(&self.playfield_mask) {
                self.current_piece.x -= 1;
            } else {
                self.update_ghost();
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.current_piece.x > 0 {
            self.current_piece.x -= 1;
            if self.current_piece.collides(&self.playfield_mask) {
                self.current_piece.x += 1;
            } else {
                self.update_ghost();
            }
        }
    }

    pub fn rotate_left(&mut self) {
        self.current_piece = self
            .rotation
            .rotate_left(&self.current_piece, &self.playfield_mask);
        self.update_ghost();
    }

    pub fn rotate_right(&mut self) {
        self.current_piece = self
            .rotation
            .rotate_right(&self.current_piece, &self.playfield_mask);
        self.update_ghost();
    }

    pub fn update(&mut self) {
        if self.current_piece.y > 0 {
            self.current_piece.y -= 1;
            if self.current_piece.collides(&self.playfield_mask) {
                self.current_piece.y += 1;
            }
        }
    }
}
