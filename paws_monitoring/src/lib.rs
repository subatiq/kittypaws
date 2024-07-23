pub mod telegraf;
use std::collections::HashMap;

use telegraf::TelegrafClient;

#[derive(Debug)]
pub enum StatusValue {
    Int(i64),
    Float(f64),
    String(String),
}

pub enum MonitoringBackend {
    Telegraf,
}

pub trait MetricSender: Send {
    fn send_metric(
        &mut self,
        tags: HashMap<String, String>,
        fields: HashMap<String, StatusValue>,
    ) -> Result<(), String>;
}

pub fn init_monitoring_backend(option: MonitoringBackend, dsn: &str) -> Box<dyn MetricSender> {
    Box::new(match option {
        MonitoringBackend::Telegraf => TelegrafClient::new(dsn).unwrap(),
    })
}
