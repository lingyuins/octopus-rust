use serde_json::Value;

/// Evaluate a JSON condition against a context
pub fn evaluate_condition(condition: &str, _context: &Value) -> bool {
    if condition.is_empty() {
        return true;
    }
    // Parse JSON condition array: [{"key":"model","op":"contains","value":"gpt-4"}]
    if let Ok(rules) = serde_json::from_str::<Vec<ConditionRule>>(condition) {
        for rule in &rules {
            if !rule.evaluate(_context) {
                return false;
            }
        }
        return true;
    }
    true
}

#[derive(serde::Deserialize)]
struct ConditionRule {
    key: String,
    op: String,
    value: Option<Value>,
}

impl ConditionRule {
    fn evaluate(&self, context: &Value) -> bool {
        let actual = context.get(&self.key);
        let expected = &self.value;
        match self.op.as_str() {
            "contains" => {
                if let (Some(Value::String(a)), Some(Value::String(b))) = (actual, expected) {
                    a.contains(b)
                } else { true }
            }
            "equals" => actual == expected.as_ref(),
            "exists" => actual.is_some(),
            _ => true,
        }
    }
}