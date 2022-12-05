use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Spanned, Value};
use std::collections::HashMap;

pub struct DotenvPlugin;

impl DotenvPlugin {
    pub fn parse_dotenv(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let span = call.head;
        let binary = input.as_binary()?;

        if binary.is_empty() {
            return Ok(Value::Nothing { span });
        }

        let all = call.has_flag("all");
        let items = dotenvy::from_read_iter(binary);
        let mut collected = Spanned {
            item: HashMap::new(),
            span,
        };

        for item in items {
            match item {
                Ok((key, value)) => {
                    if all || std::env::var(&key).is_err() {
                        collected
                            .item
                            .insert(key, Value::String { val: value, span });
                    }
                }
                Err(err) => {
                    return Err(LabeledError {
                        label: "Error parsing dotenv source".into(),
                        msg: format!("{}", err),
                        span: None,
                    })
                }
            }
        }

        Ok(Value::from(collected))
    }
}
