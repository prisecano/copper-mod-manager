#[cfg(windows)]
fn enable_ansi_support() {
    colored::control::set_virtual_terminal(true).unwrap();
}
#[cfg(not(windows))]
fn enable_ansi_support() {}

#[tokio::main]
async fn main() {
    enable_ansi_support();

    cmm::run().await;
}
