use crate::{CurrentPiece, Piece, PlayfieldMask, Rotation};

pub trait Rotate {
    fn rotate_left(&self, piece: &CurrentPiece, playfield: &PlayfieldMask) -> Option<CurrentPiece>;
    fn rotate_right(&self, piece: &CurrentPiece, playfield: &PlayfieldMask) -> Option<CurrentPiece>;
}

pub struct SuperRotationSystem {}

impl SuperRotationSystem {
    fn rotate(&self, piece: &CurrentPiece, playfield: &PlayfieldMask, clockwise: bool) -> Option<CurrentPiece> {
        // JLSTZ
        // 0->R     (0, 0)     (-1, 0)     (-1, +1)     (0, -2)     (-1, -2)
        // R->2     (0, 0)     (+1, 0)     (+1, -1)     (0, +2)     (+1, +2)
        // 2->L     (0, 0)     (+1, 0)     (+1, +1)     (0, -2)     (+1, -2)
        // L->0     (0, 0)     (-1, 0)     (-1, -1)     (0, +2)     (-1, +2)

        // I
        // 0->R     (0, 0)     (-2, 0)     (+1, 0)     (-2, -1)     (+1, +2)
        // R->2     (0, 0)     (-1, 0)     (+2, 0)     (-1, +2)     (+2, -1)
        // 2->L     (0, 0)     (+2, 0)     (-1, 0)     (+2, +1)     (-1, -2)
        // L->0     (0, 0)     (+1, 0)     (-2, 0)     (+1, -2)     (-2, +1)

        let mut new_position = piece.clone();
        let pos;

        if clockwise {
            pos = new_position.rotation;
            new_position.rotation = new_position.rotation.right();
        } else {
            new_position.rotation = new_position.rotation.left();
            pos = new_position.rotation;
        }

        let tests = match &new_position.piece {
            Piece::I => match &pos {
                Rotation::Rotate0 => [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                Rotation::Rotate90 => [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                Rotation::Rotate180 => [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                Rotation::Rotate270 => [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
            },
            Piece::J | Piece::L | Piece::O | Piece::S | Piece::T | Piece::Z => match &pos {
                Rotation::Rotate0 => [(0,  0),  (-1,  0),  (-1, 1),  (0, -2),  (-1, -2)],
                Rotation::Rotate90 => [(0,  0),  (1,  0),  (1, -1),  (0, 2),  (1, 2)],
                Rotation::Rotate180 => [(0,  0),  (1,  0),  (1, 1),  (0, -2),  (1, -2)],
                Rotation::Rotate270 => [(0,  0),  (-1,  0),  (-1, -1),  (0, 2),  (-1, 2)],
            },
        };

        let mult = if clockwise { 1 } else { -1 };

        let x = (piece.x as i32) + (match &new_position.piece {
            Piece::I => match &pos {
                Rotation::Rotate0 => 2,
                Rotation::Rotate90 => -2,
                Rotation::Rotate180 => 1,
                Rotation::Rotate270 => -1,
            },
            Piece::J => match &pos {
                Rotation::Rotate0 => 1,
                Rotation::Rotate90 => -1,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
            Piece::L => match &pos {
                Rotation::Rotate0 => 1,
                Rotation::Rotate90 => -1,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
            Piece::O => match &pos {
                Rotation::Rotate0 => 0,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
            Piece::S => match &pos {
                Rotation::Rotate0 => 1,
                Rotation::Rotate90 => -1,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
            Piece::T => match &pos {
                Rotation::Rotate0 => 1,
                Rotation::Rotate90 => -1,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
            Piece::Z => match &pos {
                Rotation::Rotate0 => 1,
                Rotation::Rotate90 => -1,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
        } * mult);

        let y = (piece.y as i32) + (match &new_position.piece {
            Piece::I => match &pos {
                Rotation::Rotate0 => -2,
                Rotation::Rotate90 => 1,
                Rotation::Rotate180 => -1,
                Rotation::Rotate270 => 2,
            },
            Piece::J => match &pos {
                Rotation::Rotate0 => -1,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 1,
            },
            Piece::L => match &pos {
                Rotation::Rotate0 => -1,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 1,
            },
            Piece::O => match &pos {
                Rotation::Rotate0 => 0,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 0,
            },
            Piece::S => match &pos {
                Rotation::Rotate0 => -1,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 1,
            },
            Piece::T => match &pos {
                Rotation::Rotate0 => -1,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 1,
            },
            Piece::Z => match &pos {
                Rotation::Rotate0 => -1,
                Rotation::Rotate90 => 0,
                Rotation::Rotate180 => 0,
                Rotation::Rotate270 => 1,
            },
        } * mult);
        
        for test in tests {
            let x_tmp = x + (test.0 * mult);
            let y_tmp = y + (test.1 * mult);

            if x_tmp >= 0 && y_tmp >= 0 {
                new_position.x = x_tmp as u32;
                new_position.y = y_tmp as u32;
                if !new_position.collides(playfield) {
                    return Some(new_position);
                }
            }
        }

        None
    }
}

impl Rotate for SuperRotationSystem {
    fn rotate_left(&self, piece: &CurrentPiece, playfield: &PlayfieldMask) -> Option<CurrentPiece> {
        self.rotate(piece, playfield, false)
    }

    fn rotate_right(&self, piece: &CurrentPiece, playfield: &PlayfieldMask) -> Option<CurrentPiece> {
        self.rotate(piece, playfield, true)
    }
}