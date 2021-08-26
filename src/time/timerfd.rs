use crate::imp;
use crate::io::{self, OwnedFd};
use crate::time::{ClockId, Itimerspec, TimerfdFlags, TimerfdTimerFlags};
use io_lifetimes::AsFd;

/// `timerfd_create(clockid, flags)`—Create a timer.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/timerfd_create.2.html
#[inline]
pub fn timerfd_create(clockid: ClockId, flags: TimerfdFlags) -> io::Result<OwnedFd> {
    Ok(imp::syscalls::timerfd_create(clockid, flags)?.into())
}

/// `timerfd_settime(clockid, flags, new_value)`—Set the time on a timer.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/timerfd_settime.2.html
#[inline]
pub fn timerfd_settime<Fd: AsFd>(
    fd: &Fd,
    flags: TimerfdTimerFlags,
    new_value: &Itimerspec,
) -> io::Result<Itimerspec> {
    let fd = fd.as_fd();
    imp::syscalls::timerfd_settime(fd, flags, new_value)
}

/// `timerfd_gettime(clockid, flags)`—Query a timer.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html
#[inline]
pub fn timerfd_gettime<Fd: AsFd>(fd: &Fd) -> io::Result<Itimerspec> {
    let fd = fd.as_fd();
    imp::syscalls::timerfd_gettime(fd)
}
