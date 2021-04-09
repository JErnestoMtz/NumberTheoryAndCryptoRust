
// this is an attempt of implametation of the "Blum Blum shub" pseudo random 
// number generator
// takes two primes p, q where p%4 =3 and q%4=3, and a seed s,  S< p*q
// calculates m = p*q
// x_0 = seed^2 % m
// X_i = X_{i-1}^2 % m


struct Gen{
    seed: u64,
    m: u64,
    state: u64,
}

pub fn to_bit(n: u64)->char{
    match n.trailing_ones() {
        0 =>  '0',
        _ =>  '1',
    }
}

// this is the bit stream
// if x_i is odd => 0, else 1 
impl Iterator for Gen{
    type Item = char;
    
    fn next(&mut self) ->  Option<char> {
        let old_state = self.state;
        self.state = self.state.wrapping_pow(2) % self.m;
        Some(to_bit(old_state))
    }
}


impl Gen {
    fn new(seed:u64, p: u64, q: u64)-> Self{
        Gen{
            seed: seed,
            m: p.wrapping_mul(q),
            state: seed % p.wrapping_mul(q),
        }
    }
    // tranform a N bi stream into a binary number on N bits
    fn bits(&mut self,n: usize) -> String{
        let s: String = self.take(n).collect();
        s
    }
}



fn main() {
    let mut gen = Gen::new(7330570639,30000000091,40000000003);
    for i in  0..20{
        let p = isize::from_str_radix( &gen.bits(8), 2).unwrap();
        print!("{} \n", p);
    }
}
