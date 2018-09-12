#![feature(specialization)]

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate pyo3;

mod algorithm;
mod data;
mod grbl;

use pyo3::prelude::*;

#[pyfunction]
fn solve(puzzle: data::Cells) -> PyResult<Vec<(usize, usize)>> {
    let edges = algorithm::solve(&puzzle);
    Ok(edges.get_route())
}

#[pymodinit]
fn libsuriza(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(solve))?;
    m.add_class::<grbl::GRBL>()
}
