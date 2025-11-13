use super::*;
use surrealdb::RecordId;

pub trait GatePassRepository {
    async fn create_gate_pass(&self, request: CreateGatePassRequest) -> Result<GatePass>;

    async fn create_gate_passes(
        &self,
        request: CreateGatePassBatchRequest,
    ) -> Result<CreateGatePassBatchResponse>;

    async fn update_gate_pass(&self, request: UpdateGatePassRequest) -> Result<GatePass>;

    async fn update_gate_pass_block(&self, request: UpdateGatePassBlockRequest)
    -> Result<GatePass>;

    async fn delete_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass>;

    async fn find_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass>;

    async fn search_gate_passes(&self, request: SearchGatePassRequest) -> Result<Vec<GatePass>>;

    async fn find_sync_gate_passes(
        &self,
        request: SyncGatePassRequest,
    ) -> Result<SyncGatePassResponse>;

    async fn renew_gate_passes(&self, request: RenewGatePassRequest) -> Result<()>;
}

impl GatePassRepository for Repository {
    async fn create_gate_pass(&self, request: CreateGatePassRequest) -> Result<GatePass> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $gate_pass_record = CREATE gate_passes SET
                number = type::string(fn::sequence_next('gate_pass_numbers')),
                expired_at = type::datetime($gate_pass.expired_at),
                deleted = false,
                owner = $gate_pass.owner,
                vehicles = $gate_pass.vehicles,
                allow_any_vehicle = $gate_pass.allow_any_vehicle,
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

    async fn create_gate_passes(
        &self,
        request: CreateGatePassBatchRequest,
    ) -> Result<CreateGatePassBatchResponse> {
        let mut created_gate_passes = vec![];
        let mut failed_requests = vec![];
        for create_gate_pass_request in request.requests.into_iter() {
            match self
                .create_gate_pass(create_gate_pass_request.clone())
                .await
            {
                Ok(gate_pass) => created_gate_passes.push(gate_pass),
                Err(error) => {
                    error!(
                        "Failed to create gate pass: request={:?}, error={:?}",
                        create_gate_pass_request, error
                    );
                    failed_requests.push(create_gate_pass_request)
                }
            };
        }
        Ok(CreateGatePassBatchResponse {
            created_gate_passes,
            failed_requests,
        })
    }

    async fn update_gate_pass(&self, request: UpdateGatePassRequest) -> Result<GatePass> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $gate_pass_record = SELECT * FROM ONLY type::thing('gate_passes', $gate_pass_patch.id) WHERE deleted = false;
            IF $gate_pass_record = NONE THEN {
                RETURN NONE
            } END;
            UPDATE $gate_pass_record SET
                expired_at = type::datetime($gate_pass_patch.expired_at),
                owner.title = $gate_pass_patch.owner.title,
                owner.unit = $gate_pass_patch.owner.unit;
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

