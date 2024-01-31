use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct ComponentStats {
    label: String,
    temp: String,
    critical: String,
}
#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct NetworkStats {
    name: String,
    sent: String,
    received: String,
    sent_num: u64,
    received_num: u64,
}
#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct DiskStats {
    available_space: String,
    total_space: String,
    name: String,
    is_removable: bool,
    mount_point: String,
    available_space_num: u64,
    total_space_num: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct CPUStats {
    count: usize,
    usage: f32,
}

#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct RamInfo {
    total: String,
    used: String,
    total_num: u64,
    used_num: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct SystemInfo {
    name: String,
    kernel_version: String,
    os_version: String,
    hostname: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SysStatus {
    ram: RamInfo,
    system: SystemInfo,
    cpu_stats: CPUStats,
    disks: Vec<DiskStats>,
    networks: Vec<NetworkStats>,
    components: Vec<ComponentStats>,
}

impl SysStatus {
    pub fn new() -> SysStatus {
        Default::default()
    }
    pub fn with_ram(self, ram: RamInfo) -> SysStatus {
        SysStatus {
            ram,
            cpu_stats: self.cpu_stats,
            system: self.system,
            disks: self.disks,
            networks: self.networks,
            components: self.components,
        }
    }
    pub fn with_system(self, system: SystemInfo) -> SysStatus {
        SysStatus {
            ram: self.ram,
            cpu_stats: self.cpu_stats,
            disks: self.disks,
            networks: self.networks,
            components: self.components,
            system,
        }
    }
    pub fn with_cpu_stats(self, cpu_stats: CPUStats) -> SysStatus {
        SysStatus {
            ram: self.ram,
            system: self.system,
            disks: self.disks,
            networks: self.networks,
            components: self.components,
            cpu_stats,
        }
    }
    pub fn with_disk_stats(self, disks: Vec<DiskStats>) -> SysStatus {
        SysStatus {
            ram: self.ram,
            system: self.system,
            cpu_stats: self.cpu_stats,
            networks: self.networks,
            components: self.components,
            disks,
        }
    }
    pub fn with_network_stats(self, networks: Vec<NetworkStats>) -> SysStatus {
        SysStatus {
            ram: self.ram,
            system: self.system,
            cpu_stats: self.cpu_stats,
            disks: self.disks,
            components: self.components,
            networks: networks,
        }
    }
    pub fn with_component_stats(self, components: Vec<ComponentStats>) -> SysStatus {
        SysStatus {
            ram: self.ram,
            system: self.system,
            cpu_stats: self.cpu_stats,
            disks: self.disks,
            networks: self.networks,
            components: components,
        }
    }
}
