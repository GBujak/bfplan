use rand::random;

pub fn probability(last_energy: f32, new_energy: f32, temperature: f32) -> f32 {
    f32::exp(-(new_energy - last_energy) / temperature)
}

pub fn temperature(time: f32) -> f32 {
    f32::max(0.0, 1.0 - time)
}

pub fn should_accept_state(last_energy: f32, new_energy: f32, temperature: f32) -> bool {
    if new_energy < last_energy {
        true
    } else {
        let r = random::<f32>();
        let probability = probability(last_energy, new_energy, temperature);
        probability >= r
    }
}
