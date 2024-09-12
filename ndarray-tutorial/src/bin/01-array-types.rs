// 01: Array Types
// Here we will cover the basic types of ndarray and how to create arrays with it.
// This should give you a quick overview so you feel more comfortable when seeing
// the different types in the wild!

// include this to get access to most ndarray types
use ndarray::prelude::*;

fn main() {
    // ArrayBase
    // - base type
    // - we nearly never have to touch this
    // - we mostly use type aliases
    // - only usage would be for generic functions that should accept all types of arrays
    let a = ArrayBase::<ndarray::OwnedRepr<f32>, Ix1>::zeros(10);
    let b = ArrayBase::<ndarray::ViewRepr<&f32>, Ix1>::from_shape(2, &[1.0, 2.0]).unwrap();
    let mut data = vec![1.0, 2.0];
    let c = ArrayBase::<ndarray::ViewRepr<&mut f32>, Ix1>::from_shape(2, &mut data).unwrap();
    dbg!(a, b, c);

    // Types that you will actually use

    // Array
    // - owns the data
    // - expensive to clone
    let a = Array0::<f32>::zeros(());
    let b = arr0(1.0);
    let c = Array0::from_elem((), 1.0);
    dbg!(a, b, c);

    let a = Array1::<f32>::zeros(10);
    let b = Array1::<f32>::from_shape_fn(100, |i| i as f32);
    let c = arr1(&[1, 2, 3]);
    dbg!(a, b, c);

    let a = Array2::<f32>::zeros([10, 10]);
    let b = Array2::<f32>::from_shape_fn((10, 10), |(i, j)| i as f32 + j as f32);
    let c = arr2(&[[1, 2, 3], [4, 5, 6]]);
    dbg!(a, b, c);

    // ArrayView
    // - does not own the data
    // - cheap to clone
    // - data is immutable
    let a = Array2::<f32>::zeros([10, 10]);
    let a = a.view();
    let data = vec![0.0, 1.0, 2.0, 3.0];
    let b = ArrayView1::from_shape(4, &data).unwrap();
    let c = aview1(&data);
    dbg!(a, b, c);

    // ArrayViewMut
    // - does not own the data
    // - cheap to clone
    // - data is mutable
    let mut a = Array2::<f32>::zeros([10, 10]);
    let a = a.view_mut();
    let mut data = vec![0.0, 1.0, 2.0, 3.0];
    let b = ArrayViewMut1::from_shape(4, &mut data).unwrap();
    let mut data = vec![0.0, 1.0, 2.0, 3.0];
    let c = aview_mut1(&mut data);
    dbg!(a, b, c);

    // Two more special types: CowArray and ArcArray
    // not covered here, but as small info :
    // both can own the data or borrow the data
    // if data is borrowed and you mutate something, the data will be cloned
    // cow = clone on write
    // arc = atomic reference counted (for multi threaded sharing)
}
