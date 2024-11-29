use rand::{seq::SliceRandom, Rng};

use crate::Piece;

pub trait Randomizer {
    fn get_next_piece(&mut self) -> Piece;
}

pub struct RandomGenerator<R> where R: Rng + Sized {
    bag: [Piece; 7],
    index: usize,
    rng: R
}

impl<R> RandomGenerator<R> where R: Rng + Sized {
    fn generate_bag(&mut self) {
        self.bag = [
            Piece::I,
            Piece::J,
            Piece::L,
            Piece::O,
            Piece::S,
            Piece::T,
            Piece::Z,
        ];

        self.bag.shuffle(&mut self.rng);
        self.index = 0;
    }

    pub fn new(rng: R) -> Self {
        let mut rng = RandomGenerator {
            bag: [Piece::I; 7],
            index: 0,
            rng,
        };
        rng.generate_bag();
        
        rng
    }
}

impl<R> Randomizer for RandomGenerator<R> where R: Rng + Sized {
    fn get_next_piece(&mut self) -> Piece {
        if self.index == 7 {
            self.generate_bag();
        }

        let p = self.bag[self.index];
        self.index += 1;
        p
    }
}
