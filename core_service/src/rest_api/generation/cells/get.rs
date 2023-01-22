use super::super::super::common_types::Cell as JsonCell;
use crate::database::cell::Cell;
use crate::error::ResponseError;
use crate::rest_api::common_types::{Gen, Intellect, Module, Neuron};
use crate::{rest_api::into_success_response, server_state::ServerState};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    pub cells: Vec<JsonCell>,
}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    path: web::Path<(i32, String, i32)>, // user_id, name, send_id
) -> actix_web::Result<HttpResponse> {
    let (user_id, generation_name, send_id) = path.into_inner();

    let alive: Vec<JsonCell> =
        Cell::fetch_alive(&generation_name, user_id, send_id, &st.db_connection.pool)
            .await
            .map_err(|e| e.http_status_500())?
            .into_iter()
            .map(|cell| JsonCell {
                own_id: cell.local_id,
                parent_id: cell.parent_id,
                modules: cell
                    .modules
                    .into_iter()
                    .map(|module| Module {
                        name: module.name,
                        value: module.value,
                    })
                    .collect(),
                intellect: Intellect {
                    neurons_count: cell.intellect.neurons.len() as u64,
                    gens_count: cell.intellect.gens.len() as u64,
                    input_neurons_count: cell.intellect.in_neuron_count as u64,
                    output_neurons_count: cell.intellect.out_neuron_count as u64,
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
                            el_neur_number: gen.from_id,
                            fin_neur_number: gen.to_id,
                            weight: gen.weight,
                        })
                        .collect(),
                },
            })
            .collect();

    Response { cells: alive }.into()
}
