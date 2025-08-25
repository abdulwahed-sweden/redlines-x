use async_trait::async_trait;
use crate::core::scan_result::ScanResult;

#[async_trait]
pub trait SecurityModule {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn run(&self, target: &str) -> Result<ScanResult, Box<dyn std::error::Error>>;
}

pub struct ModuleHandler {
    modules: Vec<Box<dyn SecurityModule>>,
}

impl ModuleHandler {
    pub fn new() -> Self {
        Self { modules: Vec::new() }
    }
    
    pub fn register_module(&mut self, module: Box<dyn SecurityModule>) {
        self.modules.push(module);
    }
    
    pub fn get_module(&self, name: &str) -> Option<&Box<dyn SecurityModule>> {
        self.modules.iter().find(|m| m.name() == name)
    }
}
