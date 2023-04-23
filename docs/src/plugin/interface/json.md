# JSON Functions

> This part can help you e sncode & decode json data

### Encode(value: any): string

```lua
local data = {
  name = "Dioxus",
  link = "https://dioxuslabs.com/"
}

local str = plugin.json.encode(data)
-- str == "{\"name\": \"Dioxus\", \"link\": \"https://dioxuslabs.com/\"}"
```



### Decode(value: string): any

```lua
local str = "{\"name\": \"Dioxus\", \"link\": \"https://dioxuslabs.com/\"}"

local data = plugin.json.decode(str)
-- data == data = { name = "Dioxus", link = "https://dioxuslabs.com/" }
```

