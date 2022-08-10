use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bookmarks::Entity")]
    Bookmarks,
}

impl Related<super::bookmarks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bookmarks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // fn new() -> Self {
    //     Self {
    //         id: sea_orm::ActiveValue::Set(Uuid::new_v4().to_string()),
    //         ..<Self as ActiveModelTrait>::default()
    //     }
    // }
}

impl ActiveModel {
    pub fn new_from_json(json: serde_json::Value) -> Result<Self, DbErr> {
        let mut model = Self::new();
        model.set_from_json(json)?;
        Ok(model)
    }

    pub fn from_id_and_json(id: String, json: serde_json::Value) -> Result<Self, DbErr> {
        let mut model = <Self as ActiveModelTrait>::default();
        model.set_from_json(json)?;
        model.id = Set(id);
        Ok(model)
    }
}
