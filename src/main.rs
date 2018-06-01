use std::io;

fn read_coef(x: &mut Vec<f32>) {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok();
    let split = inp.split_whitespace();
    for c in split {
        x.push(c.parse::<f32>().unwrap());
    }
}

fn subtract(l: &mut Vec<f32>, r: &Vec<f32>, c: &f32) {
    let mut i: usize = 1;
    for _r in r[1..].iter() {
        l[i] -= c * _r;
        i += 1;
    }
    l.remove(0); // Garanteed to be 0, if everything goes right
}

fn main() {
    println!("----------------------------------------------------------------------------");

    let mut cl = Vec::<f32>::new();
    read_coef(&mut cl);
    let mut cr = Vec::<f32>::new();
    read_coef(&mut cr);

    let mut q = Vec::<f32>::new();

    let mut c: f32;
    while cr.len() <= cl.len() {
        c = cl[0] / cr[0];
        q.push(c);
        subtract(&mut cl, &cr, &c);
    }
    println!("q = {:?}\nr = {:?}", &q, &cl);
}
