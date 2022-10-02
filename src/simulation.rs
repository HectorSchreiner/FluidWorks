use crate::utils::*;

pub fn diffuse(b: i32, x: Vec<f32>, x0: Vec<f32>, diff: f32, dt: f32) {
    let a = dt * diff * ((N - 2) * (N - 2)) as f32;
    lin_solve(b as f32, x, x0, a, 1.0 + 6.0 * a);
}

pub fn lin_solve(b: f32, mut x: Vec<f32>, x0: Vec<f32>, a: f32, c: f32) {
    let c_recip = 1.0 / c;
    for _k in 0..ITER {
        for j in 0..(N - 1) {
            for i in 0..(N - 1) {
                x[to_buffer(i, j)] = (x0[to_buffer(i, j)]
                    + a * (x[to_buffer(i + 1, j)]
                        + x[to_buffer(i - 1, j)]
                        + x[to_buffer(i, j + 1)]
                        + x[to_buffer(i, j - 1)]))
                    * c_recip;
            }
        }
    }
    //set_bnd(b, x, N);
}

pub fn project(mut veloc_x: Vec<f32>, mut veloc_y: Vec<f32>, mut p: Vec<f32>, mut div: Vec<f32>) {
    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            div[to_buffer(i, j)] = -0.5
                * (veloc_x[to_buffer(i + 1, j)] - veloc_x[to_buffer(i - 1, j)]
                    + veloc_y[to_buffer(i, j + 1)]
                    - veloc_y[to_buffer(i, j - 1)])
                / N as f32;
            p[to_buffer(i, j)] = 0 as f32;
        }
    }
    //set_bnd(0, div);
    //set_bnd(0, p);
    lin_solve(0.0, p.to_vec(), div, 1.0, 6.0);

    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            veloc_x[to_buffer(i, j)] -=
                0.5 * (p[to_buffer(i + 1, j)] - p[to_buffer(i - 1, j)]) * (N as f32);
            veloc_y[to_buffer(i, j)] -=
                0.5 * (p[to_buffer(i, j + 1)] - p[to_buffer(i, j - 1)]) * (N as f32);
        }
    }
    //set_bnd(1, velocX);
    //set_bnd(2, velocY);
}
