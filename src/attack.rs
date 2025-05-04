#![allow(unused)]

// fn pi(x: &u16) -> u16 {
//     let mut x_t = 0;
//     for j in 0..4 {
//         let tmp = (x >> (4 * j)) & 0xf;
//         for i in 0..4 {
//             x_t |= ((tmp >> i) & 1) << (i * 4 + j);
//         }
//     }
//     x_t
// }

fn pi(x: &u16) -> u16 {
    let mut x_t = 0;
    for j in 0..4 {
        let tmp = (x >> (4 * j)) & 0xf;
        for i in 0..4 {
            x_t |= ((tmp >> i) & 1) << (i * 4 + j);
        }
    }
    x_t
}

pub fn calc_dps(s: &[u16]) -> Vec<Vec<f64>> {
    let mut res = vec![vec![0.0; 16]; 16];
    // println!("{:?}", res);
    for a in 0..16 {
        for b in 0..16 {
            for x in 0..16 {
                // println!("{a}, {b}, {x}");
                res[a][b] += (s[x ^ a] == s[x] ^ (b as u16)) as u16 as f64;
            }
            res[a][b] /= 16.0;
        }
    }

    res
}

// DP^f(a,b) = prod_i DP^s(a_i,b_i)
fn calc_dpf(a: &u16, b: &u16, dps: &[Vec<f64>]) -> f64 {
    let mut res: f64 = 1.0;
    let b = pi(b);
    for i in 0..4 {
        res *= dps[((a >> (i * 4)) & 0xf) as usize][((b >> (4 * i)) & 0xf) as usize];
    }

    res
}

// a: { (b, DP^{f}(a,b)) | b = 0,2^16 - 1 (0xffff) }
fn get_dpf(a: &u16, dps: &[Vec<f64>]) -> Vec<(u16, f64)> {
    let mut l_1: Vec<(u16, f64)> = Vec::new();
    for b in 1..=65535 {
        let dp = calc_dpf(a, &b, dps);
        if dp != 0.0 {
            l_1.push((b, dp));
        }
    }

    l_1
}

pub fn dif_search(a: u16, p_star: f64, s: &[u16]) -> Vec<(u16, f64)> {
    let mut l_0 = vec![(a, 1.0)];
    let dps = &calc_dps(s);
    for i in 1..=5 {
        let mut l_1: Vec<(u16, f64)> = Vec::new();
        // println!("itr: {i}");
        for (b, p) in &l_0 {
            let dpf = get_dpf(b, dps);

            for (gamma, q) in dpf {
                if let Some((_, pr)) = l_1.iter_mut().find(|(x, _)| *x == gamma) {
                    *pr += p * q;
                    // println!("finded");
                } else {
                    l_1.push((gamma, p * q));
                }
            }
        }

        // println!("done finding");

        l_1.retain(|(_, p)| *p >= p_star);
        l_0 = l_1;
    }

    l_0
}
