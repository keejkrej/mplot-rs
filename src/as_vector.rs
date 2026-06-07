use num_traits::{cast, Num, NumCast};

pub trait AsVector<'a, U: 'a> {
    fn vec_size(&self) -> usize;
    fn vec_at(&self, i: usize) -> U;
}

impl<'a, U: Copy> AsVector<'a, U> for &'a [U] {
    fn vec_size(&self) -> usize {
        self.len()
    }

    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

impl<'a, U: Copy, const N: usize> AsVector<'a, U> for &'a [U; N] {
    fn vec_size(&self) -> usize {
        N
    }

    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

impl<'a, U: Copy + 'a, const N: usize> AsVector<'a, U> for [U; N] {
    fn vec_size(&self) -> usize {
        N
    }

    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

impl<'a, U: Copy + 'a> AsVector<'a, U> for Vec<U> {
    fn vec_size(&self) -> usize {
        self.len()
    }

    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

pub fn vector_to_f64<'a, T, U>(data: &'a T) -> Vec<f64>
where
    T: AsVector<'a, U>,
    U: 'a + Num + NumCast + Copy,
{
    (0..data.vec_size())
        .map(|i| cast(data.vec_at(i)).unwrap_or(0.0))
        .collect()
}
