use core::fmt;
use std::fmt::Display;

mod solver;

#[derive(Debug, Clone)]
struct Prob {
    ns: Vec<u8>,
}

struct Solution {
    state: Option<(Prob, u8)>,
}

impl<'a> Iterator for Solution {
    type Item = Prob;

    // fn solve(&self, from: u8) -> Option<(Prob, u8)> {
    //     let len = self.len();
    //
    //     for i in 0..2 * len as u8 {
    //         let mut new_ns = self.ns.clone();
    //         new_ns[from as usize] = i;
    //         let new_prob = Prob { ns: new_ns };
    //
    //         if new_prob.check_ns() {
    //             return Some((new_prob, from));
    //         }
    //
    //         if from < len as u8 - 1 {
    //             if let Some(p) = new_prob.rec_solve(from + 1) {
    //                 return Some(p);
    //             }
    //         }
    //     }
    //     None
    // }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((ref mut prob, mut counter)) = &mut self.state {
            let len = prob.len();
            let tab = &mut prob.ns;
            let max_val = 2 * len;
            let max_tab = vec![max_val; len as usize];

            while tab != &max_tab {
                // TODO
                // We reach the last digit value...
                if tab[counter as usize] == max_val {
                    // We finished
                    if counter == 0 {
                        return None;
                    } else {
                        // Increment strongest digit
                        while counter > 0 && tab[counter as usize] == max_val {
                            counter -= 1;
                        }
                        tab[counter as usize] += 1;
                        for i in counter + 1..len {
                            tab[i as usize] = 0;
                        }
                        counter = len - 1;
                    }
                } else {
                    tab[counter as usize] += 1;
                }
                if (Prob { ns: tab.clone() }).check_ns() {
                    return Some(prob.clone());
                }
            }
            None
        } else {
            None
        }
        // let len = self.state.0.ns.len();

        // if let Some((p, f)) = &self.state {
        //     let res = p.solve(f.to_owned());
        //     self.state = res
        //         .as_ref()
        //         .map(|(p, f)| (p.clone(), f.to_owned()));
        //     res.map(|(p, _)| p)
        // } else {
        //     None
        // }
    }
}

// impl Solvable<Prob> for Prob {
//     fn first_solution(&self) -> Option<Prob> {
//         self.solve()
//             .map(|(a, _)| a)
//     }
// }

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

    fn all_solutions<'a>(&'a self) -> impl Iterator<Item = Prob> + 'a {
        let sol = Solution {
            state: Some((
                Prob {
                    ns: self.ns.clone(), // }, 0)) // TODO Temp ?
                },
                (self.ns.len() - 1) as u8,
            )),
        };
        sol
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
    let prob_size: u8 = 6;
    let prob = Prob {
        ns: vec![1; prob_size as usize],
    };

    // println!("=== First Solution ===");
    // if let Some(p) = prob.first_solution() {
    //     println!("{}", p);
    // }

    println!("=== All Solutions for problem with {prob_size} variables ===");
    for p in prob.all_solutions() {
        println!("{}", p);
    }
}
