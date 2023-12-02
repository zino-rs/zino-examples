mod user;

pub(crate) use user::User;

pub fn into_insert_sql(table: &str, mut values: Vec<(&str, String)>) -> String {
    values.retain(|(_, v)| !v.is_empty());

    let (fields, values): (Vec<&str>, Vec<String>) = values
        .into_iter()
        .unzip();

    let fields = fields.join(", ");
    let values = values.join(", ");

    format!("INSERT INTO {table} ({fields}) VALUES ({values});")
}
