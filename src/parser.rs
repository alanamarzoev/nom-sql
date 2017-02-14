use nom::IResult;
use std::str;

pub use common::{FieldExpression, Operator};
use create::*;
use insert::*;
use select::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum SqlQuery {
    CreateTable(CreateTableStatement),
    Insert(InsertStatement),
    Select(SelectStatement),
}

/// Parse sequence of SQL statements, divided by semicolons or newlines
// named!(pub query_list<&[u8], Vec<SqlQuery> >,
//    many1!(map_res!(selection, |s| { SqlQuery::Select(s) }))
// );

pub fn parse_query(input: &str) -> Result<SqlQuery, &str> {
    // we process all queries in lowercase to avoid having to deal with capitalization in the
    // parser.
    let q_bytes = String::from(input.trim()).into_bytes();

    // TODO(malte): appropriately pass through errors from nom
    match creation(&q_bytes) {
        IResult::Done(_, o) => return Ok(SqlQuery::CreateTable(o)),
        _ => (),
    };

    match insertion(&q_bytes) {

        IResult::Done(_, o) => return Ok(SqlQuery::Insert(o)),
        _ => (),
    };

    match selection(&q_bytes) {
        IResult::Done(_, o) => return Ok(SqlQuery::Select(o)),
        _ => (),
    };

    Err("failed to parse query")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn hash_query() {
        let qstring = "INSERT INTO users VALUES (42, test);";
        let res = parse_query(qstring);
        assert!(res.is_ok());

        let mut h = DefaultHasher::new();
        res.unwrap().hash(&mut h);
        assert_eq!(format!("{:x}", h.finish()), "18c5663ec2a3a77b");
    }

    #[test]
    fn trim_query() {
        let qstring = "   INSERT INTO users VALUES (42, test);     ";
        let res = parse_query(qstring);
        assert!(res.is_ok());
    }
}
