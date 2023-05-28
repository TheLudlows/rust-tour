mod convert;
mod arrow_test;
use sqlparser::{dialect::{GenericDialect, Dialect}, parser::Parser};


fn main() {
    tracing_subscriber::fmt::init();
    let sql = "SELECT a a1, b, 123, myfunc(b), * \
    FROM www.baidu.com \
    WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20 \
    ORDER BY a DESC, b \
    LIMIT 50 OFFSET 10";
    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}

#[derive(Debug)]
struct MyDialect;
impl Dialect for MyDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        todo!()
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        todo!()
    }
}
