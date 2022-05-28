# Omoribin

<div align="center">
  <img src="/images/omori.gif" height=200 />
</div>

Omoribin is a web service written in Rust using the Rocket web framework. It has just enough features to function like a mini pastebin, but it's minimal enough to have a lot of potential to grow and adopt custom features. Omoribin is a small web project that was developed to learn Rust. As such, it's not recommended to see this as a production-ready project (maybe one day).

The cool thing about being written in Rust is that Omoribin is fast, secure, and safe. For instance, Rust's features allows Omoribin to be protected against [full path disclosure vulnerabilities](https://owasp.org/www-community/attacks/Full_Path_Disclosure).

Pastes are stored within generated files on your system so that they persist.
## Usage
0. Make sure you have Rust installed. Check with `cargo --version`.
1. To start the server, use `cargo run`. This will start the server on `http://localhost:8000`
2. That's it! You can explore some of the custom APIs below or contribute some features of your own.

**Optional but recommended**: If you are developing, [cargo-watch](https://crates.io/crates/cargo-watch) is recommended to automatically reload the server when source code changes. You can get this by running `cargo install cargo-watch` and running the server via `cargo watch -x run`.


## UI
Although you can visit `http://localhost:8000` to use the web interface version, there isn't much to see. I'm planning on adding a UI soon. For now, you'll just have to use the APIs below.

## APIs
| API | Description | Examples |
| ----------- | ----------- | ----------- |
| `/api/v1/read/<paste_id>` | Returns the content of the associated paste. | `curl localhost:8000/api/v1/read/<paste_id>` |
| `/api/v1/create` | Creates a new paste from the data in the request body and returns the URL and paste_id. | `echo "msg" \| curl --data-binary @- localhost:8000/api/v1/create` <br /><br /> `curl -d @/User/me/msg.txt localhost:8000/api/v1/create` |
| `/api/v1/update` | Updates the existing paste on the file system with content from the new paste. | `echo "msg" \| curl -X PUT --data-binary @- localhost:8000/api/v1/update/<paste_id>` |
| `/api/v1/delete/<paste_id>` | Deletes the paste. | `curl -X "DELETE" localhost:8000/api/v1/delete/<paste_id>` |
