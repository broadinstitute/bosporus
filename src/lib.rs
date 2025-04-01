mod shell;
mod files;

pub fn launch_shell() {
    let mut shell = shell::Shell::new();
    shell.run();
    println!("Done!");
}