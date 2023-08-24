use std::os::unix::process::CommandExt;

use actix_web::Result;
// use tokio::process::Command;

pub async fn uninstall() -> Result<&'static str> {
    // let mut cmd = Command::new("sh");
    // cmd.arg("/etc/ambassador/uninstall.sh");
    // cmd.spawn()?;

    let mut cmd = std::process::Command::new("sh");
    cmd.process_group(10000);
    cmd.arg("/etc/ambassador/uninstall.sh");
    cmd.spawn()?;
    Ok("")
}
