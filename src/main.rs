use std::io::Read;

mod annealing;
mod data_types;
mod illegal_state;
mod input;

use input::PlanInput;
use itertools::peek_nth;
use itertools::Itertools;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let plan_input = serde_json::from_str::<PlanInput>(&input).unwrap();

    let possible_lessons = plan_input.possible_lessons().unwrap();

    let possible_plans = possible_lessons
        .iter()
        .map(|x| x.cartesian_product_iter())
        .multi_cartesian_product();

    println!("Ilość planów: {}", possible_plans.clone().count());

    //possible_plans.clone().for_each(drop);

    let plan = peek_nth(possible_plans)
        .peek_nth(usize::MAX - 100)
        .unwrap()
        .clone();
    println!("{}", serde_json::to_string_pretty(&plan).unwrap());

    // for plan in possible_plans {
    //     println!("{}", serde_json::to_string_pretty(&plan).unwrap());
    // }
}
