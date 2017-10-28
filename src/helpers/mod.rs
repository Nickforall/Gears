pub mod error;

use serde_urlencoded;

use std::collections::HashMap;

pub fn decode_body(body: String) -> HashMap<String, String> {
    let mut returnmap = HashMap::new();

    let decoded =
        serde_urlencoded::from_str::<Vec<(String, String)>>(&body)
        .unwrap();

    for tuple in decoded {
        returnmap.insert(tuple.0, tuple.1);
    }

    return returnmap;
}
