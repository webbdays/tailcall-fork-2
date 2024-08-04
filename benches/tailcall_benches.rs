use criterion::{criterion_group, criterion_main, Criterion};

mod bench_synth;
mod data_loader_bench;
mod from_json_bench;
mod handle_request_bench;
mod http_execute_bench;
mod impl_path_string_for_evaluation_context;
mod json_like_bench;
mod protobuf_convert_output;
mod request_template_bench;

fn all_benchmarks(c: &mut Criterion) {
    data_loader_bench::benchmark_data_loader(c);
    impl_path_string_for_evaluation_context::bench_main(c);
    json_like_bench::benchmark_batched_body(c);
    json_like_bench::benchmark_group_by(c);
    protobuf_convert_output::benchmark_convert_output(c);
    request_template_bench::benchmark_to_request(c);
    handle_request_bench::benchmark_handle_request(c);
    http_execute_bench::benchmark_http_execute_method(c);
    from_json_bench::benchmark_from_json_method(c);
    bench_synth::bench_synth_nested(c);
    bench_synth::bench_synth_nested_borrow(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = all_benchmarks
}
criterion_main!(benches);
