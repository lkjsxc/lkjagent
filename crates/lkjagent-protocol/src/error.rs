use crate::model::ParseFault;

pub type ParseResult<T> = Result<T, ParseFault>;
