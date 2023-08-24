use std::{io::Write, net::IpAddr};

use anyhow::{bail, Result};
use rexpect::{reader::Regex, ReadUntil};
use tempfile::NamedTempFile;
use tokio::{io::AsyncReadExt, process::Command};
use tracing::{debug, warn};

pub struct Node {
    ip: IpAddr,
    password: String,
}

pub struct SysMetrics {
    pub cpu_usage: f64,
    pub mem_usage: f64,
}

pub struct EventPublisher {}

impl Node {
    pub fn new(ip: IpAddr, password: String) -> Self {
        Self { ip, password }
    }

    pub async fn install(&self) -> Result<()> {
        ssh_copy_id(&self.ip, "root", &self.password).await?;
        install(self.ip, "root").await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        todo!()
    }

    pub async fn uninstall(&self) -> Result<()> {
        todo!()
    }

    pub async fn restart(&self) -> Result<()> {
        todo!()
    }

    pub async fn update_yourself(&self) -> Result<()> {
        todo!()
    }

    pub async fn stream(self) -> Result<EventPublisher> {
        todo!()
    }

    pub async fn get_sys_info(&self) -> Result<()> {
        todo!()
    }
}

use crate::log_err_ctx;

async fn ssh_copy_id(host: &IpAddr, user: &str, password: &str) -> Result<()> {
    let remote = format!("{}@{}", user, host);
    // run `ssh-keygen -R to` to remove the old known_hosts entry
    run_script(&format!("ssh-keygen -R {remote}")).await?;

    let p_ssh = format!("ssh-copy-id -f {}", remote);
    let mut p_ssh = log_err_ctx!(rexpect::spawn(&p_ssh, Some(5_000)), remote);
    log_err_ctx!({
        p_ssh.exp_any(vec![
            ReadUntil::Regex(Regex::new("continue connecting.*").unwrap()),
            ReadUntil::EOF
        ])
        p_ssh.send_line("yes")

        p_ssh.exp_any(vec![
            ReadUntil::Regex(Regex::new("'s password.*").unwrap()),
            ReadUntil::EOF
        ])
        p_ssh.send_line(password)
        p_ssh.exp_eof()
    }, remote);

    Ok(())
}

async fn install(host: IpAddr, user: &str) -> Result<()> {
    debug!(%host, user, "install on remote");
    let ssh_dst = format!("{user}@{host}");
    #[cfg(debug_assertions)]
    let pack_path = "./target/ambassador-pack";
    #[cfg(not(debug_assertions))]
    let pack_path = "./ambassador-pack";

    let script = format!(
        r#"
        ssh {ssh_dst} "mkdir -p ~/ambassador-pack"
        scp -r {pack_path}/* {ssh_dst}:~/ambassador-pack/
        ssh {ssh_dst} "sh ~/ambassador-pack/install.sh"
        "#
    );
    run_script(script).await?;

    Ok(())
}

async fn run_script(script: impl AsRef<str>) -> anyhow::Result<()> {
    let script = script.as_ref();
    println!("{}", script);

    let mut file = NamedTempFile::new()?;
    file.write_all(script.as_bytes())?;
    let path = file.path();

    let mut cmd = Command::new("sh");
    cmd.arg(path);
    cmd.stdout(std::process::Stdio::piped());

    let mut cmd = log_err_ctx!(cmd.spawn(), script);

    let p = log_err_ctx!(cmd.wait().await, script);
    if !p.success() {
        let mut std_out = cmd.stdout.take().unwrap();
        let mut out_buf = String::new();
        std_out.read_to_string(&mut out_buf).await?;
        warn!(out_buf, "cmd failed");
        bail!("cmd failed: {}", script)
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn aa() -> anyhow::Result<()> {
        let script = r#"
        ssh root@10.0.10.59 "mkdir -p ~/ambassador-pack"
        scp -r ./target/ambassador-pack/* root@10.0.10.59:~/ambassador-pack/
        ssh root@10.0.10.59 "sh ~/ambassador-pack/install.sh"
        "#;
        run_script(script).await?;
        Ok(())
    }
}
