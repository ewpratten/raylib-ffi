//! Quality of life macros 

/// Begins drawing, creates a new scope, executes code, then cleans up
/// 
/// ## Example
/// ```no_run
/// # use raylib_ffi::draw;
/// # fn main(){ unsafe {
/// draw!({
/// // Graphics code here..
/// });
/// # }}
/// ```
#[macro_export]
macro_rules! draw {
    ($expression:expr) => {
        raylib_ffi::BeginDrawing();
        {
            $expression
        }
        raylib_ffi::EndDrawing();
    };
}

/// Converts a string to `*const i8` for use with raylib
#[macro_export]
macro_rules! rl_str {
    ($expression:expr) => {
        format!("{}\0", $expression).as_ptr() as *const i8
    };
}