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

    async fn search_gate_passes(
        &self,
        request: SearchGatePassRequest,
    ) -> Result<PageResponse<GatePass>>;

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

        let query_params = QueryParams::from_params(json!({"gate_pass": request}));

        self.execute_query_with_params(query, query_params)
            .await?
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
            LET $gate_pass_record = SELECT * FROM ONLY type::thing('gate_passes', $gate_pass.id) WHERE deleted = false;
            IF $gate_pass_record = NONE THEN {
                RETURN NONE
            } END;
            UPDATE $gate_pass_record SET
                expired_at = type::datetime($gate_pass.expired_at),
                owner = $gate_pass.owner,
                vehicles = $gate_pass.vehicles,
                allow_any_vehicle = $gate_pass.allow_any_vehicle,
                updated_by = $gate_pass.updated_by;
            RETURN SELECT *, id.id() as id FROM $gate_pass_record.id;
        COMMIT TRANSACTION;
        "#;

        let query_params = QueryParams::from_params(json!({"gate_pass": request}));

        self.execute_query_with_params(query, query_params)
            .await?
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

        let query_params = QueryParams::from_params(json!({"gate_pass_block": request}));

        self.execute_query_with_params(query, query_params)
            .await?
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

        let query_params =
            QueryParams::from_params(json!({"gate_pass_id": gate_pass_id.to_string()}));

        self.execute_query_with_params(query, query_params)
            .await?
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

        let query_params =
            QueryParams::from_params(json!({"gate_pass_id": gate_pass_id.to_string()}));

        self.execute_query_with_params(query, query_params)
            .await?
            .take::<Option<GatePass>>(0)
            .map(|gate_pass_opt| match gate_pass_opt {
                Some(gate_pass) => {
                    info!("GatePass found: id={}", &gate_pass.id);
                    Ok(gate_pass)
                }
                None => Err(DatabaseError::EntryNotFound.into()),
            })?
    }

    async fn search_gate_passes(
        &self,
        request: SearchGatePassRequest,
    ) -> Result<PageResponse<GatePass>> {
        let mut where_clauses = String::from("WHERE deleted = false");
        let id_param_name = "ids";
        let mut id_param_values = vec![];
        if let Some(ids) = request.ids.filter(|list| !list.is_empty()) {
            where_clauses.push_str(&format!(" AND id IN ${} ", id_param_name));
            id_param_values.extend(gate_pass_record_ids(ids).into_iter());
        }
        let last_name_param_name = "owner_last_names";
        let mut last_name_param_values = vec![];
        if let Some(last_names) = request.last_names.filter(|list| !list.is_empty()) {
            where_clauses.push_str(&format!(
                " AND owner.last_name IN ${} ",
                last_name_param_name
            ));
            last_name_param_values.extend(last_names.into_iter());
        }
        let number_plate_param_name = "vehicle_number_plate_names";
        let mut number_plate_param_values = vec![];
        if let Some(number_plates) = request.number_plates.filter(|list| !list.is_empty()) {
            where_clauses.push_str(&format!(
                " AND vehicles[0].number_plate IN ${} ",
                number_plate_param_name
            ));
            number_plate_param_values.extend(number_plates.into_iter());
        }

        let query_params = QueryParams {
            params: json!({
                last_name_param_name: last_name_param_values,
                number_plate_param_name: number_plate_param_values
            }),
            ids: vec![(id_param_name.to_string(), id_param_values)]
                .into_iter()
                .collect(),
        };

        let query = format!("SELECT *, id.id() as id FROM gate_passes {where_clauses}");

        let mut page_request = PageRequest::new(
            40,
            0,
            vec![OrderBy::new("created_at".to_string(), SortDirection::Desc)],
        );
        if let Some((page_size, page_index)) = request
            .page_request
            .map(|page_request| (page_request.page_size, page_request.page_index))
        {
            page_request.page_size = page_size;
            page_request.page_index = page_index;
        }

        self.execute_query_with_params_and_pagination::<GatePass>(
            &query,
            query_params,
            &page_request,
        )
        .await
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

        let query_params =
            QueryParams::from_params(json!({"last_synced_at": request.last_synced_at}));

        self.execute_query_with_params(query, query_params)
            .await?
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
        let mut where_clauses = String::from(" WHERE deleted = false ");
        let mut id_values = vec![];
        if let Some(ids) = request.ids.filter(|list| !list.is_empty()) {
            where_clauses.push_str(" AND id IN $id_values ");
            id_values.extend(gate_pass_record_ids(ids).into_iter());
        }
        let mut vehicle_number_plate_values = vec![];
        if let Some(number_plates) = request.number_plates.filter(|list| !list.is_empty()) {
            where_clauses
                .push_str(" AND vehicles[0].number_plate IN $vehicle_number_plate_values ");
            vehicle_number_plate_values.extend(number_plates.into_iter());
        }

        let query_params = QueryParams {
            params: json!({
                "new_expired_at": request.expired_at,
                "vehicle_number_plate_values": vehicle_number_plate_values
            }),
            ids: vec![("id_values".to_string(), id_values)]
                .into_iter()
                .collect(),
        };

        let query = r#"
            BEGIN TRANSACTION;
                (SELECT count() FROM (
                    UPDATE gate_passes SET expired_at = type::datetime($new_expired_at) {WHERE_CLAUSES}
                ) GROUP ALL).count;
            COMMIT TRANSACTION;
        "#.replace("{WHERE_CLAUSES}", &where_clauses);

        self.execute_query_with_params(&query, query_params)
            .await?
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

fn gate_pass_record_ids(ids: Vec<Cow<str>>) -> Vec<RecordId> {
    ids.into_iter()
        .map(|id| RecordId::from(("gate_passes", id.to_string())))
        .collect::<Vec<_>>()
}
