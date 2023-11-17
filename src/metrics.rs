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
        exponential_buckets(1e-7, 2.0, 20).unwrap() // 100ns ~ 100ms
    )
    .unwrap();

    // Grpc pool io handle duration .
    pub static ref GRPC_POOL_IO_HANDLE_DURATION: HistogramVec = register_histogram_vec!(
        "grpc_pool_io_handle_duration",
        "Bucketed histogram of grpc pool handle duration",
        &["name"],
        exponential_buckets(1e-7, 2.0, 20).unwrap() // 100ns ~ 100ms
    )
    .unwrap();

    // Grpc handle repoll duration
    pub static ref GRPC_POOL_REPOLL_DURATION: HistogramVec = register_histogram_vec!(
        "grpc_pool_repoll_duration",
        "Bucketed histogram of grpc pool repoll duration",
        &["name"],
        exponential_buckets(1e-7, 2.0, 20).unwrap() // 100ns ~ 100ms
    )
    .unwrap();

    pub static ref GRPC_POOL_TASK_TOTAL_DURATION: HistogramVec = register_histogram_vec!(
        "grpc_pool_task_total_duration",
        "Bucketed histogram of grpc pool task total duration",
        &["name"],
        exponential_buckets(1e-7, 2.0, 20).unwrap() // 100ns ~ 100ms
    )
    .unwrap();

    // Grpc pool event count task .
    pub static ref GRPC_POOL_EVENT_COUNT_VEC: IntCounterVec = register_int_counter_vec!(
        "grpc_pool_event_task_count",
        "Total event task count in grpc pool",
        &["name","event"]
    )
    .unwrap();


}
