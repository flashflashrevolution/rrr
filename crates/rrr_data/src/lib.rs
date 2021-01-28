#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[bench]
    fn bench_it_works(b: &mut Bencher) {
        b.iter(|| 1 + 1)
    }
}
