use http_req::request;
use serde::Deserialize;

const GH_API_PREFIX: &str = "https://github.purejs.icu";

extern "C" {
    fn get_event_body_length() -> i32;
    fn get_event_body(p: *mut u8) -> i32;
    fn set_flows(p: *const u8, len: i32);
}

#[derive(Deserialize)]
struct Query {
    id: String, // database id
}

#[no_mangle]
pub unsafe fn message() {
    if let Some(q) = query_from_subcription() {
        let database = q.id;

        let mut writer = Vec::new();
        let res =
            request::get(format!("{}/event/{}", GH_API_PREFIX, database), &mut writer).unwrap();

        if res.status_code().is_success() {
            if let Ok(flows) = String::from_utf8(writer) {
                set_flows(flows.as_ptr(), flows.len() as i32);
            }
        }
    }
}

fn query_from_subcription() -> Option<Query> {
    unsafe {
        let l = get_event_body_length();
        let mut event_body = Vec::<u8>::with_capacity(l as usize);
        let c = get_event_body(event_body.as_mut_ptr());
        assert!(c == l);
        event_body.set_len(c as usize);

        match serde_json::from_slice(&event_body) {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
}
