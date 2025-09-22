#![allow(clippy::needless_return)]

use jni::objects::{GlobalRef, JObject, JValue, JString};
use jni::JNIEnv;
use ndk_context::android_context;
use slint::SharedString;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::RwLock;


enum IrAction {
    RawStatic(&'static [i32]),
    Raw(Vec<i32>),
    Nec { addr: u16, cmd: u8 },
}

#[derive(Default)]
struct Profile {
    // extra actions only; core power/speeds/boost you already wired
    sleep: Option<IrAction>,
    led: Option<IrAction>,
    timer_1h: Option<IrAction>,
    timer_2h: Option<IrAction>,
    timer_4h: Option<IrAction>,
    // convenience: if handset has only a single "Timer" cycle button
    timer_cycle: Option<IrAction>,
}

static ACTIVE_PROFILE: Lazy<RwLock<Profile>> = Lazy::new(|| RwLock::new(default_renesa_profile()));

// Minimal JSON spec for custom profile
#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
enum IrActionSpec {
    Raw { pattern: Vec<i32> },
    Nec { address: u16, command: u8 },
}

#[derive(Deserialize, Default)]
struct ProfileSpec {
    sleep: Option<IrActionSpec>,
    led: Option<IrActionSpec>,
    timer_1h: Option<IrActionSpec>,
    timer_2h: Option<IrActionSpec>,
    timer_4h: Option<IrActionSpec>,
    timer_cycle: Option<IrActionSpec>,
}

fn to_action(spec: IrActionSpec) -> IrAction {
    match spec {
        IrActionSpec::Raw { pattern } => IrAction::Raw(pattern),
        IrActionSpec::Nec { address, command } => IrAction::Nec { addr: address, cmd: command },
    }
}

fn default_renesa_profile() -> Profile {
    // Use the extra raw patterns we added earlier in your file (see below)
    Profile {
        sleep: Some(IrAction::RawStatic(SLEEP_MODE)),
        led: None, // hidden by default per your request
        // Renesa: timers usually cycle; map all to the same action
        timer_1h: Some(IrAction::RawStatic(TIMER_CYCLE)),
        timer_2h: Some(IrAction::RawStatic(TIMER_CYCLE)),
        timer_4h: Some(IrAction::RawStatic(TIMER_CYCLE)),
        timer_cycle: Some(IrAction::RawStatic(TIMER_CYCLE)),
    }
}

fn send_action(env: &mut JNIEnv, a: &IrAction) -> jni::errors::Result<()> {
    match a {
        IrAction::RawStatic(p) => transmit_pattern(env, p),
        IrAction::Raw(v) => transmit_pattern(env, v.as_slice()),
        IrAction::Nec { addr, cmd } => transmit_nec(env, *addr, *cmd),
    }
}

// Try load files/profile.json from app's internal storage; if present, override ACTIVE_PROFILE
fn try_load_custom_profile(env: &mut JNIEnv) {
    if let Some(dir) = files_dir_path(env) {
        let path = format!("{}/profile.json", dir);
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(spec) = serde_json::from_str::<ProfileSpec>(&content) {
                let mut p = Profile::default();
                p.sleep = spec.sleep.map(to_action);
                p.led = spec.led.map(to_action);
                p.timer_1h = spec.timer_1h
                    .or(spec.timer_cycle.clone())
                    .map(to_action);
                p.timer_2h = spec.timer_2h
                    .or(spec.timer_cycle.clone())
                    .map(to_action);
                p.timer_4h = spec.timer_4h
                    .or(spec.timer_cycle.clone())
                    .map(to_action);
                p.timer_cycle = spec.timer_cycle.map(to_action);
                if let Ok(mut guard) = ACTIVE_PROFILE.write() {
                    *guard = p;
                }
            }
        }
    }
}

