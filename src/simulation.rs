use crate::utils::*;

pub fn diffuse(
    b: i32,
    x: &mut Box<[f32; N * N]>,
    x0: &mut Box<[f32; N * N]>,
    diffusion: f32,
    dt: &mut f32,
) {
    let a = *dt * diffusion * ((N - 2) * (N - 2)) as f32;
    lin_solve(b as f32, x, x0, a, 1.0 + 6.0 * a);
}

pub fn lin_solve(b: f32, x: &mut Box<[f32; N * N]>, x0: &mut Box<[f32; N * N]>, a: f32, c: f32) {
    let c_recip = 1.0 / c;
    for _k in 1..ITER {
        for j in 1..(N - 1) {
            for i in 1..(N - 1) {
                x[to_buffer(i, j)] = (x0[to_buffer(i, j)]
                    + a * (x[to_buffer(i + 1, j)]
                        + x[to_buffer(i - 1, j)]
                        + x[to_buffer(i, j + 1)]
                        + x[to_buffer(i, j - 1)]))
                    * c_recip;
            }
        }
    }
    set_bnd(b as i32, x);
}

pub fn project(
    velocity_x: &mut Box<[f32; N * N]>,
    velocity_y: &mut Box<[f32; N * N]>,
    p: &mut Box<[f32; N * N]>,
    div: &mut Box<[f32; N * N]>,
) {
    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            div[to_buffer(i, j)] = -0.5
                * (velocity_x[to_buffer(i + 1, j)] - velocity_x[to_buffer(i - 1, j)]
                    + velocity_y[to_buffer(i, j + 1)]
                    - velocity_y[to_buffer(i, j - 1)])
                / N as f32;
            p[to_buffer(i, j)] = 0.0;
        }
    }
    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0.0, p, div, 1.0, 6.0);

    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            velocity_x[to_buffer(i, j)] -=
                0.5 * (p[to_buffer(i + 1, j)] - p[to_buffer(i - 1, j)]) * (N as f32);
            velocity_y[to_buffer(i, j)] -=
                0.5 * (p[to_buffer(i, j + 1)] - p[to_buffer(i, j - 1)]) * (N as f32);
        }
    }
    set_bnd(1, velocity_y);
    set_bnd(2, velocity_y);
}

pub fn advect(
    b: i32,
    d: &mut Box<[f32; N * N]>,
    d0: &Box<[f32; N * N]>,
    velocity_x: &Box<[f32; N * N]>,
    velocity_y: &Box<[f32; N * N]>,
    dt: &mut f32,
) {
    let mut i0;
    let mut i1;
    let mut j0;
    let mut j1;

    let dtx = *dt * (N - 2) as f32;
    let dty = *dt * (N - 2) as f32;

    let mut s0;
    let mut s1;
    let mut t0;
    let mut t1;
    let mut tmp1;
    let mut tmp2;
    let mut x: f32;
    let mut y: f32;

    let nfloat: f32 = N as f32;
    let mut ifloat: f32;
    let mut jfloat: f32;

    for j in 0..(N - 1) {
        for i in 0..(N - 1) {
            jfloat = j as f32;
            ifloat = i as f32;
            tmp1 = dtx * velocity_x[to_buffer(i, j)];
            tmp2 = dty * velocity_y[to_buffer(i, j)];
            x = ifloat - tmp1;
            y = jfloat - tmp2;

            if x < 0.5 {
                x = 0.5;
            }
            if x > nfloat + 0.5 {
                x = nfloat + 0.5;
            }
            i0 = x.floor();
            i1 = i0 + 1.0;
            if y < 0.5 {
                y = 0.5;
            }
            if y > nfloat + 0.5 {
                y = nfloat + 0.5;
            }
            j0 = y.floor();
            j1 = j0 + 1.0;

            s1 = x - i0;
            s0 = 1.0 - s1;
            t1 = y - j0;
            t0 = 1.0 - t1;

            if let Some(_) = [i0, j0, i1, j1].iter().find(|&&v| v >= nfloat) {
                continue;
            }

            d[to_buffer(i, j)] = s0
                * (t0 * d0[to_buffer(i0 as usize, j0 as usize)]
                    + t1 * d0[to_buffer(i0 as usize, j1 as usize)])
                + s1 * (t0 * d0[to_buffer(i1 as usize, j0 as usize)]
                    + t1 * d0[to_buffer(i1 as usize, j1 as usize)]);
        }
    }
    set_bnd(b, d);
}

pub fn set_bnd(b: i32, x: &mut Box<[f32; N * N]>) {
    for j in 1..N - 1 {
        for i in 1..N - 1 {
            if b == 3 {
                x[to_buffer(i, j)] = -x[to_buffer(i, j)];
                x[to_buffer(i, j)] = -x[to_buffer(i, j)]
            } else {
                x[to_buffer(i, j)] = x[to_buffer(i, j)];
                x[to_buffer(i, j)] = x[to_buffer(i, j)];
            }
        }
    }
    for _k in 1..N - 1 {
        for i in 1..N - 1 {
            if b == 2 {
                x[to_buffer(i, 0)] = -x[to_buffer(i, 1)];
                x[to_buffer(i, N - 1)] = -x[to_buffer(i, N - 2)];
            } else {
                x[to_buffer(i, 0)] = x[to_buffer(i, 1)];
                x[to_buffer(i, N - 1)] = x[to_buffer(i, N - 2)];
            }
        }
    }

    for _k in 1..N - 1 {
        for j in 1..N - 1 {
            if b == 1 {
                x[to_buffer(0, j)] = -x[to_buffer(1, j)];
                x[to_buffer(N - 1, j)] = -x[to_buffer(N - 2, j)];
            } else {
                x[to_buffer(0, j)] = x[to_buffer(1, j)];
                x[to_buffer(N - 1, j)] = x[to_buffer(N - 2, j)];
            }
        }
    }

    x[to_buffer(0, 0)] = 0.33 * (x[to_buffer(1, 0)] + x[to_buffer(0, 1)] + x[to_buffer(0, 0)]);
    x[to_buffer(0, N - 1)] =
        0.33 * (x[to_buffer(1, N - 1)] + x[to_buffer(0, N - 2)] + x[to_buffer(0, N - 1)]);
    x[to_buffer(0, 0)] = 0.33 * (x[to_buffer(1, 0)] + x[to_buffer(0, 1)] + x[to_buffer(0, 0)]);
    x[to_buffer(0, N - 1)] =
        0.33 * (x[to_buffer(1, N - 1)] + x[to_buffer(0, N - 2)] + x[to_buffer(0, N - 1)]);
    x[to_buffer(N - 1, 0)] =
        0.33 * (x[to_buffer(N - 2, 0)] + x[to_buffer(N - 1, 1)] + x[to_buffer(N - 1, 0)]);
    x[to_buffer(N - 1, N - 1)] = 0.33
        * (x[to_buffer(N - 2, N - 1)] + x[to_buffer(N - 1, N - 2)] + x[to_buffer(N - 1, N - 1)]);
    x[to_buffer(N - 1, 0)] =
        0.33 * (x[to_buffer(N - 2, 0)] + x[to_buffer(N - 1, 1)] + x[to_buffer(N - 1, 0)]);
    x[to_buffer(N - 1, N - 1)] = 0.33
        * (x[to_buffer(N - 2, N - 1)] + x[to_buffer(N - 1, N - 2)] + x[to_buffer(N - 1, N - 1)]);
}
