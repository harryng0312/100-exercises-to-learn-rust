use serde::{Deserialize, Serialize};

fn json_to_obj<'a, T: Deserialize<'a>>(str: &'a str) -> serde_json::Result<T> {
    serde_json::from_str(str)
}

fn obj_to_json<T: Serialize>(obj: &T) -> serde_json::Result<String> {
    serde_json::to_string(obj)
}