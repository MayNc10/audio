use crate::iter::AudioIter;

pub struct Envelope<A, E: FnMut(u32) -> f32> {
    iter: A,
    tick: u32,
    envelope: E,
}

impl<A, E: FnMut(u32) -> f32> Envelope<A, E> {
    pub fn new(iter: A, envelope: E) -> Envelope<A, E> {
        Envelope {
            iter,
            tick: 0,
            envelope,
        }
    }
}

impl<A, E: FnMut(u32) -> f32> AudioIter for Envelope<A, E>
where
    A: AudioIter,
{
    fn tick(&mut self) -> f32 {
        self.tick += 1;
        (self.envelope)(self.tick - 1)
    }
}

fn lerp(step: u32, end: u32, base: Option<f32>, peak: f32) -> f32 {
    let percent = (step as f32) / (end as f32);
    let base = base.unwrap_or(0.0);
    base + percent * (peak - base)
}

/// We can't make a "normal" ADSR because we dont really have a concept of a "note", only a waveform
/// Therefore, we dont know when a note on sustain should release
/// "H", or "Hold", controls this time
pub fn adhsr(
    attack: u32,
    decay: u32,
    sustain: u32,
    hold: f32,
    release: u32,
    peak_scale: Option<f32>,
) -> Box<dyn FnMut(u32) -> f32> {
    Box::new(move |step| {
        let b2 = attack + decay;
        let b3 = b2 + sustain;
        let b4 = b3 + release;
        let peak = peak_scale.unwrap_or(1.0);

        if step < attack {
            lerp(step, attack, None, peak)
        } else if step < b2 {
            lerp(step - attack, decay, Some(peak), hold)
        } else if step < b3 {
            hold
        } else if step < b4 {
            lerp(step - b3, release, Some(hold), 0.0)
        } else {
            0.0
        }
    })
}
