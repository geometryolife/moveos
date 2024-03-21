// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rooch_benchmarks::helper::{PProfOut, PPROF_OUT};
use rooch_benchmarks::tx::TxType::{Blog, Empty, Transfer};
use rooch_benchmarks::tx::{
    create_publish_transaction, create_transaction, setup_service, DATA_DIR, TX_TYPE,
};
use rooch_key::keystore::account_keystore::AccountKeystore;
use rooch_key::keystore::memory_keystore::InMemKeystore;
use rooch_test_transaction_builder::TestTransactionBuilder;
use std::time::Duration;
use tokio::runtime::Runtime;

pub fn transaction_write_benchmark(c: &mut Criterion) {
    let keystore = InMemKeystore::new_insecure_for_tests(10);

    let rt: Runtime = Runtime::new().unwrap();
    let (rpc_service, _aggregate_service) =
        rt.block_on(async { setup_service(&DATA_DIR, &keystore).await.unwrap() });

    let default_account = keystore.addresses()[0];
    let mut test_transaction_builder = TestTransactionBuilder::new(default_account.into());

    let mut tx_cnt = 1000;
    if *TX_TYPE == Blog {
        let tx = create_publish_transaction(&test_transaction_builder, &keystore).unwrap();
        let _publish_result = rt.block_on(async { rpc_service.execute_tx(tx).await.unwrap() });
    }

    if *TX_TYPE == Transfer {
        tx_cnt = 1500;
    }

    if *TX_TYPE == Empty {
        tx_cnt = 2500;
    }

    let transactions: Vec<_> = (0..tx_cnt)
        .map(|n| create_transaction(&mut test_transaction_builder, &keystore, n).unwrap())
        .collect();
    let mut transactions_iter = transactions.into_iter().cycle();

    c.bench_function("execute_tx", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| {
            let tx = transactions_iter.next().unwrap();
            rpc_service.execute_tx(tx)
        });
    });
}

fn profiled() -> Criterion {
    let profiler = match *PPROF_OUT {
        PProfOut::Protobuf => PProfProfiler::new(2000, Output::Protobuf),
        PProfOut::Flamegraph => PProfProfiler::new(2000, Output::Flamegraph(None)),
    };
    Criterion::default()
        .with_profiler(profiler)
        .warm_up_time(Duration::from_millis(100))
        .sample_size(10)
        .measurement_time(Duration::from_secs(3))
}

criterion_group! {
    name = rooch_tx_write_bench;
    config = profiled();
    targets = transaction_write_benchmark
}

criterion_main!(rooch_tx_write_bench);
