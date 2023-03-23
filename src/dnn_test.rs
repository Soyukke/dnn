use linear_algebra::{Matrix, Vector};

pub fn hoge() {
    let x = Matrix::<f32>::rand([3, 3]);
    println!("x: {}", x);
}

pub trait Forward {
    type Input;
    type Output;
    fn forward(&self, x: Self::Input) -> Self::Output;
}

pub trait Backward {
    type Input;
    type Output;
    fn backward(&self, x: Self::Input) -> Self::Output;
}


#[derive(Debug)]
pub struct Dense {
    pub w: Matrix<f32>,
    pub b: Vector<f32>,
}

impl Forward for Dense {
    type Input = Vector<f32>;
    type Output = Vector<f32>;
    fn forward(&self, x: Self::Input) -> Self::Output {
        self.w.clone() * x + self.b.clone()
    }
}
