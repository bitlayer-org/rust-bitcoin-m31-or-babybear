use crate::karatsuba_big;
use crate::u31::{u31_add, u31_double, BabyBear};
use crate::U31ExtConfig;
// use bitvm::treepp::*;
use bitcoin_script::{script,define_pushable};
define_pushable!();
use bitcoin::ScriptBuf as Script;
pub struct BabyBear4;

impl BabyBear4 {
    fn mul_11() -> Script {
        script! {
            OP_DUP
            { u31_double::<BabyBear>() }
            OP_DUP
            { u31_double::<BabyBear>() }
            { u31_double::<BabyBear>() }
            { u31_add::<BabyBear>() }
            { u31_add::<BabyBear>() }
        }
    }
}

impl U31ExtConfig for BabyBear4 {
    type BaseFieldConfig = BabyBear;
    const DEGREE: u32 = 4;

    fn mul_impl() -> Script {
        script! {
            { karatsuba_big::<BabyBear>() }
            6 OP_ROLL
            6 OP_ROLL
            { u31_add::<BabyBear>() }
            { Self::mul_11() }
            { u31_add::<BabyBear>() }
            5 OP_ROLL
            { Self::mul_11() }
            2 OP_ROLL
            { u31_add::<BabyBear>() }
            5 OP_ROLL
            { Self::mul_11() }
            3 OP_ROLL
            4 OP_ROLL
            { u31_add::<BabyBear>() }
            { u31_add::<BabyBear>() }
            OP_SWAP
            OP_ROT
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        u31_sub_u31ext, u31_to_u31ext, u31ext_add, u31ext_add_u31, u31ext_double,
        u31ext_equalverify, u31ext_mul, u31ext_mul_u31, u31ext_mul_u31_by_constant, u31ext_neg,
        u31ext_sub, u31ext_sub_u31, QM31,
    };
    use bitcoin_script::{script,define_pushable};
    define_pushable!();
    use bitcoin::ScriptBuf as Script;
    use core::ops::{Add, Mul, Neg};
    use p3_field::{AbstractExtensionField, AbstractField, ExtensionField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use crate::execute_script;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<p3_baby_bear::BabyBear, 4>;

    #[test]
    fn test_u31ext_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 add: {}", u31ext_add::<BabyBear4>().len());

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let c = a.add(b);

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let b: &[p3_baby_bear::BabyBear] = b.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b[3].as_canonical_u32() } { b[2].as_canonical_u32() } { b[1].as_canonical_u32() } { b[0].as_canonical_u32() }
            { u31ext_add::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_double() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let c = a.double();

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { u31ext_double::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_sub() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 sub: {}", u31ext_sub::<BabyBear4>().len());

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();
        let c = a.add(b.neg());

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let b: &[p3_baby_bear::BabyBear] = b.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b[3].as_canonical_u32() } { b[2].as_canonical_u32() } { b[1].as_canonical_u32() } { b[0].as_canonical_u32() }
            { u31ext_sub::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 mul: {}", u31ext_mul::<BabyBear4>().len());

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();
        let c = a.mul(b);

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let b: &[p3_baby_bear::BabyBear] = b.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b[3].as_canonical_u32() } { b[2].as_canonical_u32() } { b[1].as_canonical_u32() } { b[0].as_canonical_u32() }
            { u31ext_mul::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
             { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_add_u31() {
        for _ in 0..20 {
            let add_script = u31ext_add_u31::<BabyBear4>();

            let mut rng = ChaCha20Rng::seed_from_u64(0u64);
            eprintln!("babybear4 mul_by_babybear: {}", add_script.len());

            let a = rng.gen::<F>();
            let b = rng.gen::<p3_baby_bear::BabyBear>();

            let c1 = a + F::from_base(b);
            let c = a + b;
            assert_eq!(c, c1);

            let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
            let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

            let script = script! {
                { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
                { b.as_canonical_u32() }
                { add_script.clone() }
                { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
                { u31ext_equalverify::<BabyBear4>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_u31ext_sub_u31() {
        for _ in 0..20 {
            let sub_script = u31ext_sub_u31::<BabyBear4>();

            let mut rng = ChaCha20Rng::seed_from_u64(0u64);
            eprintln!("babybear4 mul_by_babybear: {}", sub_script.len());

            let a = rng.gen::<F>();
            let b = rng.gen::<p3_baby_bear::BabyBear>();

            let c1 = a - F::from_base(b);
            let c = a - b;
            assert_eq!(c, c1);

            let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
            let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

            let script = script! {
                { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
                { b.as_canonical_u32() }
                { sub_script.clone() }
                { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
                { u31ext_equalverify::<BabyBear4>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_u31ext_neg() {
        for _ in 0..20 {
            let neg_script = u31ext_neg::<BabyBear4>();

            let mut rng = ChaCha20Rng::seed_from_u64(0u64);
            eprintln!("babybear4  neg: {}", neg_script.len());

            let a = rng.gen::<F>();
            let c = -a;

            let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
            let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

            let script = script! {
                { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
                { neg_script.clone() }
                { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
                { u31ext_equalverify::<BabyBear4>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_u31_sub_u31ext() {
        for _ in 0..20 {
            let sub_script = u31_sub_u31ext::<BabyBear4>();

            let mut rng = ChaCha20Rng::seed_from_u64(0u64);
            eprintln!("babybear4 mul_by_babybear: {}", sub_script.len());

            let a = rng.gen::<F>();
            let b = rng.gen::<p3_baby_bear::BabyBear>();

            let c = F::from_base(b) - a;

            let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
            let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

            let script = script! {
                { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
                { b.as_canonical_u32() }
                { sub_script}
                { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
                { u31ext_equalverify::<BabyBear4>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_u31_to_u31ext() {
        for _ in 0..20 {
            let mut rng = ChaCha20Rng::seed_from_u64(0u64);

            let b = rng.gen::<p3_baby_bear::BabyBear>();

            let c = F::from_base(b);

            let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

            let script = script! {
                { b.as_canonical_u32() }
                { u31_to_u31ext::<BabyBear4>()}
                { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
                { u31ext_equalverify::<BabyBear4>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_u31ext_mul_u31() {
        let mul_script = u31ext_mul_u31::<BabyBear4>();

        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 mul_by_babybear: {}", mul_script.len());

        let a = rng.gen::<F>();
        let b = rng.gen::<p3_baby_bear::BabyBear>();

        let c = a * F::from_base(b);

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b.as_canonical_u32() }
            { mul_script.clone() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul_u31_by_constant() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        let mut total_len = 0;

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<p3_baby_bear::BabyBear>();

            let mul_script = u31ext_mul_u31_by_constant::<BabyBear4>(b.as_canonical_u32());
            total_len += mul_script.len();

            let c = a * F::from_base(b);

            let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
            let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

            let script = script! {
                { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
                { mul_script.clone() }
                { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
                { u31ext_equalverify::<QM31>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("qm31 mul_by_m31_by_constant: {}", total_len as f64 / 100.0);
    }
}
