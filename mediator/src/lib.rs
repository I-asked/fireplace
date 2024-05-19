use wayland_client::{protocol::wl_display::WlDisplay};

#[no_mangle]
pub extern "C" fn start_mediator(_display: *mut WlDisplay) {
    panic!("Not implemented yet!");
}