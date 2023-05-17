use core::fmt;
use std::{
    eprintln,
    fmt::{format, write, Display},
};

use crate::solver::Solvable;

mod solver;

#[derive(Debug)]
struct Prob {
    ns: Vec<u8>,
}

impl Solvable<Prob> for Prob {
    fn first_solution(&self) -> Option<Prob> {
        self.rec_solve(0)
    }

    fn all_solutions(&self) -> Vec<Prob> {
        let mut res = Vec::new();
        self.rec_solve_fill(0, &mut res);
        res
    }
}

impl Prob {
    fn len(&self) -> u8 {
        self.ns.len() as u8
    }

    fn actual_counts(&self) -> Vec<u8> {
        let mut res = vec![0; self.len() as usize];

        for i in 0..self.len() {
            let mut c = 0;
            for j in 0..self.len() {
                if self.ns[j as usize] == i {
                    c += 1;
                }
            }
            res[i as usize] = c + 1;
        }
        res
    }

    fn check_ns(&self) -> bool {
        self.actual_counts() == self.ns
    }

    fn rec_solve_fill(&self, from: u8, result: &mut Vec<Prob>) {
        let len = self.len();

        for i in 0..2 * len as u8 {
            let mut new_ns = self.ns.clone();
            new_ns[from as usize] = i;
            let new_prob = Prob { ns: new_ns };

            if new_prob.check_ns() {
                return result.push(new_prob);
            }

            if from < len as u8 - 1 {
                new_prob.rec_solve_fill(from + 1, result);
            }
        }
    }

    fn rec_solve(&self, from: u8) -> Option<Prob> {
        let len = self.len();

        for i in 0..2 * len as u8 {
            let mut new_ns = self.ns.clone();
            new_ns[from as usize] = i;
            let new_prob = Prob { ns: new_ns };

            if new_prob.check_ns() {
                return Some(new_prob);
            }

            if from < len as u8 - 1 {
                if let Some(p) = new_prob.rec_solve(from + 1) {
                    return Some(p);
                }
            }
        }
        None
    }
}

impl Display for Prob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, j) in self.ns.iter().enumerate() {
            write!(f, "{} fois le chiffre {}\n", j, i)?
        }
        Ok(())
    }
}

fn main() {
    let prob = Prob { ns: vec![0; 5] };

    println!("=== First Solution ===");
    if let Some(p) = prob.first_solution() {
        println!("{}", p);
    }

    println!("=== All Solutions ===");
    for p in prob.all_solutions() {
        println!("{}", p);
    }
}
