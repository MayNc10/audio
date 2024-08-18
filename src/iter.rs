pub trait AudioIter {
    fn tick(&mut self) -> f32;

    fn compute_block<const N: usize>(&mut self) -> [f32; N] {
        let mut arr = [0.0; N];
        for entry in &mut arr {
            *entry = self.tick();
        }
        arr
    }
}
