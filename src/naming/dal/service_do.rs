use rsql_builder::B;
use rusqlite::{Connection,Row};
use serde::{Serialize,Deserialize};

use crate::common::rusqlite_utils::{get_row_value,sqlite_execute,sqlite_fetch};

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct ServiceDO {
    pub id:Option<i64>,
    pub namespace_id:Option<String>,
    pub service_name:Option<String>,
    pub group_name:Option<String>,
    pub instance_size:Option<i64>,
    pub healthy_size:Option<i64>,
    pub threshold:Option<f64>,
    pub metadata:Option<String>,
    pub extend_info:Option<String>,
    pub create_time:Option<f64>,
    pub last_time:Option<f64>,
}

impl ServiceDO {
    pub fn from_row(r:&Row) -> Self {
        let mut s = Self::default();
        s.id = get_row_value(r,"id");
        s.namespace_id = get_row_value(r,"namespace_id");
        s.service_name = get_row_value(r,"service_name");
        s.group_name = get_row_value(r,"group_name");
        s.instance_size = get_row_value(r,"instance_size");
        s.healthy_size = get_row_value(r,"healthy_size");
        s.threshold = get_row_value(r,"threshold");
        s.metadata = get_row_value(r,"metadata");
        s.extend_info = get_row_value(r,"extend_info");
        s.create_time = get_row_value(r,"create_time");
        s.last_time = get_row_value(r,"last_time");
        s
    }
}

#[derive(Debug,Default,Clone)]
pub struct ServiceParam{
    pub id:Option<i64>,
    pub limit:Option<i64>,
    pub offset:Option<i64>,
}
