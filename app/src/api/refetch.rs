use std::{any::type_name, marker::PhantomData};

use leptos::*;
use models::{Bookmark, Page, Settings, Widget};

#[derive(Debug)]
pub struct RefetchSignal<T> {
    // `()` can only signal refetch. An enum could be used to send other
    // signals, e.g. "refetch" and "clear". Though "clear" probably isn't needed
    // at the moment, because a logout should cause all components relying on
    // server side state to not be rendered anymore (discarding their
    // resources).
    s: RwSignal<()>,
    t: PhantomData<T>,
}

impl<T> Clone for RefetchSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for RefetchSignal<T> {}

impl<T> RefetchSignal<T> {
    pub fn new() -> Self {
        Self {
            s: create_rw_signal(()),
            t: PhantomData,
        }
    }
    pub fn listen(self) -> ReadSignal<()> {
        self.s.read_only()
    }
    pub fn broadcast(self) {
        // TODO maybe store cx in struct and wrap this function in a batch?
        self.s.set(())
    }
}

pub fn provide_refetch_context() {
    provide_context(RefetchSignal::<Settings>::new());
    provide_context(RefetchSignal::<Page>::new());
    provide_context(RefetchSignal::<Widget>::new());
    provide_context(RefetchSignal::<Bookmark>::new());
    // provide_context( RefetchAllSignal::new());
}

// 'static needed for type_name. if this ever causes trouble, remove the bound
// and simplify the error message.
pub fn use_refetch_signal<T: 'static>() -> RefetchSignal<T> {
    use_context().unwrap_or_else(|| panic!("should find refetch {} context", type_name::<T>()))
}
