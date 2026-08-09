use crate::core::databases::adapters::database_type::DbValue;

pub(crate) fn format_db_value(value: &DbValue) -> String {
    match value {
        DbValue::Null => "null".to_string(),
        DbValue::Text(value) => value.clone(),
        DbValue::TextArray(values) => format!("{{{}}}", values.join(",")),
        DbValue::Json(value) => serde_json::to_string_pretty(value)
            .map(|json| json.lines().map(str::trim).collect::<Vec<_>>().join(" "))
            .unwrap_or_else(|_| value.to_string()),
        DbValue::Numeric(value) => value.clone(),
        DbValue::Integer(value) => value.to_string(),
        DbValue::Float(value) if value.is_finite() => value.to_string(),
        DbValue::Float(_) => "null".to_string(),
        DbValue::Boolean(value) => value.to_string(),
    }
}
