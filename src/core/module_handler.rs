use crate::core::modules::SecurityModule;
use std::collections::HashMap;
use std::sync::Arc;

/// Manages the registration, retrieval, and execution of security modules
pub struct ModuleHandler {
    modules: HashMap<String, Arc<dyn SecurityModule + Send + Sync>>,
}

impl ModuleHandler {
    /// Creates a new ModuleHandler with an empty module registry
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    
    /// Registers a new security module
    pub fn register_module(&mut self, module: impl SecurityModule + Send + Sync + 'static) {
        let name = module.name().to_string();
        
        // حفظ اسم الوحدة قبل نقل الملكية
        let module_name = name.clone();
        let module_arc = Arc::new(module);
        
        self.modules.insert(name, module_arc);
        log::info!("Registered module: {}", module_name);
    }
    
    /// Retrieves a module by name
    pub fn get_module(&self, name: &str) -> Option<Arc<dyn SecurityModule + Send + Sync>> {
        self.modules.get(name).cloned()
    }
    
    /// Lists all available module names
    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
    
    /// Returns the number of registered modules
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }
    
    /// Executes a specific module against a target
    pub async fn run_module(
        &self,
        module_name: &str,
        target: &str,
    ) -> Result<crate::core::scan_result::ScanResult, Box<dyn std::error::Error + Send + Sync>> {
        match self.get_module(module_name) {
            Some(module) => {
                log::info!("Executing module '{}' on target: {}", module_name, target);
                module.run(target).await
            }
            None => Err(format!("Module '{}' not found", module_name).into()),
        }
    }
    
    /// Returns detailed information about all modules
    pub fn get_module_info(&self) -> Vec<ModuleInfo> {
        self.modules
            .values()
            .map(|module| ModuleInfo {
                name: module.name().to_string(),
                description: module.description().to_string(),
                author: module.author().to_string(),
                version: module.version().to_string(),
            })
            .collect()
    }
    
    /// Checks if a module exists
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }
    
    /// Removes a module from the registry
    pub fn remove_module(&mut self, name: &str) -> bool {
        self.modules.remove(name).is_some()
    }
    
    /// Clears all modules from the registry
    pub fn clear_modules(&mut self) {
        self.modules.clear();
        log::info!("All modules cleared from registry");
    }
}

/// Detailed information about a security module
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
}

/// Default implementation for ModuleHandler
impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scan_result::ScanResult;
    
    struct TestModule;

    #[async_trait::async_trait]
    impl SecurityModule for TestModule {
        fn name(&self) -> &str {
            "test_module"
        }

        fn description(&self) -> &str {
            "A test module for unit testing"
        }

        fn author(&self) -> &str {
            "Test Author"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        async fn run(&self, target: &str) -> Result<ScanResult, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ScanResult {
                success: true,
                vulnerabilities: Vec::new(),
                warnings: vec![format!("Test scan completed for {}", target)],
                duration: std::time::Duration::from_secs(1),
            })
        }
    }
    
    #[tokio::test]
    async fn test_module_registration() {
        let mut handler = ModuleHandler::new();
        let test_module = TestModule;
        
        handler.register_module(test_module);
        
        assert_eq!(handler.module_count(), 1);
        assert!(handler.has_module("test_module"));
        assert!(!handler.has_module("nonexistent"));
    }
    
    #[tokio::test]
    async fn test_module_retrieval() {
        let mut handler = ModuleHandler::new();
        handler.register_module(TestModule);
        
        let module = handler.get_module("test_module");
        assert!(module.is_some());
        assert_eq!(module.unwrap().name(), "test_module");
        
        let nonexistent = handler.get_module("nonexistent");
        assert!(nonexistent.is_none());
    }
    
    #[tokio::test]
    async fn test_module_execution() {
        let mut handler = ModuleHandler::new();
        handler.register_module(TestModule);
        
        let result = handler.run_module("test_module", "http://example.com").await;
        assert!(result.is_ok());
        
        let scan_result = result.unwrap();
        assert!(scan_result.success);
        assert!(scan_result.warnings.contains(&"Test scan completed for http://example.com".to_string()));
    }
    
    #[tokio::test]
    async fn test_module_listing() {
        let mut handler = ModuleHandler::new();
        handler.register_module(TestModule);
        
        let modules = handler.list_modules();
        assert_eq!(modules, vec!["test_module"]);
        
        let info = handler.get_module_info();
        assert_eq!(info.len(), 1);
        assert_eq!(info[0].name, "test_module");
        assert_eq!(info[0].description, "A test module for unit testing");
    }
    
    #[tokio::test]
    async fn test_module_removal() {
        let mut handler = ModuleHandler::new();
        handler.register_module(TestModule);
        
        assert!(handler.has_module("test_module"));
        assert!(handler.remove_module("test_module"));
        assert!(!handler.has_module("test_module"));
        assert!(!handler.remove_module("nonexistent"));
    }
}