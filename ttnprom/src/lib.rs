



/// plugin mod for the Plugin trait
pub mod plugin;

/// data mod holding the datatypes
pub mod data;

pub struct Config {
    pub plugins: Vec<PluginConf>,
}

impl Config {
    /// create new Config instance
    pub fn new() -> Self {
        Default::default()
    }
    /// run application
    pub fn run(self) {
        for v in self.plugins.iter() {
            println!("plugin: {} with args: {:?}", v.file, v.args);
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

/// Plugin Conf struct
pub struct PluginConf {
    pub file: String,
    pub args: Vec<String>,
}

impl PluginConf {
    pub fn parse(name: &str) -> Self {
        let v: Vec<&str> = name.split(":").collect();
        let file = PluginConf::parse_file(v[0]);
        let mut args: Vec<String> = Vec::new();
        for v in v.iter().skip(1) {
            args.push(v.to_string());
        }
        Self {
            file,
            args,
        }
    }

    #[cfg(not(windows))]
    fn parse_file(name: &str) -> String {
        let mut ret = String::from(name);
        if !ret.ends_with(".so") {
            ret = format!("{}.so", ret);
        }

        if !ret.starts_with("lib") {
            ret = format!("lib{}", ret);
        }
        ret
    }

    #[cfg(windows)]
    fn parse_file(name: &str) -> String {
        let mut ret = String::from(name);
        
        if !ret.ends_with(".dll") {
            ret = format!("{}.dll", ret);
        }
    }
}