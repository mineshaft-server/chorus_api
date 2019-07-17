trace_macros!(true);
macro_rules! each_tt {
    () => {};
    ($field:tt $($rest:tt)*) => {$(each_tt!($rest))*};
}
fn main() {
  each_tt!(field += optional i8);
}