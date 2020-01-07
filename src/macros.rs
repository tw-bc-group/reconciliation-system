#[macro_export]
macro_rules! declare_flush_plugin {
    ($constructor:path) => {
        #[no_mangle]
        pub extern "C" fn probe_plugin() -> Box<dyn Flush> {
            Box::new($constructor())
        }
    };
}
