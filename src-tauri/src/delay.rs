use crate::delay_buffer::DelayBuffer;

enum FeedbackSource {
    Internal,
    External,
}

pub struct DelayLine {
    buffer: DelayBuffer,
    feedback_source: FeedbackSource,
    delay_samples: usize,
    internal_feedback: f64,
    wet_dry_ratio: f64,
}

pub fn set_delay_sampes(mut self, delay_samples: usize) {
    self.delay_samples = delay_samples;
}
pub fn set_internal_feedback(mut self, feedback: f64) {
    self.internal_feedback = feedback;
}
pub fn set_wet_dry_ratio(mut self, wet_dry_ratio: f64) {
    self.wet_dry_ratio = wet_dry_ratio;
}

impl DelayLine {
    pub fn new(
        max_delay_samples: usize,
        delay_samples: usize,
        feedback_source: FeedbackSource,
        internal_feedback: f64,
        wet_dry_ratio: f64,
    ) -> Self {
        DelayLine {
            buffer: DelayBuffer::new(max_delay_samples),
            delay_samples,
            feedback_source,
            internal_feedback,
            wet_dry_ratio,
        }
    }

    pub fn process_with_feedback(&mut self, xn: f64, external_feedback: f64) -> (f64, f64) {
        let delay_signal = self.buffer.read(self.delay_samples);
        let internal_feedback_signal = delay_signal + self.internal_feedback;
        let feedback = match self.feedback_source {
            FeedbackSource::Internal => internal_feedback_signal,
            FeedbackSource::External => external_feedback_signal,
        };
        self.buffer.write(xn + feedback);

        let wet = self.wet_dry_ratio;
        let dry = 1 - self.dry_wet_ratio;
        let yn = delay_signal + dry * xn;
        (yn, internal_feedback_signal)
    }
}
