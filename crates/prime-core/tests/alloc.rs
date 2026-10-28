//! ClassNames укладывается в одну аллокацию при достаточной ёмкости.

use prime_core::ClassNames;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn class_names_single_allocation() {
    let _profiler = dhat::Profiler::builder().testing().build();
    let before = dhat::HeapStats::get();

    let mut cn = ClassNames::new();
    cn.add("p-button p-component").add_if(true, "p-button-sm p-button-outlined").add_if(false, "p-disabled").add("p-button");
    let s = cn.finish();

    let after = dhat::HeapStats::get();
    assert_eq!(s, "p-button p-component p-button-sm p-button-outlined");
    assert_eq!(after.total_blocks - before.total_blocks, 1, "ожидалась одна аллокация");
}
