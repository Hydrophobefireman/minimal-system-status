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

    let ram = {
        // RAM and swap information:
        let total_ram_num = sys.total_memory();
        let used_ram_num = sys.used_memory();
        let total_ram = human_bytes(total_ram_num as f64);
        let used_ram = human_bytes(used_ram_num as f64);
        RamInfo::new(total_ram, used_ram, total_ram_num, used_ram_num)
    };

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
            let available_space_num = disk.available_space();
            let total_space_num = disk.total_space();
            let space = human_bytes(available_space_num as f64);
            let total = human_bytes(total_space_num as f64);

            let name = disk.name().to_string_lossy().into();
            let removable = disk.is_removable();
            let mount_point = disk.mount_point().to_string_lossy().into();
            DiskStats::new(
                space,
                total,
                name,
                removable,
                mount_point,
                available_space_num,
                total_space_num,
            )
        })
        .collect::<Vec<_>>();

    // Network interfaces name, data received and data transmitted:

    networks.refresh_list();
    let networks = networks
        .into_iter()
        .map(|(interface, data)| {
            let total_transmitted_num = data.total_transmitted();
            let total_received_num = data.total_received();
            NetworkStats::new(
                interface.clone(),
                human_bytes(total_transmitted_num as f64),
                human_bytes(total_received_num as f64),
                total_transmitted_num,
                total_received_num,
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
