use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};

const NO_ERROR: u8 = ErrorKind::NoError as u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ErrorKind {
    NoError = 0,
    DeviceDisconnected,
    StreamError,
    ClockDriftUnrecoverable,
}

impl ErrorKind {
    fn from_code(code: u8) -> Option<Self> {
        match code {
            x if x == Self::NoError as u8 => None,
            x if x == Self::DeviceDisconnected as u8 => Some(Self::DeviceDisconnected),
            x if x == Self::StreamError as u8 => Some(Self::StreamError),
            x if x == Self::ClockDriftUnrecoverable as u8 => {
                Some(Self::ClockDriftUnrecoverable)
            }
            _ => None,
        }
    }
}

pub struct AudioStatus {
    code: AtomicU8,
    dropped_errors: AtomicU32,
    underruns: AtomicU64,
    overflows: AtomicU64,
}

impl AudioStatus {
    pub const fn new() -> Self {
        Self {
            code: AtomicU8::new(NO_ERROR),
            dropped_errors: AtomicU32::new(0),
            underruns: AtomicU64::new(0),
            overflows: AtomicU64::new(0),
        }
    }

    /// Clears the pending error and every counter. Called when a new routing is
    /// applied so the telemetry describes the current streams, not the whole
    /// process lifetime.
    pub fn reset(&self) {
        self.code.store(NO_ERROR, Ordering::Release);
        self.dropped_errors.store(0, Ordering::Relaxed);
        self.underruns.store(0, Ordering::Relaxed);
        self.overflows.store(0, Ordering::Relaxed);
    }

    pub fn set_error(&self, kind: ErrorKind) {
        if kind == ErrorKind::NoError {
            return;
        }

        if self
            .code
            .compare_exchange(
                NO_ERROR,
                kind as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_err()
        {
            self.dropped_errors.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn take_error(&self) -> Option<ErrorKind> {
        ErrorKind::from_code(self.code.swap(NO_ERROR, Ordering::AcqRel))
    }

    pub fn dropped_errors(&self) -> u32 {
        self.dropped_errors.load(Ordering::Relaxed)
    }

    pub fn take_dropped_errors(&self) -> u32 {
        self.dropped_errors.swap(0, Ordering::AcqRel)
    }

    pub fn note_underrun(&self) {
        self.underruns.fetch_add(1, Ordering::Relaxed);
    }

    pub fn note_overflow(&self) {
        self.overflows.fetch_add(1, Ordering::Relaxed);
    }

    pub fn underruns(&self) -> u64 {
        self.underruns.load(Ordering::Relaxed)
    }

    pub fn overflows(&self) -> u64 {
        self.overflows.load(Ordering::Relaxed)
    }
}

impl Default for AudioStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{AudioStatus, ErrorKind};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn set_and_take_error_round_trip() {
        let status = AudioStatus::default();

        status.set_error(ErrorKind::DeviceDisconnected);

        assert_eq!(
            status.take_error(),
            Some(ErrorKind::DeviceDisconnected)
        );
    }

    #[test]
    fn take_error_returns_none_when_clean() {
        let status = AudioStatus::default();

        assert_eq!(status.take_error(), None);
    }

    #[test]
    fn concurrent_set_and_take_accounts_for_every_error() {
        const PRODUCERS: usize = 8;
        const ERRORS_PER_PRODUCER: usize = 10_000;

        let status = Arc::new(AudioStatus::default());
        let producers_done = Arc::new(AtomicBool::new(false));

        let consumer_status = Arc::clone(&status);
        let consumer_done = Arc::clone(&producers_done);
        let consumer = thread::spawn(move || {
            let mut taken = 0_u64;
            while !consumer_done.load(Ordering::Acquire) {
                if consumer_status.take_error().is_some() {
                    taken += 1;
                }
                thread::yield_now();
            }
            while consumer_status.take_error().is_some() {
                taken += 1;
            }
            taken
        });

        let producers: Vec<_> = (0..PRODUCERS)
            .map(|_| {
                let status = Arc::clone(&status);
                thread::spawn(move || {
                    for _ in 0..ERRORS_PER_PRODUCER {
                        status.set_error(ErrorKind::StreamError);
                    }
                })
            })
            .collect();

        for producer in producers {
            producer.join().unwrap();
        }
        producers_done.store(true, Ordering::Release);

        let taken = consumer.join().unwrap();
        let total = (PRODUCERS * ERRORS_PER_PRODUCER) as u64;
        assert_eq!(taken + status.dropped_errors() as u64, total);
    }
}
