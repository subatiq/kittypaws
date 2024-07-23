use telegraf::{
    protocol::{Field, Tag},
    FieldData,
};

use crate::{MetricSender, StatusValue};

pub struct TelegrafClient {
    client: telegraf::Client,
}

impl TelegrafClient {
    pub fn new(uri: &str) -> Result<Self, String> {
        match telegraf::Client::new(uri) {
            Ok(client) => Ok(TelegrafClient { client }),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl From<&StatusValue> for FieldData {
    fn from(value: &StatusValue) -> Self {
        match value {
            StatusValue::Int(val) => FieldData::Number(val.to_owned()),
            StatusValue::Float(val) => FieldData::Float(val.to_owned()),
            StatusValue::String(val) => FieldData::Str(val.to_owned()),
        }
    }
}

impl MetricSender for TelegrafClient {
    fn send_metric(
        &mut self,
        tags: std::collections::HashMap<String, String>,
        fields: std::collections::HashMap<String, crate::StatusValue>,
    ) -> Result<(), String> {
        let tags: Vec<Tag> = tags
            .iter()
            .map(|(key, value)| Tag {
                name: key.to_owned(),
                value: value.to_owned(),
            })
            .collect();
        let fields: Vec<Field> = fields
            .iter()
            .map(|(key, value)| Field {
                name: key.to_owned(),
                value: value.into(),
            })
            .collect();
        let point = telegraf::Point {
            measurement: "kittypaws".to_string(),
            tags,
            fields,
            timestamp: None,
        };
        if let Err(err) = self.client.write_point(&point) {
            return Err(err.to_string());
        }
        Ok(())
    }
}
