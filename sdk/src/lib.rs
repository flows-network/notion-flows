#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use http_req::request;
use std::future::Future;

use notion::models::Page;
pub use notion_wasi as notion;

const API_PREFIX: &str = "https://notion.flows.network";

extern "C" {
    // Flag if current running is for listening(1) or message receving(0)
    fn is_listening() -> i32;

    // Return the user id of the flows platform
    fn get_flows_user(p: *mut u8) -> i32;

    // Return the flow id
    fn get_flow_id(p: *mut u8) -> i32;

    fn get_event_body_length() -> i32;
    fn get_event_body(p: *mut u8) -> i32;
    fn set_error_log(p: *const u8, len: i32);
}

unsafe fn _get_flows_user() -> String {
    let mut flows_user = Vec::<u8>::with_capacity(100);
    let c = get_flows_user(flows_user.as_mut_ptr());
    flows_user.set_len(c as usize);
    String::from_utf8(flows_user).unwrap()
}

unsafe fn _get_flow_id() -> String {
    let mut flow_id = Vec::<u8>::with_capacity(100);
    let c = get_flow_id(flow_id.as_mut_ptr());
    if c == 0 {
        panic!("Failed to get flow id");
    }
    flow_id.set_len(c as usize);
    String::from_utf8(flow_id).unwrap()
}

#[allow(rustdoc::bare_urls)]
/// Create a listener for modification of `database`.
///
/// If you have not install
/// [Flows.network platform](https://test.flows.network)'s app to your GitHub,
/// you will receive an error in the flow's building log or running log.
///
/// `callback` is a callback function which will be called with the [Page](https://docs.rs/notion-wasi/latest/notion_wasi/models/struct.Page.html) object.
pub async fn listen_to_event<S, F, Fut>(database: S, callback: F)
where
    S: AsRef<str>,
    F: FnOnce(Page) -> Fut,
    Fut: Future<Output = ()>,
{
    unsafe {
        match is_listening() {
            // Calling register
            1 => {
                let flows_user = _get_flows_user();
                let flow_id = _get_flow_id();

                let mut writer = Vec::new();
                let res = request::get(
                    format!(
                        "{}/{}/{}/listen?database={}",
                        API_PREFIX,
                        flows_user,
                        flow_id,
                        database.as_ref(),
                    ),
                    &mut writer,
                )
                .unwrap();

                match res.status_code().is_success() {
                    true => {
                        if let Ok(event) = serde_json::from_slice(&writer) {
                            callback(event).await;
                        }
                    }
                    false => {
                        set_error_log(writer.as_ptr(), writer.len() as i32);
                    }
                }
            }
            _ => {
                if let Some(event) = event_from_subcription() {
                    callback(event).await;
                }
            }
        }
    }
}

fn event_from_subcription() -> Option<Page> {
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
