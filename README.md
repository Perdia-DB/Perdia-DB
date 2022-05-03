# Perdia-DB
A simple key value database for storing simple structures.

## Commands
|Name|Description|
|-|-|
|QUERY|Used to get a Object by it's name|
||

## Example

### Query Objects

```
CREATE "Monday";
QUERY "Monday" PUT "First" VALUE "Science";
QUERY "Monday" GET "First";
```

### Declare Templates

```toml
[Day]
first = { type = String }
second = { type = String }
third = { type = String }
week_day = { type = Number, default = 1 }
```