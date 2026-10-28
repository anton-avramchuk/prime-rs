//! Статический `Prop` не создаёт записей в арене и не аллоцирует; сигнал — аллоцирует.

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Size, all_static};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn static_props_do_not_allocate_signals_do() {
    let _profiler = dhat::Profiler::builder().testing().build();
    Owner::new().with(|| {
        let blocks = || dhat::HeapStats::get().total_blocks;

        let before = blocks();
        let a: Prop<bool> = true.into();
        let b: Prop<bool> = Prop::default();
        let c: OptProp<Size> = Size::Small.into();
        let d: OptProp<Size> = OptProp::default();
        let read = (a.get(), b.get(), c.get(), d.get());
        let all = all_static!(a, b, c, d);
        let static_blocks = blocks() - before;
        assert_eq!(read, (true, false, Some(Size::Small), None));
        assert!(all);
        assert_eq!(static_blocks, 0, "статические свойства и проверка all_static! не должны аллоцировать");

        let before = blocks();
        let sig = RwSignal::new(true);
        let live: Prop<bool> = sig.into();
        assert!(live.get());
        assert!(blocks() - before > 0, "контрольный случай: сигнал аллоцирует");
    });
}
