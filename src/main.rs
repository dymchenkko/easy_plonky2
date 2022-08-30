use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::{PartialWitness, Witness};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, 2>::new(config.clone());
    let a = builder.add_virtual_target();
    let b = builder.add_virtual_target();
    builder.connect(a, b);
    let data = builder.build::<C>();
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(a, GoldilocksField(10));
    pw.set_target(b, GoldilocksField(10));
    let proof = data.prove(pw).unwrap();
    match  data.verify(proof) {
        Ok(()) => println!("They are equal"),
        Err(x) => println!("{}", x)
    }
}