use core_affinity::{get_core_ids, set_for_current};

#[inline]
pub fn switch_to_performance_core() {
    let core_ids = get_core_ids().expect("Failed to get core IDs");
    let selected_core = core_ids.first().expect("Core ID out of range");
    set_for_current(*selected_core);
}
