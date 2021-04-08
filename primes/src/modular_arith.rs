
#[macro_export]
macro_rules! madd {
    ($a:expr,$b:expr) => {
        {
           $a.wrapping_add($b)
        }
    };
    ($a:expr, $b:expr, $c:expr)=>{
        {
            $a.wrapping_add($b).wrapping_add($c)
        }
    };
}


#[macro_export]
macro_rules! msub {
    ($a:expr,$b:expr) => {
        {
           $a.wrapping_sub($b)
        }
    };
}


#[macro_export]
macro_rules! mmult {
    ($a:expr,$b:expr) => {
        {
           $a.wrapping_mul($b)
        }
    };
    ($a:expr, $b:expr, $c:expr)=>{
        {
            $a.wrapping_mul($b).wrapping_mul($c)
        }
    };
}


#[macro_export]
macro_rules! mexp {
    ($a:expr,$b:expr) => {
        {
           $a.wrapping_exp($b)
        }
    };
}

pub trait Gcd {
    fn gcd_euclid(self, other: Self) -> Self;
}

macro_rules! gcd_impl {
    ($($t:ty),*) => ($(
        impl Gcd for $t {
            fn gcd_euclid(self, other: Self) -> Self {
                // variable names based off Euclidean divison equation: a = b · q + r
                let (mut a, mut b) = if self > other {
                    (self, other)
                } else {
                    (other, self)
                };

                while b != 0 {
                    let r = a % b;
                    a = b;
                    b = r;
                }

                a
            }
        }
    )*)
}

gcd_impl! { u8, u16, u32, u64, u128, usize }