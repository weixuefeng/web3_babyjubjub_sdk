extern crate num;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};

pub fn modulus(a: &BigInt, m: &BigInt) -> BigInt {
    ((a % m) + m) % m
}

pub fn modinv(a: &BigInt, q: &BigInt) -> Result<BigInt, String> {
    let big_zero: BigInt = Zero::zero();
    if a == &big_zero {
        return Err("no mod inv of Zero".to_string());
    }

    let mut mn = (q.clone(), a.clone());
    let mut xy: (BigInt, BigInt) = (Zero::zero(), One::one());

    while mn.1 != big_zero {
        xy = (xy.1.clone(), xy.0 - (mn.0.clone() / mn.1.clone()) * xy.1);
        mn = (mn.1.clone(), modulus(&mn.0, &mn.1));
    }

    while xy.0 < Zero::zero() {
        xy.0 = modulus(&xy.0, q);
    }
    Ok(xy.0)
}

pub fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    x.iter().chain(y).cloned().collect()
}

pub fn modsqrt(a: &BigInt, q: &BigInt) -> Result<BigInt, String> {
    // Tonelli-Shanks Algorithm (https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm)
    //
    // This implementation is following the Go lang core implementation https://golang.org/src/math/big/int.go?s=23173:23210#L859
    // Also described in https://www.maa.org/sites/default/files/pdf/upload_library/22/Polya/07468342.di020786.02p0470a.pdf
    // -> section 6

    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();
    if legendre_symbol(&a, q) != 1 {
        return Err("not a mod p square".to_string());
    } else if a == &zero {
        return Err("not a mod p square".to_string());
    } else if q == &2.to_bigint().unwrap() {
        return Err("not a mod p square".to_string());
    } else if q % 4.to_bigint().unwrap() == 3.to_bigint().unwrap() {
        let r = a.modpow(&((q + one) / 4), &q);
        return Ok(r);
    }

    let mut s = q - &one;
    let mut e: BigInt = Zero::zero();
    while &s % 2 == zero {
        s = s >> 1;
        e = e + &one;
    }

    let mut n: BigInt = 2.to_bigint().unwrap();
    while legendre_symbol(&n, q) != -1 {
        n = &n + &one;
    }

    let mut y = a.modpow(&((&s + &one) >> 1), q);
    let mut b = a.modpow(&s, q);
    let mut g = n.modpow(&s, q);
    let mut r = e;

    loop {
        let mut t = b.clone();
        let mut m: BigInt = Zero::zero();
        while &t != &one {
            t = modulus(&(&t * &t), q);
            m = m + &one;
        }

        if m == zero {
            return Ok(y.clone());
        }

        t = g.modpow(&(2.to_bigint().unwrap().modpow(&(&r - &m - 1), q)), q);
        g = g.modpow(&(2.to_bigint().unwrap().modpow(&(r - &m), q)), q);
        y = modulus(&(y * t), q);
        b = modulus(&(b * &g), q);
        r = m.clone();
    }
}

#[allow(dead_code)]
pub fn modsqrt_v2(a: &BigInt, q: &BigInt) -> Result<BigInt, String> {
    // Tonelli-Shanks Algorithm (https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm)
    //
    // This implementation is following this Python implementation by Dusk https://github.com/dusk-network/dusk-zerocaf/blob/master/tools/tonelli.py

    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();
    if legendre_symbol(&a, q) != 1 {
        return Err("not a mod p square".to_string());
    } else if a == &zero {
        return Err("not a mod p square".to_string());
    } else if q == &2.to_bigint().unwrap() {
        return Err("not a mod p square".to_string());
    } else if q % 4.to_bigint().unwrap() == 3.to_bigint().unwrap() {
        let r = a.modpow(&((q + one) / 4), &q);
        return Ok(r);
    }

    let mut p = q - &one;
    let mut s: BigInt = Zero::zero();
    while &p % 2.to_bigint().unwrap() == zero {
        s = s + &one;
        p = p >> 1;
    }

    let mut z: BigInt = One::one();
    while legendre_symbol(&z, q) != -1 {
        z = &z + &one;
    }
    let mut c = z.modpow(&p, q);

    let mut x = a.modpow(&((&p + &one) >> 1), q);
    let mut t = a.modpow(&p, q);
    let mut m = s;

    while &t != &one {
        let mut i: BigInt = One::one();
        let mut e: BigInt = 2.to_bigint().unwrap();
        while i < m {
            if t.modpow(&e, q) == one {
                break;
            }
            e = e * 2.to_bigint().unwrap();
            i = i + &one;
        }

        let b = c.modpow(&(2.to_bigint().unwrap().modpow(&(&m - &i - 1), q)), q);
        x = modulus(&(x * &b), q);
        t = modulus(&(t * &b * &b), q);
        c = modulus(&(&b * &b), q);
        m = i.clone();
    }
    return Ok(x);
}

pub fn legendre_symbol(a: &BigInt, q: &BigInt) -> i32 {
    // returns 1 if has a square root modulo q
    let one: BigInt = One::one();
    let ls: BigInt = a.modpow(&((q - &one) >> 1), &q);
    if &(ls) == &(q - one) {
        return -1;
    }
    1
}
