# try-collect - a Rust Library to collect into fixed-sized collections

The `Iterator::collect()` method in the Rust standard library doesn't allow to directly collect into an array or other fixed-sized collections. Instead, you need to collect into a `Vec` first and then use `TryInto::try_into()` to try and convert the vector into an array.

This library provides the `TryCollect` trait, an extension trait for iterators, that allows to avoid the additional allocations involved in creating a `Vec`, and instead allows to directly collect into an array. If the iterator does not yield exactly the right number of elements for the target array, an error is returned. Since we don't need to allocate, this works even in `no-std` environments.

## Other solutions to collect into an array

[collect_array][1] – This crate offers a `CollectArrayResult` type that you can collect into using the standard `Iterator::collect()` method. You can extract an array from the result if the iterator yields the right number of items. I think the approach in `try-collect` is more ergonomic since `try_collect()` returns a standard `Result`, so it integrates naturally with Rusts error handling. Otherwise, the functionality is mostly identical.

[`arrayvec::ArrayVec`][3] and [`tinyvec::ArrayVec`][3] – These are two similar array-backed vector implementations. They can be created from an iterator and then return the backing array, but they will panic on overflow.

[1]: https://doc.rust-lang.org/std/collections/linked_list/struct.CursorMut.html
[2]: https://docs.rs/arrayvec/0.7.1/arrayvec/struct.ArrayVec.html
[3]: https://docs.rs/tinyvec/1.5.0/tinyvec/struct.ArrayVec.html
