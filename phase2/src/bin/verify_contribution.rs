extern crate phase2;
extern crate exitcode;
extern crate itertools;

use itertools::Itertools;
use std::fs::OpenOptions;

use phase2::parameters::*;
use phase2::circom_circuit::circuit_from_json_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: \n<in_circuit.json> <in_old_params.params> <in_new_params.params>");
        std::process::exit(exitcode::USAGE);
    }
    let circuit_filename = &args[1];
    let old_params_filename = &args[2];
    let new_params_filename = &args[3];

    let disallow_points_at_infinity = false;

    let old_reader = OpenOptions::new()
                                .read(true)
                                .open(old_params_filename)
                                .expect("unable to open old params");
    let old_params = MPCParameters::read(old_reader, disallow_points_at_infinity, true).expect("unable to read old params");

    let new_reader = OpenOptions::new()
                                .read(true)
                                .open(new_params_filename)
                                .expect("unable to open new params");
    let new_params = MPCParameters::read(new_reader, disallow_points_at_infinity, true).expect("unable to read new params");

    println!("Checking contribution {}...", new_params_filename);
    let contribution = verify_contribution(&old_params, &new_params).expect("should verify");

    let should_filter_points_at_infinity = false;
    let verification_result = new_params.verify(circuit_from_json_file(&circuit_filename), should_filter_points_at_infinity).unwrap();
    assert!(contains_contribution(&verification_result, &contribution));
    println!("Contribution {} verified.\nContribution hash: 0x{:02x}", new_params_filename, contribution.iter().format(""));
}
