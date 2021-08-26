use rustix::time::{
    timerfd_create, timerfd_gettime, timerfd_settime, ClockId, Itimerspec, TimerfdFlags,
    TimerfdTimerFlags, Timespec,
};

#[test]
fn test_timerfd() {
    let fd = timerfd_create(ClockId::Monotonic, TimerfdFlags::CLOEXEC).unwrap();

    let set = Itimerspec {
        it_interval: Timespec {
            tv_sec: 3,
            tv_nsec: 4,
        },
        it_value: Timespec {
            tv_sec: 5,
            tv_nsec: 6,
        },
    };
    let _old: Itimerspec = timerfd_settime(&fd, TimerfdTimerFlags::ABSTIME, &set).unwrap();

    let new = timerfd_gettime(&fd).unwrap();

    assert_eq!(set.it_interval.tv_sec, new.it_interval.tv_sec);
    assert_eq!(set.it_interval.tv_nsec, new.it_interval.tv_nsec);
    assert!(new.it_value.tv_sec <= set.it_value.tv_sec);
    assert!(
        set.it_value.tv_nsec <= new.it_value.tv_nsec || set.it_value.tv_sec < new.it_value.tv_sec
    );
}
