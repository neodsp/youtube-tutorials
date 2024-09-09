use ndarray::prelude::*;

fn main() {
    let mut array = Array3::<f32>::zeros((2, 2, 2));
    array[[0, 0, 0]] = 1.0;
    array[[1, 1, 1]] = 1.0;

    println!("{:?}", array);
}
