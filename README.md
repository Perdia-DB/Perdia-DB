# Perdia-DB
A simple key value database for storing simple structures.

## Tokens

|Name|Description|
|-|-|
|QUERY|Used to get a Object by it's name.|
|CREATE|Cerate an object from an template.|
|GET|Get a value from an object with a key.|
|PUT|Overwrite a value from an object using a key.|
|VALUE|Define the value for the PUT command.|
|TYPE|Defines field type of template or defines new template.|
|NAME|Declare field name.|
|STARTING|Declare value of field if nothing is set.|
|END|Mark end of template definition.|

## Responses

|Name|Descirption
|-|-|
|OK|No get queries were done and everything went fine.|


## Example

### Declare Templates

```
TYPE "DAY";
NAME "First" TYPE STRING STARTING "Nothing";
NAME "Second" TYPE STRING STARTING "Nothing";
NAME "Third" TYPE STRING STARTING "Nothing";
NAME "Day" TYPE INTEGER STARTING 1;
NAME "Seconds" TYPE FLOAT;
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
QUERY TYPE GET "DAY" GET "First";
QUERY TYPE GET "DAY" GET "First" "Seconds";
```

Query all types
```
QUERY TYPE
```