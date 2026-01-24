#[cfg(windows)]
fn enable_ansi_support() {
    colored::control::set_virtual_terminal(true).unwrap();
}

#[tokio::main]
async fn main() {
    enable_ansi_support();

    cmm::run().await;
}
