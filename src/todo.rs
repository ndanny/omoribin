/// Todo list
/// - Manage partial uploads over the 128KB upload limit with a 206 status code.
/// - Return a unique token after each upload.
/// - Use unique token in delete requests.
/// - Dispatch a thread before launching Rocket that periodically cleans up old pastes.
///
/// Finished list
/// - Add an API to delete pastes.
/// - Add an API to replace an existing paste.
/// - Update README to document APIs.
/// - Handle stricter validation when processing requests with a PasteId.
/// - Add a web form for users to manually input new pastes.

struct _IGNORE {}
