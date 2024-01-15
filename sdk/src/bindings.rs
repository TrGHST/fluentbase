#[link(wasm_import_module = "fluentbase_v1alpha")]
extern "C" {
    pub(crate) fn _crypto_keccak256(data_offset: *const u8, data_len: i32, output_offset: *mut u8);
    pub(crate) fn _crypto_poseidon(data_offset: *const u8, data_len: i32, output_offset: *mut u8);
    pub(crate) fn _crypto_poseidon2(
        fa32_offset: *const u8,
        fb32_offset: *const u8,
        fd32_offset: *const u8,
        output32_offset: *mut u8,
    ) -> bool;
    pub(crate) fn _crypto_ecrecover(
        digest32_offset: *const u8,
        sig64_offset: *const u8,
        output65_offset: *mut u8,
        rec_id: u32,
    );

    pub(crate) fn _sys_halt(code: i32);
    pub(crate) fn _sys_write(offset: *const u8, length: u32);
    pub(crate) fn _sys_input_size() -> u32;
    pub(crate) fn _sys_read(target: *mut u8, offset: u32, length: u32);
    pub(crate) fn _sys_state() -> u32;

    pub(crate) fn _mpt_open();
    pub(crate) fn _mpt_update(
        key_offset: *const u8,
        key_len: i32,
        value_offset: *const u8,
        value_len: i32,
    );
    pub(crate) fn _mpt_get(key_offset: *const u8, key_len: u32, output_offset: *mut u8) -> i32;
    pub(crate) fn _mpt_get_root(output_offset: *mut u8) -> i32;

    pub(crate) fn _rwasm_compile(
        input_ptr: *const u8,
        input_len: u32,
        output_ptr: *mut u8,
        output_len: u32,
    ) -> i32;
    pub(crate) fn _rwasm_transact(
        code_offset: *const u8,
        code_len: u32,
        input_offset: *const u8,
        input_len: u32,
        output_offset: *mut u8,
        output_len: u32,
        state: u32,
        fuel_limit: u32,
    ) -> i32;

    pub(crate) fn _zktrie_open(root32_ptr: *const u8) -> u32;
    pub(crate) fn _zktrie_update(
        trie: u32,
        key32_offset: *const u8,
        flags: u32,
        val32_offset: *const u8,
        val32_len: u32,
    );
    pub(crate) fn _zktrie_get(trie: u32, key32_offset: *const u8, output_offset: *mut u8);
    pub(crate) fn _zktrie_root(trie: u32, output32_offset: *mut u8);
}
