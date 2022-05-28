/// Todo list
/// - Manage partial uploads over the 128KB upload limit with a 206 status code.
/// - Return a unique token after each upload to be used during delete requests.
/// - Dispatch a thread before launching Rocket that periodically cleans up old pastes.
/// - Replace std::fs::read_dir with tokio's fs::read_dir in index().
/// - Give pastes a title.
/// - In the all pastes section, add a date and display the paste title.
///
/// Finished list
/// - Create Omoribin.
/// - API to create pastes.
/// - API to read pastes.
/// - API to update pastes.
/// - API to delete pastes.
/// - Update README to document APIs.
/// - Handle stricter validation when processing requests with a PasteId.
/// - Add a web form for users to manually input new pastes.
/// - Separate web API from service API.
/// - Integrate Rocket form parsing for user paste requests on the web.
/// - Create CSS for better paste display.
/// - Show list of all pastes.

struct _IGNORE {}
