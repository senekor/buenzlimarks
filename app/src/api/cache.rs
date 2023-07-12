use std::{cell::RefCell, collections::HashMap, rc::Rc};

use leptos::*;
use models::{Bookmark, Entity, Id, Page, Settings, Widget};

use super::{
    auth::Token,
    resources::{
        create_entities_resource, create_entity_resource, create_filtered_entities_resource,
        create_settings_resource,
    },
};

#[allow(clippy::type_complexity)]
#[derive(Debug)]
struct InnerResourceCache<T: Entity> {
    all: Resource<Option<Token>, Vec<T>>,
    by_parent: HashMap<Id<T::Parent>, Resource<Option<Token>, Vec<T>>>,
    individual: HashMap<Id<T>, Resource<Option<Token>, Option<T>>>,
}

#[derive(Debug)]
pub struct ResourceCache<T: Entity>(Rc<RefCell<InnerResourceCache<T>>>);

impl<T: Entity> Clone for ResourceCache<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: Entity> ResourceCache<T> {
    fn new(cx: Scope) -> Self {
        let entities = create_entities_resource::<T>(cx);
        Self(Rc::new(RefCell::new(InnerResourceCache {
            all: entities,
            by_parent: HashMap::new(),
            individual: HashMap::new(),
        })))
    }
    pub fn refetch(&self) {
        let ref_cell = self.0.borrow();
        // This refetching may happen outside a reactive context. That's fine,
        // but it may produce a false positive warning in the console.
        ref_cell.all.refetch();
        ref_cell.by_parent.iter().for_each(|(_, v)| v.refetch());
        ref_cell.individual.iter().for_each(|(_, v)| v.refetch());
    }
    pub fn clear(&self) {
        let mut ref_cell = self.0.borrow_mut();
        // This refetching may happen outside a reactive context. That's fine,
        // but it may produce a false positive warning in the console.
        ref_cell.all.set(Vec::new());
        ref_cell.by_parent = HashMap::new();
        ref_cell.individual = HashMap::new();
    }
}

pub fn provide_cache_contexts(cx: Scope) {
    provide_context(cx, create_settings_resource(cx));
    provide_context(cx, ResourceCache::<Page>::new(cx));
    provide_context(cx, ResourceCache::<Widget>::new(cx));
    provide_context(cx, ResourceCache::<Bookmark>::new(cx));
}

pub fn use_settings(cx: Scope) -> Resource<Option<Token>, Option<Settings>> {
    use_context(cx).expect("should find settings context")
}

pub fn use_entity_cache<T: Entity>(cx: Scope) -> ResourceCache<T> {
    use_context(cx).expect("should find entity cache context")
}

pub fn use_entities<T: Entity>(cx: Scope) -> Resource<Option<Token>, Vec<T>> {
    use_entity_cache(cx).0.borrow().all
}

pub fn use_filtered_entities<T: Entity>(
    cx: Scope,
    parent_id: Signal<Id<T::Parent>>,
) -> Memo<Vec<T>> {
    create_memo(cx, move |_| {
        let rc = use_entity_cache::<T>(cx).0;
        let mut ref_cell = rc.borrow_mut();
        let resource = ref_cell
            .by_parent
            .entry(parent_id())
            .or_insert_with(|| create_filtered_entities_resource(cx, parent_id.get_untracked()));
        resource.read(cx).unwrap_or_default()
    })
}

pub fn use_entity<T: Entity>(cx: Scope, id: Signal<Id<T>>) -> Memo<Option<T>> {
    create_memo(cx, move |_| {
        let rc = use_entity_cache::<T>(cx).0;
        let mut ref_cell = rc.borrow_mut();
        let resource = ref_cell
            .individual
            .entry(id())
            .or_insert_with(|| create_entity_resource(cx, id.get_untracked()));
        resource.read(cx).flatten()
    })
}

#[derive(Debug, Clone)]
pub struct Caches {
    settings: Resource<Option<Token>, Option<Settings>>,
    pages: ResourceCache<Page>,
    widgets: ResourceCache<Widget>,
    bookmarks: ResourceCache<Bookmark>,
}

impl Caches {
    pub fn refetch(&self) {
        self.settings.refetch();
        self.pages.refetch();
        self.widgets.refetch();
        self.bookmarks.refetch();
    }
    pub fn clear(&self) {
        self.settings.set(None);
        self.pages.clear();
        self.widgets.clear();
        self.bookmarks.clear();
    }
}

pub fn use_caches(cx: Scope) -> Caches {
    Caches {
        settings: use_settings(cx),
        pages: use_entity_cache(cx),
        widgets: use_entity_cache(cx),
        bookmarks: use_entity_cache(cx),
    }
}
