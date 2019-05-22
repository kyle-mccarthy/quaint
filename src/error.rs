use failure::{Error as FError, Fail};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error querying the database: {}", _0)]
    QueryError(FError),
    #[fail(display = "Query returned no data")]
    NotFound,
    #[fail(display = "Unique constraint failed: {}", field_name)]
    UniqueConstraintViolation { field_name: String },
    #[fail(display = "Error creating a database connection.")]
    ConnectionError(FError),
    #[fail(display = "Error reading the column value: {}", _0)]
    ColumnReadFailure(FError),
    #[fail(display = "Error accessing result set, index out of bounds: {}", _0)]
    ResultIndexOutOfBounts(usize),
    #[fail(
        display = "Error accessing result set, type missmatch, expected: {}",
        _0
    )]
    ResultTypeMissmatch(&'static str),
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Error {
        Error::ConnectionError(e.into())
    }
}
