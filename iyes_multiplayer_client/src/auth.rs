use quinn::Connection;

pub struct AuthConnection {
    conn: Connection,
}

impl AuthConnection {
    pub(crate) fn new(conn: Connection) -> Self {
        AuthConnection { conn }
    }
}
