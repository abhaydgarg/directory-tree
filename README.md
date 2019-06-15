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
│   └── june
│       └── windsurf.jpg
└── winter
    └── january
        ├── ski.png
        └── snowboard.jpg
```

`directory-tree` will write JSON to file:

```js
{
  "kind": "Directory",
  "name": "photos",
  "path": "photos",
  "size": 152407986,
  "created": 83642258,
  "modified": 64940080,
  "children": [
    {
      "kind": "Directory",
      "name": "summer",
      "path": "/photos/summer",
      "size": 400,
      "created": 83642258,
      "modified": 64940080,
      "children": [
        {
          "kind": "Directory",
          "name": "june",
          "path": "/photos/summer/june",
          "size": 400,
          "created": 83642258,
          "modified": 64940080,
          "children": [
            {
              "kind": "File",
              "name": "windsurf.jpg",
              "path": "/photos/summer/june/windsurf.jpg",
              "size": 400,
              "created": 83642258,
              "modified": 64940080
            }
          ]
        }
      ]
    },
    {
      "kind": "Directory",
      "name": "winter",
      "path": "/photos/winter",
      "size": 200,
      "created": 83642258,
      "modified": 64940080,
      "children": [
        {
          "kind": "Directory",
          "name": "january",
          "path": "/photos/winter/january",
          "size": 200,
          "created": 83642258,
          "modified": 64940080,
          "children": [
            {
              "kind": "File",
              "name": "ski.png",
              "path": "/photos/winter/january/ski.png",
              "size": 100,
              "created": 83642258,
              "modified": 64940080,
            },
            {
              "kind": "File",
              "name": "snowboard.jpg",
              "path": "/photos/winter/january/snowboard.jpg",
              "size": 100,
              "created": 83642258,
              "modified": 64940080,
            }
          ]
        }
      ]
    }
  ]
}
```
