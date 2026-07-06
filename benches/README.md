# Running benchmarks

Use `cargo bench` to run benchmarks.

# Example benchmark output

```
% cargo bench
Running benches/benchmark.rs (target/release/deps/benchmark-b2a89f5f4c1a5769)
Gnuplot not found, using plotters backend
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096)
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 267.2ms.
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 267.25 ms (10 iterations)
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Analyzing
PathOsam::initialization/(Capacity: 16384 Blocksize: 4096)
                        time:   [26.366 ms 26.641 ms 26.980 ms]
                        change: [-1.4469% +0.0811% +1.5536%] (p = 0.91 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096)
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 1.2s.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 1.1824 s (10 iterations)
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Analyzing
PathOsam::initialization/(Capacity: 65536 Blocksize: 4096)
                        time:   [105.48 ms 106.17 ms 106.91 ms]
                        change: [-0.0561% +1.2640% +2.4745%] (p = 0.08 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096)
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 23.5s.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 23.535 s (10 iterations)
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Analyzing
PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096)
                        time:   [2.3060 s 2.3340 s 2.3604 s]
                        change: [-5.2177% -3.1377% -1.1485%] (p = 0.01 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096)
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 100.00 ms (2.9M iterations)
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): Analyzing
PathOsam::alloc/(Capacity: 16384 Blocksize: 4096)
                        time:   [33.849 ns 34.034 ns 34.357 ns]
                        change: [+0.2853% +1.6419% +2.9219%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096)
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 100.00 ms (3.2M iterations)
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): Analyzing
PathOsam::alloc/(Capacity: 65536 Blocksize: 4096)
                        time:   [31.339 ns 31.435 ns 31.535 ns]
                        change: [+0.4011% +1.1739% +1.9165%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096)
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 100.00 ms (2.9M iterations)
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096): Analyzing
PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096)
                        time:   [33.830 ns 33.870 ns 33.905 ns]
                        change: [-0.8756% +0.0563% +0.9406%] (p = 0.92 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096)
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 106.69 ms (165 iterations)
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): Analyzing
PathOsam::read/(Capacity: 16384 Blocksize: 4096)
                        time:   [645.53 µs 647.23 µs 650.45 µs]
                        change: [-1.4966% -0.5218% +0.2975%] (p = 0.32 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096)
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 119.37 ms (165 iterations)
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): Analyzing
PathOsam::read/(Capacity: 65536 Blocksize: 4096)
                        time:   [721.01 µs 721.72 µs 722.64 µs]
                        change: [-6.4280% -5.2108% -4.0008%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096)
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 107.10 ms (110 iterations)
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): Analyzing
PathOsam::read/(Capacity: 1048576 Blocksize: 4096)
                        time:   [945.65 µs 958.52 µs 994.81 µs]
                        change: [-1.6904% +11.446% +29.226%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 108.55 ms (165 iterations)
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2: Analyzing
PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [656.23 µs 684.64 µs 707.14 µs]
                        change: [-1.1263% +1.1726% +4.1315%] (p = 0.44 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 119.90 ms (165 iterations)
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2: Analyzing
PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [725.34 µs 731.61 µs 740.07 µs]
                        change: [-1.5498% -0.3828% +0.7277%] (p = 0.56 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimated 106.02 ms (110 iterations)
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2: Analyzing
PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [937.44 µs 940.49 µs 947.02 µs]
                        change: [-2.0096% -0.9266% +0.2523%] (p = 0.16 > 0.05)
                        No change in performance detected.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low mild
  2 (20.00%) high severe

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096)
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 109.65 ms (110 iterations)
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): Analyzing
PathOsam::write/(Capacity: 16384 Blocksize: 4096)
                        time:   [1.8623 ms 1.9676 ms 2.0391 ms]
                        change: [-6.6058% -0.5219% +5.3987%] (p = 0.87 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096)
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 126.51 ms (110 iterations)
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): Analyzing
PathOsam::write/(Capacity: 65536 Blocksize: 4096)
                        time:   [2.3164 ms 2.3689 ms 2.3917 ms]
                        change: [-2.6891% -0.0971% +2.6024%] (p = 0.94 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096)
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 115.96 ms (110 iterations)
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096): Analyzing
PathOsam::write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [1.4118 ms 1.5625 ms 1.6536 ms]
                        change: [-9.2430% -1.2918% +7.2360%] (p = 0.78 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 137.04 ms (165 iterations)
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2: Analyzing
PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [1.7114 ms 2.1504 ms 2.3882 ms]
                        change: [-19.644% -0.9541% +23.431%] (p = 0.93 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 148.96 ms (165 iterations)
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2: Analyzing
PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [1.8659 ms 2.2872 ms 2.5069 ms]
                        change: [-21.425% -0.5066% +25.205%] (p = 0.97 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimated 131.26 ms (110 iterations)
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2: Analyzing
PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [1.9018 ms 2.2277 ms 2.4149 ms]
                        change: [-27.501% -14.132% +0.6831%] (p = 0.11 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096)
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 101.11 ms (770 iterations)
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096): Analyzing
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096)
                        time:   [324.26 µs 381.25 µs 415.43 µs]
                        change: [-19.149% -6.3475% +9.3036%] (p = 0.43 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096)
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 100.44 ms (770 iterations)
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096): Analyzing
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096)
                        time:   [321.15 µs 376.49 µs 407.88 µs]
                        change: [-15.283% -0.6145% +15.638%] (p = 0.94 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096)
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 100.12 ms (770 iterations)
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096): Analyzing
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [324.35 µs 375.68 µs 406.40 µs]
                        change: [-12.425% +1.4612% +17.866%] (p = 0.86 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 100.41 ms (770 iterations)
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2: Analyzing
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [322.67 µs 374.49 µs 404.45 µs]
                        change: [-17.754% -2.2070% +14.349%] (p = 0.80 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 106.42 ms (825 iterations)
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2: Analyzing
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [330.57 µs 391.77 µs 425.13 µs]
                        change: [-15.156% +0.6930% +19.560%] (p = 0.95 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2: Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimated 106.57 ms (825 iterations)
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2: Analyzing
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [329.49 µs 386.04 µs 422.45 µs]
                        change: [-11.235% +2.9855% +19.794%] (p = 0.73 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64)
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 334.8ms.
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 334.78 ms (10 iterations)
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Analyzing
PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64)
                        time:   [51.271 ms 60.982 ms 70.222 ms]
                        change: [-19.838% +0.9081% +26.619%] (p = 0.94 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64)
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 349.7ms.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 349.66 ms (10 iterations)
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Analyzing
PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64)
                        time:   [37.935 ms 40.648 ms 43.466 ms]
                        change: [-7.9256% +1.4138% +12.221%] (p = 0.79 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64)
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 1.3s.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 1.2809 s (10 iterations)
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Analyzing
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64)
                        time:   [61.644 ms 66.593 ms 72.041 ms]
                        change: [-25.503% -11.409% +6.0370%] (p = 0.24 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 64)
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 103.99 ms (330 iterations)
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 64): Analyzing
PathOsam::initialization/(Capacity: 16384 Blocksize: 64)
                        time:   [324.57 µs 338.59 µs 346.95 µs]
                        change: [+1.6127% +6.6736% +11.647%] (p = 0.02 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64)
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 114.8ms or enable flat sampling.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 114.81 ms (55 iterations)
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Analyzing
PathOsam::initialization/(Capacity: 65536 Blocksize: 64)
                        time:   [1.9076 ms 1.9493 ms 2.0160 ms]
                        change: [-7.4783% -1.2815% +4.9934%] (p = 0.73 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64)
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms

Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 344.2ms.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 344.19 ms (10 iterations)
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Analyzing
PathOsam::initialization/(Capacity: 1048576 Blocksize: 64)
                        time:   [31.866 ms 32.200 ms 32.631 ms]
                        change: [+0.1764% +1.4713% +2.9905%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low mild
  2 (20.00%) high severe

Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64)
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 100.00 ms (3.0M iterations)
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): Analyzing
PathOsam::alloc/(Capacity: 16384 Blocksize: 64)
                        time:   [33.454 ns 33.483 ns 33.534 ns]
                        change: [-1.4238% -0.5280% +0.3536%] (p = 0.31 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64)
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 100.00 ms (3.2M iterations)
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): Analyzing
PathOsam::alloc/(Capacity: 65536 Blocksize: 64)
                        time:   [31.096 ns 31.136 ns 31.168 ns]
                        change: [-2.6971% -1.1714% -0.2624%] (p = 0.09 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64)
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 100.00 ms (3.0M iterations)
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): Analyzing
PathOsam::alloc/(Capacity: 1048576 Blocksize: 64)
                        time:   [33.766 ns 33.799 ns 33.845 ns]
                        change: [-2.3052% -0.9934% -0.1921%] (p = 0.09 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64)
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 100.16 ms (2365 iterations)
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Analyzing
PathOsam::read/(Capacity: 16384 Blocksize: 64)
                        time:   [42.193 µs 42.232 µs 42.268 µs]
                        change: [-1.8914% -0.9216% -0.3617%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64)
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 102.45 ms (2035 iterations)
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Analyzing
PathOsam::read/(Capacity: 65536 Blocksize: 64)
                        time:   [50.234 µs 51.859 µs 52.696 µs]
                        change: [-0.5842% +1.0611% +2.5882%] (p = 0.28 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64)
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 100.99 ms (1375 iterations)
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): Analyzing
PathOsam::read/(Capacity: 1048576 Blocksize: 64)
                        time:   [72.686 µs 73.070 µs 73.412 µs]
                        change: [+5.1404% +6.3006% +7.1503%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: Collecting 10 samples in estimated 100.39 ms (2310 iterations)
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: Analyzing
PathOsam::read/(Capacity: 16384 Blocksize: 64) #2
                        time:   [43.125 µs 43.478 µs 43.853 µs]
                        change: [+1.0066% +1.8855% +2.6760%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: Collecting 10 samples in estimated 102.06 ms (1980 iterations)
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: Analyzing
PathOsam::read/(Capacity: 65536 Blocksize: 64) #2
                        time:   [50.128 µs 50.268 µs 50.431 µs]
                        change: [-0.7648% -0.2197% +0.3011%] (p = 0.46 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 101.56 ms (1485 iterations)
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2: Analyzing
PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [68.019 µs 68.107 µs 68.229 µs]
                        change: [-4.7064% -2.4317% -0.5115%] (p = 0.03 < 0.05)
                        Change within noise threshold.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64)
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 108.72 ms (440 iterations)
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): Analyzing
PathOsam::write/(Capacity: 16384 Blocksize: 64)
                        time:   [613.95 µs 669.53 µs 701.04 µs]
                        change: [-9.2513% -0.7154% +7.9186%] (p = 0.88 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64)
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 104.51 ms (385 iterations)
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): Analyzing
PathOsam::write/(Capacity: 65536 Blocksize: 64)
                        time:   [638.91 µs 678.62 µs 703.59 µs]
                        change: [-7.8410% -1.2472% +5.6404%] (p = 0.73 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64)
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 109.09 ms (330 iterations)
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): Analyzing
PathOsam::write/(Capacity: 1048576 Blocksize: 64)
                        time:   [729.77 µs 758.94 µs 778.91 µs]
                        change: [-3.9956% -0.5979% +2.6126%] (p = 0.76 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: Collecting 10 samples in estimated 102.64 ms (330 iterations)
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: Analyzing
PathOsam::write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [701.88 µs 757.75 µs 790.36 µs]
                        change: [-20.394% -13.003% -4.8178%] (p = 0.01 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: Collecting 10 samples in estimated 109.83 ms (330 iterations)
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: Analyzing
PathOsam::write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [736.53 µs 801.82 µs 841.83 µs]
                        change: [-9.2172% -1.8681% +6.7977%] (p = 0.67 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 110.45 ms (550 iterations)
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2: Analyzing
PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [502.96 µs 616.68 µs 681.85 µs]
                        change: [-20.026% -1.2284% +19.885%] (p = 0.91 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64)
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 100.22 ms (2915 iterations)
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64): Analyzing
PathOsam::local_write/(Capacity: 16384 Blocksize: 64)
                        time:   [77.140 µs 83.595 µs 87.347 µs]
                        change: [-8.3879% -0.4214% +7.7485%] (p = 0.93 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64)
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 101.79 ms (2970 iterations)
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64): Analyzing
PathOsam::local_write/(Capacity: 65536 Blocksize: 64)
                        time:   [76.964 µs 83.804 µs 87.907 µs]
                        change: [-8.3161% -0.4897% +8.3555%] (p = 0.91 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64)
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 100.54 ms (2915 iterations)
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Analyzing
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64)
                        time:   [76.877 µs 83.251 µs 87.014 µs]
                        change: [-8.7476% -0.9264% +7.1863%] (p = 0.84 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Collecting 10 samples in estimated 101.15 ms (2915 iterations)
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Analyzing
PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [79.583 µs 85.468 µs 88.688 µs]
                        change: [-6.7501% +1.6952% +10.245%] (p = 0.71 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Collecting 10 samples in estimated 100.39 ms (2860 iterations)
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Analyzing
PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [80.544 µs 86.990 µs 90.675 µs]
                        change: [-4.6391% +3.6842% +12.222%] (p = 0.41 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Warming up for 100.00 ms
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 101.52 ms (2805 iterations)
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Analyzing
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [81.326 µs 88.217 µs 92.006 µs]
                        change: [+1.4890% +11.335% +23.141%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64)
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64): Warming up for 100.00 ms
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 128.28 ms (20 iterations)
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64): Analyzing
PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64)
                        time:   [22.384 ms 24.681 ms 26.831 ms]
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64)
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64): Warming up for 100.00 ms
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 110.54 ms (20 iterations)
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64): Analyzing
PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64)
                        time:   [17.240 ms 19.818 ms 22.569 ms]
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64)
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64): Warming up for 100.00 ms
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 118.89 ms (20 iterations)
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64): Analyzing
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64)
                        time:   [16.885 ms 19.902 ms 22.933 ms] 

Physical reads and writes incurred by 1 PathOsam::read:
OSAM Capacity   | OSAM Blocksize  | Physical Reads  | Physical Writes
64              | 64              | 6               | 6              
256             | 64              | 8               | 8              
64              | 4096            | 6               | 6              
256             | 4096            | 8               | 8              

Physical reads and writes incurred by 1 PathOsam::write:
OSAM Capacity   | OSAM Blocksize  | Physical Reads  | Physical Writes
64              | 64              | 6               | 6              
256             | 64              | 8               | 8              
64              | 4096            | 6               | 6              
256             | 4096            | 8               | 8              

Physical reads and writes incurred by 64 random PathOsam operations:
OSAM Capacity   | OSAM Blocksize  | Physical Reads  | Physical Writes
64              | 64              | 384             | 384            
256             | 64              | 512             | 512            
64              | 4096            | 384             | 384            
256             | 4096            | 512             | 512                             
```
