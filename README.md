# Perdia-DB
A simple key value database for storing simple structures.
No nesting of structures is supported, but may be implemented in the future.

## Environment Variables

|Name|Description|
|-|-|
|PORT|Port on which the db-server will host it's service.|
|AES_KEY|Key used for traffic encryption using AES-128.|
|DIR|Disk-save directory path.|
|SAVE_FREQ|Disk-save interval in seconds.|

## Tokens

|Name|Description|
|-|-|
|QUERY|Used to get an Object by it's name.|
|CREATE|Create an object from a template.|
|GET|Get a value from an object with a key.|
|SET|Overwrite a value from an object using a key.|
|VALUE|Defines the value for the PUT command.|
|TYPE|Defines field type of a template or defines a new template.|
|NAME|Declare field name.|
|STARTING|Declares the value of a field if nothing is set.|
|END|Marks the end of template definition.|
|DELETE|Used to delete instances and templates.|

## Responses

The DB responds with the requested instances or templates with their specified fields.


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

Make an instance of a defined template with a key.
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

This will only the delete the given instance.
```
DELETE "Monday"
```

This will delete all instances with the type and the template itself.
```
DELETE TYPE "DAY"
```