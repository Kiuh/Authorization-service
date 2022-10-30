use super::super::super::common_types::{Cell as JsonCell, Diff};
use crate::database::cell::intellect::gen::Gen;
use crate::database::cell::intellect::neuron::Neuron;
use crate::database::cell::intellect::Intellect;
use crate::database::cell::module::Module;
use crate::database::cell::Cell;
use crate::database::diff::created::Created;
use crate::database::diff::module_changed::ModuleChanged;
use crate::database::diff::removed::Removed;
use crate::error::ResponseError;
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    pub login: String,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub added: Vec<JsonCell>,
    pub changes: Vec<Diff>,
    pub deleted: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    path: web::Path<(String, i32)>, // generation_name / sendId
    login: web::Query<QueryData>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let data = data.into_inner();
    let login = login.into_inner().login;
    let (generation_name, send_id) = path.into_inner();

    let (added_ids, added_cells): (Vec<_>, Vec<_>) = data
        .added
        .into_iter()
        .map(|cell| {
            (
                cell.own_id,
                Cell {
                    parent_id: cell.parent_id,
                    local_id: cell.own_id,
                    modules: cell
                        .modules
                        .into_iter()
                        .map(|module| Module {
                            name: module.name,
                            value: module.value,
                        })
                        .collect(),
                    intellect: Intellect {
                        in_neuron_count: cell.intellect.input_neurons_count as u32,
                        out_neuron_count: cell.intellect.output_neurons_count as u32,
                        neurons: cell
                            .intellect
                            .neurons
                            .into_iter()
                            .map(|neuron| Neuron { bias: neuron.bias })
                            .collect(),
                        gens: cell
                            .intellect
                            .gens
                            .into_iter()
                            .map(|gen| Gen {
                                from_id: gen.el_neur_number,
                                to_id: gen.fin_neur_number,
                                weight: gen.weight,
                            })
                            .collect(),
                    },
                },
            )
        })
        .unzip();

    Cell::insert_many(
        added_cells,
        &generation_name,
        &login,
        &st.db_connection.pool,
    )
    .await
    .map_err(|e| e.http_status_500())?;

    Created::insert_many(
        added_ids
            .into_iter()
            .map(|local_id| Created {
                local_id: local_id as i32,
            })
            .collect(),
        &login,
        &generation_name,
        send_id,
        &st.db_connection.pool,
    )
    .await
    .map_err(|e| e.http_status_500())?;

    ModuleChanged::insert_many(
        data.changes
            .into_iter()
            .map(|change| ModuleChanged {
                local_id: change.cell_id as i32,
                module: change.name,
                new_value: change.value,
            })
            .collect(),
        &login,
        &generation_name,
        send_id,
        &st.db_connection.pool,
    )
    .await
    .map_err(|e| e.http_status_500())?;

    Removed::insert_many(
        data.deleted
            .into_iter()
            .map(|local_id| Removed {
                local_id: local_id as i32,
            })
            .collect(),
        &login,
        &generation_name,
        send_id,
        &st.db_connection.pool,
    )
    .await
    .map_err(|e| e.http_status_500())?;

    Response {}.into()
}
