use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use ndk_context::android_context;

// IR constants
pub const FREQ_HZ: i32 = 38_000;

// NEC protocol timings
const HDR_MARK: i32 = 9000;
const HDR_SPACE: i32 = 4500;
const BIT_MARK: i32 = 560;
const ONE_SPACE: i32 = 1690;
const ZERO_SPACE: i32 = 560;
const TRAIL_MARK: i32 = 560;

/// Pattern definitions
/// Source: WGLabz OpenHAB/Tasmota capture (Atomberg BLDC)
/// NEC ~38kHz patterns
pub mod patterns {
    pub const FREQ_HZ: i32 = 38_000;
    pub const POWER_TOGGLE: &[i32] = &[
        8930, 4420, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520,
        580, 1670, 580, 1620, 580, 570, 580, 520, 580, 1670, 580, 1620, 580, 1670, 580, 1670, 580,
        1620, 580, 570, 580, 520, 580, 570, 580, 1620, 580, 570, 580, 520, 580, 1670, 580, 520, 580,
        1670, 580, 1670, 580, 1620, 580, 570, 580, 1620, 580, 1670, 580, 520, 580,
    ];
    pub const SPEED_1: &[i32] = &[
        8880, 4420, 580, 570, 580, 520, 580, 570, 530, 570, 580, 570, 530, 570, 580, 520, 580, 570,
        580, 1670, 530, 1670, 580, 570, 580, 520, 580, 1620, 630, 1620, 580, 1670, 580, 1670, 530,
        1670, 580, 1620, 630, 520, 580, 1620, 630, 520, 580, 570, 580, 520, 580, 1620, 630, 520, 580,
        570, 580, 1620, 580, 570, 580, 1620, 580, 1670, 580, 1620, 630, 520, 580,
    ];
    pub const SPEED_2: &[i32] = &[
        8930, 4420, 580, 520, 580, 570, 580, 520, 630, 520, 580, 520, 580, 520, 630, 520, 580, 570,
        580, 1620, 580, 1670, 580, 520, 580, 570, 580, 1620, 580, 1670, 580, 1670, 580, 1620, 630, 520,
        580, 520, 580, 570, 580, 520, 580, 1670, 580, 520, 630, 520, 580, 1620, 580, 1670, 580, 1670,
        580, 1620, 580, 1670, 580, 520, 580, 1670, 580, 1670, 580, 520, 580,
    ];
    pub const SPEED_3: &[i32] = &[
        8930, 4420, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 630, 520, 580, 520,
        580, 1670, 580, 1620, 630, 520, 580, 520, 580, 1670, 580, 1620, 580, 1670, 580, 1670, 580, 520,
        580, 1670, 580, 520, 580, 1670, 580, 520, 580, 570, 580, 520, 580, 1670, 580, 1670, 530, 570,
        580, 1620, 630, 520, 580, 1670, 580, 1620, 580, 1670, 580, 520, 580,
    ];
    pub const SPEED_4: &[i32] = &[
        8930, 4370, 630, 520, 580, 520, 630, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 520,
        630, 1620, 580, 1670, 580, 520, 580, 520, 630, 1620, 630, 1620, 580, 1670, 580, 1620, 580,
        1670, 580, 1620, 630, 520, 580, 570, 580, 1620, 580, 570, 580, 520, 580, 1670, 580, 520, 580,
        570, 580, 1620, 580, 1670, 580, 520, 580, 1670, 580, 1620, 630, 520, 580,
    ];
    pub const SPEED_5: &[i32] = &[
        8930, 4420, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 520,
        630, 1620, 580, 1670, 580, 520, 630, 470, 630, 1620, 580, 1670, 580, 1670, 580, 1620, 630, 470,
        630, 520, 580, 520, 630, 1620, 580, 520, 630, 520, 580, 520, 630, 1620, 580, 1670, 580, 1670,
        580, 1620, 580, 520, 630, 1620, 580, 1670, 580, 1670, 580, 520, 580,
    ];
    pub const BOOST: &[i32] = &[
        8930, 4420, 580, 570, 580, 520, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570,
        580, 1620, 580, 1670, 580, 570, 580, 520, 580, 1670, 580, 1620, 580, 1670, 580, 1620, 580,
        1670, 580, 1670, 580, 1620, 580, 1670, 580, 520, 580, 570, 580, 520, 580, 1670, 580, 520, 580,
        570, 580, 520, 580, 570, 580, 1670, 580, 1620, 580, 1670, 580, 520, 580,
    ];

    // Extra Atomberg (NEC 38kHz) raw patterns from the same capture set (address 0xF300) 
    // Timer: cycles the built-in timer mode on many handsets.
    pub const TIMER_CYCLE: &[i32] = &[
        8880, 4420, 630, 520, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520,
        580, 1670, 580, 1670, 580, 520, 580, 520, 630, 1620, 580, 1670, 580, 1620, 580, 1670, 580,
        520, 580, 1670, 580, 1620, 630, 520, 580, 1670, 580, 520, 580, 520, 630, 1620, 580, 1670,
        580, 520, 580, 520, 630, 1620, 580, 570, 580, 1620, 630, 1620, 580, 520, 630,
    ];

