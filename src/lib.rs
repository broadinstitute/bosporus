mod rt;
mod files;

pub fn launch_shell() {
    let mut shell = rt::Runtime::new();
    shell.interactive();
    println!("Done!");
}