use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, System, SystemExt};

use actix_web::{web::Json, Result};

pub async fn get_sys_info() -> Result<Json<SysInfo>> {
    let info = SysInfo::get();
    Ok(Json(info))
}

#[derive(Serialize, Deserialize)]
pub struct SysInfo {
    cpu: Cpuinfo,
    memory: MemoryInfo,
    system: SystemInfo,
}

#[derive(Serialize, Deserialize)]
pub struct Cpuinfo {
    core_count: usize,
    brand: String,
    frequency_mhz: u64,
    verdor_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryInfo {
    total: u64,
    used: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    name: Option<String>,
    kernel_version: Option<String>,
    host_name: Option<String>,
}

static SYSINFO: OnceLock<RwLock<System>> = OnceLock::new();

fn sys_write() -> RwLockWriteGuard<'static, System> {
    let sys = SYSINFO.get_or_init(|| RwLock::new(System::new_all()));
    sys.write().unwrap()
}

fn sys_read() -> RwLockReadGuard<'static, System> {
    let sys = SYSINFO.get_or_init(|| RwLock::new(System::new_all()));
    sys.read().unwrap()
}

impl SysInfo {
    pub fn get() -> SysInfo {
        Self {
            cpu: Cpuinfo::get(),
            memory: MemoryInfo::get(),
            system: SystemInfo::get(),
        }
    }
}

impl Cpuinfo {
    pub fn get() -> Self {
        let mut sys = sys_write();
        sys.refresh_cpu();
        let cpu = sys.global_cpu_info();
        Self {
            core_count: sys.cpus().len(),
            brand: cpu.brand().to_string(),
            frequency_mhz: cpu.frequency(),
            verdor_id: cpu.vendor_id().to_string(),
        }
    }
}

impl MemoryInfo {
    pub fn get() -> Self {
        let mut sys = sys_write();
        sys.refresh_memory();
        Self {
            total: sys.total_memory(),
            used: sys.used_memory(),
        }
    }
}

impl SystemInfo {
    pub fn get() -> Self {
        let sys = sys_read();
        Self {
            name: sys.name(),
            kernel_version: sys.kernel_version(),
            host_name: sys.host_name(),
        }
    }
}
