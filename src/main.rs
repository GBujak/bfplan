use std::io::Read;

mod annealing;
mod data_types;
mod illegal_state;
mod input;

use annealing::{adapter::AnnealingAdapter, energy::EnergyWeights};
use input::PlanInput;
use itertools::peek_nth;
use itertools::Itertools;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let plan_input = serde_json::from_str::<PlanInput>(&input).unwrap();

    let annealing_adapter = AnnealingAdapter::of_plan_input(&plan_input);
    let mut buffer = annealing_adapter.create_annealing_buffer();

    dbg!(&buffer);

    buffer.anneal_iterations(
        100_000,
        &EnergyWeights {
            student_gap_weight: 1.0,
            teacher_gap_weight: 1.0,
        },
    );

    dbg!(buffer);
}
