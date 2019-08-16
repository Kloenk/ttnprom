



pub struct Config {
    pub plugins: Vec<String>,
}

impl Config {
    /// create new Config instance
    pub fn new() -> Self {
        Default::default()
    }
    /// run application
    pub fn run(self) {
        for v in self.plugins.iter() {
            println!("plugin: {}", v);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }            
}