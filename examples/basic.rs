use corosensei::{Coroutine, Yielder};
use std::os::unix::thread::JoinHandleExt;

fn main() {
    let join = std::thread::spawn(|| {
        println!("thread works");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let mut coroutine = Coroutine::new(|_: &Yielder<(), ()>, ()| {
            println!("[coroutine] exiting coroutine");
        });
        // The core code in `switch_and_link` is assembly. When the `t1` thread executes to
        // the assembly part of `switch_and_link`, the `t1` is interrupted by `pthread_kill`
        // at the same timestamp, can the `switch_and_link` be restored normally in the future?
        coroutine.resume(());
    });
    // The `t1` thread will obviously be interrupted by a signal during execution,
    // and then continue to execute after the signal processing function `sigurg_handler` ends
    let t1 = join.as_pthread_t();
    extern "C" fn sigurg_handler(_: libc::c_int) {
        println!("sigurg handler works");
    }
    unsafe {
        assert_eq!(
            0,
            libc::sigaction(
                libc::SIGURG,
                &libc::sigaction {
                    sa_sigaction: sigurg_handler as usize,
                    sa_mask: 0,
                    sa_flags: libc::SA_RESTART,
                },
                std::ptr::null_mut()
            )
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
        libc::pthread_kill(t1, libc::SIGURG);
    }
    join.join().unwrap()
}

#[test]
fn basic() {
    main()
}
