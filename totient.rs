fn isprime(n: uint) -> bool {
    if n & 1 == 1 {
        range(2, (n as float).sqrt() as uint).all(|x| n % x != 0)
    } else {
        false
    }
}

fn main() {
    let mut p = 2u;
    let mut p_prev = 1u;
    while p < 0xFFFF_FFFF_FFFF_FFFF {
        if p % 50000 == 0 {
            println(p.to_str());
        }

        if !isprime(p) {
            p += 1;
            loop;
        }

        println!("Here's what's up: p = {}, p_prev = {}, sqrt = {}, quot = {}",
        p, p_prev, (p as float).sqrt() as uint, p / (p - p_prev));

        if (p/(p - p_prev)) < ((p as float).sqrt() as uint) {
            println!("Candidate found: {}", p_prev);
        }

        p_prev = p;
        p += 1;
    }
}
