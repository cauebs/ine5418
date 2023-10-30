use std::{io::Write, thread, time::Duration};

use anyhow::Result;
use serde::Serialize;

pub fn serialize_into_and_flush<W: Write, T: Serialize>(conn: &mut W, contents: &T) -> Result<()> {
    bincode::serialize_into(conn.by_ref(), contents)?;
    conn.flush()?;
    Ok(())
}

pub struct ExponentialBackoff {
    initial_wait: Duration,
    wait: Duration,
    increasing_factor: u32,
}

impl ExponentialBackoff {
    pub fn new(initial_wait: Duration, increasing_factor: u32) -> Self {
        Self {
            initial_wait,
            wait: Duration::ZERO,
            increasing_factor,
        }
    }
}

impl Iterator for ExponentialBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        thread::sleep(self.wait);

        if self.wait.is_zero() {
            self.wait = self.initial_wait;
        } else {
            self.wait *= self.increasing_factor;
        }

        Some(self.wait)
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self::new(Duration::from_secs(1), 2)
    }
}
