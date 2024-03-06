use fluentbase_core_macros::derive_codec_structs_from_extern_bindings;

derive_codec_structs_from_extern_bindings! {
    extern "C" {
        fn _evm_create(
            value32_offset: *const u8,
            code_offset: *const u8,
            code_len: u32,
            gas_limit: u32,
        ) -> *mut u8; // out_address20_offset

        fn _evm_create2(
            value32_offset: *const u8,
            salt32_offset: *const u8,
            code_offset: *const u8,
            code_len: u32,
            gas_limit: u32,
        ) -> *mut u8; // out_address20_offset

        fn _evm_call(
            callee_address20_offset: *const u8,
            value32_offset: *const u8,
            args_offset: *const u8,
            args_size: u32,
            gas_limit: u32,
        ) -> *mut u8; // ret_offset
    }
}
