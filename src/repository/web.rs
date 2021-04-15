use diesel::prelude::*;

use crate::db::Connection;
use crate::models::web::Web;
use crate::schema::webs::dsl::webs;

impl Web {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Web>> {
        webs.get_results::<Web>(conn)
    }
    pub fn create(conn: &Connection, web_rq: Web) -> bool {
        diesel::insert_into(webs)
            .values(&web_rq)
            .execute(conn).is_ok()
    }
}
