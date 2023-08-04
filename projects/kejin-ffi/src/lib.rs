#![feature(generators)]
#![feature(iter_from_generator)]

use std::iter::from_generator;

/// A vertex in graph
pub trait Hamiltonian
where
    Self: Sized + Clone,
{
    fn count(&self) -> usize;
    fn current(&self) -> usize;
    /// walk to node
    fn go_walk(&mut self, node: usize);
    /// back to node
    fn go_back(&mut self, steps: usize);
    /// all valid moves, should also check if the node is visited
    fn possible_moves(&self, start: usize) -> Vec<usize>;
}

pub fn backtracking<T: Hamiltonian>(graph: T) -> impl Iterator<Item = Vec<usize>> {
    let mut stack: Vec<(Vec<usize>, usize)> = vec![(vec![graph.current()], graph.current())];
    from_generator(move || {
        while let Some((path, current)) = stack.pop() {
            if path.len() == graph.count() {
                yield path;
            }
            else {
                for next in graph.possible_moves(current) {
                    let mut path = path.clone();
                    path.push(next);
                    stack.push((path, next));
                }
            }
        }
    })
}
