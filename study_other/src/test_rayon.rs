#[cfg(test)]
mod rayon_tests {

    use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

    #[test]
    fn it_works() {
        let mut arr = [0, 7, 9, 11];
        arr.par_iter_mut().for_each(|p| *p -= 1);
        println!("{:?}", arr);
    }
}
