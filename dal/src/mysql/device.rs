use std::marker::PhantomData;
use mysql_async::params;
use mysql_async::prelude::Queryable;
use crate::mysql::{generate_id, Mysql, MysqlResult};

#[derive(Debug, Clone)]
pub struct Device<Type = Pump> {
    mysql: Mysql,
    id: String,
    ty: DeviceType,
    _type: PhantomData<Type>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub enum DeviceType {
    Pump,
}

pub struct Pump;
pub struct Sensor;

impl From<DeviceType> for mysql_async::Value {
    fn from(x: DeviceType) -> Self {
        match x {
            DeviceType::Pump => Self::from("Pump"),
        }
    }
}

impl Device {
    pub async fn new(mysql: Mysql, ty: DeviceType) -> MysqlResult<Self> {
        let id = generate_id();
        mysql.get_conn().await?.exec_drop("INSERT INTO devices (id, ty) VALUES (:id, :ty)", params! {
            "id" => &id,
            "ty" => &ty
        }).await?;

        let this = match ty {
            DeviceType::Pump => Self {
                mysql,
                id,
                ty,
                _type: PhantomData
            }
        };

        Ok(this)
    }
}