// Get /data/data/<pkg>/files via Context.getFilesDir().getAbsolutePath()
fn files_dir_path(env: &mut JNIEnv) -> Option<String> {
    let ctx = android_context();
    let context_obj = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    let file_obj = env
        .call_method(context_obj, "getFilesDir", "()Ljava/io/File;", &[])
        .ok()?
        .l()
        .ok()?;
    let path_j = env
        .call_method(file_obj, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?;
    let s = env.get_string(&path_j.into()).ok()?.into();
    Some(s)
}

slint::include_modules!();
// Source: WGLabz OpenHAB/Tasmota capture (Atomberg BLDC)

// NEC ~38kHz patterns
const FREQ_HZ: i32 = 38_000;
const POWER_TOGGLE: &[i32] = &[
    8930, 4420, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520,
    580, 1670, 580, 1620, 580, 570, 580, 520, 580, 1670, 580, 1620, 580, 1670, 580, 1670, 580,
    1620, 580, 570, 580, 520, 580, 570, 580, 1620, 580, 570, 580, 520, 580, 1670, 580, 520, 580,
    1670, 580, 1670, 580, 1620, 580, 570, 580, 1620, 580, 1670, 580, 520, 580,
];
const SPEED_1: &[i32] = &[
    8880, 4420, 580, 570, 580, 520, 580, 570, 530, 570, 580, 570, 530, 570, 580, 520, 580, 570,
    580, 1670, 530, 1670, 580, 570, 580, 520, 580, 1620, 630, 1620, 580, 1670, 580, 1670, 530,
    1670, 580, 1620, 630, 520, 580, 1620, 630, 520, 580, 570, 580, 520, 580, 1620, 630, 520, 580,
    570, 580, 1620, 580, 570, 580, 1620, 580, 1670, 580, 1620, 630, 520, 580,
];
const SPEED_2: &[i32] = &[
    8930, 4420, 580, 520, 580, 570, 580, 520, 630, 520, 580, 520, 580, 520, 630, 520, 580, 570,
    580, 1620, 580, 1670, 580, 520, 580, 570, 580, 1620, 580, 1670, 580, 1670, 580, 1620, 630, 520,
    580, 520, 580, 570, 580, 520, 580, 1670, 580, 520, 630, 520, 580, 1620, 580, 1670, 580, 1670,
    580, 1620, 580, 1670, 580, 520, 580, 1670, 580, 1670, 580, 520, 580,
];
const SPEED_3: &[i32] = &[
    8930, 4420, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 630, 520, 580, 520,
    580, 1670, 580, 1620, 630, 520, 580, 520, 580, 1670, 580, 1620, 580, 1670, 580, 1670, 580, 520,
    580, 1670, 580, 520, 580, 1670, 580, 520, 580, 570, 580, 520, 580, 1670, 580, 1670, 530, 570,
    580, 1620, 630, 520, 580, 1670, 580, 1620, 580, 1670, 580, 520, 580,
];
const SPEED_4: &[i32] = &[
    8930, 4370, 630, 520, 580, 520, 630, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 520,
    630, 1620, 580, 1670, 580, 520, 580, 520, 630, 1620, 630, 1620, 580, 1670, 580, 1620, 580,
    1670, 580, 1620, 630, 520, 580, 570, 580, 1620, 580, 570, 580, 520, 580, 1670, 580, 520, 580,
    570, 580, 1620, 580, 1670, 580, 520, 580, 1670, 580, 1620, 630, 520, 580,
];
const SPEED_5: &[i32] = &[
    8930, 4420, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 520,
    630, 1620, 580, 1670, 580, 520, 630, 470, 630, 1620, 580, 1670, 580, 1670, 580, 1620, 630, 470,
    630, 520, 580, 520, 630, 1620, 580, 520, 630, 520, 580, 520, 630, 1620, 580, 1670, 580, 1670,
    580, 1620, 580, 520, 630, 1620, 580, 1670, 580, 1670, 580, 520, 580,
];
const BOOST: &[i32] = &[
    8930, 4420, 580, 570, 580, 520, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570,
    580, 1620, 580, 1670, 580, 570, 580, 520, 580, 1670, 580, 1620, 580, 1670, 580, 1620, 580,
    1670, 580, 1670, 580, 1620, 580, 1670, 580, 520, 580, 570, 580, 520, 580, 1670, 580, 520, 580,
    570, 580, 520, 580, 570, 580, 1670, 580, 1620, 580, 1670, 580, 520, 580,
];

// Extra Atomberg (NEC 38kHz) raw patterns from the same capture set (address 0xF300) 
// Timer: cycles the built-in timer mode on many handsets.
const TIMER_CYCLE: &[i32] = &[
    8880, 4420, 630, 520, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520,
    580, 1670, 580, 1670, 580, 520, 580, 520, 630, 1620, 580, 1670, 580, 1620, 580, 1670, 580,
    520, 580, 1670, 580, 1620, 630, 520, 580, 1670, 580, 520, 580, 520, 630, 1620, 580, 1670,
    580, 520, 580, 520, 630, 1620, 580, 570, 580, 1620, 630, 1620, 580, 520, 630,
];

// OSC (breeze mode), may not exist on all Atomberg remotes.
const OSC_MODE: &[i32] = &[
    8930, 4420, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 520, 630, 520,
    580, 1670, 580, 1670, 530, 570, 580, 520, 580, 1670, 580, 1670, 580, 1620, 580, 1670, 580,
    1670, 580, 1620, 580, 1670, 580, 520, 580, 1670, 580, 520, 580, 570, 580, 1620, 580, 570,
    580, 520, 580, 570, 580, 1620, 580, 570, 580, 1670, 530, 1670, 580, 570, 580,
];

// Sleep mode
const SLEEP_MODE: &[i32] = &[
    8930, 4370, 630, 520, 580, 520, 630, 520, 630, 520, 580, 520, 580, 570, 580, 520, 580, 520,
    630, 1620, 630, 1620, 580, 520, 630, 470, 630, 1620, 630, 1620, 630, 1620, 580, 1620, 630,
    520, 580, 1620, 580, 1670, 580, 1620, 630, 520, 580, 570, 580, 520, 580, 1670, 580, 1620,
    630, 520, 580, 520, 580, 570, 580, 1620, 630, 1620, 580, 1620, 630, 520, 580,
];

// Get a GlobalRef to ConsumerIrManager to avoid JNIEnv aliasing
fn ir_manager(env: &mut JNIEnv) -> jni::errors::Result<GlobalRef> {
    let ctx = android_context();

    // Wrap as a non-owning JObject; don't delete on drop
    let context_obj = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

    let class_ctx = env.find_class("android/content/Context")?;
    let svc_const = env
        .get_static_field(class_ctx, "CONSUMER_IR_SERVICE", "Ljava/lang/String;")?
        .l()?;

    let mgr_local = env
        .call_method(
            context_obj,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&svc_const)],
        )?
        .l()?;

    env.new_global_ref(mgr_local)
}

