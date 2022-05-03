pub mod template;

enum Keywords {
    Type,
    Name,
    Fields,
    Create,
    Query,
    Put,
    Get,
    String,
    Integer,
    Float,
    Starting,
    ENDL,
    Literal(String)
}