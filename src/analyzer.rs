use ark_ff::Field;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};

#[derive(Debug, Clone)]
pub struct CircuitReport {
    pub circuit_name: String,
    pub constraints: usize,
    pub public_variables: usize,
    pub private_variables: usize,
    pub is_satisfied: bool,
    pub warnings: Vec<String>,
}

pub fn analyze_circuit<F, C>(
    circuit: C,
    circuit_name: &str,
) -> Result<CircuitReport, SynthesisError>
where
    F: Field,
    C: ConstraintSynthesizer<F>,
{
    let cs = ConstraintSystem::<F>::new_ref();

    circuit.generate_constraints(cs.clone())?;

    let constraints = cs.num_constraints();
    let public_variables = cs.num_instance_variables().saturating_sub(1);
    let private_variables = cs.num_witness_variables();

    let mut warnings = Vec::new();

    if private_variables > 0 && constraints == 0 {
        warnings.push("Private variables exist, but no constraints were generated.".to_string());
    }

    let report = CircuitReport {
        circuit_name: circuit_name.to_string(),
        constraints,
        public_variables,
        private_variables,
        is_satisfied: cs.is_satisfied()?,
        warnings,
    };

    Ok(report)
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fr;

    use crate::analyzer::analyze_circuit;
    use crate::circuits::{BooleanCheckCircuit, RangeCheckCircuit, UnderconstrainedCircuit};

    #[test]
    fn accepts_valid_boolean_witness() {
        let circuit = BooleanCheckCircuit {
            x: Some(Fr::from(1u64)),
        };

        let report = analyze_circuit(circuit, "Boolean Check").unwrap();

        assert_eq!(report.constraints, 1);
        assert_eq!(report.public_variables, 0);
        assert_eq!(report.private_variables, 1);
        assert!(report.is_satisfied);
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn rejects_invalid_boolean_witness() {
        let circuit = BooleanCheckCircuit {
            x: Some(Fr::from(5u64)),
        };

        let report = analyze_circuit(circuit, "Boolean Check").unwrap();

        assert!(!report.is_satisfied);
    }

    #[test]
    fn demonstrates_underconstrained_circuit() {
        let circuit = UnderconstrainedCircuit {
            x: Some(Fr::from(10u64)),
            y: Some(Fr::from(99u64)),
        };

        let report = analyze_circuit(circuit, "Underconstrained").unwrap();

        assert_eq!(report.constraints, 0);
        assert_eq!(report.private_variables, 2);
        assert!(report.is_satisfied);
        assert_eq!(report.warnings.len(), 1);
    }

    #[test]
    fn accepts_valid_range_value() {
        let circuit = RangeCheckCircuit {
            value: Some(Fr::from(11u64)),
            bits: [
                Some(Fr::from(1u64)),
                Some(Fr::from(1u64)),
                Some(Fr::from(0u64)),
                Some(Fr::from(1u64)),
            ],
        };

        let report = analyze_circuit(circuit, "Range Check").unwrap();

        assert_eq!(report.constraints, 5);
        assert_eq!(report.private_variables, 5);
        assert!(report.is_satisfied);
    }

    #[test]
    fn rejects_value_outside_four_bit_range() {
        let circuit = RangeCheckCircuit {
            value: Some(Fr::from(16u64)),
            bits: [
                Some(Fr::from(0u64)),
                Some(Fr::from(0u64)),
                Some(Fr::from(0u64)),
                Some(Fr::from(0u64)),
            ],
        };

        let report = analyze_circuit(circuit, "Range Check").unwrap();

        assert!(!report.is_satisfied);
    }
}
