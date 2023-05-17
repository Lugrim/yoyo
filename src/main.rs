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

impl Prob {
    fn actual_counts(&self) -> Vec<u8> {
        let mut res = vec![0; 10];

        for i in 0..self.ns.len() as u8 {
            let mut c = 0;
            for j in 0..self.ns.len() {
                if self.ns[j] == i {
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

    fn solve(&self, from: u8) -> Option<Prob> {
        eprint!("{:?}\r", self.ns);
        let len = self.ns.len();

        if self.check_ns() {
            Some(Prob {
                ns: self.ns.clone()
            })
        } else {
            for i in 0..len as u8 {
                let mut new_ns = self.ns.clone();
                new_ns[from as usize] = i;
                let new_prob = Prob {
                        ns: new_ns,
                    };
                if from < len as u8 - 1 {
                    if let Some(p) = new_prob.solve(from + 1) {
                            return Some(p)
                    }
                }
            }
            None
        }
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
    let prob = Prob { ns: vec![0; 10] };

    println!("{:?}", prob.solve(0));
}
