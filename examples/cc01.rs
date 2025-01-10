// chatgpt creation

use ark_bls12_381::{Bls12_381, Fr as ScalarField};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::kzg10::KZG10;
use ark_std::test_rng;
use ark_serialize::CanonicalSerialize;

fn main() {
    // Define the field and RNG
    type PC = KZG10<Bls12_381, DensePolynomial<ScalarField>>;
    let rng = &mut test_rng();

    // Universal setup for Plonk
    let max_degree = 8; // Maximum polynomial degree supported
    let universal_srs = PC::setup(max_degree, rng).unwrap();

    // Save universal SRS to file
    let mut srs_file = std::fs::File::create("universal_srs.bin").unwrap();
    universal_srs.serialize_unchecked(&mut srs_file).unwrap();

    println!("Universal trusted setup completed. SRS saved to file.");
}