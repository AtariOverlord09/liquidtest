#[cfg(target_os = "ios")]
unsafe extern "C" {
    fn liquid_glass_install_overlay();
}

#[cfg(target_os = "ios")]
pub fn install_liquid_glass_overlay() {
    // Bridge into Swift. The Swift side guards against duplicate mounting.
    unsafe {
        liquid_glass_install_overlay();
    }
}

#[cfg(not(target_os = "ios"))]
pub fn install_liquid_glass_overlay() {}
