use tch::CModule;

pub struct AppEnv {
    model: CModule,
    address: String,
    port: u16,
}

impl AppEnv {
    pub fn new(model: CModule, address: String, port: u16) -> Self {
        Self {
            model,
            address,
            port,
        }
    }

    pub fn model(&self) -> &CModule {
        &self.model
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn listen_addr(&self) -> String {
        format!("http://{}:{}", self.address(), self.port())
    }
}
