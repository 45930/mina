use crate::bigint_384::{CamlBigint384, CamlBigint384Ptr};
use algebra::{
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    bn_382::fq::{Fq, FqParameters as Fq_params},
    FftField, One, UniformRand, Zero,
};
use ff_fft::{EvaluationDomain, Radix2EvaluationDomain as Domain};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Copy, Clone)]
pub struct CamlBn382Fq(Fq);

pub type CamlBn382FqPtr = ocaml::Pointer<CamlBn382Fq>;

extern "C" fn caml_bn_382_fq_compare_raw(x: ocaml::Value, y: ocaml::Value) -> libc::c_int {
    let x: CamlBn382FqPtr = ocaml::FromValue::from_value(x);
    let y: CamlBn382FqPtr = ocaml::FromValue::from_value(y);

    match x.as_ref().0.cmp(&y.as_ref().0) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

impl From<&CamlBn382Fq> for CamlBigint384 {
    fn from(x: &CamlBn382Fq) -> CamlBigint384 {
        CamlBigint384(x.0.into_repr())
    }
}

impl From<&CamlBigint384> for CamlBn382Fq {
    fn from(x: &CamlBigint384) -> CamlBn382Fq {
        CamlBn382Fq(Fq::from_repr(x.0))
    }
}

impl std::fmt::Display for CamlBn382Fq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        CamlBigint384::from(self).fmt(f)
    }
}

ocaml::custom!(CamlBn382Fq {
    compare: caml_bn_382_fq_compare_raw,
});

#[ocaml::func]
pub fn caml_bn_382_fq_size_in_bits() -> ocaml::Int {
    Fq_params::MODULUS_BITS as isize
}

#[ocaml::func]
pub fn caml_bn_382_fq_size() -> CamlBigint384 {
    CamlBigint384(Fq_params::MODULUS)
}

#[ocaml::func]
pub fn caml_bn_382_fq_add(x: CamlBn382FqPtr, y: CamlBn382FqPtr) -> CamlBn382Fq {
    CamlBn382Fq(x.as_ref().0 + y.as_ref().0)
}

#[ocaml::func]
pub fn caml_bn_382_fq_sub(x: CamlBn382FqPtr, y: CamlBn382FqPtr) -> CamlBn382Fq {
    CamlBn382Fq(x.as_ref().0 - y.as_ref().0)
}

#[ocaml::func]
pub fn caml_bn_382_fq_negate(x: CamlBn382FqPtr) -> CamlBn382Fq {
    CamlBn382Fq(-x.as_ref().0)
}

#[ocaml::func]
pub fn caml_bn_382_fq_mul(x: CamlBn382FqPtr, y: CamlBn382FqPtr) -> CamlBn382Fq {
    CamlBn382Fq(x.as_ref().0 * y.as_ref().0)
}

#[ocaml::func]
pub fn caml_bn_382_fq_div(x: CamlBn382FqPtr, y: CamlBn382FqPtr) -> CamlBn382Fq {
    CamlBn382Fq(x.as_ref().0 / y.as_ref().0)
}

#[ocaml::func]
pub fn caml_bn_382_fq_inv(x: CamlBn382FqPtr) -> Option<CamlBn382Fq> {
    match x.as_ref().0.inverse() {
        Some(x) => Some(CamlBn382Fq(x)),
        None => None,
    }
}

#[ocaml::func]
pub fn caml_bn_382_fq_square(x: CamlBn382FqPtr) -> CamlBn382Fq {
    CamlBn382Fq(x.as_ref().0.square())
}

#[ocaml::func]
pub fn caml_bn_382_fq_is_square(x: CamlBn382FqPtr) -> bool {
    let s = x.as_ref().0.pow(Fq_params::MODULUS_MINUS_ONE_DIV_TWO);
    s.is_zero() || s.is_one()
}

#[ocaml::func]
pub fn caml_bn_382_fq_sqrt(x: CamlBn382FqPtr) -> Option<CamlBn382Fq> {
    match x.as_ref().0.sqrt() {
        Some(x) => Some(CamlBn382Fq(x)),
        None => None,
    }
}

