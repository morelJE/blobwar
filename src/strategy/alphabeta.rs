//! Alpha - Beta algorithm.
use std::fmt;
use std::cmp;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use rayon::prelude::*;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {


        fn alpha_beta_max(state: &Configuration, alpha: i8, beta: i8, depth: u8) -> i8 {

            let mut new_alpha = alpha;
            if depth == 0 {
                return -state.value();
            }

            let mut maxi = -63i8;
            let mut res;

            for it in state.movements() {
                res = alpha_beta_min(&state.play(&it), new_alpha, beta, depth - 1);
                if res >= beta {
                    return res;
                }
                if res > maxi {
                    maxi = res;
                }
                new_alpha = cmp::max(new_alpha, res);
            }
            maxi
        }

        fn alpha_beta_min(state: &Configuration, alpha: i8, beta: i8, depth: u8) -> i8 {

            let mut new_beta = beta;
            if depth == 0 {
                return state.value();
            }

            let mut mini = 63i8;
            let mut res;

            for it in state.movements() {
                res = alpha_beta_max(&state.play(&it), alpha, beta, depth - 1);
                if res <= alpha {
                    return res;
                }
                if res < mini {
                    mini = res;
                }
                new_beta = cmp::min(new_beta, res);
            }
            mini
        }

        // let mut maxi = -63i8;
        // let mut res;
        // let mut mouv = None;

        //println!("hihi : {}", state.movements().map(|x| min(&state.play(&x), self.0 - 1)).zip(state.movements()).max_by(|&(_, item)| item));

        state.movements().collect::<Vec<Movement>>().into_par_iter().max_by_key(|x| alpha_beta_min(&state.play(&x), -63i8, 63i8, self.0 - 1))
    }
}
