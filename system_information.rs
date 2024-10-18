use sysinfo::{
    Components, Disks, Networks, System,
};
use serde_json::{json, Value};
use socketioxide::extract::{SocketRef, Data};
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use tracing::{debug, info};
use horizon_data_types::*;

pub fn init(socket: SocketRef) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let system_info = json!({
        "memory": {
            "total": sys.total_memory(),
            "used": sys.used_memory(),
            "total_swap": sys.total_swap(),
            "used_swap": sys.used_swap(),
        },
        "system": {
            "name": System::name(),
            "kernel_version": System::kernel_version(),
            "cpu_count": sys.cpus().len(),
            "os_version": System::os_version(),
            "host_name": System::host_name(),
        },
        "disk": {
           
        },
    });

    socket.emit("system_information", system_info).ok();
}