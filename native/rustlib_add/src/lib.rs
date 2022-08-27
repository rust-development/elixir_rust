use std::{thread, time};

static mut FOO: i32 = 0;

fn wait_for(delay: u64) {
    let ten_millis = time::Duration::from_millis(delay);
    thread::sleep(ten_millis);
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    unsafe {
        FOO += 2;
        println!("{}", FOO);
    }
    a + b + 4
}

#[rustler::nif]
fn start() {
    thread::spawn(move || {
        wait_for(100);
        println!("{}", "start new thread!!!");
        wait_for(100);
        println!("{}", "line 1");
        wait_for(100);
        println!("{}", "line 2");
        wait_for(100);
        println!("{}", "line 3");
    });

}

rustler::init!("Elixir.ElixirRust", [add, start]);
