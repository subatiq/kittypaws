use std::collections::HashMap;

pub type PlugConfig = HashMap<String, String>;

pub trait PlugInterface {
    fn run(&self, config: &PlugConfig);
}

