#![no_std]

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
            Rotation::Rotate270 => Rotation::Rotate0,
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
    pub fn new(piece: Piece, x: u32, y: u32, rotation: Rotation) -> Self {
        CurrentPiece { piece, x, y, rotation }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn mask(&self) -> [u16; 4] {
        match self.piece {
            Piece::I => match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => [0b1111 << self.x, 0, 0, 0],
                Rotation::Rotate90 | Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x;
                    [piece_mask, piece_mask, piece_mask, piece_mask]
                }
            },
            Piece::J => match self.rotation {
                Rotation::Rotate0 => [0b111 << self.x, 0b1 << self.x, 0, 0],
                Rotation::Rotate90 => {
                    let piece_mask = 0b1 << self.x;
                    [piece_mask, piece_mask, 0b11 << self.x, 0]
                }
                Rotation::Rotate180 => [0b1 << self.x + 2, 0b111 << self.x, 0, 0],
                Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x + 1;
                    [0b11 << self.x, piece_mask, piece_mask, 0]
                }
            },
            Piece::L => match self.rotation {
                Rotation::Rotate0 => [0b111 << self.x, 0b1 << self.x + 2, 0, 0],
                Rotation::Rotate90 => {
                    let piece_mask = 0b1 << self.x;
                    [0b11 << self.x, piece_mask, piece_mask, 0]
                }
                Rotation::Rotate180 => [0b1 << self.x, 0b111 << self.x, 0, 0],
                Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x + 1;
                    [piece_mask, piece_mask, 0b11 << self.x, 0]
                }
            },
            Piece::O => {
                let piece_mask = 0b11 << self.x;
                [piece_mask, piece_mask, 0, 0]
            }
            Piece::S => match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => {
                    [0b11 << self.x, 0b11 << self.x + 1, 0, 0]
                }
                Rotation::Rotate90 | Rotation::Rotate270 => {
                    [0b1 << self.x + 1, 0b11 << self.x, 0b1 << self.x, 0]
                }
            },
            Piece::T => match self.rotation {
                Rotation::Rotate0 => [0b111 << self.x, 0b1 << self.x + 1, 0, 0],
                Rotation::Rotate90 => {
                    let piece_mask = 0b1 << self.x;
                    [piece_mask, 0b11 << self.x, piece_mask, 0]
                }
                Rotation::Rotate180 => [0b1 << self.x + 1, 0b111 << self.x, 0, 0],
                Rotation::Rotate270 => {
                    let piece_mask = 0b1 << self.x + 1;
                    [piece_mask, 0b11 << self.x, piece_mask, 0]
                }
            },
            Piece::Z => match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => {
                    [0b11 << self.x + 1, 0b11 << self.x, 0, 0]
                }
                Rotation::Rotate90 | Rotation::Rotate270 => {
                    [0b1 << self.x, 0b11 << self.x, 0b1 << self.x + 1, 0]
                }
            },
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        self.piece.color()
    }

    pub fn collides(&self, playfield: &PlayfieldMask) -> bool {
        if self.y >= 40 - 4 {
            true
        } else {
            let mask = self.mask();
            for i in 0..4 {
                if mask[i] & playfield[(self.y as usize) + i] != 0 {
                    return true;
                }
            }
            false
        }
    }
}

pub type PlayfieldMask = [u16; 40];

