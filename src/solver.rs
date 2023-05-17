/// A solvable problem, with one to many solutions
pub trait Solvable {

}

pub trait Solver<T> {
    fn first_solution() -> T;

    fn all_solutions() -> [T];
}
