use rusty_cogs_lib::run;

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}
