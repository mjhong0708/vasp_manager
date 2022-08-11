use tera::Context;
use toml::Value;

pub fn default_job_script_context() -> Context {
    let mut context = Context::new();
    context.insert("partition", "g1");
    context.insert("num_nodes", "1");
    context.insert("num_tasks", "16");
    context.insert("vasp_version", "6.3.1");
    context.insert("vasp_cmd", "vasp_std");
    context
}

pub fn format_value(value: &Value) -> String {
    let formatted = format!("{}", value);
    if formatted.starts_with('"') && formatted.ends_with('"') {
        formatted[1..formatted.len() - 1].to_string()
    } else {
        formatted
    }
}
