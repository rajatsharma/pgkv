use pgrx::prelude::*;

pgrx::pg_module_magic!();

extension_sql!(
    r#"
CREATE TABLE pgkv (
    id text not null primary key,
    value json not null
);
"#,
    name = "create_pgkv_table",
);

#[pg_extern]
fn get(key: &str) -> Result<Option<pgrx::Json>, spi::Error> {
    let result = Spi::get_one_with_args(
        "SELECT value FROM pgkv WHERE id = $1",
        vec![(PgBuiltInOids::TEXTOID.oid(), key.into_datum())],
    );

    if let Err(_) = result {
        return Ok(None);
    }

    result
}

#[pg_extern]
fn set(key: &str, value: &str) -> Result<Option<String>, spi::Error> {
    Spi::get_one_with_args(
        "INSERT INTO pgkv (id, value) VALUES ($1, $2) RETURNING id",
        vec![
            (PgBuiltInOids::TEXTOID.oid(), key.into_datum()),
            (PgBuiltInOids::JSONOID.oid(), value.into_datum()),
        ],
    )
}

#[pg_extern]
fn del(key: &str) -> Result<Option<String>, spi::Error> {
    Spi::get_one_with_args(
        "DELETE FROM pgkv WHERE id = $1 RETURNING id",
        vec![(PgBuiltInOids::TEXTOID.oid(), key.into_datum())],
    )
}
