use std::ops::Range;

use wyhash::wyhash;

const SEED: u64 = 0;

/// This works best on Copy types. It will treat two references to the same
/// thing as different.
pub fn rand<D: Sized, S, T: FromRandom<S>>(input: D, spec: S) -> T {
    let ptr = (&input) as *const D;
    let as_bytes =
        unsafe { std::slice::from_raw_parts(ptr as *const u8, std::mem::size_of::<D>()) };
    let mut hash = wyhash(as_bytes, SEED);
    let mut seeds = vec![0; T::num_seeds()];
    for seed_index in 0..T::num_seeds() {
        seeds[seed_index] = wyhash::wyrng(&mut hash);
    }
    T::generate(spec, &seeds[..])
}

pub trait FromRandom<Spec> {
    fn num_seeds() -> usize;
    fn generate(spec: Spec, seeds: &[u64]) -> Self;
}

impl FromRandom<Range<i8>> for i8 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<i8>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as i8
    }
}

impl FromRandom<Range<i16>> for i16 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<i16>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as i16
    }
}

impl FromRandom<Range<i32>> for i32 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<i32>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as i32
    }
}

impl FromRandom<Range<i64>> for i64 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<i64>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as i64
    }
}

impl FromRandom<Range<u8>> for u8 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<u8>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as u8
    }
}

impl FromRandom<Range<u16>> for u16 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<u16>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as u16
    }
}

impl FromRandom<Range<u32>> for u32 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<u32>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as u32
    }
}

impl FromRandom<Range<u64>> for u64 {
    fn num_seeds() -> usize {
        1
    }

    fn generate(spec: Range<u64>, seeds: &[u64]) -> Self {
        assert!(spec.end > spec.start);
        let range = spec.end - spec.start - 1;
        let seed = seeds[0];
        // ItS nOt EvEnLy DiStRiBuTeD i literally do not care.
        spec.start + (seed % range as u64) as u64
    }
}

impl FromRandom<()> for () {
    fn num_seeds() -> usize {
        0
    }

    fn generate(_spec: (), _seeds: &[u64]) -> Self {
        ()
    }
}

macro_rules! gen_tuple {
    ($($s:ident, $t:ident, $i:tt);*) => {
        impl<$($s,)* $($t: FromRandom<$s>),*> FromRandom<($($s,)*)> for ($($t,)*) {
            fn num_seeds() -> usize {
                $($t::num_seeds() +)* 0
            }

            fn generate(spec: ($($s,)*), seeds: &[u64]) -> Self {
                $(
                    let (seeds_here, seeds) = seeds.split_at($t::num_seeds());
                    debug_assert_eq!(seeds_here.len(), $t::num_seeds());
                    let $s = $t::generate(spec.$i, seeds_here);
                )*
                debug_assert_eq!(seeds.len(), 0);
                ($($s,)*)
            }
        }
    };
}

gen_tuple!(S0, T0, 0);
gen_tuple!(S0, T0, 0; S1, T1, 1);
gen_tuple!(S0, T0, 0; S1, T1, 1; S2, T2, 2);
gen_tuple!(S0, T0, 0; S1, T1, 1; S2, T2, 2; S3, T3, 3);
