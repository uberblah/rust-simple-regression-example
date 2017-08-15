#![feature(fn_traits, unboxed_closures)]

#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {

    }
}

#[derive(Clone,Debug)]
pub struct Quad {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Quad {
    pub fn eval(&self, x: f64) -> f64 {
        self.a * x * x + self.b * x + self.c
    }
    pub fn update(&mut self, x: f64, y: f64, r: f64) {
        let e = y - self.eval(x);
        self.a += x * x * e * r;
        self.b += x * e * r;
        self.c += e * r;
    }
}

impl FnOnce<(f64,)> for Quad {
    type Output = f64;
    extern "rust-call" fn call_once(self, x: (f64,)) -> f64 {
        self.eval(x.0)
    }
}

impl FnMut<(f64,)> for Quad {
    extern "rust-call" fn call_mut(&mut self, x: (f64,)) -> f64 {
        self.eval(x.0)
    }
}

impl Fn<(f64,)> for Quad {
    extern "rust-call" fn call(&self, x: (f64,)) -> f64 {
        self.eval(x.0)
    }
}
