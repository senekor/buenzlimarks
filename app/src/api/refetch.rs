use std::marker::PhantomData;

use leptos::*;
use models::{Bookmark, Entity, Page, Settings, Widget};

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
    pub fn new(cx: Scope) -> Self {
        Self {
            s: create_rw_signal(cx, ()),
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

#[derive(Debug, Clone, Copy)]
pub struct RefetchAllSignal {
    cx: Scope,
    s: RefetchSignal<Settings>,
    p: RefetchSignal<Page>,
    w: RefetchSignal<Widget>,
    b: RefetchSignal<Bookmark>,
}

impl RefetchAllSignal {
    pub fn new(cx: Scope) -> Self {
        Self {
            cx,
            s: use_refetch_settings(cx),
            p: use_refetch_entities(cx),
            w: use_refetch_entities(cx),
            b: use_refetch_entities(cx),
        }
    }
    pub fn broadcast(self) {
        self.cx.batch(|| {
            self.s.broadcast();
            self.p.broadcast();
            self.w.broadcast();
            self.b.broadcast();
        })
    }
}

pub fn provide_refetch_context(cx: Scope) {
    provide_context(cx, RefetchSignal::<Settings>::new(cx));
    provide_context(cx, RefetchSignal::<Page>::new(cx));
    provide_context(cx, RefetchSignal::<Widget>::new(cx));
    provide_context(cx, RefetchSignal::<Bookmark>::new(cx));
    provide_context(cx, RefetchAllSignal::new(cx));
}

pub fn use_refetch_settings(cx: Scope) -> RefetchSignal<Settings> {
    use_context(cx).expect("should find refetch settings context")
}
pub fn use_refetch_entities<T: Entity>(cx: Scope) -> RefetchSignal<T> {
    use_context(cx).expect("should find refetch entities context")
}
pub fn use_refetch_all(cx: Scope) -> RefetchAllSignal {
    use_context(cx).expect("should find refetch all context")
}
