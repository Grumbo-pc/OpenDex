use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf, process::Command, sync::{atomic::{AtomicBool, Ordering}, Arc}};
#[cfg(windows)] use windows::{core::PCWSTR, Win32::{Foundation::RECT, UI::WindowsAndMessaging::{FindWindowW, GetClientRect}}};
#[derive(Serialize)] struct Device { serial: String, state: String, model: String }
#[derive(Deserialize)] struct Settings { adb_path: String, scrcpy_path: String, resolution: String, max_fps: u16, bitrate: String, desktop_mode: bool, keep_awake: bool }
fn binary_name(name: &str) -> String { if cfg!(windows) { format!("{name}.exe") } else { name.into() } }
fn bundled(name: &str) -> Option<PathBuf> { let file = binary_name(name); let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tools/scrcpy").join(&file); if dev.is_file() { return Some(dev); } let installed = env::current_exe().ok()?.parent()?.join("resources").join("resources/scrcpy").join(file); installed.is_file().then_some(installed) }
fn exe(config: Option<&str>, name: &str) -> Result<PathBuf, String> {
    if let Some(path) = config.filter(|p| !p.trim().is_empty()) {
        let configured = PathBuf::from(path);
        let resolved = if configured.is_dir() { configured.join(binary_name(name)) } else { configured };
        if resolved.is_file() { return Ok(resolved); }
        return Err(format!("{} does not point to {}.", path, binary_name(name)));
    }
    if let Some(path) = bundled(name) { return Ok(path); }
    which::which(name).map_err(|_| format!("{name} was not found. Configure its path in Settings."))
}
fn adb_executable(config: Option<&str>) -> Result<PathBuf, String> { exe(config, "adb") }
fn adb(path: Option<&str>, serial: Option<&str>, args: &[&str]) -> Result<String, String> { let mut command = Command::new(adb_executable(path)?); if let Some(serial) = serial { command.arg("-s").arg(serial); } let output = command.args(args).output().map_err(|e| format!("Could not run adb: {e}"))?; if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).trim().into()); } Ok(String::from_utf8_lossy(&output.stdout).trim().into()) }
fn newest_display_id(output: &str) -> Option<String> { output.lines().filter_map(|line| line.split_once("mDisplayId=")?.1.split_whitespace().next()?.parse::<u32>().ok()).max().filter(|id| *id > 0).map(|id| id.to_string()) }
#[cfg(windows)] fn scrcpy_size() -> Option<(u32, u32)> { let title: Vec<u16> = "OpenDex - Android Desktop\0".encode_utf16().collect(); let window = unsafe { FindWindowW(None, PCWSTR(title.as_ptr())) }.ok()?; let mut rect = RECT::default(); unsafe { GetClientRect(window, &mut rect).ok()?; } let width = (rect.right - rect.left).max(320) as u32; let height = (rect.bottom - rect.top).max(240) as u32; Some((width & !1, height & !1)) }
#[cfg(windows)] fn watch_scrcpy_resize(adb_path: Option<String>, serial: String, display_id: String, stop: Arc<AtomicBool>) { std::thread::spawn(move || { let mut last_size = None; while !stop.load(Ordering::Relaxed) { if let Some((width, height)) = scrcpy_size() { let size = format!("{width}x{height}"); if last_size.as_deref() != Some(size.as_str()) { let _ = adb(adb_path.as_deref(), Some(&serial), &["shell", "wm", "size", "-d", &display_id, &size]); last_size = Some(size); } } std::thread::sleep(std::time::Duration::from_millis(250)); } }); }
#[tauri::command] fn list_devices(adb_path: Option<String>) -> Result<Vec<Device>, String> { let output = adb(adb_path.as_deref(), None, &["devices", "-l"])?; Ok(output.lines().skip(1).filter_map(|line| { let mut parts = line.split_whitespace(); let serial = parts.next()?.into(); let state = parts.next()?.into(); let details: Vec<_> = parts.collect(); let model = details.iter().find_map(|part| part.strip_prefix("model:")).unwrap_or("").replace('_', " "); Some(Device { serial, state, model }) }).collect()) }
#[tauri::command] fn launch_session(serial: String, settings: Settings) -> Result<String, String> {
    let adb_path = (!settings.adb_path.is_empty()).then_some(settings.adb_path.as_str());
    let adb_bin = adb_executable(adb_path)?;
    if settings.desktop_mode {
        for (key, value) in [("enable_freeform_support", "1"), ("force_desktop_mode_on_external_displays", "1"), ("enable_non_resizable_multi_window", "1"), ("window_animation_scale", "1"), ("transition_animation_scale", "1"), ("animator_duration_scale", "1")] {
            adb(adb_path, Some(&serial), &["shell", "settings", "put", "global", key, value])?;
        }
    }
    if settings.keep_awake && !settings.desktop_mode { let _ = adb(adb_path, Some(&serial), &["shell", "svc", "power", "stay_on", "3"]); }
    let resolution = if settings.resolution.contains('x') { settings.resolution.clone() } else { "1664x936".into() };
    if settings.desktop_mode {
        let overlay = format!("{resolution}/160");
        adb(adb_path, Some(&serial), &["shell", "settings", "put", "global", "overlay_display_devices", &overlay])?;
    }
    let display_id = if settings.desktop_mode {
        let id = (0..30).find_map(|_| {
            let displays = adb(adb_path, Some(&serial), &["shell", "dumpsys", "display"]).ok()?;
            let id = newest_display_id(&displays);
            if id.is_none() { std::thread::sleep(std::time::Duration::from_millis(100)); }
            id
        }).ok_or_else(|| "Android did not create the desktop display.".to_string())?;
        adb(adb_path, Some(&serial), &["shell", "wm", "set-display-windowing-mode", "-d", &id, "5"])?;
        id
    } else {
        "0".into()
    };
    let fps = settings.max_fps.to_string();
    let mut command = Command::new(exe(Some(&settings.scrcpy_path), "scrcpy")?);
    command.env("ADB", adb_bin).args(["--serial", &serial, "--display-id", &display_id, "--turn-screen-off", "--display-ime-policy=hide", "--max-fps", &fps, "--video-bit-rate", &settings.bitrate, "--window-title", "OpenDex - Android Desktop", "--push-target=/sdcard/Download/", "--keyboard=uhid", "--mouse=uhid"]);
    let mut child = command.spawn().map_err(|e| format!("Could not start scrcpy: {e}"))?;
    if settings.desktop_mode {
        let cleanup_adb = adb_path.map(str::to_owned);
        let cleanup_serial = serial.clone();
        let stop_resize = Arc::new(AtomicBool::new(false));
        #[cfg(windows)] watch_scrcpy_resize(adb_path.map(str::to_owned), serial.clone(), display_id.clone(), Arc::clone(&stop_resize));
        std::thread::spawn(move || {
            let _ = child.wait();
            stop_resize.store(true, Ordering::Relaxed);
            let _ = adb(cleanup_adb.as_deref(), Some(&cleanup_serial), &["shell", "wm", "size", "-d", &display_id, "reset"]);
            let _ = adb(cleanup_adb.as_deref(), Some(&cleanup_serial), &["shell", "settings", "put", "global", "overlay_display_devices", "null"]);
        });
    }
    Ok("Desktop session started.".into())
}
fn main() { tauri::Builder::default().invoke_handler(tauri::generate_handler![list_devices, launch_session]).run(tauri::generate_context!()).expect("error while running OpenDex"); }
