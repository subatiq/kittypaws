use std::collections::HashMap;

pub type PlugConfig = HashMap<String, String>;

pub trait PlugInterface {
    fn run(&self, config: &PlugConfig);
    fn clone_dyn(&self) -> Box<dyn PlugInterface + Send>;
}