fn nec_encode_extended(address: u16, command: u8) -> Vec<i32> {
    const HDR_MARK: i32 = 9000;
    const HDR_SPACE: i32 = 4500;
    const BIT_MARK: i32 = 560;
    const ONE_SPACE: i32 = 1690;
    const ZERO_SPACE: i32 = 560;
    const TRAIL_MARK: i32 = 560;

    #[inline]
    fn push_bit(out: &mut Vec<i32>, bit: bool) {
        out.push(BIT_MARK);
        out.push(if bit { ONE_SPACE } else { ZERO_SPACE });
    }

    let mut out: Vec<i32> = Vec::with_capacity(2 + 32 * 2 + 1);

    // Header
    out.push(HDR_MARK);
    out.push(HDR_SPACE);

    // 16-bit address, LSB first
    for i in 0..16 {
        push_bit(&mut out, ((address >> i) & 1) != 0);
    }
    // 8-bit command, LSB first
    for i in 0..8 {
        push_bit(&mut out, ((command >> i) & 1) != 0);
    }
    // 8-bit inverted command, LSB first
    let inv = !command;
    for i in 0..8 {
        push_bit(&mut out, ((inv >> i) & 1) != 0);
    }

    // Trailer
    out.push(TRAIL_MARK);
    out
}

fn transmit_nec(env: &mut jni::JNIEnv, address: u16, command: u8) -> jni::errors::Result<()> {
    let pattern = nec_encode_extended(address, command);
    transmit_pattern(env, &pattern)
}

