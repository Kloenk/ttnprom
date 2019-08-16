

use super::data::Payload;

/// Plugin trait to be implemented in every plugin
pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used 
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    /// decode string from ttn
    fn decode(&self, _data: Payload, _response: &mut String, _parsed: &mut Vec<String>) {}
}