use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{PluginSignature, Type, Value};

struct Test;

impl Test {
    fn new() -> Self {
        Self
    }
}

impl Plugin for Test {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("test")
            .usage("multiplies value by two")
            .input_output_types(vec![(Type::Int, Type::Float), (Type::Int, Type::Float)])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "test");
        match input {
            Value::Int { val, span } => Ok(Value::Int {
                val: val * 2 as i64,
                span: span.clone(),
            }),
            Value::Float { val, span } => Ok(Value::Float {
                val: val * 2 as f64,
                span: span.clone(),
            }),
            _ => Err(LabeledError {
                label: "Expected integer or float input from pipeline".to_string(),
                msg: "requires integer or float input; got {input.get_type()}".to_string(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Test::new(), MsgPackSerializer)
}
