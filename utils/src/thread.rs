use rayon::iter::{Chunks, Enumerate};
use rayon::prelude::*;
use rayon::vec::IntoIter;
use std::thread::available_parallelism;

#[inline]
pub fn activate_all_threads<T: Sync + Send>(
    items: Vec<T>,
    opt_chunk_size: Option<usize>,
) -> Enumerate<Chunks<IntoIter<T>>> {
    let threads = available_parallelism().unwrap().get();
    let chunk_size = opt_chunk_size.unwrap_or(items.len().div_ceil(threads));
    items.into_par_iter().chunks(chunk_size).enumerate()
}
