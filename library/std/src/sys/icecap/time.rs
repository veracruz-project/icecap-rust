#![allow(dead_code)]

use crate::cmp::Ordering;
use crate::time::Duration;
use core::hash::{Hash, Hasher};

use super::icecap_impl as imp;

#[derive(Copy, Clone, Debug)]
struct Timespec;

impl Timespec {
    const fn zero() -> Timespec {
        Timespec
    }

    fn sub_timespec(&self, _other: &Timespec) -> Result<Duration, Duration> {
        unsupported!()
    }

    fn checked_add_duration(&self, _other: &Duration) -> Option<Timespec> {
        unsupported!()
    }

    fn checked_sub_duration(&self, _other: &Duration) -> Option<Timespec> {
        unsupported!()
    }
}

impl PartialEq for Timespec {
    fn eq(&self, _other: &Timespec) -> bool {
        unsupported!()
    }
}

impl Eq for Timespec {}

impl PartialOrd for Timespec {
    fn partial_cmp(&self, _other: &Timespec) -> Option<Ordering> {
        unsupported!()
    }
}

impl Ord for Timespec {
    fn cmp(&self, _other: &Timespec) -> Ordering {
        unsupported!()
    }
}

impl Hash for Timespec {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        unsupported!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant;

impl Instant {
    pub fn now() -> Instant {
        Instant
    }

    pub const fn zero() -> Instant {
        Instant
    }

    pub fn actually_monotonic() -> bool {
        false
    }

    pub fn checked_sub_instant(&self, _other: &Instant) -> Option<Duration> {
        Some(Duration::from_secs(0))
    }

    pub fn checked_add_duration(&self, _other: &Duration) -> Option<Instant> {
        Some(Instant)
    }

    pub fn checked_sub_duration(&self, _other: &Duration) -> Option<Instant> {
        Some(Instant)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

impl SystemTime {
    pub fn now() -> SystemTime {
        SystemTime(imp::now())
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}
