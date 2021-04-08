#[macro_use]
mod modular_arith;
use modular_arith::Gcd;


#[test]
fn mod_arithmetic(){
    let a: u8 = u8::MAX;
    let b: u8 = 1;
    assert_eq!(madd!(a,1),0);
    assert_eq!(madd!(a,1,1),1);
    assert_eq!(msub!(b,a),2);
    assert_eq!(10, 10u8.gcd_euclid(20));
}
