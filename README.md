# Omoribin

<div align="center">
  <img src="/images/omori.gif" height=200 />
</div>

Omoribin is a web service written in Rust using the Rocket web framework. It has just enough features to function like a mini pastebin, but it's minimal enough to have a lot of potential to grow and adopt custom features.

The cool thing about being written in Rust is that Omoribin is fast, secure, and safe. For instance, Rust's features allows Omoribin to be protected against [full path disclosure vulnerabilities](https://owasp.org/www-community/attacks/Full_Path_Disclosure).

It's worth noting that any pastes will be stored within generated files on your system so that they persist.

## Usage
0. Make sure you have Rust installed. Check with `cargo --version`.
1. To start the server, use `cargo run`. This will start the server on `http://localhost:8000`
2. That's it! You can explore some of the custom APIs below or contribute some features of your own.

## UI
Although you can visit `http://localhost:8000` to use the web interface version, there isn't much to see. I'm planning on adding a UI soon. For now, you'll just have to use the APIs below.

## APIs
### `GET`
Returns the content of the associated paste.

|         |                                         |
|---------|-----------------------------------------|
| API     | /<paste_id>                             |
| Example | `curl http://localhost:8000/<paste_id>` |

### `POST`
Creates a new paste from the data in the request body and returns the URL and paste_id.

|                           |                                                                   |
|---------------------------|-------------------------------------------------------------------|
| API                       | /new                                                              |
| Example (from raw string) | `echo "hello" \| curl --data-binary @- http://localhost:8000/new` |
| Example (from file)       | `curl -d @/User/me/file.txt http://localhost:8000/new`            |

### `PUT`
Replaces the existing paste on the file system with content from the new paste.

|                           |                                                                                         |
|---------------------------|-----------------------------------------------------------------------------------------|
| API                       | /replace/<paste_id>                                                                     |
| Example (from raw string) | `echo "hello" \| curl -X PUT --data-binary @- http://localhost:8000/replace/<paste_id>` |

### `DELETE`
Deletes the paste if the given paste_id exists.

|         |                                                            |
|---------|------------------------------------------------------------|
| API     | /delete/<paste_id>                                         |
| Example | `curl -X "DELETE" http://localhost:8000/delete/<paste_id>` |
