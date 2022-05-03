# Perdia-DB
A simple key value database for storing simple structures.

## Commands
|Name|Description|
|-|-|
|QUERY|Used to get a Object by it's name|
|CREATE|Cerate an object from an template|
|GET|Get a value from an object with a key|
|PUT|Overwrite a value from an object using a key|
|VALUE|Define the value for the PUT command|

## Example

### Declare Templates

```
TYPE "DAY" FIELDS;
NAME "First" TYPE STRING STARTING "Nothing";
NAME "Second" TYPE STRING STARTING "Nothing";
NAME "Third" TYPE STRING STARTING "Nothing";
NAME "Day" TYPE INTEGER STARTING 0;
NAME "Seconds" TYPE FLOAT STARTING 0.0;
END;
```

### Create Objects

```
CREATE "Monday" TYPE "DAY";
```

### Query Objects

Query object by name.
```
QUERY "Monday" PUT "First" VALUE "Science";
QUERY "Monday" PUT "Second" VALUE "CS";
QUERY "Monday" GET "First";
QUERY "Monday" GET "First" "Second";
```

Query objects by type
```
QUERY TYPE "DAY" GET "First";
QUERY TYPE "DAY" GET "First" "Seconds";
```