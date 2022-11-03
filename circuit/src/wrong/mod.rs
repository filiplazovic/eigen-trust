#![allow(missing_docs)]
// Rns
// bit_len_lookup: 17,
// wrong_modulus:
// 21888242871839275222246405745257275088696311157297823662689037894645226208583,
// native_modulus:
// 21888242871839275222246405745257275088548364400416034343698204186575808495617,
// binary_modulus:
// 7588550360256754183279148073529370729071901715047420004889892225542594864082845696,
// crt_modulus:
// 166100033330483263771891769974495097228807904130411393260304576971769623221437250863502951190734612532350192541290114096968998888229427253981337422670103314432,
// right_shifters: [
//     0x0000000000000000000000000000000000000000000000000000000000000001,
//     0x0b603a5609b3f6f81dbc9c192fc7933ab42e346981868e480f8e4610fb396ee5,
//     0x1b7c016fe8acfaed1a908db2cea9b991a31a140f219532a9568bea8e0766f9dd,
//     0x0523513296c10199338287b1e0bedd9955a33201cd88df51769b0bf04e2f27cc,
// ],
// left_shifters: [
//     0x0000000000000000000000000000000000000000000000000000000000000001,
//     0x0000000000000000000000000000000000000000000000100000000000000000,
//     0x0000000000000000000000000000010000000000000000000000000000000000,
//     0x0000000000001000000000000000000000000000000000000000000000000000,
// ],
// base_aux: [
//     488280579659007654542,
//     510955945554286098768,
//     301160387202582250159,
//     1702635872462387,
// ],
// negative_wrong_modulus_decomposed: [
//     0x000000000000000000000000000000000000000000000002c3df73e9278302b9,
//     0x00000000000000000000000000000000000000000000000a2687e956e978e357,
//     0x00000000000000000000000000000000000000000000000fd647afba497e7ea7,
//     0x00000000000000000000000000000000000000000000000ffffcf9bb18d1ece5,
// ],
// wrong_modulus_decomposed: [
//     0x00000000000000000000000000000000000000000000000d3c208c16d87cfd47,
//     0x000000000000000000000000000000000000000000000005d97816a916871ca8,
//     0x00000000000000000000000000000000000000000000000029b85045b6818158,
//     0x00000000000000000000000000000000000000000000000000030644e72e131a,
// ],
// wrong_modulus_minus_one: [
//     0x00000000000000000000000000000000000000000000000d3c208c16d87cfd46,
//     0x000000000000000000000000000000000000000000000005d97816a916871ca8,
//     0x00000000000000000000000000000000000000000000000029b85045b6818158,
//     0x00000000000000000000000000000000000000000000000000030644e72e131a,
// ],
// wrong_modulus_in_native_modulus:
// 0x000000000000000000000000000000006f4d8248eeb859fbf83e9682e87cfd46,
// max_reduced_limb: 295147905179352825855,
// max_unreduced_limb: 5070602400912917605986812821503,
// max_remainder:
// 28948022309329048855892746252171976963317496166410141009864396001978282409983,
// max_operand:
// 7410693711188236507108543040556026102609279018600996098525285376506440296955903,
// max_mul_quotient:
// 3794275180128377091639574036764685364535950857523710002444946112771297432041422847,
// max_most_significant_reduced_limb: 1125899906842623,
// max_most_significant_operand_limb: 288230376151711743,
// max_most_significant_mul_quotient_limb: 147573952589676412927,
// mul_v_bit_len: 71,
// red_v_bit_len: 69,

use std::{result, str::FromStr};

use halo2wrong::{
	curves::{
		bn256::{Fq, Fr},
		group::ff::PrimeField,
	},
	halo2::arithmetic::FieldExt,
};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{FromPrimitive, Num};
use std::ops::Shl;

trait RnsParams<W: FieldExt, N: FieldExt, const NUM_LIMBS: usize, const NUM_BITS: usize> {
	fn native_modulus() -> BigUint;
	fn wrong_modulus() -> BigUint;
	fn negative_wrong_modulus_decomposed() -> [N; NUM_LIMBS];
	fn right_shifters() -> [N; NUM_LIMBS];
	fn left_shifters() -> [N; NUM_LIMBS];
}

pub fn modulus<F: FieldExt>() -> BigUint {
	BigUint::from_str_radix(&F::MODULUS[2..], 16).unwrap()
}

pub fn big_to_fe<F: FieldExt>(e: BigUint) -> F {
	let modulus = modulus::<F>();
	let e = e % modulus;
	F::from_str_vartime(&e.to_str_radix(10)[..]).unwrap()
}

pub fn decompose_big<F: FieldExt, const NUM_LIMBS: usize, const BIT_LEN: usize>(
	mut e: BigUint,
) -> [F; NUM_LIMBS] {
	let mask = BigUint::from(1usize).shl(BIT_LEN) - 1usize;
	let mut limbs = [F::zero(); NUM_LIMBS];
	for i in 0..NUM_LIMBS {
		let limb = mask.clone() & e.clone();
		e = e.clone() >> BIT_LEN;
		limbs[i] = big_to_fe(limb);
	}

	limbs
}

struct Bn256_4_68;

