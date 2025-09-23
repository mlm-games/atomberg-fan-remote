#![allow(clippy::needless_return)]

mod ir;
mod profile;
mod ui_handlers;

use jni::JNIEnv;
use ndk_context::android_context;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::profile::{Profile, load_profile};

slint::include_modules!();

// Global active profile
pub static ACTIVE_PROFILE: Lazy<RwLock<Profile>> = 
    Lazy::new(|| RwLock::new(profile::default_renesa_profile()));

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) {
    // Initialize logger

    use crate::ui_handlers::setup_ui_handlers;
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("atomberg-remote")
            .with_max_level(log::LevelFilter::Info),
    );

    // Initialize Slint
    slint::android::init(app).expect("Failed to initialize Slint");
    
    // Create UI
    let ui = MainWindow::new().unwrap();
    
    // Initialize profile and UI
    initialize_app(&ui);

    setup_ui_handlers(&ui);
    
    // Run the application
    ui.run().unwrap();
}

fn initialize_app(ui: &MainWindow) {
    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();
    
    // Load custom profile if available
    load_profile(&mut env);
    
    // Update UI visibility based on profile
    if let Ok(profile) = ACTIVE_PROFILE.read() {
        ui.set_show_sleep(profile.sleep.is_some());
        ui.set_show_led(profile.led.is_some());
        ui.set_show_timer(profile.timer_cycle.is_some());
    }
}

#[cfg(not(target_os = "android"))]
pub fn main() {
    println!("This is an Android-only application. Use `cargo apk build` to build for Android.");
}