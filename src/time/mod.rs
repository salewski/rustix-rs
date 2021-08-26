//! Time-related operations.

use crate::imp;

#[cfg(not(target_os = "redox"))]
mod clock;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod timerfd;

// TODO: Convert WASI'S clock APIs to use handles rather than ambient clock
// identifiers, update `wasi-libc`, and then add support in `rustix`.
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use clock::{clock_getres, clock_gettime, clock_gettime_dynamic, ClockId, DynamicClockId};
#[cfg(not(target_os = "redox"))]
pub use clock::{nanosleep, NanosleepRelativeResult};

#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub use clock::{clock_nanosleep_absolute, clock_nanosleep_relative};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use timerfd::{timerfd_create, timerfd_gettime, timerfd_settime};

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use imp::time::{Itimerspec, TimerfdFlags, TimerfdTimerFlags};
pub use imp::time::{Nsecs, Secs, Timespec};
