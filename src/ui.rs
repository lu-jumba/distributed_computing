#[tauri::command]
fn set_resource_limits(ram_limit: u64, storage_limit: u64, cpu_limit: u64, bandwidth_limit: u64) {
    println!("Setting limits: RAM: {}MB, Storage: {}GB, CPU: {}%, Bandwidth: {}Mbps",
        ram_limit, storage_limit, cpu_limit, bandwidth_limit);
    // Call the Node and ResourceManager functions here
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_resource_limits])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
