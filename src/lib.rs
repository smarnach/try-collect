#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

pub trait TryFromIterator<A>: Sized {
    type Error;

    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator<Item = A>;
}

#[derive(Copy, Clone, Debug)]
pub struct NonMatchingLenError;

impl fmt::Display for NonMatchingLenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iterator length and array length do not match")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NonMatchingLenError {}

impl<A, const N: usize> TryFromIterator<A> for [A; N] {
    type Error = NonMatchingLenError;
    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator<Item = A>,
    {
        let mut partial = partial_array::PartialArray::<A, N>::new();
        for val in iter {
            if partial.full() {
                return Err(NonMatchingLenError);
            }
            partial.push(val);
        }
        if !partial.full() {
            return Err(NonMatchingLenError);
        }
        Ok(partial.into_array())
    }
}

pub trait IteratorExt: Iterator {
    fn try_collect<B>(self) -> Result<B, B::Error>
    where
        B: TryFromIterator<Self::Item>,
        Self: Sized,
    {
        TryFromIterator::try_from_iter(self)
    }
}

impl<I: Iterator> IteratorExt for I {}

mod partial_array {
    use core::mem::MaybeUninit;

    pub struct PartialArray<A, const N: usize> {
        array: [MaybeUninit<A>; N],
        len: usize,
    }

    impl<A, const N: usize> PartialArray<A, N> {
        pub fn new() -> Self {
            Self {
                // assume_init() is safe here, since the value we are assuming to be initialized
                // is an array of `MaybeUninit`s. This can be replaced with uninit_array() once
                // it is stabilized.
                array: unsafe { MaybeUninit::uninit().assume_init() },
                len: 0,
            }
        }

        pub fn push(&mut self, val: A) {
            assert!(self.len < N, "PartialArray already full.");
            self.array[self.len].write(val);
            self.len += 1;
        }

        pub fn full(&self) -> bool {
            self.len == N
        }

        pub fn into_array(self) -> [A; N] {
            assert!(self.len == N, "PartialArray not yet fully initialized.");
            // Converting to an array is safe since we initialized all values.
            // We can't transmute const generic arrays, so we have to convert pointers.
            // We can't use array_assume_int() because it is unstable.
            unsafe { (&self.array as *const _ as *const [A; N]).read() }
        }
    }

    impl<A, const N: usize> Drop for PartialArray<A, N> {
        fn drop(&mut self) {
            for i in 0..self.len {
                unsafe {
                    // We can't use `assume_init()`, since we don't have ownership of the values.
                    // We can't use `assume_init_drop()`, since it's unstable.
                    core::ptr::drop_in_place(self.array[i].as_mut_ptr());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NonMatchingLenError;

    use super::IteratorExt;

    fn try_collect_vec<const N: usize>() -> Result<[i32; N], NonMatchingLenError> {
        IntoIterator::into_iter([1, 2, 3]).try_collect()
    }

    #[test]
    fn try_collect_array() {
        assert_eq!(try_collect_vec::<3>().unwrap(), [1, 2, 3]);
    }

    #[test]
    fn try_collect_array_too_short() {
        assert!(try_collect_vec::<2>().is_err());
    }

    #[test]
    fn try_collect_array_too_long() {
        assert!(try_collect_vec::<4>().is_err());
    }
}
