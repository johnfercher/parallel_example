extern crate rand;
extern crate rayon;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;

fn main() {
    let colecao_aleatoria: [u16; 32] = rand::random();

    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    println!("{:?}", colecao_aleatoria);

    let colecao_de_pares = thread_pool.install(|| {
        colecao_aleatoria
            .to_vec()
            .into_par_iter()
            .filter(|x| x % 2 == 0)
            .collect::<Vec<u16>>()
    });

    println!("{:?}", colecao_de_pares);
}