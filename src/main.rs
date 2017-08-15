extern crate rustreg;
extern crate rand;

use rand::Rng;
use rustreg::Quad;

fn quad(a: f64, b: f64, c: f64) -> Box<Quad> {
    Box::new(Quad { a: a, b: b, c: c })
}

fn make_noise() -> Box<FnMut() -> f64> {
    let mut rng = rand::thread_rng();
    Box::new(move || (rng.next_f64() - 0.5) * 2.0)
}

fn mse(h: Box<Fn(f64) -> f64>, data: &Vec<(f64, f64)>) -> f64 {
    data.iter()
        .cloned()
        .map(|p| f64::powf(h(p.0) - p.1, 2.0))
        .sum::<f64>() / (data.len() as f64)
}

fn main() {
    let rate = 0.0001;
    let iter_n = 100000;
    let data_n = 100000;
    let x_scale = 2.0;
    let m_noise = 0.5;
    let mut noise = make_noise();

    // make a data generator
    let eqn = quad(2.0, -100.0, 100.0);
    let mut data_func = move |x: f64| eqn(x) + noise() * m_noise;
    let data = (1..data_n)
        .map(|x: u64| ((x as f64) / (data_n as f64)) * x_scale)
        .map(move |x: f64| (x, data_func(x)))
        .collect::<Vec<(f64,f64,)>>();

    let mut hy = quad(0.0, 0.0, 0.0);
    let mut e = mse(hy.clone(), &data);
    for i in 0..iter_n {
        let ne = mse(hy.clone(), &data);
        if ne > e || (e == ne && i != 0) {
            // if we've converged, stop regressing
            break
        }

        let dir = if ne > e {"+"} else {"-"};
        println!("{} {} MSE {}", dir, i, ne);
        e = ne;

        for datum in data.iter() {
            hy.update(datum.0, datum.1, rate);
        }
    }

    println!("{:?}", hy);
}

#[test]
// the noise should return values between -1 and 1, and with a mean of 0
fn noise_properties() {
    let n = 100000;
    let mut noise = make_noise();

    let noise_vec: Vec<f64> = (0..n).map(|_| noise()).collect();
    let (min, max, sum) = noise_vec
        .iter()
        .cloned()
        .fold((noise_vec[0], noise_vec[0], 0.0),
              |x, y| (f64::min(x.0, y), f64::max(x.1, y), x.2 + y));

    assert!(min < 0.01);
    assert!(max > 0.99);
    assert!((sum / (n as f64)).abs() < 0.01);
}

#[test]
// the mse function should return the correct mse for a given example
fn test_mse() {
    let n = 10000;
    let q = quad(1.0, 2.0, 3.0);
    let data = (0..n)
        .map(|x| x as f64)
        .map(|x| (x, q(x)))
        .collect::<Vec<(f64, f64)>>();
    let correct_mse = data.iter().cloned().map(|p| p.1 * p.1).sum::<f64>() / (data.len() as f64);
    let h = |_| 0.0;

    assert!(mse(Box::new(h), &data) == correct_mse);
}
