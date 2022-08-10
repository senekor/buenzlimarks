use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bookmarks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: String,
    #[serde(skip_deserializing)]
    pub user_id: String,
    pub name: String,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let mut model = <Self as ActiveModelTrait>::default();
        model.id = Set(Uuid::new_v4().to_string());
        model
    }

    fn before_save(mut self, _: bool) -> Result<Self, DbErr> {
        if !self.url.as_ref().starts_with("http") {
            self.url = Set(format!("https://{}", self.url.as_ref()));
        }
        Ok(self)
    }
}

impl ActiveModel {
    pub fn new_from_json(user_id: String, json: serde_json::Value) -> Result<Self, DbErr> {
        let mut model = Self::new();
        model.set_from_json(json)?;
        model.user_id = Set(user_id);
        Ok(model)
    }

    pub fn parse(id: String, user_id: String, json: serde_json::Value) -> Result<Self, DbErr> {
        let mut model = <Self as ActiveModelTrait>::default();
        model.set_from_json(json)?;
        model.id = Set(id);
        model.user_id = Set(user_id);
        Ok(model)
    }
}
