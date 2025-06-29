


use serde::{Serialize};
use serde::Deserialize;
use sea_orm::{prelude::*, ActiveValue, IntoActiveValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", enum_name = "gender", rename_all = "snake_case")]
pub enum Gender {

    // 如果需要做一些转换
    // #[sea_orm(string_value = "01")]
    // #[serde(rename = "01")]
    Male,

    // 如果需要做一些转换
    // #[sea_orm(string_value = "02")] // 数据库存储的时候
    // #[serde(rename = "02")]  // 序列化的时候 
    Female,
}

impl IntoActiveValue<Gender> for Gender {
    fn into_active_value(self) -> ActiveValue<Gender> {
        ActiveValue::Set(self)
    }
}
