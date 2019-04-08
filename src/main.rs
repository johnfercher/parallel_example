extern crate rand;
extern crate rayon;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let colecao_aleatoria: [u16; 32] = rand::random();

    let colecao_de_pares = colecao_aleatoria
        .to_vec()
        .into_par_iter()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<u16>>();

    println!("{:?}", colecao_aleatoria);
    println!("{:?}", colecao_de_pares);
}