//! ClassNames: one allocation in `finish()`, zero in `finish_cow()` for constant strings (after the first interning).

use prime_core::ClassNames;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// dhat allows one profiler at a time, so both checks run in one test.
#[test]
fn class_names_allocations() {
    single_allocation_in_finish();
    finish_cow_does_not_allocate_for_constant_strings();
}

fn single_allocation_in_finish() {
    let _profiler = dhat::Profiler::builder().testing().build();
    let before = dhat::HeapStats::get();

    let mut cn = ClassNames::new();
    cn.add("p-button p-component").add_if(true, "p-button-sm p-button-outlined").add_if(false, "p-disabled").add("p-button");
    let s = cn.finish();

    let after = dhat::HeapStats::get();
    assert_eq!(s, "p-button p-component p-button-sm p-button-outlined");
    assert_eq!(after.total_blocks - before.total_blocks, 1, "one allocation expected");
}

fn finish_cow_does_not_allocate_for_constant_strings() {
    let build = || {
        let mut cn = ClassNames::new();
        cn.add("p-checkbox p-component").add_if(true, "p-checkbox-checked").add_if(false, "p-disabled");
        cn.finish_cow()
    };
    let first = build();
    let _profiler = dhat::Profiler::builder().testing().build();
    let before = dhat::HeapStats::get();
    let second = build();
    let after = dhat::HeapStats::get();
    assert_eq!(first, second);
    assert_eq!(after.total_blocks - before.total_blocks, 0, "rebuilding a constant string must not allocate");
}
