use crate::DotenvPlugin;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, Signature, Value};

impl Plugin for DotenvPlugin {
    fn signature(&self) -> Vec<Signature> {
        // It is possible to declare multiple signature in a plugin
        // Each signature will be converted to a command declaration once the
        // plugin is registered to nushell
        vec![Signature::build("from dotenv")
            .usage("Parse a dotenv source")
            .switch(
                "all",
                "return all parsed values (even if an env var already exist)",
                Some('a'),
            )
            .category(Category::Formats)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // You can use the name to identify what plugin signature was called
        match name {
            "from dotenv" => self.parse_dotenv(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