    async fn update_gate_pass_block(
        &self,
        request: UpdateGatePassBlockRequest,
    ) -> Result<GatePass> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $gate_pass_record = SELECT * FROM ONLY type::thing('gate_passes', $gate_pass_block.id) WHERE deleted = false;
            IF $gate_pass_record = NONE THEN {
                RETURN NONE
            } END;
            IF $gate_pass_block.block = NONE THEN {
                UPDATE $gate_pass_record SET
                    block = NONE;
            } ELSE {
                UPDATE $gate_pass_record SET
                    block = {
                        expired_at: type::datetime($gate_pass_block.block.expired_at),
                        reason: $gate_pass_block.block.reason
                    },
                    updated_by = $gate_pass_block.updated_by;
            } END;
            RETURN SELECT *, id.id() as id FROM $gate_pass_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("gate_pass_block", request))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|gate_pass_opt| {
                if let Some(gate_pass) = gate_pass_opt {
                    info!("GatePass block updated: id={}", &gate_pass.id);
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
            UPDATE $gate_pass_record SET vehicles = [], deleted = true;
            RETURN SELECT *, id.id() as id FROM $gate_pass_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("gate_pass_id", gate_pass_id.to_string()))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|gate_pass_opt| {
                if let Some(gate_pass) = gate_pass_opt {
                    info!("GatePass deleted: id={}", gate_pass.id);
                    Ok(gate_pass)
                } else {
                    Err(DatabaseError::EntryNotFound.into())
                }
            })?
    }

    async fn find_gate_pass(&self, gate_pass_id: impl ToString) -> Result<GatePass> {
        let query = r#"
            SELECT *, id.id() as id FROM ONLY type::thing('gate_passes', $gate_pass_id);
        "#;
        self.database
            .query(query)
            .bind(("gate_pass_id", gate_pass_id.to_string()))
            .await
            .map(take_successful_response)??
            .take::<Option<GatePass>>(0)
            .map(|gate_pass_opt| match gate_pass_opt {
                Some(gate_pass) => {
                    info!("GatePass found: id={}", &gate_pass.id);
                    Ok(gate_pass)
                }
                None => Err(DatabaseError::EntryNotFound.into()),
            })?
    }

    async fn search_gate_passes(&self, request: SearchGatePassRequest) -> Result<Vec<GatePass>> {
        let mut where_clauses = String::from("WHERE deleted = false");
        let id_param_name = "ids";
        let mut id_param_values = vec![];
        if let Some(ids) = request.ids {
            let param_value = ids
                .into_iter()
                .map(|id| RecordId::from(("gate_passes", id.to_string())))
                .collect::<Vec<_>>();
            where_clauses.push_str(&format!(" AND id IN ${} ", id_param_name));
            id_param_values.extend(param_value.into_iter());
        }
        let last_name_param_name = "owner_last_names";
        let mut last_name_param_values = vec![];
        if let Some(last_names) = request.last_names {
            where_clauses.push_str(&format!(
                " AND owner.last_name IN ${} ",
                last_name_param_name
            ));
            last_name_param_values.extend(last_names.into_iter());
        }
        let number_plate_param_name = "vehicle_number_plate_names";
        let mut number_plate_param_values = vec![];
        if let Some(number_plates) = request.number_plates {
            where_clauses.push_str(&format!(
                " AND vehicles[0].number_plate IN ${} ",
                number_plate_param_name
            ));
            number_plate_param_values.extend(number_plates.into_iter());
        }
        let query = format!(
            "SELECT *, id.id() as id FROM gate_passes {} ORDER BY created_at DESC LIMIT {}",
            where_clauses,
            request.number_of_results.unwrap_or(25)
        );

        self.database
            .query(query)
            .bind((id_param_name, id_param_values))
            .bind((last_name_param_name, last_name_param_values))
            .bind((number_plate_param_name, number_plate_param_values))
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
                LET $now = time::now();
                LET $start_updated_at = IF $last_synced_at != NONE {
                    type::datetime($last_synced_at);
                } ELSE {
                    (SELECT updated_at FROM ONLY gate_passes ORDER BY updated_at ASC LIMIT 1).updated_at ?? $now;
                };
                LET $end_updated_at = ($start_updated_at + 30d);
                RETURN {
                    "last_synced_at": time::min([$end_updated_at, $now]),
                    "gate_passes": SELECT *, id.id() as id FROM gate_passes WHERE updated_at >= $start_updated_at AND updated_at < $end_updated_at
                };
        "#;
        self.database
            .query(query)
            .bind(("last_synced_at", request.last_synced_at))
            .await
            .map(take_successful_response)??
            .take::<Option<SyncGatePassResponse>>(3)
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

    async fn renew_gate_passes(&self, request: RenewGatePassRequest) -> Result<()> {
        let mut vehicle_number_plate_clause = String::new();
        let mut vehicle_number_plate_values = vec![];
        if let Some(number_plates) = request.number_plates {
            vehicle_number_plate_clause
                .push_str(" AND vehicles[0].number_plate IN $vehicle_number_plate_values");
            vehicle_number_plate_values.extend(number_plates.into_iter());
        }
        let mut query = String::new();
        query.push_str("BEGIN TRANSACTION; ");
        query.push_str(" (SELECT count() FROM ( ");
        query.push_str(format!(" UPDATE gate_passes SET expired_at = type::datetime($new_expired_at) WHERE deleted = false {vehicle_number_plate_clause} ").as_str());
        query.push_str(" ) GROUP ALL).count; ");
        query.push_str("COMMIT TRANSACTION;");

        self.database
            .query(query)
            .bind(("new_expired_at", request.expired_at))
            .bind(("vehicle_number_plate_values", vehicle_number_plate_values))
            .await
            .map(take_successful_response)??
            .take::<Vec<usize>>(0)
            .map(|gate_pass_ids| {
                info!(
                    "GatePasses renewed: number_of_gate_passes={}",
                    gate_pass_ids.first().unwrap_or(&0)
                );
                Ok(())
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
