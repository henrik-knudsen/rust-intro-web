use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::ToSql, NoTls, Row};

pub struct DataAccess(pub DatabaseConnection);

pub struct DatabaseConnection {
    pub conn: PooledConnection<'static, PostgresConnectionManager<NoTls>>,
}

impl DatabaseConnection {
    pub async fn query<T>(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<T>, tokio_postgres::Error>
    where
        T: From<Row>,
    {
        let rows = self.conn.query(query, params).await?;

        let movies = rows.into_iter().map(|row| row.into()).collect();

        Ok(movies)
    }

    pub async fn query_one<T>(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<T, tokio_postgres::Error>
    where
        T: From<Row>,
    {
        let row = self.conn.query_one(query, params).await?;

        Ok(row.into())
    }

    pub async fn query_opt<T>(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<T>, tokio_postgres::Error>
    where
        T: From<Row>,
    {
        let row = self.conn.query_opt(query, params).await?;

        let Some(inner) = row else {
            return Ok(None);
        };

        Ok(Some(inner.into()))
    }

    pub async fn execute(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        let num_affected = self.conn.execute(statement, params).await?;

        Ok(num_affected)
    }
}
