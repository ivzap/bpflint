/// Overwrite the `dbg` macro in Web contexts to emit data to the
/// browser console.
#[macro_export]
macro_rules! dbg {
  ($($t:tt)*) => {
    ::web_sys::console::log_1(&format!($($t)*).into())
  };
}
