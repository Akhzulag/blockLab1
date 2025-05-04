#![allow(unused)]

mod attack;
mod heys;

use heys::pi;
use heys::s_layer;

use rayon::{prelude::*, range};

fn brut_dif(i: usize) {
    let s: [u16; 16] = [0xF, 6, 5, 8, 0xE, 0xB, 0xA, 4, 0xC, 0, 3, 7, 2, 9, 1, 0xD];

    (1u16..=0xF).into_par_iter().for_each(|a_i| {
        let a: u16 = a_i << (i * 4);
        let x = attack::dif_search(a, 0.0005, &s);

        for (b, p) in &x {
            println!("a = {a:>5}, {a:016b}: b --- {b:016b} --- {b:>5} --- {p:>1.5}");
        }
    });
}

use std::collections::btree_set::Range;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::usize;

fn write_all_2byte_combinations(filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for value in 0u16..=u16::MAX {
        writer.write_all(&value.to_le_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

fn read_u16_pairs(filename: &str) -> io::Result<Vec<u16>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 2];

    let mut numbers: Vec<u16> = Vec::new();
    while reader.read_exact(&mut buffer).is_ok() {
        let number = u16::from_le_bytes(buffer);
        numbers.push(number);
    }

    Ok(numbers)
}

use rand::Rng;

use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::sync::Mutex;

const s_rev: [u16; 16] = [9, 14, 12, 10, 7, 2, 1, 11, 3, 13, 6, 5, 8, 15, 4, 0];

fn write_key(filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::rng();

    for _ in 1..=14 {
        let value: u8 = rng.random_range(1..=0xff);
        writer.write_all(&value.to_be_bytes());
    }

    writer.flush()?;
    Ok(())
}

fn last_round_attack(a: u16, b: u16, n: usize, cyphers: &Vec<u16>) -> Vec<(u16, u32)> {
    let mut stats: Vec<(u16, u32)> = Vec::new();

    let mut rng = rand::rng();

    let mut x: Vec<_> = (0..=0xffff).collect(); // створюємо вектор з діапазону

    x.shuffle(&mut rng); // тепер можна перемішувати

    // for i in x.iter_mut() {
    //     let x = rng.random_range(0..=0xffff);
    // }
    let x = &x[0..n];
    // pr/* intln!("{:?}", x); */
    let stats = Mutex::new(Vec::new());

    (0..=0xffffu16).into_par_iter().for_each(|k_r| {
        let mut r: u32 = 0;
        for &x1 in x {
            let x2 = x1 ^ a;
            let c1 = cyphers[x1 as usize] ^ k_r;
            let c2 = cyphers[x2 as usize] ^ k_r;
            let y1 = s_layer(&s_rev, &pi(&c1));
            let y2 = s_layer(&s_rev, &pi(&c2));
            r += (y1 == (y2 ^ b)) as u32;
        }
        // println!("{k_r}");
        if r > 3 {
            let mut stats_lock = stats.lock().unwrap();
            stats_lock.push((k_r, r));
            // println!("{k_r}");
        }
    });

    // println!("{:?}", stats.into_inner().unwrap());
    // stats.get_mut(
    // println!("{:?}", stats.len());
    stats.into_inner().unwrap()
}

use rand::rngs::StdRng;
use rand::SeedableRng;

fn write_value(filename: &str, x: &u16) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(&x.to_be_bytes())?;

    writer.flush()?;
    Ok(())
}

const S: [u16; 16] = [0xF, 6, 5, 8, 0xE, 0xB, 0xA, 4, 0xC, 0, 3, 7, 2, 9, 1, 0xD];
fn main() {
    // write_all_2byte_combinations("alltexts.bin");

    // let cyphers = read_u16_pairs("alltexts.bin").unwrap();

    // println!("{:?}", cyphers);
    // let mut rng = StdRng::seed_from_u64(32);

    // println!("{:#?}", attack::calc_dps(&S));
    // for i in 0..=0xf {
    //     print!("{i:x} &")
    // }
    // println!(" ");
    // for i in attack::calc_dps(&S) {
    //     for j in i {
    //         print!("{j:>1.3} & ")
    //     }
    //     println!(" ")
    // }
    //
    let mut rng = rand::rng();

    let n = 20000;
    // let cyphers = read_u16_pairs("resvar3key1.bin").unwrap();
    let cyphers = read_u16_pairs("resdefkey.bin").unwrap();

    // for _ in 0..100 {
    //     let mut x: Vec<_> = (0..=0xffff).collect(); // створюємо вектор з діапазону
    //
    //     x.shuffle(&mut rng);
    //     let x = &x[0..n];
    //     // println!("heys {:?}", heys(&x[0]));
    //     println!("cypher {:?}", cyphers[x[0] as usize]);
    //     let a = 20480;
    //     let b: u16 = 17476;
    //     let k_r = 0x3699;
    //     // let k_r = 35385;
    //     let y = find_stat(a, b, k_r, x, &cyphers);
    //     println!("{:?}", y);
    // }

    // let y = find_stat(a, b, k_r, x, &cyphers);
    // println!("{:?}", y);
    //
    // let y = find_stat(a, b, k_r, x, &cyphers);
    // println!("{:?}", y);
    let a = 20480;
    let b: u16 = 17476;

    // last_round_attack(a, b, 10000, &cyphers);
    println!("{a:016b}");
    println!("{b:016b}");
    let x = last_round_attack(a, b, 10000, &cyphers);

    for (key, r) in x {
        print!("$({key:x}, {r})$, ");
    }
    println!(" ");
    let a = 1280;
    let b = 17476;

    println!("{a:016b}");
    println!("{b:016b}");
    let x = last_round_attack(a, b, 10000, &cyphers);
    for (key, r) in x {
        print!("$({key:x}, {r})$, ");
    }
    println!(" ");

    //
    // // let x = attack::dif_search(a, 0.0004, &s);
    // // println!("{:?}", x);
    // // for (a, p) in &x {
    // //     println!("a = {a:>5}: b --- {a:016b} --- {a:>5} --- {p:>1.5}");
    // // }
    // println!("i = 0");
    // brut_dif(0);
    // println!(" ");
    // println!(" ");
    // println!(" ");
    // println!(" ");
    // println!("i = 1");
    // brut_dif(1);
    //    / Mutex { data: [(47561, 11)], poisoned: false, .. }
    // println!(" ");
    // println!(" ");
    // println!(" ");
    // println!(" ");
    // println!("i = 2");
    // brut_dif(2);
    //
    // println!(" ");
    // println!(" ");
    // println!(" ");
    // println!(" ");
    // println!("i = 3");
    // brut_dif(3);

    // println!("{:?}", dps);

    // println!("dps:");
    // for i in 0..0x20 {
    //     let mut s = 0.0;
    //     for (_, p) in get_dpf(&dps, &i) {
    //         s += p;
    //     }
    //     println!("s = {s}");
    //     println!("{:?}", get_dpf(&dps, &i));
    // }
}
