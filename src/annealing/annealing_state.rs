#[derive(Debug, Default)]
pub struct AnnealingState {
    iteration: usize,
    max_iterations: usize,
    temperature: f32,
}

impl AnnealingState {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            iteration: 0,
            max_iterations,
            temperature: 1.0,
        }
    }

    pub fn should_accept_state(&mut self, last_energy: f32, new_energy: f32) -> bool {
        use super::annealing_functions::should_accept_state as should_accept;
        should_accept(last_energy, new_energy, self.temperature)
    }

    pub fn do_step(&mut self) {
        use super::annealing_functions::temperature;
        self.iteration += 1;
        self.temperature = temperature(self.iteration as f32 / self.max_iterations as f32);
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }
}
