use corosensei::stack::DefaultStack;
use corosensei::{CoroutineResult, ScopedCoroutine};

fn main() {
    println!("[main] creating coroutine");

    let mut coroutine =
        ScopedCoroutine::<'static, i32, (), i32, (), DefaultStack>::new(|yielder, input| {
            println!("[coroutine] coroutine started with input {}", input);
            for i in 0..5 {
                println!("[coroutine] yielding {}", i);
                yielder.suspend(i);
            }
            println!("[coroutine] exiting coroutine");
        });

    let counter = 100;
    println!("[main] resuming coroutine with argument {}", counter);
    match coroutine.run(counter) {
        CoroutineResult::Yield(i) => println!("[main] got {:?} from coroutine", i),
        CoroutineResult::Return(()) => {}
    }
    loop {
        match coroutine.resume(()) {
            CoroutineResult::Yield(i) => println!("[main] got {:?} from coroutine", i),
            CoroutineResult::Return(()) => break,
        }
    }

    println!("[main] exiting");
}

#[test]
fn basic2() {
    main()
}
