# directory-tree

Creates a JSON representing a directory tree.

- Ignore symbolic link.
- Ignore file and directory to which permission is denied.

## Tested on

- OS X
- Unix
- ~~Windows~~ _It has not been implemented by keeping windows's file system in mind._

## Installation

> **Prerequisite:** Install **Rust**.

```bash
cargo install --git https://github.com/abhaydgarg/directory-tree.git
```

## Usage

```
Directory tree 0.0.1

USAGE:
    directory-tree --scan <DIR> <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --scan <DIR> <FILE>    Provide directory to scan and file to write JSON
```

### Example

```bash
$ directory-tree -s /root /some-dir/tree.json

# OR

$ directory-tree --scan /root /some-dir/tree.json
```

## Result

Given a directory structured like this:

```
photos
├── summer
│   └── windsurf.jpg
```

`directory-tree` will write JSON to file:

```js
{
  "kind": "Directory",
  "id": 0,
  "name": "photos",
  "abspath": "/user/photos",
  "path": "photos",
  "size": 152407986,
  "value": 152407986,
  "created": 83642258,
  "modified": 64940080,
  "parent": null,
  "children": [
    {
      "kind": "Directory",
      "id": 1,
      "name": "summer",
      "abspath": "/user/photos/summer",
      "path": "photos/summer",
      "size": 400,
      "value": 400,
      "created": 83642258,
      "modified": 64940080,
      "parent": 0,
      "children": [
        {
          "kind": "File",
          "id": 2,
          "name": "windsurf.jpg",
          "abspath": "/user/photos/summer/windsurf.jpg",
          "path": "photos/summer/windsurf.jpg",
          "size": 400,
          "value": 400,
          "created": 83642258,
          "modified": 64940080,
          "parent": 1,
          "children": null
        }
      ]
    }
  ]
}
```

## Todo

- CLI Option to exclude certain properties in final JSON to reduce file size. Exclude should be implemented while serializing using custom `serde` serializer.
