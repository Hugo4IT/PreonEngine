use std::time::{Instant};

use preon_core::{PreonData};

fn main() {
    let time = Instant::now();

    {
        let mut data = PreonData::new(76);
        data.set_u8(0, 254);
        data.set_u16(1, 25432);
        data.set_u32(3, 2345);
        data.set_u64(7, 235);
        data.set_u128(15, 245);
        data.set_i8(31, -45);
        data.set_i16(32, -2345);
        data.set_i32(34, -54);
        data.set_i64(38, -23543);
        data.set_i128(46, -34563);
        data.set_f32(62, -78.736);
        data.set_f64(66, 763.2);
        data.set_bool(74, false);
        data.set_bools(75, [true, false, true, true, false, false, true, true]);
    
        let a = data.get_u8(0);
        let b = data.get_u16(1);
        let c = data.get_u32(3);
        let d = data.get_u64(7);
        let e = data.get_u128(15);
        let f = data.get_i8(31);
        let g = data.get_i16(32);
        let h = data.get_i32(34);
        let i = data.get_i64(38);
        let j = data.get_i128(46);
        let k = data.get_f32(62);
        let l = data.get_f64(66);
        let m = data.get_bool(74);
        let n = data.get_bools(75);
    
        println!("u8:    {} \
                \nu16:   {} \
                \nu32:   {} \
                \nu64:   {} \
                \nu128:  {} \
                \ni8:    {} \
                \ni16:   {} \
                \ni32:   {} \
                \ni64:   {} \
                \ni128:  {} \
                \nf32:   {} \
                \nf64:   {} \
                \nbool:  {} \
                \nbools: {} \
                \n       {} \
                \n       {} \
                \n       {} \
                \n       {} \
                \n       {} \
                \n       {} \
                \n       {}", a, b, c, d, e, f, g, h, i, j, k, l, m, n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7]);
    }

    let elapsed = time.elapsed();
    println!("Elapsed: {:.3?} (MicroSeconds)", elapsed);
}
