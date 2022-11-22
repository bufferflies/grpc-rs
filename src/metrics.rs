// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

//! Metrics of the grpc pool.

use lazy_static::lazy_static;
use prometheus::*;

lazy_static! {
     /// Grpc wait duration of one task.
     pub static ref GRPC_TASK_WAIT_DURATION: HistogramVec = register_histogram_vec!(
        "grpc_task_wait_duration",
        "Bucketed histogram of grpc wait time",
        &["name"],
        exponential_buckets(0.00001, 2.0, 20).unwrap()
    )
    .unwrap();

    // Grpc pool pending task .
    pub static ref GRPC_POOL_PENDING_TASK_COUNT: IntGaugeVec = register_int_gauge_vec!(
        "grpc_pool_pending_task_count",
        "Total pending task in grpc pool",
        &["name"]
    )
    .unwrap();
}