#[ocaml::func]
pub fn caml_bn_382_fq_of_int(i: ocaml::Int) -> CamlBn382Fq {
    CamlBn382Fq(Fq::from(i as u64))
}

#[ocaml::func]
pub fn caml_bn_382_fq_to_string(x: CamlBn382FqPtr) -> String {
    x.as_ref().to_string()
}

#[ocaml::func]
pub fn caml_bn_382_fq_of_string(s: &[u8]) -> Result<CamlBn382Fq, ocaml::Error> {
    match BigUint::parse_bytes(s, 10) {
        Some(data) => Ok(CamlBn382Fq::from(&(CamlBigint384::from(&data)))),
        None => Err(ocaml::Error::invalid_argument("caml_bn_382_fq_of_string")
            .err()
            .unwrap()),
    }
}

#[ocaml::func]
pub fn caml_bn_382_fq_print(x: CamlBn382FqPtr) {
    println!("{}", x.as_ref());
}

#[ocaml::func]
pub fn caml_bn_382_fq_copy(x: CamlBn382FqPtr) -> CamlBn382Fq {
    *x.as_ref()
}

#[ocaml::func]
pub fn caml_bn_382_fq_mut_add(mut x: CamlBn382FqPtr, y: CamlBn382FqPtr) {
    x.as_mut().0 += y.as_ref().0;
}

#[ocaml::func]
pub fn caml_bn_382_fq_mut_sub(mut x: CamlBn382FqPtr, y: CamlBn382FqPtr) {
    x.as_mut().0 -= y.as_ref().0;
}

#[ocaml::func]
pub fn caml_bn_382_fq_mut_mul(mut x: CamlBn382FqPtr, y: CamlBn382FqPtr) {
    x.as_mut().0 *= y.as_ref().0;
}

#[ocaml::func]
pub fn caml_bn_382_fq_mut_square(mut x: CamlBn382FqPtr) {
    x.as_mut().0.square_in_place();
}

#[ocaml::func]
pub fn caml_bn_382_fq_compare(x: CamlBn382FqPtr, y: CamlBn382FqPtr) -> ocaml::Int {
    match x.as_ref().0.cmp(&y.as_ref().0) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

#[ocaml::func]
pub fn caml_bn_382_fq_equal(x: CamlBn382FqPtr, y: CamlBn382FqPtr) -> bool {
    x.as_ref().0 == y.as_ref().0
}

#[ocaml::func]
pub fn caml_bn_382_fq_random() -> CamlBn382Fq {
    CamlBn382Fq(UniformRand::rand(&mut rand::thread_rng()))
}

#[ocaml::func]
pub fn caml_bn_382_fq_rng(i: ocaml::Int) -> CamlBn382Fq {
    // We only care about entropy here, so we force a conversion i32 -> u32.
    let i: u64 = (i as u32).into();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(i);
    CamlBn382Fq(UniformRand::rand(&mut rng))
}

#[ocaml::func]
pub fn caml_bn_382_fq_to_bigint(x: CamlBn382FqPtr) -> CamlBigint384 {
    x.as_ref().into()
}

#[ocaml::func]
pub fn caml_bn_382_fq_of_bigint(x: CamlBigint384Ptr) -> CamlBn382Fq {
    x.as_ref().into()
}

#[ocaml::func]
pub fn caml_bn_382_fq_two_adic_root_of_unity() -> CamlBn382Fq {
    CamlBn382Fq(FftField::two_adic_root_of_unity())
}

#[ocaml::func]
pub fn caml_bn_382_fq_domain_generator(
    log2_size: ocaml::Int,
) -> Result<CamlBn382Fq, ocaml::Error> {
    match Domain::new(1 << log2_size) {
        Some(x) => Ok(CamlBn382Fq(x.group_gen)),
        None => Err(ocaml::Error::invalid_argument("caml_bn_382_fq_domain_generator")
            .err()
            .unwrap()),
    }
}
