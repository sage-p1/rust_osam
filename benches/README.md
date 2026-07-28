# Running benchmarks

Use `cargo bench` to run benchmarks.

# Example benchmark output

```
% cargo bench
Running benches/benchmark.rs (target/release/deps/benchmark-57e8b7d72679c6a5)
Gnuplot not found, using plotters backend
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 409
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 339.4ms.
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 409
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 409
PathOsam::initialization/(Capacity: 16384 Blocksize: 4096)
                        time:   [27.401 ms 27.691 ms 28.018 ms]
                        change: [-34.021% -24.216% -14.982%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 409
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 1.8s.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 409
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 409
PathOsam::initialization/(Capacity: 65536 Blocksize: 4096)
                        time:   [114.98 ms 122.34 ms 130.22 ms]
                        change: [-39.081% -29.574% -18.222%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 24.8s.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4
PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096)
                        time:   [2.4397 s 2.4813 s 2.5288 s]
                        change: [-37.753% -35.267% -32.818%] (p = 0.00 < 0.05)
                        Performance has improved.

PathOsam::alloc/(Capacity: 16384 Blocksize: 4096)
                        time:   [33.335 ns 33.415 ns 33.554 ns]
                        change: [-39.488% -25.416% -10.256%] (p = 0.01 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
PathOsam::alloc/(Capacity: 65536 Blocksize: 4096)
                        time:   [31.068 ns 31.142 ns 31.234 ns]
                        change: [-5.1061% -4.0076% -2.9559%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096)
                        time:   [34.065 ns 34.756 ns 35.468 ns]
                        change: [-13.706% -8.7432% -4.7094%] (p = 0.00 < 0.05)
                        Performance has improved.

PathOsam::read/(Capacity: 16384 Blocksize: 4096)
                        time:   [1.0350 ms 1.0380 ms 1.0448 ms]
                        change: [-50.675% -43.013% -36.226%] (p = 0.00 < 0.05)
                        Performance has improved.
PathOsam::read/(Capacity: 65536 Blocksize: 4096)
                        time:   [1.2115 ms 1.2279 ms 1.2487 ms]
                        change: [-29.856% -23.011% -15.631%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
PathOsam::read/(Capacity: 1048576 Blocksize: 4096)
                        time:   [1.6223 ms 1.6569 ms 1.7271 ms]
                        change: [-19.965% -15.409% -10.323%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [1.0273 ms 1.0298 ms 1.0350 ms]
                        change: [-15.077% -10.789% -6.7727%] (p = 0.00 < 0.05)
                        Performance has improved.
PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [1.1874 ms 1.1914 ms 1.1960 ms]
                        change: [-21.193% -15.955% -11.196%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [1.5686 ms 1.6034 ms 1.6508 ms]
                        change: [-99.624% -99.597% -99.565%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

PathOsam::write/(Capacity: 16384 Blocksize: 4096)
                        time:   [969.95 µs 976.26 µs 984.74 µs]
                        change: [-65.682% -53.899% -38.686%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::write/(Capacity: 65536 Blocksize: 4096)
                        time:   [1.1439 ms 1.1718 ms 1.2104 ms]
                        change: [-64.774% -48.490% -21.778%] (p = 0.02 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [1.5347 ms 1.5601 ms 1.5869 ms]
                        change: [-79.337% -68.033% -46.077%] (p = 0.01 < 0.05)
                        Performance has improved.

PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [2.1065 ms 2.3875 ms 2.5577 ms]
                        change: [-52.825% -42.268% -26.011%] (p = 0.00 < 0.05)
                        Performance has improved.
PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [2.2616 ms 2.5398 ms 2.6995 ms]
                        change: [-15.716% +2.8599% +22.710%] (p = 0.81 > 0.05)
                        No change in performance detected.
PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [2.1529 ms 2.3968 ms 2.5348 ms]
                        change: [-9.2406% +2.6487% +16.577%] (p = 0.70 > 0.05)
                        No change in performance detected.

PathOsam::local_write/(Capacity: 16384 Blocksize: 4096)
                        time:   [327.82 µs 382.86 µs 416.21 µs]
                        change: [-53.640% -32.885% +0.8950%] (p = 0.12 > 0.05)
                        No change in performance detected.
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096)
                        time:   [331.37 µs 377.03 µs 401.64 µs]
                        change: [-62.142% -53.813% -42.468%] (p = 0.00 < 0.05)
                        Performance has improved.
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [323.85 µs 374.19 µs 404.17 µs]
                        change: [-23.106% -8.1106% +12.722%] (p = 0.43 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 106.40 ms (825 iterations
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [327.89 µs 383.01 µs 415.78 µs]
                        change: [-17.395% -6.7374% +5.3951%] (p = 0.31 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 107.28 ms (660 iterations
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [315.91 µs 358.33 µs 383.34 µs]
                        change: [-20.281% -9.8989% +2.3124%] (p = 0.13 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimated 100.86 ms (770 iteratio
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [325.88 µs 374.46 µs 402.63 µs]
                        change: [-65.897% -52.689% -30.739%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 480.4ms.
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 480.37 ms (10
PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64)
                        time:   [54.233 ms 60.670 ms 67.511 ms]
                        change: [-53.306% -41.492% -26.087%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 624.4ms.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 624.41 ms (10
PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64)
                        time:   [60.519 ms 62.757 ms 65.390 ms]
                        change: [-5.2122% +0.1378% +5.7013%] (p = 0.97 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 2.4s.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 2.4264 s (1
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64)
                        time:   [85.722 ms 90.265 ms 94.873 ms]
                        change: [-19.092% -6.4924% +5.2126%] (p = 0.45 > 0.05)
                        No change in performance detected.

PathOsam::initialization/(Capacity: 16384 Blocksize: 64)
                        time:   [290.87 µs 296.02 µs 308.90 µs]
                        change: [-5.6716% +4.4906% +15.990%] (p = 0.46 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 123.8ms or enable flat sampling.
PathOsam::initialization/(Capacity: 65536 Blocksize: 64)
                        time:   [1.8960 ms 1.9827 ms 2.1460 ms]
                        change: [-7.8054% -1.4970% +4.6122%] (p = 0.67 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 322.2ms.
PathOsam::initialization/(Capacity: 1048576 Blocksize: 64)
                        time:   [31.682 ms 31.994 ms 32.385 ms]
                        change: [-0.7567% +0.5215% +1.8736%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

PathOsam::alloc/(Capacity: 16384 Blocksize: 64)
                        time:   [33.370 ns 33.467 ns 33.663 ns]
                        change: [-3.1762% -2.0442% -0.4759%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::alloc/(Capacity: 65536 Blocksize: 64)
                        time:   [31.022 ns 31.137 ns 31.354 ns]
                        change: [-5.0842% -3.7177% -2.4288%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
PathOsam::alloc/(Capacity: 1048576 Blocksize: 64)
                        time:   [33.774 ns 33.855 ns 33.998 ns]
                        change: [-3.6996% -1.9284% -0.5353%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

PathOsam::read/(Capacity: 16384 Blocksize: 64)
                        time:   [66.097 µs 66.237 µs 66.574 µs]
                        change: [-0.7854% +0.4427% +1.6531%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::read/(Capacity: 65536 Blocksize: 64)
                        time:   [80.419 µs 80.653 µs 81.136 µs]
                        change: [-1.2311% +0.0978% +1.5156%] (p = 0.90 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::read/(Capacity: 1048576 Blocksize: 64)
                        time:   [116.62 µs 117.01 µs 117.75 µs]
                        change: [+1.0491% +2.5118% +3.7783%] (p = 0.00 < 0.05)
                        Performance has regressed.

PathOsam::read/(Capacity: 16384 Blocksize: 64) #2
                        time:   [67.597 µs 67.873 µs 68.493 µs]
                        change: [+0.1181% +2.2219% +3.8929%] (p = 0.04 < 0.05)
                        Change within noise threshold.
PathOsam::read/(Capacity: 65536 Blocksize: 64) #2
                        time:   [80.173 µs 80.285 µs 80.556 µs]
                        change: [-0.7445% +0.2587% +1.1782%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [113.78 µs 114.22 µs 114.86 µs]
                        change: [-2.3849% -0.7693% +0.8699%] (p = 0.39 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

PathOsam::write/(Capacity: 16384 Blocksize: 64)
                        time:   [209.63 µs 234.69 µs 250.28 µs]
                        change: [-6.5822% +1.1085% +8.7106%] (p = 0.80 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
PathOsam::write/(Capacity: 65536 Blocksize: 64)
                        time:   [175.17 µs 218.94 µs 274.29 µs]
                        change: [-7.4406% +22.417% +72.594%] (p = 0.32 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
PathOsam::write/(Capacity: 1048576 Blocksize: 64)
                        time:   [206.99 µs 218.41 µs 226.83 µs]
                        change: [-10.183% -3.1668% +4.7671%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high mild

PathOsam::write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [751.38 µs 813.03 µs 849.51 µs]
                        change: [+43.462% +64.782% +92.407%] (p = 0.00 < 0.05)
                        Performance has regressed.
PathOsam::write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [477.60 µs 573.77 µs 630.74 µs]
                        change: [-16.399% -0.3936% +18.938%] (p = 0.97 > 0.05)
                        No change in performance detected.
PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [546.90 µs 632.69 µs 682.18 µs]
                        change: [-13.764% +0.5876% +18.027%] (p = 0.94 > 0.05)
                        No change in performance detected.

PathOsam::local_write/(Capacity: 16384 Blocksize: 64)
                        time:   [77.228 µs 83.698 µs 87.639 µs]
                        change: [-8.5604% -1.4188% +6.2864%] (p = 0.73 > 0.05)
                        No change in performance detected.
PathOsam::local_write/(Capacity: 65536 Blocksize: 64)
                        time:   [77.349 µs 83.745 µs 87.514 µs]
                        change: [-8.7644% -1.5763% +6.3340%] (p = 0.71 > 0.05)
                        No change in performance detected.
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64)
                        time:   [77.489 µs 83.853 µs 87.683 µs]
                        change: [-7.9760% -0.8434% +7.3079%] (p = 0.84 > 0.05)
                        No change in performance detected.

PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [78.742 µs 85.160 µs 89.244 µs]
                        change: [-6.3299% +0.9027% +8.7284%] (p = 0.82 > 0.05)
                        No change in performance detected.
PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [77.517 µs 84.355 µs 88.311 µs]
                        change: [-8.8375% -1.2706% +6.8486%] (p = 0.77 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 100.89 ms (2915 iteration
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [77.427 µs 83.807 µs 87.601 µs]
                        change: [-9.6425% -1.5315% +7.3133%] (p = 0.73 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 113.89 ms (20 i
PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64)
                        time:   [14.772 ms 17.318 ms 19.933 ms]
                        change: [-18.765% +0.6591% +25.224%] (p = 0.96 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 164.04 ms (20 i
PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64)
                        time:   [23.282 ms 26.714 ms 30.286 ms]
                        change: [+61.988% +105.70% +163.56%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 135.99 ms (20
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64)
                        time:   [13.512 ms 15.745 ms 17.994 ms]
                        change: [-18.214% +0.0082% +22.256%] (p = 1.00 > 0.05)
                        No change in performance detected.

Each read and write read a root-to-leaf path of data. Additionally, both also deterministically download an eviction path. The eviction path may coincide with the first path read, so only `height` blocks are downloaded. At worst, the eviction path, besides the root, is completely different than the first path. This means `2*height - 1` buckets are downloaded. Writes are always the same since we evict one deterministic path.

Physical reads and writes incurred by 1 PathOsam::read:
OSAM Capacity   | OSAM Blocksize  | Physical Reads  | Physical Writes
64              | 64              | 6-11            | 6              
256             | 64              | 8-15            | 8              
64              | 4096            | 6-11            | 6              
256             | 4096            | 8-15            | 8              

Physical reads and writes incurred by 1 PathOsam::write:
OSAM Capacity   | OSAM Blocksize  | Physical Reads  | Physical Writes
64              | 64              | 6-11            | 6              
256             | 64              | 8-15            | 8              
64              | 4096            | 6-11            | 6              
256             | 4096            | 8-15            | 8              

Physical reads and writes incurred by 64 random PathOsam operations:
OSAM Capacity   | OSAM Blocksize  | Physical Reads  | Physical Writes
64              | 64              | 384-704         | 384            
256             | 64              | 512-960         | 512            
64              | 4096            | 384-704         | 384            
256             | 4096            | 512-960         | 512                              
```
