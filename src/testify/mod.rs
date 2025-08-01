use std::time::Instant;
// use rand::prelude::ThreadRng;

// lazy_static! {
//     pub static ref rng: ThreadRng = ThreadRng::default();
// }

/// Execute a function and print the elapsed time for execution.
/// IF the function returns a result, print the result.
///
pub fn exec_mes(name: &str, f: fn() -> String) {
    let now = Instant::now();
    let res = f();
    println!("[{}] elapsed time: {:.2?}", name, now.elapsed());
    assert_ne!(res, "");
    if print() {
        println!("\t -> value of fn: {res}");
    }
}

fn print() -> bool {
    true
}
