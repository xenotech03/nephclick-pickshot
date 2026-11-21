//! Give memory the system allocator keeps after frees back to the system
//! ([`picshot_engine::memory::set_release_hook`]). The platform call lives in
//! `picshot-sysmem`, the workspace's only crate allowed `unsafe`.

/// Install the hook.
pub fn install() {
    picshot_engine::memory::set_release_hook(release);
}

fn release() {
    let _ = picshot_sysmem::release_free_memory();
}
