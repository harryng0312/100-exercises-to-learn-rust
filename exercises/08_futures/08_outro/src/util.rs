
fn json_to_obj<T>(str: &str) -> serde_json::Result<T> {
    serde_json::from_str(str)
}

fn obj_to_json<T>(obj: &T) -> serde_json::Result<String> {
    serde_json::to_string(obj)
}