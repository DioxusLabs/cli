# FileSystem Functions

> you can use FileSystem functions to handle file.

### create_dir(path: string, recursive: boolean): boolean

This function can help you create a directory, if `recursive` is `true`, API will auto-create missing directory.

```lua
local path = "~/MyProject/temp/hello"

-- before
-- - MyProject

plugin.fs.create_dir(path, true)
-- after (recursive)
-- - MyProject
-- - - temp
-- - - - hello
```

### remove_dir(path: string): boolean

This function will remove a single directory.

```lua
local path = "~/MyProject/temp/hello"

-- before
-- - MyProject
-- - - temp
-- - - - hello

plugin.fs.create_dir(path, true)
-- after
-- - MyProject
-- - - temp
```

### file_get_content(path: string): string

This function can read a file's content.

```lua
local path = "~/MyProject/hello.txt"
local content = plugin.fs.file_get_content(path)
-- content = "hello world"
```

### file_set_content(path:  string, content: string): boolean

This function can write file's content.

```lua
local path = "~/MyProject/hello.txt"
-- before: "Hello World"
plugin.fs.file_set_content(path, "Hello Dioxus")
-- after: "Hello Dioxus"
```

### remove_file(path: string): boolean

This function can remove a file from path.

```lua
local file = "~/MyProject/hello.txt"
plugin.fs.remove_file(file)
```

### move_file(path: string, target: string): boolean

This function can move a file from a path to another path.

```lua
local file = "~/MyProject/hello.txt"
plugin.fs.move_file(file, "~/MyProject/temp/hello.txt")
```

### copy_file(path: string, target: string): boolean

This function can move a file from a path to another path.

> The different and `move_file` is `copy_file` will keep origin file.

```lua
local file = "~/MyProject/hello.txt"
plugin.fs.copy_file(file, "~/MyProject/temp/hello.txt")
```

### unzip_file(path: string, target: string): boolean

This function can let you **unzip** a `.zip` file.

```lua
local file = "~/MyProject/package.zip"
plugin.fs.unzip(file, "~/MyProject/unpackaged")
```

### untar_gz_file(path: string, target: string): boolean

This function can let you **unpackage** a `.tar.gz` file.

```lua
local file = "~/MyProject/package.tar.gz"
plugin.fs.untar_gz_file(file, "~/MyProject/unpackaged")
```

### read_dir(path: string): string []

This function can read a directory and get a list.

```lua
local path = "~/MyProject/"
local list = plugin.fs.read_dir(path)
-- list = ["temp", "package.zip", "package.tar.gz"]
```