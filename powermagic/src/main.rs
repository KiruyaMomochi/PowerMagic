pub mod error;
mod manager;
mod model;
mod unit;

use sqlx::sqlite::SqlitePoolOptions;
use unit::*;

#[derive(Debug)]
pub enum Foo {
    Foo,
    Bar,
}

impl Foo {
    pub fn bar(&mut self) {
        *self = Foo::Bar;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new().connect("powermagic.db").await?;
    let data_manager = manager::DataManager::with_pool(pool.clone()).await?;

    let unit_data =
        sqlx::query_as::<_, model::UnitData>("SELECT * FROM unit_data WHERE unit_name == $1")
            .bind("優衣（公主）")
            .fetch_one(&pool)
            .await?;

    let unit_cache = data_manager.unit_cache(unit_data.unit_id).await?;
    let mut calculator = UnitCalculator::new(&unit_cache)
        .set_star(5)
        .set_all_level(190)
        .set_rank(20)
        .unequip_all_equipments()
        .watch_all_stories()
        .memorized();

    let power = calculator.power();
    println!("Power: {}", power.round());

    Ok(())
}
