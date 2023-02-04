use tch::CModule;

pub struct AppConfig {
    model: CModule,
}

impl AppConfig {
    pub fn new(model: CModule) -> Self {
        Self { model }
    }

    pub fn model(&self) -> &CModule {
        &self.model
    }
}