fn transmit_pattern(env: &mut JNIEnv, pattern: &[i32]) -> jni::errors::Result<()> {
    let mgr = ir_manager(env)?;

    let has = env
        .call_method(mgr.as_obj(), "hasIrEmitter", "()Z", &[])?
        .z()?;
    if !has {
        log::warn!("No IR emitter found");
        return Ok(());
    }

    let arr = env.new_int_array(pattern.len() as i32)?;
    // set_int_array_region takes ownership of the JIntArray argument; clone the handle so we can still pass it to Java below.
    env.set_int_array_region(&arr, 0, pattern)?;
    let arr_obj = JObject::from(arr);

    env.call_method(
        mgr.as_obj(),
        "transmit",
        "(I[I)V",
        &[JValue::Int(FREQ_HZ), JValue::Object(&arr_obj)],
    )?;

    Ok(())
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("atomberg-remote")
            .with_max_level(log::LevelFilter::Info),
    );

    // Initialize Slint Android backend
    slint::android::init(app).expect("slint android init failed");

    let ui = MainWindow::new().unwrap();

    {
        // Attach env once to probe files dir and load custom profile if present
        let ctx = android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let mut env = vm.attach_current_thread().unwrap();
        try_load_custom_profile(&mut env);

        // Set UI visibility flags based on active profile
        if let Ok(g) = ACTIVE_PROFILE.read() {
            ui.set_show_sleep(g.sleep.is_some());
            ui.set_show_led(g.led.is_some());
            ui.set_show_timer_1h(g.timer_1h.is_some());
            ui.set_show_timer_2h(g.timer_2h.is_some());
            ui.set_show_timer_4h(g.timer_4h.is_some());
        }
    }

    // Power button
    {
        let weak = ui.as_weak();
        ui.on_power(move || {
            if let Some(ui) = weak.upgrade() {
                let ctx = android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                let mut env = vm.attach_current_thread().unwrap();

                let _ = transmit_pattern(&mut env, POWER_TOGGLE);
                ui.set_status(SharedString::from("Power toggled"));
            }
        });
    }

    // Sleep
    {
        let w = ui.as_weak();
        ui.on_sleep(move || {
            if let Some(ui) = w.upgrade() {
                let ctx = android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                let mut env = vm.attach_current_thread().unwrap();

                if let Ok(g) = ACTIVE_PROFILE.read() {
                    match &g.sleep {
                        Some(a) => {
                            let _ = send_action(&mut env, a);
                            ui.set_status(SharedString::from("Sleep sent"));
                        }
                        None => ui.set_status(SharedString::from("Missing IR code: Sleep")),
                    }
                }
            }
        });
    }

    // LED
    {
        let w = ui.as_weak();
        ui.on_led(move || {
            if let Some(ui) = w.upgrade() {
                let ctx = android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                let mut env = vm.attach_current_thread().unwrap();

                if let Ok(g) = ACTIVE_PROFILE.read() {
                    match &g.led {
                        Some(a) => {
                            let _ = send_action(&mut env, a);
                            ui.set_status(SharedString::from("LED sent"));
                        }
                        None => ui.set_status(SharedString::from("Missing IR code: LED")),
                    }
                }
            }
        });
    }

    // Timers (each may map to the same cycle action on Renesa)
    for (setter, label) in [
        (MainWindow::on_timer_1h, "Timer"),
    ] {
        let ui_ref = ui.as_weak();
        setter(&ui, move || {
            if let Some(ui) = ui_ref.upgrade() {
                let ctx = android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                let mut env = vm.attach_current_thread().unwrap();

                if let Ok(g) = ACTIVE_PROFILE.read() {
                    // choose the right action or fall back to cycle
                    let act = match label {
                        "Timer 1h" => g.timer_1h.as_ref().or(g.timer_cycle.as_ref()),
                        "Timer 2h" => g.timer_2h.as_ref().or(g.timer_cycle.as_ref()),
                        _ => g.timer_4h.as_ref().or(g.timer_cycle.as_ref()),
                    };
                    match act {
                        Some(a) => {
                            let _ = send_action(&mut env, a);
                            ui.set_status(SharedString::from(label));
                        }
                        None => ui.set_status(SharedString::from(format!("Missing IR code: {}", label))),
                    }
                }
            }
        });
    }

    fn make_sender(
        ui: &MainWindow,
        pattern: &'static [i32],
        label: &'static str,
    ) -> impl Fn() + 'static {
        let weak = ui.as_weak();
        move || {
            if let Some(ui) = weak.upgrade() {
                let ctx = android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                let mut env = vm.attach_current_thread().unwrap();
                let _ = transmit_pattern(&mut env, pattern);
                ui.set_status(SharedString::from(label));
            }
        }
    }

    // Speed/Boost bindings (all literals/consts are 'static)
    ui.on_speed_1(make_sender(&ui, SPEED_1, "Speed 1"));
    ui.on_speed_2(make_sender(&ui, SPEED_2, "Speed 2"));
    ui.on_speed_3(make_sender(&ui, SPEED_3, "Speed 3"));
    ui.on_speed_4(make_sender(&ui, SPEED_4, "Speed 4"));
    ui.on_speed_5(make_sender(&ui, SPEED_5, "Speed 5"));
    ui.on_boost(make_sender(&ui, BOOST, "Boost"));

    ui.run().unwrap();
}

#[cfg(not(target_os = "android"))]
pub fn main() {
    println!("Android cdylib; use `cargo apk build`.");
}
