use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct EditMode {
    signal: RwSignal<bool>,
}

impl EditMode {
    fn new(cx: Scope) -> Self {
        let signal = create_rw_signal(cx, false);
        Self { signal }
    }

    pub fn read(self) -> ReadSignal<bool> {
        self.signal.read_only()
    }

    pub fn write(self) -> WriteSignal<bool> {
        self.signal.write_only()
    }
}

pub fn provide_edit_mode(cx: Scope) {
    provide_context(cx, EditMode::new(cx))
}

pub fn use_edit_mode(cx: Scope) -> EditMode {
    use_context(cx).expect("should find edit mode context")
}
