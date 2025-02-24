/*
    My EC simple implementation of key generation for secp256k1
    by JecikPo <jecikpo at gmail.com>
*/

use num_bigint::{BigInt};
use num_traits::{Zero, One, Num};
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: BigInt,
    y: BigInt,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait P_mod {
    fn modulo_p_curve(&self) -> Self;

    fn modulo_n(&self, n: BigInt) -> Self;
}

impl P_mod for BigInt {
    //(lm % n + n) % n
    fn modulo_p_curve(&self) -> Self {
        self.modulo_n(
            bigint_from_hex(Point::P_str)
        )
    }

    fn modulo_n(&self, n: BigInt) -> Self {
        (self % &n + &n) % &n
    }
}

impl Point {
    // order of the curve
    const N_str: &str  = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

    // Prime field: 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
    const P_str: &str  = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

    // Generator Point
    const Gx_str: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
    const Gy_str: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";

    // Curve equation: y^2 = x^3 + A_curve * x + B_curve
    const A_curve: u64 = 0;
    //const B_curve: u64 = 7; // not needed

    fn new(x: BigInt, y: BigInt) -> Self {
        Self {
            x,
            y,
        }
    }

    /*
        Point addition of two points Pa(xa, ya) and Pb (xb, yb):
        x = lam^2 - xa - xb
        y = lam * (xa - x) - ya

        where: 
        lam = (yb - ya) / (xb - xa)
    */
    fn add(&self, b: &Point) -> Self {
        let p_curve = bigint_from_hex(Point::P_str);
        let lambda: BigInt = ((&b.y - &self.y) * modulo_inverse(&(&b.x - &self.x), &p_curve)).modulo_p_curve();
        let x = (&lambda * &lambda - &self.x - &b.x).modulo_p_curve();
        let y = (&lambda * (&self.x - &x) - &self.y).modulo_p_curve();
        Self {
            x,
            y
        }
    }
    /*
        Point doubling P(xp, yp):
        x = lam^2 - 2 * xp
        y = lam * (xp - x) - yp

        where:
        lam = (3 * xp^2 + a) / 2 * yp
    */
    fn double(&self) -> Self {
        let p_curve = bigint_from_hex(Point::P_str);
        let lambda = ((BigInt::from(3) * &self.x * &self.x + BigInt::from(Point::A_curve)) * modulo_inverse(&(BigInt::from(2) * &self.y), &p_curve)).modulo_p_curve();
        let x = (&lambda * &lambda - BigInt::from(2) * &self.x).modulo_p_curve();
        let y = (&lambda * (&self.x - &x) - &self.y).modulo_p_curve();
        Self {
            x,
            y
        }
    }

    /* double and add algorithm */
    fn multiply(self, scalar: &BigInt) -> Self {
        // TODO: add check scalar !=0 and < N
        let binary_scalar = format!("{:b}", scalar);

        let mut q = Point::new(
            self.x.clone(),
            self.y.clone()
        );

        for (index, bit) in binary_scalar.chars().enumerate() {
            if index == 0 { continue; }
            q = q.double();
            match bit {
                '1' => {
                    q = q.add(&self);
                }
                '0' => {}
                _ => { unreachable!(); }
            }
        }
        q
    }
}

fn main() {
    let g = Point::new(
        bigint_from_hex(Point::Gx_str),
        bigint_from_hex(Point::Gy_str)
    );

    let priv_key = bigint_from_dec("75263518707598184987916378021939673586055614731957507592904438851787542395619");

    let pub_key = g.multiply(&priv_key);

    println!("Private key (dec): {:?}", priv_key);
    println!("");

    // Print uncompressed public key
    println!("The uncompressed public key (HEX):");
    println!("04{:064x}{:064x}\n", &pub_key.x, &pub_key.y);
    
    // Print compressed public key
    println!("The official Public Key - compressed:");
    if &pub_key.y % BigInt::from(2u32) == BigInt::from(1u32) { // If Y coordinate is odd
        println!("03{:064x}", &pub_key.x);
    } else { // If Y coordinate is even
        println!("02{:064x}", &pub_key.x);
    }

    /***** Signature Generation *****/
    let g = g_point();
    let random_number = bigint_from_dec("28695618543805844332113829720373285210420739438570883203839696518176414791234");
    let random_signing_point = &g.multiply(&random_number);
    let n = bigint_from_hex(Point::N_str);
    let hash_to_sign = bigint_from_dec("86032112319101611046176971828093669637772856272773459297323797145286374828050");

    let r = (&random_signing_point.x % &n + &n) % &n;
    let s = ((&hash_to_sign + &r * &priv_key) * (modulo_inverse(&random_number, &n))) % &n;

    println!("");
    println!("r = {}", &r);
    println!("s = {}", &s);

    /***** Signature Verification *****/
    let w = modulo_inverse(&s, &n);
    let p1 = g_point().multiply(&(&hash_to_sign * &w).modulo_n(n.clone()));
    let p2 = pub_key.multiply(&(&r * &w).modulo_n(n.clone()));

    let signature = p2.add(&p1);

    println!("signature r = {}", signature.x);
}

/* --------------------- [ HELPERS ] --------------------- */

fn modulo_inverse(a: &BigInt, n: &BigInt) -> BigInt {
    let mut lm = BigInt::one();
    let mut hm = BigInt::zero();
    let mut low = (a % n + n) % n;
    let mut high = n.clone();
    let limit = BigInt::from(1u64);
    while low > limit {
        let ratio = &high / &low;
        let nm = &hm - &lm * &ratio;
        let new = &high - &low * &ratio;
        hm = lm.clone();
        lm = nm.clone();
        high = low.clone();
        low = new;
    }
    (lm % n + n) % n
}

fn bigint_from_dec(dec_str: &str) -> BigInt {
    BigInt::from_str_radix(dec_str, 10).unwrap()
}

fn bigint_from_hex(dec_str: &str) -> BigInt {
    BigInt::from_str_radix(dec_str, 16).unwrap()
}

fn g_point() -> Point {
    Point::new(
        bigint_from_hex(Point::Gx_str),
        bigint_from_hex(Point::Gy_str)
    )
}

/* --------------------- [ TESTS ] --------------------- */

#[test]
fn test_modulo_inverse() {
    let a = bigint_from_dec("55066263022277343669578718895168534326250603453777594175500187360389116729240");
    let b = bigint_from_hex(Point::P_str);
    let c = modulo_inverse(&a, &b);

    assert_eq!(
        c,
        bigint_from_dec("16048257703666452242803569546805946138055448571451565585555302070354637922038")
    );
}

#[test]
fn test_point_double() {
    let g = Point::new(
        bigint_from_dec("55066263022277343669578718895168534326250603453777594175500187360389116729240"),
        bigint_from_dec("32670510020758816978083085130507043184471273380659243275938904335757337482424")
    );

    let g_double = g.double();

    let g_double_expected = Point::new(
        bigint_from_dec("89565891926547004231252920425935692360644145829622209833684329913297188986597"),
        bigint_from_dec("12158399299693830322967808612713398636155367887041628176798871954788371653930")
    );

    assert_eq!(
        g_double, 
        g_double_expected
    );
}
