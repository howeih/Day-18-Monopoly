extern crate gnuplot;
use gnuplot::{Caption, Color, Figure};

fn probability(n: i32) -> Vec<f64> {
    // initial probabilities
    let mut p = vec![0., 0., 0., 0., 0., 1.];
    for _ in 0..n {
        let prob = {
            let (_, right) = p.split_at(p.len() - 6);
            let sum: f64 = right.iter().sum();
            sum / 6.
        };
        p.push(prob);
    }
    p.split_at(6).1.to_vec()
}

fn main() {
    let x = (0..25).collect::<Vec<i32>>();
    let y = probability(24);
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")]);
    fg.show();
}
