use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct EditMode {
    signal: RwSignal<bool>,
}

impl EditMode {
    fn new() -> Self {
        let signal = RwSignal::new(false);
        Self { signal }
    }

    pub fn read(self) -> ReadSignal<bool> {
        self.signal.read_only()
    }

    pub fn write(self) -> WriteSignal<bool> {
        self.signal.write_only()
    }
}

pub fn provide_edit_mode() {
    provide_context(EditMode::new())
}

pub fn use_edit_mode() -> EditMode {
    use_context().expect("should find edit mode context")
}
