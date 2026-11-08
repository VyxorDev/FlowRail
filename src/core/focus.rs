//! Deterministic, UI-independent focus-session state machine.
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusPhase {
    Idle,
    Running,
    Paused,
    Complete,
}

#[derive(Debug)]
pub struct FocusTimer {
    duration: Duration,
    remaining_when_paused: Duration,
    running_since: Option<Instant>,
    phase: FocusPhase,
}

impl FocusTimer {
    pub fn new(duration: Duration) -> Self {
        assert!(!duration.is_zero(), "focus duration must be non-zero");
        Self {
            duration,
            remaining_when_paused: duration,
            running_since: None,
            phase: FocusPhase::Idle,
        }
    }

    pub fn phase(&self) -> FocusPhase {
        self.phase
    }

    pub fn start(&mut self, now: Instant) {
        self.remaining_when_paused = self.duration;
        self.running_since = Some(now);
        self.phase = FocusPhase::Running;
    }

    pub fn pause(&mut self, now: Instant) {
        if self.phase == FocusPhase::Running {
            self.remaining_when_paused = self.remaining(now);
            self.running_since = None;
            self.phase = FocusPhase::Paused;
        }
    }

    pub fn resume(&mut self, now: Instant) {
        if self.phase == FocusPhase::Paused {
            self.running_since = Some(now);
            self.phase = FocusPhase::Running;
        }
    }

    pub fn reset(&mut self) {
        self.remaining_when_paused = self.duration;
        self.running_since = None;
        self.phase = FocusPhase::Idle;
    }

    pub fn remaining(&self, now: Instant) -> Duration {
        if self.phase == FocusPhase::Complete {
            return Duration::ZERO;
        }
        match self.running_since {
            Some(start) if self.phase == FocusPhase::Running => {
                self.remaining_when_paused
                    .saturating_sub(now.saturating_duration_since(start))
            }
            _ => self.remaining_when_paused,
        }
    }

    pub fn progress(&self, now: Instant) -> f32 {
        1.0 - (self.remaining(now).as_secs_f32() / self.duration.as_secs_f32())
    }

    /// Returns true once, when a running session completes.
    pub fn tick(&mut self, now: Instant) -> bool {
        if self.phase == FocusPhase::Running && self.remaining(now).is_zero() {
            self.running_since = None;
            self.remaining_when_paused = Duration::ZERO;
            self.phase = FocusPhase::Complete;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_completes_exactly_once() {
        let now = Instant::now();
        let mut timer = FocusTimer::new(Duration::from_secs(60));
        timer.start(now);
        assert!(!timer.tick(now + Duration::from_secs(59)));
        assert!(timer.tick(now + Duration::from_secs(60)));
        assert!(!timer.tick(now + Duration::from_secs(61)));
        assert_eq!(timer.phase(), FocusPhase::Complete);
    }

    #[test]
    fn pause_and_resume_preserve_remaining_time() {
        let now = Instant::now();
        let mut timer = FocusTimer::new(Duration::from_secs(100));
        timer.start(now);
        timer.pause(now + Duration::from_secs(30));
        assert_eq!(timer.remaining(now + Duration::from_secs(80)).as_secs(), 70);
        timer.resume(now + Duration::from_secs(90));
        assert_eq!(timer.remaining(now + Duration::from_secs(100)).as_secs(), 60);
    }

    #[test]
    fn reset_clears_session() {
        let now = Instant::now();
        let mut timer = FocusTimer::new(Duration::from_secs(90));
        timer.start(now);
        timer.reset();
        assert_eq!(timer.phase(), FocusPhase::Idle);
        assert_eq!(timer.remaining(now).as_secs(), 90);
    }
}
