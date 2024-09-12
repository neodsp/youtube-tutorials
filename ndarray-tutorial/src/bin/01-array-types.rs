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

    // Array0 is a 0-dimensional array, what means it represents a single number (e.g. 1.0)
    // in this case we create the shape from a unit type ()
    let a = Array0::<f32>::zeros(());
    // you can use the type alias Array0 or create an Array and set the dimensionality with Ix0
    let c = Array::<f32, Ix0>::from_elem((), 1.0);
    // there is also a handy shortcut to create an array
    let b = arr0(1.0);
    dbg!(a, b, c);

    // Array1 is 1-dimensional, so it behaves like a usual vector
    // here we can create a shape object from a single number, to determine the length of the array
    let a = Array1::<f32>::zeros(10);
    let b = Array::<f32, Ix1>::from_shape_fn(100, |i| i as f32);
    let c = arr1(&[1, 2, 3]);
    dbg!(a, b, c);

    // Array2 is a matrix with 2 dimensions: rows and columns
    // we can create a shape object from a tuple (x, y) or an array [x, y]
    let a = Array2::<f32>::zeros((10, 10));
    let b = Array::<f32, Ix2>::from_shape_fn((10, 10), |(i, j)| i as f32 + j as f32);
    // in the shortcut arr2 the data needs to be layed out as it should be in the matrix
    let c = arr2(&[[1, 2, 3], [4, 5, 6]]);
    // but you can create the array as well from a straight vector
    let d = Array2::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
    dbg!(a, b, c, d);

    // ... this goes until Array6, a 6D Matrix
    let a = Array6::<f32>::zeros([2; 6]);
    dbg!(a);

    // and as a final dimension there is even an ArrayD type
    // this is dynamic in dimensionality: IxDyn
    // so the dimension must not be known at compiletime
    // and can change during runtime.
    // Only use this type if you absolutely need it,
    // because performance and clarity in the code will be worse!
    // Example: loading an Array from a .npy file and you don't know which dimensionality it has

    // you have to create the shape wrapped in IxDyn!
    let a = ArrayD::<f32>::zeros(IxDyn(&[2, 2, 2, 2]));
    dbg!(a);

    // ArrayView
    // - does not own the data
    // - cheap to clone
    // - data is immutable
    let a = Array3::<f32>::zeros([2, 5, 10]);
    // here we create a view of the data, which is very cheap
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
