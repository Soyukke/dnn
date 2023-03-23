use dnn::dnn_test::*;
use linear_algebra::{Matrix, Vector};



fn main() {
    hoge();
    let l = Dense{
        w: Matrix::rand([3, 3]),
        b: Vector::rand([3]),
    };
    println!("dense: {:?}", l);
    let x = Vector::rand([3]);
    let x = l.forward(x);
    println!("forward: {}", x);
}
