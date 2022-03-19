use std::collections::HashMap;

use crate::model;
use sqlx::sqlite::SqlitePoolOptions;
use tokio_stream::StreamExt;

pub struct DataManager {
    pub(crate) pool: sqlx::Pool<sqlx::Sqlite>,
    pub status_coefficient: model::UnitStatusCoefficient,
    pub equipment_enhance_data: HashMap<i64, Vec<model::EquipmentEnhanceData>>,
    pub unique_equipment_enhance_data: HashMap<i64, Vec<model::UniqueEquipmentEnhanceData>>,
}

impl std::fmt::Debug for DataManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let equipment_enhance_data = format!("[{}]", &self.equipment_enhance_data.len());
        let unique_equipment_enhance_data =
            format!("[{}]", &self.unique_equipment_enhance_data.len());
        f.debug_struct("DataManager")
            .field("status_coefficient", &self.status_coefficient)
            .field("equipment_enhance_data", &equipment_enhance_data)
            .field(
                "unique_equipment_enhance_data",
                &unique_equipment_enhance_data,
            )
            .finish()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DataManagerError {
    #[error("Unlock rarity 6 data for slot {0} is not found")]
    UnlockRarity6NotFound(usize),
    #[error("Unlock rarity 6 slot {0} is out of 1-3 range")]
    UnlockRarity6SlotOutOfRange(i64),
    #[error("SQLite error {0}")]
    Sqlite(#[from] sqlx::Error),
    #[error("Equip {0} does not exist")]
    EquipNotFound(i64),
    #[error("Equip enhance data for {0} does not exist")]
    EquipmentEnhanceData(i64),
    #[error("Equip enhance data for {0} does not exist")]
    UniqueEquipmentEnhanceData(i64),
    #[error("Unit {0} has more than one unique equipment")]
    InvalidUniqueEquipMoreThanOne(i64),
}

// Constructor
impl DataManager {
    pub async fn new(connection: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new().connect(connection).await?;
        Self::with_pool(pool).await
    }

    pub async fn with_pool(
        pool: sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let status_coefficient = sqlx::query_as::<_, model::UnitStatusCoefficient>(
            "SELECT * FROM unit_status_coefficient",
        )
        .fetch_one(&pool)
        .await?;

        let mut equipment_enhance_data = HashMap::new();
        let mut query = sqlx::query_as::<_, model::EquipmentEnhanceData>(
            "SELECT * FROM equipment_enhance_data ORDER BY promotion_level, equipment_enhance_level",
        )
        .fetch(&pool);

        equipment_enhance_data.insert(0, vec![]);

        while let Some(row) = query.try_next().await? {
            equipment_enhance_data
                .entry(row.promotion_level)
                .or_insert_with(Vec::new)
                .push(row);
        }

        let mut unique_equipment_enhance_data = HashMap::new();
        let mut query = sqlx::query_as::<_, model::UniqueEquipmentEnhanceData>(
            "SELECT * FROM unique_equipment_enhance_data ORDER BY equip_slot, enhance_level",
        )
        .fetch(&pool);

        while let Some(row) = query.try_next().await? {
            unique_equipment_enhance_data
                .entry(row.equip_slot)
                .or_insert_with(Vec::new)
                .push(row);
        }

        Ok(Self {
            pool: pool.clone(),
            status_coefficient,
            equipment_enhance_data,
            unique_equipment_enhance_data,
        })
    }
}
