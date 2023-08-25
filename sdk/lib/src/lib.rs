#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use http_req::request;

pub use notion_wasi as notion;

pub use notion_flows_macros::*;

lazy_static::lazy_static! {
    static ref API_PREFIX: String = String::from(
        std::option_env!("NOTION_API_PREFIX").unwrap_or("https://notion.flows.network")
    );
}

extern "C" {
    // Return the user id of the flows platform
    fn get_flows_user(p: *mut u8) -> i32;
    // Return the flow id
    fn get_flow_id(p: *mut u8) -> i32;

    fn set_output(p: *const u8, len: i32);
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
/// Create a listener for update (page creation/modification or properties modification) of `database`.
///
/// If you have not install
/// [Flows.network platform](https://test.flows.network)'s app to your GitHub,
/// you will receive an error in the flow's building log or running log.
pub async fn listen_to_database_update<S>(database: S)
where
    S: AsRef<str>,
{
    unsafe {
        let flows_user = _get_flows_user();
        let flow_id = _get_flow_id();

        let mut writer = Vec::new();
        let res = request::get(
            format!(
                "{}/{}/{}/listen?database={}&handler_fn={}",
                API_PREFIX.as_str(),
                flows_user,
                flow_id,
                database.as_ref(),
                "__notion__on_database_updated"
            ),
            &mut writer,
        )
        .unwrap();

        match res.status_code().is_success() {
            true => {
                let output = format!(
                    "[{}] Listening update on database `{}`.",
                    std::env!("CARGO_CRATE_NAME"),
                    database.as_ref()
                );
                set_output(output.as_ptr(), output.len() as i32);
            }
            false => {
                set_error_log(writer.as_ptr(), writer.len() as i32);
            }
        }
    }
}
