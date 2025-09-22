#![allow(clippy::needless_return)]

use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use ndk_context::android_context;
use slint::SharedString;

slint::include_modules!();

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
