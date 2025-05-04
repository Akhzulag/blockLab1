pub fn s_layer(s: &[u16], x: &u16) -> u16 {
    let mut y: u16 = 0;
    for i in 0..4 {
        y |= s[((x >> (4 * i)) & 0xf) as usize] << (4 * i);
    }

    y
}

pub fn pi(x: &u16) -> u16 {
    let mut x_t: u16 = 0;
    for j in 0..4 {
        for i in 0..4 {
            x_t |= ((x >> (4 * j + i)) & 1) << (i * 4 + j);
        }
    }
    x_t
}

const S: [u16; 16] = [0xF, 6, 5, 8, 0xE, 0xB, 0xA, 4, 0xC, 0, 3, 7, 2, 9, 1, 0xD];
pub fn round(x: &u16, key: u16) -> u16 {
    let mut c = x ^ key;
    c = s_layer(&S, &c);
    pi(&c)
}

fn heys(x: &u16) -> u16 {
    let mut c = round(x, 0x5DE1);
    c = round(&c, 0xB053);
    c = round(&c, 0xBD30);
    c = round(&c, 0xC124);
    c = round(&c, 0x2D9B);
    c = round(&c, 0x8D7D);
    c ^ 0x3699
}
