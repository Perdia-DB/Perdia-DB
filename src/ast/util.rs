#[macro_export]
macro_rules! gr {
    ($arg:tt) => (grammar_rule($arg, false, false));
    ($arg:tt, $expand:tt) => (grammar_rule($arg, $expand, false));
    ($arg:tt, $expand:tt, $inner:tt) => (grammar_rule($arg, $expand, $inner));
}