use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref VERBOSE: AtomicBool = AtomicBool::new(false);
    static ref QUIET: AtomicBool = AtomicBool::new(false);
}

/// Sets the global verbose flag.
///
/// # Arguments
///
/// * `verbose` - If true, enables verbose mode; otherwise disables it.
pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::Relaxed);
    print_verbose("set up '-v' environment")
}
/// Sets the global quiet flag.
///
/// # Arguments
///
/// * `quiet` - If true, enables quiet mode; otherwise disables it.
pub fn set_quiet(quiet: bool) {
    QUIET.store(quiet, Ordering::Relaxed);
}
/// Returns the current value of the global verbose flag.
///
/// # Returns
///
/// * `bool` indicating whether verbose mode is enabled.
pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}
/// Returns the current value of the global quiet flag.
///
/// # Returns
///
/// * `bool` indicating whether quiet mode is enabled.
pub fn is_quiet() -> bool {
    QUIET.load(Ordering::Relaxed)
}
/// Prints a message if quiet mode is not enabled.
///
/// # Arguments
///
/// * `message` - The message to print if not in quiet mode.
pub fn print_normal(message: &str) {
    if !is_quiet() {
        println!("{}", message);
    }
}
/// Prints a message if verbose mode is enabled and quiet mode is not.
///
/// # Arguments
///
/// * `message` - The message to print if verbose mode is enabled.
pub fn print_verbose(message: &str) {
    if is_verbose() && !is_quiet() {
        println!("[VERBOSE] {}", message);
    }
}

/// Prints a warning message unless in quiet mode.
///
/// # Arguments
///
/// * `message` - The warning message to print if not in quiet mode.
pub fn print_warning(message: &str) {
    if !is_quiet() {
        eprintln!("⚠️  {}", message);
    }
}
