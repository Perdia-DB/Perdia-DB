# Perdia-DB
A simple key value database for storing simple structures.
No nesting of structures is supported, but may be implemented in the future.

## Tokens

|Name|Description|
|-|-|
|QUERY|Used to get a Object by it's name.|
|CREATE|Cerate an object from an template.|
|GET|Get a value from an object with a key.|
|SET|Overwrite a value from an object using a key.|
|VALUE|Define the value for the PUT command.|
|TYPE|Defines field type of template or defines new template.|
|NAME|Declare field name.|
|STARTING|Declare value of field if nothing is set.|
|END|Mark end of template definition.|

## Responses

DB responds with the requested instances or templates with the specified fields.


## Example

### Declare Templates

Declaration of templates should only occur in their own request. Although everything works fine when it is not.
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

Make a instance of a defined template with a key.
```
CREATE "Monday" TYPE "DAY";
```

### Query Objects

Set/Get multiple fields.
```
QUERY "Monday" THEN; 
SET "First" VALUE "Science";
SET "Second" VALUE "CS";
END;
```

Query objects by type
```
QUERY TYPE "DAY" GET "First";
QUERY TYPE "DAY" GET "First" "Seconds";
```

Query all types
```
QUERY TYPE;
```

### Delete Objects

This will delete just the instance.
```
DELETE "Monday"
```

This will remove all instances with the type and the type itself.
```
REMOVE "DAY"
```