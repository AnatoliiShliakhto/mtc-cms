use crate::error::Error;
use crate::prelude::{Deserialize, Serialize};
use indexed_db::{Database, Factory};
use mtc_common::prelude::SyncGatePass;
use std::borrow::Cow;
use tracing::error;
use wasm_bindgen::JsValue;

static MTC_242_INDEXED_DB: &str = "mtc-242";
static GATE_PASS_INDEXED_DB_STORE: &str = "gate_passes";
static SYNC_ENTRY_INDEXED_DB_STORE: &str = "sync_entries";

pub async fn indexed_db() -> Result<Database<Error>, Error> {
    let indexed_db = Factory::<Error>::get()?
        .open(MTC_242_INDEXED_DB, 1, |event| async move {
            event
                .database()
                .build_object_store(GATE_PASS_INDEXED_DB_STORE)
                .key_path("id")
                .create()?;
            event
                .database()
                .build_object_store(SYNC_ENTRY_INDEXED_DB_STORE)
                .key_path("id")
                .create()?;
            Ok(())
        })
        .await?;
    Ok(indexed_db)
}

pub async fn process_gate_passes(
    indexed_db: &Database<Error>,
    mut gate_passes: Vec<SyncGatePass>,
) -> Result<(), Error> {
    let batch_size: usize = 100;
    while !gate_passes.is_empty() {
        let gate_pass_batch = gate_passes
            .drain(0..batch_size.min(gate_passes.len()))
            .collect::<Vec<SyncGatePass>>();

        indexed_db
            .transaction(&[GATE_PASS_INDEXED_DB_STORE])
            .rw()
            .run(|transaction| async move {
                let store = transaction.object_store(GATE_PASS_INDEXED_DB_STORE)?;
                for gate_pass in gate_pass_batch {
                    if gate_pass.deleted == false {
                        let js_value = serde_wasm_bindgen::to_value(&gate_pass).unwrap();
                        store.put(&js_value).await?;
                    } else {
                        let js_value = serde_wasm_bindgen::to_value(&gate_pass.id).unwrap();
                        store.delete(&js_value).await?;
                    }
                }
                Ok(())
            })
            .await?;
    }
    Ok(())
}

pub async fn get_gate_pass(
    indexed_db: &Database<Error>,
    gate_pass_id: Cow<'static, str>,
) -> Result<SyncGatePass, Error> {
    let indexed_db_gate_pass_id = gate_pass_id.clone();
    indexed_db
        .transaction(&[GATE_PASS_INDEXED_DB_STORE])
        .rw()
        .run(|transaction| async move {
            transaction
                .object_store(GATE_PASS_INDEXED_DB_STORE)?
                .get(&JsValue::from_str(indexed_db_gate_pass_id.as_ref()))
                .await
                .and_then(|gate_pass_js_value_opt| match gate_pass_js_value_opt {
                    Some(gate_pass_js_value) => Ok(gate_pass_js_value),
                    None => Err(indexed_db::Error::DoesNotExist),
                })
        })
        .await
        .map_err(Error::from)
        .and_then(|gate_pass_js_value| parse(gate_pass_js_value, gate_pass_id))
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyncEntry {
    pub id: Cow<'static, str>,
    pub last_synced_at: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncEntryId {
    #[default]
    GatePassSync,
}

impl SyncEntryId {
    pub fn name(&self) -> Cow<'static, str> {
        Cow::Owned(format!("{:?}", self))
    }
}

pub async fn put_sync_entry(
    indexed_db: &Database<Error>,
    sync_entry: SyncEntry,
) -> Result<(), Error> {
    indexed_db
        .transaction(&[SYNC_ENTRY_INDEXED_DB_STORE])
        .rw()
        .run(|transaction| async move {
            let store = transaction.object_store(SYNC_ENTRY_INDEXED_DB_STORE)?;
            let js_value = serde_wasm_bindgen::to_value(&sync_entry).unwrap();
            store.put(&js_value).await?;
            Ok(())
        })
        .await?;
    Ok(())
}

pub async fn get_sync_entry(
    indexed_db: &Database<Error>,
    sync_entry_id: SyncEntryId,
) -> Result<SyncEntry, Error> {
    let sync_entry_id_name = sync_entry_id.name();
    indexed_db
        .transaction(&[SYNC_ENTRY_INDEXED_DB_STORE])
        .rw()
        .run(|transaction| async move {
            transaction
                .object_store(SYNC_ENTRY_INDEXED_DB_STORE)?
                .get(&JsValue::from(sync_entry_id_name.as_ref()))
                .await
                .and_then(|scan_entry_js_value_opt| match scan_entry_js_value_opt {
                    Some(scan_entry_js_value) => Ok(scan_entry_js_value),
                    None => Err(indexed_db::Error::DoesNotExist),
                })
        })
        .await
        .map_err(Error::from)
        .and_then(|scan_entry_js_value| parse(scan_entry_js_value, sync_entry_id.name()))
}

fn parse<T: serde::de::DeserializeOwned>(
    js_value: JsValue,
    object_id: Cow<'static, str>,
) -> Result<T, Error> {
    serde_wasm_bindgen::from_value(js_value).map_err(|js_value| {
        let message = format!("failed to deserialize indexed db object: object_id={object_id}");
        error!("{}, js_value={:?}", message, js_value);
        Error::Deserialization(Cow::Owned(message))
    })
}
