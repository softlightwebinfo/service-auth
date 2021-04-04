// db
use diesel::{PgConnection, r2d2::ConnectionManager};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PgConnection;