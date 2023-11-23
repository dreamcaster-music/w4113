//GRANULIZER
pub struct Granulizer {
    pub grain_start: f32,
    pub grain_end: f32,
    pub grain_duration: f32,
    pub grain_pitch: f32,
    pub grain_out: f32,
    pub grain_envelope: GrainEnvelope,
}

//break down sample input into grains
impl Granulize for Granulizer {
    fn granulize(&self) -> f32 {
        let grain_start = self.grain_start;
        let grain_end = self.grain_end;
        let grain_duration = self.grain_duration;
        let grain_pitch = self.grain_pitch;
        let grain_out = self.grain_out;
        let grain_envelope = self.grain_envelope;

        let grain = Grain {
            grain_start,
            grain_end,
            grain_duration,
            grain_pitch,
            grain_out,
            grain_envelope,
        };

        grain.granulize()
    }
}

//BEST ONE https://github.com/backtail/granulator-rs
//https://www.youtube.com/watch?v=Z4P5f6ZJ_nE
//https://github.com/PatrickWulfe/Granulizor/tree/master/src
//https://github.com/topics/granular-synthesis?l=rust

//grain start point high / low

// grain end point high / low

// grain duration ms

// grain pitch (interval ratio 12tones)

// grain out

// Envelope Formulas
//expr 1*(((sin(($i1)-255.5)*1/1))/(1*((1*$i1)-255.5)))
//expr 5*(sin((3.14*$i1)-255.5)/(1*((1*$i1)-255.5)))
//expr exp(-0.5*pow(($i1-((512-1)/2))/(0.4*((512-1)/2)),2))
//sinc
pub enum GrainEnvelope {
    Sine,
    Triangle,
    Gaussian,
    Sinc,
}

//https://www.youtube.com/watch?v=fJUmmcGKZMI
//frequency domain transform

//  pub struct FreqDom {}

//input waveform
//use granulizer rs
impl Granulize for FreqDom {}

//break the input waveform into small chuncks
//block based processing, tapering small chunks of audio to zero with a sign function so they start and end at the same place (sin(nt)).
//  imply Granulizer for FreqDom {

//imply Effect for FreqDom {}

//overlap and add
//Phase compensation for previous blocks - each one is shifted slightly more than previous.
//N = FFT length (samples), t = reference-time offset (samples), f = integer frequency index. Equation is [f] = e^(2*pi*i*f*t/N)

//output waveform

//controls - frequency
