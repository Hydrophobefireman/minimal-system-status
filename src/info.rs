use human_bytes::human_bytes;

use sysinfo::{Components, Disks, System};

use crate::{
    common::{CPUStats, ComponentStats, DiskStats, NetworkStats, RamInfo, SysStatus, SystemInfo},
    NETWORKS, SYS,
};

pub fn get_stats() -> SysStatus {
    let mut sys = SYS.lock().unwrap();
    let mut networks = NETWORKS.lock().unwrap();
    sys.refresh_all();

    // RAM and swap information:
    let total_ram = human_bytes(sys.total_memory() as f64);
    let available_ram = human_bytes(sys.used_memory() as f64);

    let ram = RamInfo::new(total_ram, available_ram);

    // system information:
    let system_info = SystemInfo::new(
        System::name().unwrap_or("Unknown".into()),
        System::kernel_version().unwrap_or("Unknown".into()),
        System::os_version().unwrap_or("Unknown".into()),
        System::host_name().unwrap_or("Unknown".into()),
    );

    // Number of CPUs:
    sys.refresh_cpu();
    let cpu_stats = CPUStats::new(sys.cpus().len(), sys.global_cpu_info().cpu_usage());

    // all disks' information:
    let disks = Disks::new_with_refreshed_list();
    let disks = disks
        .into_iter()
        .map(|disk| {
            let space = human_bytes(disk.available_space() as f64);
            let total = human_bytes(disk.total_space() as f64);

            let name = disk.name().to_string_lossy().into();
            let removable = disk.is_removable();
            let mount_point = disk.mount_point().to_string_lossy().into();
            DiskStats::new(space, total, name, removable, mount_point)
        })
        .collect::<Vec<_>>();

    // Network interfaces name, data received and data transmitted:

    networks.refresh_list();
    let networks = networks
        .into_iter()
        .map(|(interface, data)| {
            NetworkStats::new(
                interface.clone(),
                human_bytes(data.total_transmitted() as f64),
                human_bytes(data.total_received() as f64),
            )
        })
        .collect::<Vec<_>>();

    // Components temperature:
    let components = Components::new_with_refreshed_list();

    let component_stats = components
        .into_iter()
        .map(|c| {
            ComponentStats::new(
                c.label().into(),
                format!("{}°C", c.temperature()),
                c.critical()
                    .map(|f| format!("{f}°C"))
                    .unwrap_or("<unknown>".into()),
            )
        })
        .collect::<Vec<_>>();

    SysStatus::new()
        .with_component_stats(component_stats)
        .with_cpu_stats(cpu_stats)
        .with_disk_stats(disks)
        .with_network_stats(networks)
        .with_ram(ram)
        .with_system(system_info)
}
