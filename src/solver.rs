/// A solvable problem, with one to many solutions
pub trait Solvable<T> {
    fn first_solution(&self) -> Option<T>;

    // fn all_solutions<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>;
}
