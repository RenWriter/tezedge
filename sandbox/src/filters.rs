use slog::Logger;
use warp::Filter;

use crate::handlers::{
    activate_protocol, bake_block_with_client, bake_block_with_client_arbitrary, get_wallets,
    handle_rejection, init_client_data, start_node_with_config, stop_node,
};
use crate::node_runner::LightNodeRunnerRef;
use crate::tezos_client_runner::{
    BakeRequest, SandboxWallets, TezosClientRunnerRef, TezosProtcolActivationParameters,
};

pub fn sandbox(
    log: Logger,
    runner: LightNodeRunnerRef,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Allow cors from any origin
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);

    start(log.clone(), runner.clone())
        .or(stop(log.clone(), runner, client_runner.clone()))
        .or(init_client(log.clone(), client_runner.clone()))
        .or(wallets(log.clone(), client_runner.clone()))
        .or(activate(log.clone(), client_runner.clone()))
        .or(bake(log.clone(), client_runner.clone()))
        .or(bake_random(log, client_runner))
        .recover(handle_rejection)
        .with(cors)
}

pub fn start(
    log: Logger,
    runner: LightNodeRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("start")
        .and(warp::post())
        .and(json_body())
        .and(with_log(log))
        .and(with_runner(runner))
        .and_then(start_node_with_config)
}

pub fn stop(
    log: Logger,
    runner: LightNodeRunnerRef,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("stop")
        .and(warp::get())
        .and(with_log(log))
        .and(with_runner(runner))
        .and(with_client_runner(client_runner))
        .and_then(stop_node)
}

pub fn init_client(
    log: Logger,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("init_client")
        .and(warp::post())
        .and(init_client_json_body())
        .and(with_log(log))
        .and(with_client_runner(client_runner))
        .and_then(init_client_data)
}

pub fn wallets(
    log: Logger,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("wallets")
        .and(warp::get())
        .and(with_log(log))
        .and(with_client_runner(client_runner))
        .and_then(get_wallets)
}

pub fn activate(
    log: Logger,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("activate_protocol")
        .and(warp::post())
        .and(activation_json_body())
        .and(with_log(log))
        .and(with_client_runner(client_runner))
        .and_then(activate_protocol)
}

pub fn bake(
    log: Logger,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("bake")
        .and(warp::post())
        .and(bake_json_body())
        .and(with_log(log))
        .and(with_client_runner(client_runner))
        .and_then(bake_block_with_client)
}

pub fn bake_random(
    log: Logger,
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("bake")
        .and(warp::get())
        .and(with_log(log))
        .and(with_client_runner(client_runner))
        .and_then(bake_block_with_client_arbitrary)
}

fn json_body() -> impl Filter<Extract = (serde_json::Value,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn init_client_json_body(
) -> impl Filter<Extract = (SandboxWallets,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn activation_json_body(
) -> impl Filter<Extract = (TezosProtcolActivationParameters,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body and serialize it to TezosProtcolActivationParameters
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn bake_json_body() -> impl Filter<Extract = (BakeRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body with the deserialized BakeRequest
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_log(
    log: Logger,
) -> impl Filter<Extract = (Logger,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || log.clone())
}

fn with_runner(
    runner: LightNodeRunnerRef,
) -> impl Filter<Extract = (LightNodeRunnerRef,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || runner.clone())
}

fn with_client_runner(
    client_runner: TezosClientRunnerRef,
) -> impl Filter<Extract = (TezosClientRunnerRef,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client_runner.clone())
}