#[derive(Debug, Clone)]
pub struct Ruleset {
    das_delay: u32,
    das_gravity: f32,
    drop_gravity_multipler: f32,
    lock_delay: u32,
    lock_resets: u32,
    line_clear_constant: u32,
    line_clear_coeff: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HeldDirection {
    #[default]
    None,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Game<RNG, ROT>
where
    RNG: Randomizer,
    ROT: Rotate,
{
    game_ended: bool,
    current_piece: CurrentPiece,
    ghost_piece: CurrentPiece,
    next_pieces: [Piece; 6],
    playfield_mask: PlayfieldMask,
    playfield_colors: [[(u8, u8, u8); 10]; 40],
    randomizer: RNG,
    rotation: ROT,
    ruleset: Ruleset,
    held_piece: Option<Piece>,
    hold_lock: bool,
    level: u32,
    line_clear_total: u32,
    line_clear_count: u32,
    gravity: f32,
    movement: f32,
    lock_ticks: u32,
    lock_tries: u32,
    das_movement: f32,
    das_ticks: u32,
    left_held: bool,
    right_held: bool,
    held_direction: HeldDirection,
    drop_held: bool,
    line_clears: ([u32; 4], usize),
}

impl<RNG: Randomizer, ROT: Rotate> Game<RNG, ROT> {
    pub fn new(mut rng: RNG, rot: ROT) -> Game<RNG, ROT> {
        let piece = rng.get_next_piece().spawn();
        let mut g = Game {
            game_ended: false,
            current_piece: piece.clone(),
            ghost_piece: piece,
            next_pieces: [
                rng.get_next_piece(),
                rng.get_next_piece(),
                rng.get_next_piece(),
                rng.get_next_piece(),
                rng.get_next_piece(),
                rng.get_next_piece(),
            ],
            // Make it so outside the playfeild x >= 10 is masked as something there
            playfield_mask: [0b1111110000000000; 40],
            playfield_colors: [[(0, 0, 0); 10]; 40],
            randomizer: rng,
            rotation: rot,
            ruleset: Ruleset {
                // 300 ms (18 ticks / 60 fps = 3/10 s)
                das_delay: 18,
                // 1 Tile / 2 Tick
                das_gravity: 0.5,
                // 20x Normal Drop Speed
                drop_gravity_multipler: 20.0,
                // 1 s (60 ticks / 60 fps = 1 s)
                lock_delay: 60,
                // 25 Moves to reset lock delay
                lock_resets: 25,
                // 10 / 0 for fixed and 5 / 5 for variable
                // line_clear_constant: 10,
                line_clear_constant: 10,
                line_clear_coeff: 0,
            },
            held_piece: None,
            hold_lock: false,
            gravity: 0.0,
            level: 0,
            line_clear_total: 0,
            line_clear_count: 0,
            movement: 0.0,
            lock_ticks: 0,
            lock_tries: 0,
            held_direction: HeldDirection::default(),
            left_held: false,
            right_held: false,
            drop_held: false,
            das_movement: 0.0,
            das_ticks: 0,
            line_clears: ([0, 0, 0, 0], 0),
        };

        g.update_ghost();
        g.update_gravity();

        g
    }

    pub fn running(&self) -> bool {
        !self.game_ended
    }

    pub fn current_piece(&self) -> &CurrentPiece {
        &self.current_piece
    }

    pub fn ghost_piece(&self) -> &CurrentPiece {
        &self.ghost_piece
    }

    pub fn held_piece(&self) -> Option<Piece> {
        self.held_piece
    }

    pub fn next_pieces(&self) -> [Piece; 6] {
        self.next_pieces
    }

    pub fn board(&self) -> &[[(u8, u8, u8); 10]; 40] {
        &self.playfield_colors
    }

    fn update_gravity(&mut self) {
        self.gravity = 1.0 / (((0.8 - ((self.level as f32) * 0.007)).powi(self.level as i32)) * 60.0);
    }

    fn update_ghost(&mut self) {
        self.ghost_piece = self.current_piece.clone();
        for i in (0..=self.current_piece.y).rev() {
            self.ghost_piece.y = i;
            if self.ghost_piece.collides(&self.playfield_mask) {
                self.ghost_piece.y += 1;
                break;
            }
        }
    }

    fn lock_piece(&mut self, piece: &CurrentPiece) {
        if piece.y > 20 {
            self.game_ended = true;
        }

        let c = piece.color();
        for (i, m) in piece.mask().iter().enumerate() {
            let y = piece.y as usize + i;
            self.playfield_mask[y] |= *m;
            if y < 40 {
                for x in 0..10 {
                    if ((1 << x) & *m) != 0 {
                        self.playfield_colors[y][x] = c;
                    }
                }
            }
        }

        let piece = self.get_next_piece();
        self.new_piece(piece);
        self.hold_lock = false;
    }

    fn get_next_piece(&mut self) -> Piece {
        let next = self.next_pieces[0];
        for i in 0..self.next_pieces.len() - 1 {
            self.next_pieces[i] = self.next_pieces[i + 1];
        }

        self.next_pieces[self.next_pieces.len() - 1] = self.randomizer.get_next_piece();

        next
    }

    fn new_piece(&mut self, piece: Piece) {
        self.current_piece = piece.spawn();
        if self.current_piece.collides(&self.playfield_mask) {
            // Lock out
            self.game_ended = true;
        }
        self.movement = 0.0;
        self.lock_ticks = 0;
        self.lock_tries = 0;
        self.update_ghost();
    }

    fn reset_lock(&mut self) {
        if self.lock_ticks > 0 && self.lock_tries < self.ruleset.lock_resets {
            self.lock_tries += 1;
            self.lock_ticks = 0;
        }
    }

    pub fn hold(&mut self) {
        if !self.hold_lock {
            let new_piece = if let Some(held) = self.held_piece {
                held
            } else {
                self.get_next_piece()
            };

            self.held_piece = Some(self.current_piece.piece);
            
            self.update_ghost();
            self.new_piece(new_piece);
            self.hold_lock = true;
        }
    }

    pub fn move_left(&mut self) {
        if self.current_piece.x > 0 {
            self.current_piece.x -= 1;
            if self.current_piece.collides(&self.playfield_mask) {
                self.current_piece.x += 1;
            } else {
                self.reset_lock();
                self.update_ghost();
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.current_piece.x < 10 {
            self.current_piece.x += 1;
            if self.current_piece.collides(&self.playfield_mask) {
                self.current_piece.x -= 1;
            } else {
                self.reset_lock();
                self.update_ghost();
            }
        }
    }

    pub fn rotate_left(&mut self) {
        if let Some(rot) = self
            .rotation
            .rotate_left(&self.current_piece, &self.playfield_mask)
        {
            self.current_piece = rot;
            self.reset_lock();
            self.update_ghost();
        }
    }

    pub fn rotate_right(&mut self) {
        if let Some(rot) = self
            .rotation
            .rotate_right(&self.current_piece, &self.playfield_mask)
        {
            self.current_piece = rot;
            self.reset_lock();
            self.update_ghost();
        }
    }

    pub fn hard_drop(&mut self) {
        let ghost = self.ghost_piece.clone();
        self.lock_piece(&ghost);
    }

    pub fn set_drop(&mut self, state: bool) {
        self.drop_held = state;
    }

    pub fn set_left(&mut self, state: bool) {
        if state != self.left_held {
            if state {
                self.move_left();
            }
            self.das_ticks = 0;
            self.das_movement = 0.0;
            self.left_held = state;
            self.held_direction = if state {
                HeldDirection::Left
            } else {
                if self.right_held {
                    HeldDirection::Right
                } else {
                    HeldDirection::None
                }
            };
        }
    }

    pub fn set_right(&mut self, state: bool) {
        if state != self.right_held {
            if state {
                self.move_right();
            }
            self.das_ticks = 0;
            self.das_movement = 0.0;
            self.right_held = state;
            self.held_direction = if state {
                HeldDirection::Right
            } else {
                if self.left_held {
                    HeldDirection::Left
                } else {
                    HeldDirection::None
                }
            };
        }
    }

    pub fn update(&mut self) {
        if self.line_clears.1 > 0 {
            self.line_clear_count += self.line_clears.1 as u32;
            self.line_clear_total += self.line_clears.1 as u32;
            let limit = self.ruleset.line_clear_constant + (self.level * self.ruleset.line_clear_coeff);
            if self.line_clear_count >= limit {
                self.line_clear_count = 0;
                self.level += 1;
                self.update_gravity();
            }

            for i in (0..self.line_clears.1).rev() {
                let l = self.line_clears.0[i] as usize;
                for i in l..39 {
                    self.playfield_mask[i] = self.playfield_mask[i+1];
                    for c in 0..10 {
                        self.playfield_colors[i][c] = self.playfield_colors[i+1][c];
                    }
                }

                self.playfield_mask[39] = 0b1111110000000000;
                for c in 0..10 {
                    self.playfield_colors[39][c] = (0, 0, 0);
                }
            }

            self.line_clears.1 = 0;
            return;
        }


        let mut g = self.gravity;
        if self.drop_held {
            g *= self.ruleset.drop_gravity_multipler;
        }
        
        self.movement += g;
        while self.movement > 1.0 {
            if self.current_piece.y > 0 {
                self.current_piece.y -= 1;
                if self.current_piece.collides(&self.playfield_mask) {
                    self.current_piece.y += 1;
                }
            }

            self.movement -= 1.0;
        }

        if self.held_direction != HeldDirection::None {
            if self.das_ticks < self.ruleset.das_delay {
                self.das_ticks += 1;

            } else {
                self.das_movement += self.ruleset.das_gravity;
                while self.das_movement > 1.0 {
                    match &self.held_direction {
                        HeldDirection::None => unreachable!(),
                        HeldDirection::Left => {self.move_left()},
                        HeldDirection::Right => {self.move_right()},
                    }
        
                    self.das_movement -= 1.0;
                }
            }
        }

        // If the piece will lock since it can't move down more
        if self.ghost_piece.y == self.current_piece.y {
            self.lock_ticks += 1;
        }

        if self.lock_ticks >= self.ruleset.lock_delay {
            let piece = self.current_piece.clone();
            self.lock_piece(&piece);
        }

        // Check line clears
        for (i, l) in self.playfield_mask.iter().enumerate() {
            if !(*l) == 0 {
                // Full line
                self.line_clears.0[self.line_clears.1] = i as u32;
                self.line_clears.1 += 1;

                if self.line_clears.1 >= 4 {
                    break;
                }
            }
        }
    }
}
