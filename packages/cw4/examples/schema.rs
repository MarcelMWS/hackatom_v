use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

pub use cw4::{
    AdminResponse, Cw4HandleMsg, Cw4InitMsg, Cw4QueryMsg, Member, MemberListResponse,
    MemberResponse, TotalWeightResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Cw4HandleMsg), &out_dir);
    export_schema(&schema_for!(Cw4InitMsg), &out_dir);
    export_schema(&schema_for!(Cw4QueryMsg), &out_dir);
    export_schema(&schema_for!(AdminResponse), &out_dir);
    export_schema(&schema_for!(MemberListResponse), &out_dir);
    export_schema(&schema_for!(MemberResponse), &out_dir);
    export_schema(&schema_for!(TotalWeightResponse), &out_dir);
}
