#[link(name="chess", kind="static")]
unsafe extern "C" {
    pub unsafe fn init_board() -> bool;
    pub unsafe fn deinit_board();
}