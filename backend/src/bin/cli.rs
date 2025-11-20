#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let db = db::init_db().await?;
    main_menu(&db).await?;
    Ok(())
}
