#![feature(const_generics)]

use core::mem::{align_of, size_of};

#[doc(hidden)]
pub enum Size<const SIZE: usize> {}

#[doc(hidden)]

pub enum Align<const ALIGN: usize> {}

#[doc(hidden)]
pub unsafe trait CL {
    type Size;
    type Align;
}

#[doc(hidden)]
unsafe impl<T> CL for T {
    type Size = Size<{ size_of::<T>() }>;
    type Align = Align<{ align_of::<T>() }>;
}

/// # EqSize
/// Implemented for types with same `mem::size_of` value
/// 
/// 
/// 
/// ```rust
/// use core::mem::size_of;
/// 
/// use const_layout::EqSize;
/// 
/// fn must_have_same_size<I: EqSize<O>, O>(input: I) -> O {
///     assert_eq!(size_of::<I>(), size_of::<O>());
///     unimplemented!()
/// }
/// 
/// ```
pub unsafe trait EqSize<Rhs> {}

#[doc(hidden)]
unsafe impl<X, Y, const S: usize> EqSize<Y> for X
where
    X: CL<Size = Size<S>>,
    Y: CL<Size = Size<S>>,
{
}

/// # EqAlign
/// Implemented for types with same `mem::align_of` value
/// 
/// 
/// 
/// ```rust
/// use core::mem::align_of;
/// 
/// use const_layout::EqAlign;
/// 
/// fn must_have_same_align<I: EqAlign<O>, O>(input: I) -> O {
///     assert_eq!(align_of::<I>(), align_of::<O>());
///     unimplemented!()
/// }
/// 
pub unsafe trait EqAlign<Rhs> {}

#[doc(hidden)]
unsafe impl<X, Y, const A: usize> EqAlign<Y> for X
where
    X: CL<Align = Align<A>>,
    Y: CL<Align = Align<A>>,
{
}

/// # EqLayout
/// Implemented for types with same `mem::size_of` and `mem::align_of` value
/// 
/// 
/// 
/// ```rust
/// use core::alloc::Layout;
/// 
/// use const_layout::EqLayout;
/// 
/// fn must_have_same_layout<A: EqLayout<B>, B>(a: A, b: B) {
///     assert_eq!(Layout::new::<A>(), Layout::new::<B>());
/// }
/// 
pub unsafe trait EqLayout<Rhs>: EqSize<Rhs> + EqAlign<Rhs> {}

#[doc(hidden)]
unsafe impl<X, Y> EqLayout<Y> for X where X: EqSize<Y> + EqAlign<Y> {}

#[cfg(test)]
mod tests {
    use super::EqSize;
    fn same_size<X: EqSize<Y>, Y>(_x: X, _y: Y) {}
    #[test]
    fn it_works() {
        same_size(1u16, 1i16);
    }
}
