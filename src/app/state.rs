use tch::CModule;

pub struct AppState {
    model: CModule,
}

impl AppState {
    pub fn new(model: CModule) -> Self {
        Self { model }
    }

    pub fn model(&self) -> &CModule {
        &self.model
    }
}
