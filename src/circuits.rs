use ark_ff::Field;
use ark_relations::lc;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
};

pub struct BooleanCheckCircuit<F: Field> {
    pub x: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for BooleanCheckCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let x = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;

        // Enforce x * (x - 1) = 0
        cs.enforce_constraint(lc!() + x, lc!() + x - Variable::One, lc!())?;

        Ok(())
    }
}

pub struct UnderconstrainedCircuit<F: Field> {
    pub x: Option<F>,
    pub y: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for UnderconstrainedCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let _x = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;

        let _y = cs.new_witness_variable(|| self.y.ok_or(SynthesisError::AssignmentMissing))?;

        // Intentionally missing a constraint relating x and y.
        Ok(())
    }
}

pub struct RangeCheckCircuit<F: Field> {
    pub value: Option<F>,
    pub bits: [Option<F>; 4],
}

impl<F: Field> ConstraintSynthesizer<F> for RangeCheckCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let value =
            cs.new_witness_variable(|| self.value.ok_or(SynthesisError::AssignmentMissing))?;

        let mut bit_variables = Vec::new();

        for bit in self.bits {
            let bit_var =
                cs.new_witness_variable(|| bit.ok_or(SynthesisError::AssignmentMissing))?;

            // Enforce each bit is either 0 or 1.
            cs.enforce_constraint(lc!() + bit_var, lc!() + bit_var - Variable::One, lc!())?;

            bit_variables.push(bit_var);
        }

        // Reconstruct the value from its four bits.
        let mut reconstructed = LinearCombination::<F>::zero();

        for (i, bit_var) in bit_variables.iter().enumerate() {
            reconstructed += (F::from(1u64 << i), *bit_var);
        }

        cs.enforce_constraint(reconstructed, lc!() + Variable::One, lc!() + value)?;

        Ok(())
    }
}
