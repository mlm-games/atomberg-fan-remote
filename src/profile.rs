use std::path;

use serde::{Deserialize, Serialize};
use jni::JNIEnv;
use jni::objects::JString;
use jni::objects::JObject;
use crate::ir::{IrAction, patterns};
use crate::ACTIVE_PROFILE;

#[derive(Default)]
pub struct Profile {
    pub sleep: Option<IrAction>,
    pub led: Option<IrAction>,
    pub timer_1h: Option<IrAction>,
    pub timer_2h: Option<IrAction>,
    pub timer_4h: Option<IrAction>,
    pub timer_cycle: Option<IrAction>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
enum IrActionSpec {
    Raw { pattern: Vec<i32> },
    Nec { address: u16, command: u8 },
}

#[derive(Serialize, Deserialize, Default)]
struct ProfileSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    sleep: Option<IrActionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    led: Option<IrActionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timer_1h: Option<IrActionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timer_2h: Option<IrActionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timer_4h: Option<IrActionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timer_cycle: Option<IrActionSpec>,
}

impl From<IrActionSpec> for IrAction {
    fn from(spec: IrActionSpec) -> Self {
        match spec {
            IrActionSpec::Raw { pattern } => IrAction::Raw(pattern),
            IrActionSpec::Nec { address, command } => IrAction::Nec { addr: address, cmd: command },
        }
    }
}

pub fn default_renesa_profile() -> Profile {
    Profile {
        sleep: Some(IrAction::RawStatic(patterns::SLEEP_MODE)),
        led: None,
        timer_1h: Some(IrAction::RawStatic(patterns::TIMER_CYCLE)),
        timer_2h: Some(IrAction::RawStatic(patterns::TIMER_CYCLE)),
        timer_4h: Some(IrAction::RawStatic(patterns::TIMER_CYCLE)),
        timer_cycle: Some(IrAction::RawStatic(patterns::TIMER_CYCLE)),
    }
}

pub fn load_profile(env: &mut JNIEnv) {
    if let Some(profile_path) = get_profile_path(env) {
        match std::fs::read_to_string(&profile_path) {
            Ok(content) => {
                match serde_json::from_str::<ProfileSpec>(&content) {
                    Ok(spec) => {
                        let profile = Profile {
                            sleep: spec.sleep.map(Into::into),
                            led: spec.led.map(Into::into),
                            timer_1h: spec.timer_1h
                                .or(spec.timer_cycle.clone())
                                .map(Into::into),
                            timer_2h: spec.timer_2h
                                .or(spec.timer_cycle.clone())
                                .map(Into::into),
                            timer_4h: spec.timer_4h
                                .or(spec.timer_cycle.clone())
                                .map(Into::into),
                            timer_cycle: spec.timer_cycle.map(Into::into),
                        };
                        
                        if let Ok(mut guard) = ACTIVE_PROFILE.write() {
                            *guard = profile;
                            log::info!("Loaded custom profile from {}", profile_path);
                        }
                    }
                    Err(e) => log::error!("Failed to parse profile: {}", e),
                }
            }
            Err(e) => log::debug!("No custom profile found: {}", e),
        }
    }
}

fn get_profile_path(env: &mut JNIEnv) -> Option<String> {
    use ndk_context::android_context;
    
    let ctx = android_context();
    let context_obj = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    
    let file_obj = env
        .call_method(context_obj, "getFilesDir", "()Ljava/io/File;", &[])
        .ok()?
        .l()
        .ok()?;
        
    let path_jstring = env
        .call_method(file_obj, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?;
    
    let path_string: String = env
        .get_string(&JString::from(path_jstring))
        .ok()?
        .into();
        
    Some(format!("{}/profile.json", path_string))
}