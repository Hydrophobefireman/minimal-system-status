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
}
#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct DiskStats {
    space: String,
    total: String,
    name: String,
    is_removable: bool,
    mount_point: String,
}

#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct CPUStats {
    count: usize,
    usage: f32,
}

#[derive(Debug, Serialize, Deserialize, Default, new)]
pub struct RamInfo {
    total: String,
    available: String,
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
