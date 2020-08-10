use crate::{messages::MessageLevel, Progress, Unit};
use std::time::Duration;

pub struct Log {
    name: String,
    max: Option<usize>,
    unit: Option<Unit>,
    last_set: Option<std::time::SystemTime>,
    step: usize,
    current_level: usize,
    max_level: usize,
}

const EMIT_LOG_EVERY_S: f32 = 0.5;

impl Log {
    pub fn new(name: impl Into<String>, max_level: Option<usize>) -> Self {
        Log {
            name: name.into(),
            current_level: 0,
            max_level: max_level.unwrap_or(usize::MAX),
            max: None,
            step: 0,
            unit: None,
            last_set: None,
        }
    }
}

impl Progress for Log {
    type SubProgress = Log;

    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
        Log {
            name: format!("{}::{}", self.name, Into::<String>::into(name)),
            current_level: self.current_level + 1,
            max_level: self.max_level,
            step: 0,
            max: None,
            unit: None,
            last_set: None,
        }
    }

    fn init(&mut self, max: Option<usize>, unit: Option<Unit>) {
        self.max = max;
        self.unit = unit;
    }

    fn set(&mut self, step: usize) {
        self.step = step;
        if self.current_level > self.max_level {
            return;
        }
        let now = std::time::SystemTime::now();
        if self
            .last_set
            .map(|last| {
                now.duration_since(last)
                    .unwrap_or_else(|_| Duration::default())
                    .as_secs_f32()
            })
            .unwrap_or_else(|| EMIT_LOG_EVERY_S * 2.0)
            > EMIT_LOG_EVERY_S
        {
            self.last_set = Some(now);
            match (self.max, &self.unit) {
                (max, Some(unit)) => log::info!("{} → {}", self.name, unit.display(step, max, None)),
                (Some(max), None) => log::info!("{} → {} / {}", self.name, step, max),
                (None, None) => log::info!("{} → {}", self.name, step),
            }
        }
    }

    fn inc_by(&mut self, step: usize) {
        self.set(self.step + step)
    }

    fn message(&mut self, level: MessageLevel, message: impl Into<String>) {
        let message: String = message.into();
        match level {
            MessageLevel::Info => log::info!("ℹ{} → {}", self.name, message),
            MessageLevel::Failure => log::error!("𐄂{} → {}", self.name, message),
            MessageLevel::Success => log::info!("✓{} → {}", self.name, message),
        }
    }
}
