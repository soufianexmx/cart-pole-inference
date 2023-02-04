use tch::CModule;

pub struct AppEnv {
    model: CModule,
}

impl AppEnv {
    pub fn new(model: CModule) -> Self {
        Self { model }
    }

    pub fn model(&self) -> &CModule {
        &self.model
    }
}
