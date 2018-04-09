//! Implementation of the min max algorithm.

use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use rayon::prelude::*;


/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        fn min(state: &Configuration, depth: u8) -> i8 {
            if depth == 0 {
                return state.value();
            }
            state.movements().collect::<Vec<Movement>>().into_par_iter().map(|x| max(&state.play(&x), depth - 1)).min().unwrap()
        }
        fn max(state: &Configuration, depth: u8) -> i8 {
            if depth == 0 {
                return -state.value();
            }
            state.movements().collect::<Vec<Movement>>().into_par_iter().map(|x| min(&state.play(&x), depth - 1)).max().unwrap()
        }
        state.movements().collect::<Vec<Movement>>().into_par_iter().max_by_key(|x| min(&state.play(&x), self.0 - 1))
    }

}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