impl RnsParams<Fq, Fr, 4, 68> for Bn256_4_68 {
	fn native_modulus() -> BigUint {
		BigUint::from_str(
			"21888242871839275222246405745257275088548364400416034343698204186575808495617",
		)
		.unwrap()
	}

	fn wrong_modulus() -> BigUint {
		BigUint::from_str(
			"21888242871839275222246405745257275088696311157297823662689037894645226208583",
		)
		.unwrap()
	}

	fn negative_wrong_modulus_decomposed() -> [Fr; 4] {
		let limb0 = Fr::from_str_vartime("51007615349848998585").unwrap();
		let limb1 = Fr::from_str_vartime("187243884991886189399").unwrap();
		let limb2 = Fr::from_str_vartime("292141664167738113703").unwrap();
		let limb3 = Fr::from_str_vartime("295147053861416594661").unwrap();
		[limb0, limb1, limb2, limb3]
	}

	fn right_shifters() -> [Fr; 4] {
		let limb0 = Fr::from_str_vartime("1").unwrap();
		let limb1 = Fr::from_str_vartime(
			"5145461273500026213567655774445647387857717597658424792456619466422411161317",
		)
		.unwrap();
		let limb2 = Fr::from_str_vartime(
			"12431545866584750573138504686521816626666974057179777438265775398014169840093",
		)
		.unwrap();
		let limb3 = Fr::from_str_vartime(
			"2323964295511240700372762788508197460016882700897121076795132393931913766860",
		)
		.unwrap();
		[limb0, limb1, limb2, limb3]
	}

	fn left_shifters() -> [Fr; 4] {
		let limb0 = Fr::from_str_vartime("1").unwrap();
		let limb1 = Fr::from_str_vartime("295147905179352825856").unwrap();
		let limb2 = Fr::from_str_vartime("87112285931760246646623899502532662132736").unwrap();
		let limb3 =
			Fr::from_str_vartime("25711008708143844408671393477458601640355247900524685364822016")
				.unwrap();
		[limb0, limb1, limb2, limb3]
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn residues(n: &[Fr; 4], t: &[Fr; 4]) -> Vec<Fr> {
		let lsh1 = Bn256_4_68::left_shifters()[1];
		let rsh2 = Bn256_4_68::right_shifters()[2];

		let mut res = Vec::new();
		let mut carry = Fr::zero();
		for i in (0..4).step_by(2) {
			let (t_0, t_1) = (t[i], t[i + 1]);
			let (r_0, r_1) = (n[i], n[i + 1]);
			let u = t_0 + (t_1 * lsh1) - r_0 - (lsh1 * r_1) + carry;
			let v = u * rsh2;
			carry = v;
			res.push(v)
		}
		res
	}

	fn to_native(x: [Fr; 4], q: [Fr; 4]) -> Fr {
		let mut x_sum = Fr::zero();
		for i in 0..4 {
			x_sum += x[i] * q[i];
		}
		x_sum
	}

	#[test]
	pub fn wrong_mul() {
		let native_mod_bn = Bn256_4_68::native_modulus();
		let wrong_mod_bn = Bn256_4_68::wrong_modulus();
		let p_prime = Bn256_4_68::negative_wrong_modulus_decomposed();

		let a_bn = BigUint::from_str(
			"91888242871839275222246405745257275088548364400416034343698204186575808495807",
		)
		.unwrap();
		let b_bn = BigUint::from_u64(123134).unwrap();
		let (quotient, result_bn) = (a_bn.clone() * b_bn.clone()).div_rem(&wrong_mod_bn);

		let a = decompose_big::<Fr, 4, 68>(a_bn.clone());
		let b = decompose_big::<Fr, 4, 68>(b_bn.clone());
		let q = decompose_big::<Fr, 4, 68>(quotient.clone());
		let result = decompose_big::<Fr, 4, 68>(result_bn.clone());
		let p = decompose_big::<Fr, 4, 68>(wrong_mod_bn.clone());

		let mut t: [Fr; 4] = [Fr::zero(); 4];

		for k in 0..4 {
			for i in 0..=k {
				let j = k - i;
				t[i + j] = t[i + j] + a[i] * b[j] + p_prime[i] * q[j];
			}
		}

		let residues = residues(&result, &t);

		// a = 4
		// b = 2
		// q = 1
		// p = 5
		// result = 1
		// a * b - q * p - result = 4 * 2 - 1 * 5 - 1 = 6 - 5 - 1 = 0
		// let res_real = a_bn * b_bn - quotient * wrong_mod_bn - result_bn;
		// let res = to_native(a, q) * to_native(b, q) - to_native(q, q);
		// println!("res {:?}", res);

		let mut new_t: Vec<Fr> = vec![];
		for (i, inter) in t.iter().enumerate() {
			let mut inter = *inter;
			for j in 0..=i {
				let k = i - j;

				let prev_inter = inter;
				let next_inter = inter - (a[j] * b[k] + q[k] * p_prime[j]);

				if j == i {
					println!("{:?} {:?}", next_inter, prev_inter);
				}
				if j == 0 {
					new_t.push(prev_inter);
				}
				// CONSTRAINT
				println!(
					"{:?}",
					a[j] * b[k] + q[k] * p_prime[j] - prev_inter + next_inter
				);

				inter = next_inter;
			}
		}

		println!("{:?}", new_t);
	}
}