    // OSC (breeze mode), may not exist on all Atomberg remotes.
    pub const OSC_MODE: &[i32] = &[
        8930, 4420, 580, 570, 580, 520, 580, 570, 580, 520, 580, 570, 580, 520, 580, 520, 630, 520,
        580, 1670, 580, 1670, 530, 570, 580, 520, 580, 1670, 580, 1670, 580, 1620, 580, 1670, 580,
        1670, 580, 1620, 580, 1670, 580, 520, 580, 1670, 580, 520, 580, 570, 580, 1620, 580, 570,
        580, 520, 580, 570, 580, 1620, 580, 570, 580, 1670, 530, 1670, 580, 570, 580,
    ];

    // Sleep mode
    pub const SLEEP_MODE: &[i32] = &[
        8930, 4370, 630, 520, 580, 520, 630, 520, 630, 520, 580, 520, 580, 570, 580, 520, 580, 520,
        630, 1620, 630, 1620, 580, 520, 630, 470, 630, 1620, 630, 1620, 630, 1620, 580, 1620, 630,
        520, 580, 1620, 580, 1670, 580, 1620, 630, 520, 580, 570, 580, 520, 580, 1670, 580, 1620,
        630, 520, 580, 520, 580, 570, 580, 1620, 630, 1620, 580, 1620, 630, 520, 580,
    ];
}

pub enum IrAction {
    RawStatic(&'static [i32]),
    Raw(Vec<i32>),
    Nec { addr: u16, cmd: u8 },
}

impl IrAction {
    pub fn send(&self, env: &mut JNIEnv) -> jni::errors::Result<()> {
        match self {
            IrAction::RawStatic(pattern) => transmit_pattern(env, pattern),
            IrAction::Raw(pattern) => transmit_pattern(env, pattern.as_slice()),
            IrAction::Nec { addr, cmd } => transmit_nec(env, *addr, *cmd),
        }
    }
}

pub fn nec_encode_extended(address: u16, command: u8) -> Vec<i32> {
    let mut pattern = Vec::with_capacity(68);
    
    // Header
    pattern.push(HDR_MARK);
    pattern.push(HDR_SPACE);
    
    // Address (16 bits, LSB first)
    for i in 0..16 {
        pattern.push(BIT_MARK);
        pattern.push(if ((address >> i) & 1) != 0 { ONE_SPACE } else { ZERO_SPACE });
    }
    
    // Command (8 bits, LSB first)
    for i in 0..8 {
        pattern.push(BIT_MARK);
        pattern.push(if ((command >> i) & 1) != 0 { ONE_SPACE } else { ZERO_SPACE });
    }
    
    // Inverted command (8 bits, LSB first)
    let inv_cmd = !command;
    for i in 0..8 {
        pattern.push(BIT_MARK);
        pattern.push(if ((inv_cmd >> i) & 1) != 0 { ONE_SPACE } else { ZERO_SPACE });
    }
    
    // Trailer
    pattern.push(TRAIL_MARK);
    
    pattern
}

pub fn transmit_pattern(env: &mut JNIEnv, pattern: &[i32]) -> jni::errors::Result<()> {
    let mgr = get_ir_manager(env)?;
    
    // Check if IR emitter is available
    let has_emitter = env
        .call_method(mgr.as_obj(), "hasIrEmitter", "()Z", &[])?
        .z()?;
        
    if !has_emitter {
        log::warn!("No IR emitter found on this device");
        return Ok(());
    }
    
    // Create pattern array
    let arr = env.new_int_array(pattern.len() as i32)?;
    env.set_int_array_region(&arr, 0, pattern)?;
    
    // Transmit
    env.call_method(
        mgr.as_obj(),
        "transmit",
        "(I[I)V",
        &[JValue::Int(FREQ_HZ), JValue::Object(&JObject::from(arr))],
    )?;
    
    log::debug!("Transmitted IR pattern with {} elements", pattern.len());
    Ok(())
}

pub fn transmit_nec(env: &mut JNIEnv, address: u16, command: u8) -> jni::errors::Result<()> {
    let pattern = nec_encode_extended(address, command);
    transmit_pattern(env, &pattern)
}

fn get_ir_manager(env: &mut JNIEnv) -> jni::errors::Result<GlobalRef> {
    let ctx = android_context();
    let context_obj = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    
    let class_ctx = env.find_class("android/content/Context")?;
    let service_name = env
        .get_static_field(class_ctx, "CONSUMER_IR_SERVICE", "Ljava/lang/String;")?
        .l()?;
    
    let mgr = env
        .call_method(
            context_obj,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&service_name)],
        )?
        .l()?;
    
    env.new_global_ref(mgr)
}