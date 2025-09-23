use slint::{ComponentHandle, SharedString};
use ndk_context::android_context;
use crate::{MainWindow, ACTIVE_PROFILE};
use crate::ir::{IrAction, patterns, transmit_pattern};

pub fn setup_ui_handlers(ui: &MainWindow) {
    // Power button handler
    {
        let weak = ui.as_weak();
        ui.on_power(move || {
            if let Some(ui) = weak.upgrade() {
                execute_ir_command(patterns::POWER_TOGGLE, "Power toggled");
                ui.set_status(SharedString::from("Power toggled"));
            }
        });
    }

    // Speed button handlers
    setup_speed_handler(ui, 1, patterns::SPEED_1);
    setup_speed_handler(ui, 2, patterns::SPEED_2);
    setup_speed_handler(ui, 3, patterns::SPEED_3);
    setup_speed_handler(ui, 4, patterns::SPEED_4);
    setup_speed_handler(ui, 5, patterns::SPEED_5);

    // Boost button handler
    {
        let weak = ui.as_weak();
        ui.on_boost(move || {
            if let Some(ui) = weak.upgrade() {
                execute_ir_command(patterns::BOOST, "Boost mode");
                ui.set_current_speed(0);
                ui.set_status(SharedString::from("Boost mode"));
            }
        });
    }

    // Sleep button handler
    {
        let weak = ui.as_weak();
        ui.on_sleep(move || {
            if let Some(ui) = weak.upgrade() {
                if let Ok(profile) = ACTIVE_PROFILE.read() {
                    if let Some(action) = &profile.sleep {
                        execute_ir_action(action, "Sleep mode");
                        ui.set_status(SharedString::from("Sleep mode"));
                    } else {
                        ui.set_status(SharedString::from("Sleep not configured"));
                    }
                }
            }
        });
    }

    // LED button handler
    {
        let weak = ui.as_weak();
        ui.on_led(move || {
            if let Some(ui) = weak.upgrade() {
                if let Ok(profile) = ACTIVE_PROFILE.read() {
                    if let Some(action) = &profile.led {
                        execute_ir_action(action, "LED toggled");
                        ui.set_status(SharedString::from("LED toggled"));
                    } else {
                        ui.set_status(SharedString::from("LED not configured"));
                    }
                }
            }
        });
    }

    // Single Timer cycle handler
    {
        let weak = ui.as_weak();
        ui.on_timer(move || {
            if let Some(ui) = weak.upgrade() {
                if let Ok(profile) = ACTIVE_PROFILE.read() {
                    if let Some(action) = &profile.timer_cycle {
                        execute_ir_action(action, "Timer cycle");
                        ui.set_status(SharedString::from("Timer cycle (1h→2h→4h→8h→Off)"));
                    } else {
                        ui.set_status(SharedString::from("Timer not configured"));
                    }
                }
            }
        });
    }
}

fn setup_speed_handler(ui: &MainWindow, speed: i32, pattern: &'static [i32]) {
    let weak = ui.as_weak();
    let callback = match speed {
        1 => MainWindow::on_speed_1,
        2 => MainWindow::on_speed_2,
        3 => MainWindow::on_speed_3,
        4 => MainWindow::on_speed_4,
        5 => MainWindow::on_speed_5,
        _ => return,
    };

    callback(ui, move || {
        if let Some(ui) = weak.upgrade() {
            execute_ir_command(pattern, &format!("Speed {}", speed));
            ui.set_current_speed(speed);
            ui.set_status(SharedString::from(format!("Speed {}", speed)));
        }
    });
}

fn execute_ir_command(pattern: &'static [i32], status: &str) {
    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    
    match vm.attach_current_thread() {
        Ok(mut env) => {
            if let Err(e) = transmit_pattern(&mut env, pattern) {
                log::error!("Failed to transmit {}: {}", status, e);
            } else {
                log::info!("Transmitted: {}", status);
            }
        }
        Err(e) => {
            log::error!("Failed to attach JNI thread: {}", e);
        }
    }
}

fn execute_ir_action(action: &IrAction, status: &str) {
    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    
    match vm.attach_current_thread() {
        Ok(mut env) => {
            if let Err(e) = action.send(&mut env) {
                log::error!("Failed to send {}: {}", status, e);
            } else {
                log::info!("Sent: {}", status);
            }
        }
        Err(e) => {
            log::error!("Failed to attach JNI thread: {}", e);
        }
    }
}