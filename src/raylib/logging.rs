use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use vsprintf::vsprintf;

/// Direct mapping of raylib's log levels
/// See: https://github.com/raysan5/raylib/blob/d875891a3c2621ab40733ca3569eca9e054a6506/parser/raylib_api.json#L985-L1026
#[derive(FromPrimitive)]
enum RaylibLogLevel {
    All = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    Fatal = 6,
    None = 7,
}

#[no_mangle]
unsafe extern "C" fn rl_log_callback(
    level: i32,
    message: *const i8,
    args: *mut super::ffi::__va_list_tag,
) {
    // Format the message
    let formatted_message = vsprintf(message, args);

    // If all is well, we can continue logging
    if let Ok(formatted_message) = formatted_message {
        // Convert the string prefix to a target if possible
        let mut target = "raylib";
        let final_message;
        if formatted_message.starts_with("RLGL: ") {
            target = "raylib::rlgl";
            final_message =
                std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("RLGL: "));
        }
        // else if formatted_message.starts_with("DISPLAY: ") {
        //     target = "raylib::display";
        //     final_message = std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("DISPLAY: "));
        // }
        // else if formatted_message.starts_with("GL: ") {
        //     target = "raylib::gl";
        //     final_message = std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("GL: "));
        // }
        else if formatted_message.starts_with("GLAD: ") {
            target = "raylib::glad";
            final_message =
                std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("GLAD: "));
        } else if formatted_message.starts_with("TEXTURE: ") {
            target = "raylib::texture";
            final_message =
                std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("TEXTURE: "));
        } else if formatted_message.starts_with("SHADER: ") {
            target = "raylib::shader";
            final_message =
                std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("SHADER: "));
        } else if formatted_message.starts_with("FONT: ") {
            target = "raylib::font";
            final_message =
                std::borrow::Cow::Borrowed(formatted_message.trim_start_matches("FONT: "));
        } else {
            final_message = std::borrow::Cow::Borrowed(&formatted_message);
        }

        // Handle the log level and fall back on info!
        match RaylibLogLevel::from_u32(level as u32) {
            Some(level) => match level {
                RaylibLogLevel::Trace => log::trace!(target: target, "{}", final_message),
                RaylibLogLevel::Debug => log::debug!(target: target, "{}", final_message),
                RaylibLogLevel::Warning => log::warn!(target: target, "{}", final_message),
                RaylibLogLevel::Error => log::error!(target: target, "{}", final_message),
                RaylibLogLevel::Fatal => log::error!(target: target, "{}", final_message),
                _ => log::info!(target: target, "{}", final_message),
            },
            None => {
                log::info!(target:"raylib", "{}", final_message)
            }
        }
    }
}

/// Configures raylib to use our custom logging code
pub unsafe fn rl_use_rust_logging() {
    log::debug!("Configuring raylib to use the `log` crate");
    super::ffi::SetTraceLogCallback(Some(rl_log_callback));
}
