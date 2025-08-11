use super::*;
use std::collections::BTreeMap;
use tracing_subscriber::fmt::format;

pub trait GatePassRepository {
    async fn create_gate_pass(&self, request: CreateGatePassRequest) -> Result<GatePass>;

    async fn update_gate_pass(&self, request: UpdateGatePassRequest) -> Result<GatePass>;

    async fn delete_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass>;

    async fn find_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass>;

    async fn find_gate_passes(&self, request: SearchGatePassRequest) -> Result<Vec<GatePass>>;

    async fn find_sync_gate_passes(
        &self,
        request: SyncGatePassRequest,
    ) -> Result<SyncGatePassResponse>;
}

impl GatePassRepository for Repository {
    async fn create_gate_pass(&self, request: CreateGatePassRequest) -> Result<GatePass> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $gate_pass_record = CREATE gate_passes SET
                expired_at = type::datetime($gate_pass.expired_at),
                deleted = false,
                owner = $gate_pass.owner,
                vehicle = $gate_pass.vehicle,
                created_by = $gate_pass.created_by,
                updated_by = $gate_pass.updated_by;
            RETURN SELECT *, id.id() as id FROM $gate_pass_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("gate_pass", request))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|gate_pass_opt| {
                if let Some(gate_pass) = gate_pass_opt {
                    info!("GatePass created: id={}", &gate_pass.id);
                    Ok(gate_pass)
                } else {
                    Err(DatabaseError::SomethingWentWrong.into())
                }
            })?
    }

    async fn update_gate_pass(&self, request: UpdateGatePassRequest) -> Result<GatePass> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $gate_pass_record = SELECT * FROM ONLY type::thing('gate_passes', $gate_pass_patch.id) WHERE deleted = false;
            IF $gate_pass_record = NONE THEN {
                RETURN NONE
            } END;
            UPDATE $gate_pass_record MERGE $gate_pass_patch;
            RETURN SELECT *, id.id() as id FROM $gate_pass_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("gate_pass_patch", request))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|gate_pass_opt| {
                if let Some(gate_pass) = gate_pass_opt {
                    info!("GatePass updated: id={}", &gate_pass.id);
                    Ok(gate_pass)
                } else {
                    Err(DatabaseError::EntryNotFound.into())
                }
            })?
    }

    async fn delete_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $gate_pass_record = SELECT * FROM ONLY type::thing('gate_passes', $gate_pass_id) WHERE deleted = false;
            IF $gate_pass_record = NONE THEN {
                RETURN NONE
            } END;
            UPDATE $gate_pass_record SET deleted = true;
            RETURN SELECT *, id.id() as id FROM $gate_pass_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("gate_pass_id", gate_pass_id.to_string()))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|gate_path_opt| {
                if let Some(gate_path) = gate_path_opt {
                    info!("GatePath deleted: id={}", gate_path.id);
                    Ok(gate_path)
                } else {
                    Err(DatabaseError::EntryNotFound.into())
                }
            })?
    }

    async fn find_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass> {
        let query = r#"
            SELECT *, id.id() as id FROM ONLY type::thing('gate_passes', $gate_pass_id) WHERE deleted = false;
        "#;
        self.database
            .query(query)
            .bind(("gate_pass_id", gate_pass_id.to_string()))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|quiz_opt| match quiz_opt {
                Some(gate_pass) => {
                    info!("GatePass found: id={}", &gate_pass.id);
                    Ok(gate_pass)
                }
                None => Err(DatabaseError::EntryNotFound.into()),
            })?
    }

    async fn find_gate_passes(&self, request: SearchGatePassRequest) -> Result<Vec<GatePass>> {
        let mut where_clauses = String::new();
        let mut where_clause_params = BTreeMap::new();
        let mut add_where_clause = |(subpath, column, value)| {
            if where_clauses.is_empty() {
                where_clauses.push_str("WHERE ");
            } else {
                where_clauses.push_str("AND ");
            }
            let column_param = format!("{subpath}_{column}");
            where_clauses.push_str(format!("{subpath}.{column} = ${column_param} ").as_str());
            where_clause_params.insert(column_param, value);
        };
        if let Some(first_name) = request.first_name {
            add_where_clause(("owner", "first_name", first_name.to_string()));
        }
        if let Some(middle_name) = request.middle_name {
            add_where_clause(("owner", "middle_name", middle_name.to_string()));
        }
        if let Some(last_name) = request.last_name {
            add_where_clause(("owner", "last_name", last_name.to_string()));
        }
        if let Some(number) = request.number_plate {
            add_where_clause(("vehicle", "number_plate", number.to_string()));
        }
        if let Some(manufacturer) = request.manufacturer {
            add_where_clause(("vehicle", "manufacturer", manufacturer.to_string()));
        }
        if let Some(color) = request.color {
            add_where_clause(("vehicle", "color", color.to_string()));
        }
        let query = format!("SELECT *, id.id() as id FROM gate_passes {where_clauses}");
        self.database
            .query(query)
            .bind(where_clause_params)
            .await
            .map(take_successful_response)??
            .take::<Vec<GatePass>>(0)
            .map(|gate_passes| {
                info!(
                    "GatePasses found: number_of_gate_passes={}",
                    gate_passes.len()
                );
                Ok(gate_passes)
            })?
    }

    async fn find_sync_gate_passes(
        &self,
        request: SyncGatePassRequest,
    ) -> Result<SyncGatePassResponse> {
        let query = r#"
            LET $start_updated_at = IF $updated_after != NONE {
                type::datetime($updated_after);
            } ELSE {
                (SELECT updated_at FROM ONLY gate_passes WHERE deleted = false ORDER BY updated_at ASC LIMIT 1).updated_at;
            };
            LET $end_updated_at = IF $updated_before != NONE {
                type::datetime($updated_before);
            } ELSE {
                ($start_updated_at + 30d);
            };
            RETURN {
                "updated_after": $start_updated_at,
                "updated_before": $end_updated_at,
                "gate_passes": SELECT *, id.id() as id FROM gate_passes WHERE updated_at >= $start_updated_at AND updated_at < $end_updated_at
            };
        "#;
        self.database
            .query(query)
            .bind(("updated_after", request.updated_after))
            .bind(("updated_before", request.updated_before))
            .await
            .map(take_successful_response)??
            .take::<Option<SyncGatePassResponse>>(2)
            .map(|response_opt| match response_opt {
                Some(response) => {
                    info!(
                        "GatePasses found: number_of_gate_passes={}",
                        response.gate_passes.len()
                    );
                    Ok(response)
                }
                None => Err(DatabaseError::EntryNotFound.into()),
            })?
    }
}

fn take_successful_response(mut response: surrealdb::Response) -> Result<surrealdb::Response> {
    let errors = response.take_errors();
    if errors.is_empty() {
        Ok(response)
    } else {
        let errors = errors
            .values()
            .map(|error| error.to_string())
            .collect::<String>();
        error!("Error occurred: {}", errors);
        Err(DatabaseError::SomethingWentWrong.into())
    }
}
