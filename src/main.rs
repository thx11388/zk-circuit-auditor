use ark_bn254::Fr;

use zk_circuit_auditor::analyzer::analyze_circuit;
use zk_circuit_auditor::circuits::{
    BooleanCheckCircuit, RangeCheckCircuit, UnderconstrainedCircuit,
};

fn print_report(
    name: &str,
    report: Result<
        zk_circuit_auditor::analyzer::CircuitReport,
        ark_relations::r1cs::SynthesisError,
    >,
) {
    match report {
        Ok(report) => {
            println!("\n=== {} ===", name);
            println!("Constraints: {}", report.constraints);
            println!("Public variables: {}", report.public_variables);
            println!("Private variables: {}", report.private_variables);
            println!("Satisfied: {}", report.is_satisfied);

            for warning in &report.warnings {
                println!("WARNING: {}", warning);
            }
        }
        Err(error) => {
            eprintln!("Analysis failed: {error}");
        }
    }
}

fn main() {
    println!("================================");
    println!("       ZK CIRCUIT AUDITOR");
    println!("================================");

    let boolean_circuit = BooleanCheckCircuit {
        x: Some(Fr::from(1u64)),
    };

    print_report(
        "BOOLEAN CHECK",
        analyze_circuit(boolean_circuit, "Boolean Check"),
    );

    let range_circuit = RangeCheckCircuit {
        value: Some(Fr::from(11u64)),
        bits: [
            Some(Fr::from(1u64)),
            Some(Fr::from(1u64)),
            Some(Fr::from(0u64)),
            Some(Fr::from(1u64)),
        ],
    };

    print_report("RANGE CHECK", analyze_circuit(range_circuit, "Range Check"));

    let underconstrained_circuit = UnderconstrainedCircuit {
        x: Some(Fr::from(10u64)),
        y: Some(Fr::from(99u64)),
    };

    print_report(
        "UNDERCONSTRAINED CHECK",
        analyze_circuit(underconstrained_circuit, "Underconstrained Circuit"),
    );
}
