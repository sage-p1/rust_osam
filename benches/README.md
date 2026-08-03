# Running benchmarks

Use `cargo bench` to run benchmarks. Note that all these tests do not use encryption.

# Example benchmark output

```
% cargo bench
Running benches/benchmark.rs (target/release/deps/benchmark-57e8b7d72679c6a5)
Gnuplot not found, using plotters backend
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 333.9ms.
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in esti
PathOsam::initialization/(Capacity: 16384 Blocksize: 4096)
                        time:   [26.681 ms 27.095 ms 27.553 ms]
                        change: [-3.6328% -1.7181% +0.3587%] (p = 0.12 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 1.7s.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in esti
PathOsam::initialization/(Capacity: 65536 Blocksize: 4096)
                        time:   [110.52 ms 115.80 ms 122.11 ms]
                        change: [-4.8413% +1.3157% +8.1602%] (p = 0.70 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 23.9s.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in es
PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096)
                        time:   [2.3285 s 2.3913 s 2.4675 s]
                        change: [-44.267% -38.041% -30.041%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 100
PathOsam::alloc/(Capacity: 16384 Blocksize: 4096)
                        time:   [33.410 ns 33.589 ns 34.084 ns]
                        change: [-14.868% -9.1928% -4.2572%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 100
PathOsam::alloc/(Capacity: 65536 Blocksize: 4096)
                        time:   [32.182 ns 32.497 ns 32.814 ns]
                        change: [-6.1678% -3.3555% -0.7713%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 1
PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096)
                        time:   [33.798 ns 33.929 ns 34.259 ns]
                        change: [-14.216% -8.8855% -4.6471%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 115.
PathOsam::read/(Capacity: 16384 Blocksize: 4096)
                        time:   [1.0357 ms 1.0389 ms 1.0461 ms]
                        change: [-53.076% -49.303% -44.764%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 137.
PathOsam::read/(Capacity: 65536 Blocksize: 4096)
                        time:   [1.2311 ms 1.2744 ms 1.3445 ms]
                        change: [-63.418% -57.527% -51.259%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 101.8ms or enable flat sampling.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 10
PathOsam::read/(Capacity: 1048576 Blocksize: 4096)
                        time:   [1.6810 ms 1.7518 ms 1.8313 ms]
                        change: [-61.426% -56.020% -49.583%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 1
PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [1.0260 ms 1.0291 ms 1.0355 ms]
                        change: [-48.855% -43.806% -38.035%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 1
PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [1.2147 ms 1.2387 ms 1.2752 ms]
                        change: [-55.338% -50.809% -45.834%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimated
PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [1.5801 ms 1.5849 ms 1.5935 ms]
                        change: [-53.627% -47.155% -38.958%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 109
PathOsam::write/(Capacity: 16384 Blocksize: 4096)
                        time:   [975.19 µs 981.91 µs 991.12 µs]
                        change: [-63.787% -51.271% -30.320%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 128
PathOsam::write/(Capacity: 65536 Blocksize: 4096)
                        time:   [1.1647 ms 1.1966 ms 1.2308 ms]
                        change: [-56.124% -44.688% -27.806%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 1
PathOsam::write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [1.5602 ms 1.6230 ms 1.6839 ms]
                        change: [-67.270% -61.989% -55.250%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 
PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [2.2022 ms 2.5670 ms 2.9675 ms]
                        change: [-28.788% -5.4327% +29.333%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 
PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [2.2853 ms 2.5483 ms 2.7030 ms]
                        change: [-30.592% -18.151% -0.8057%] (p = 0.06 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimate
PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [2.2186 ms 2.4878 ms 2.6303 ms]
                        change: [-59.525% -52.163% -43.437%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimat
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096)
                        time:   [325.65 µs 377.55 µs 408.37 µs]
                        change: [-32.603% -9.3029% +15.625%] (p = 0.65 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimat
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096)
                        time:   [328.21 µs 378.10 µs 407.48 µs]
                        change: [-19.056% -7.5904% +4.4138%] (p = 0.27 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estim
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [326.63 µs 375.48 µs 403.88 µs]
                        change: [-17.603% -6.3108% +7.3511%] (p = 0.37 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in esti
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [325.04 µs 375.37 µs 405.13 µs]
                        change: [-29.224% -10.707% +8.3580%] (p = 0.41 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in esti
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [325.59 µs 375.73 µs 405.47 µs]
                        change: [-47.042% -34.990% -18.691%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in es
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [325.81 µs 376.90 µs 406.78 µs]
                        change: [-31.324% -14.287% +5.2732%] (p = 0.23 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 481.4ms.
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Collecting 10 sam
PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64)
                        time:   [54.158 ms 60.658 ms 67.532 ms]
                        change: [-29.852% -16.504% -1.9958%] (p = 0.06 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 619.9ms.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Collecting 10 sam
PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64)
                        time:   [60.353 ms 62.609 ms 65.241 ms]
                        change: [-35.236% -32.217% -28.756%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 1.7s.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Collecting 10 s
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64)
                        time:   [85.157 ms 98.983 ms 121.47 ms]
                        change: [-95.739% -90.099% -15.804%] (p = 0.11 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estima
PathOsam::initialization/(Capacity: 16384 Blocksize: 64)
                        time:   [307.37 µs 311.74 µs 317.99 µs]
                        change: [-72.991% -66.972% -58.449%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 129.5ms or enable flat sampling.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estima
PathOsam::initialization/(Capacity: 65536 Blocksize: 64)
                        time:   [2.2276 ms 2.2681 ms 2.3158 ms]
                        change: [-24.834% -19.240% -13.718%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 319.4ms.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in esti
PathOsam::initialization/(Capacity: 1048576 Blocksize: 64)
                        time:   [33.810 ms 36.349 ms 39.062 ms]
                        change: [-18.589% -4.5437% +9.1612%] (p = 0.62 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 100.0
PathOsam::alloc/(Capacity: 16384 Blocksize: 64)
                        time:   [33.484 ns 34.505 ns 35.944 ns]
                        change: [-12.315% -8.7304% -5.1088%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 100.0
PathOsam::alloc/(Capacity: 65536 Blocksize: 64)
                        time:   [31.540 ns 31.698 ns 32.010 ns]
                        change: [-44.699% -24.519% -4.4789%] (p = 0.16 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 100
PathOsam::alloc/(Capacity: 1048576 Blocksize: 64)
                        time:   [34.603 ns 35.330 ns 35.756 ns]
                        change: [-77.003% -69.787% -57.264%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 103.66
PathOsam::read/(Capacity: 16384 Blocksize: 64)
                        time:   [66.677 µs 67.247 µs 68.347 µs]
                        change: [-8.9056% -5.7882% -2.6297%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 104.01
PathOsam::read/(Capacity: 65536 Blocksize: 64)
                        time:   [81.604 µs 88.203 µs 96.729 µs]
                        change: [-9.0939% -4.9014% +1.1060%] (p = 0.09 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 104.
PathOsam::read/(Capacity: 1048576 Blocksize: 64)
                        time:   [115.76 µs 116.34 µs 117.28 µs]
                        change: [-58.583% -42.937% -12.998%] (p = 0.02 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high mild

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: Collecting 10 samples in estimated 103
PathOsam::read/(Capacity: 16384 Blocksize: 64) #2
                        time:   [66.118 µs 66.286 µs 66.672 µs]
                        change: [-16.042% -14.099% -11.990%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: Collecting 10 samples in estimated 103
PathOsam::read/(Capacity: 65536 Blocksize: 64) #2
                        time:   [80.858 µs 81.072 µs 81.514 µs]
                        change: [-16.893% -14.872% -12.988%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 1
PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [113.90 µs 114.27 µs 115.05 µs]
                        change: [-16.251% -13.361% -10.193%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 107.02 ms 
PathOsam::write/(Capacity: 16384 Blocksize: 64)
                        time:   [220.77 µs 238.92 µs 253.41 µs]
                        change: [+18.557% +27.026% +36.756%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 100.87 ms 
PathOsam::write/(Capacity: 65536 Blocksize: 64)
                        time:   [168.39 µs 197.86 µs 216.97 µs]
                        change: [-21.171% -12.341% -2.2023%] (p = 0.03 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 107.13 m
PathOsam::write/(Capacity: 1048576 Blocksize: 64)
                        time:   [212.32 µs 225.02 µs 233.17 µs]
                        change: [-25.240% -19.687% -13.852%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: Collecting 10 samples in estimated 115.75 
PathOsam::write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [757.20 µs 816.98 µs 854.06 µs]
                        change: [+21.814% +43.347% +69.266%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: Collecting 10 samples in estimated 103.30 
PathOsam::write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [481.17 µs 576.08 µs 631.75 µs]
                        change: [-33.239% -20.312% -5.4308%] (p = 0.02 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 104.4
PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [535.86 µs 611.89 µs 653.67 µs]
                        change: [-22.686% -12.750% -1.3918%] (p = 0.06 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in es
PathOsam::local_write/(Capacity: 16384 Blocksize: 64)
                        time:   [94.876 µs 102.74 µs 112.59 µs]
                        change: [+16.853% +34.808% +65.075%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64): Warming 
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64): Collecting 1
PathOsam::local_write/(Capacity: 65536 Blocksize: 64)
                        time:   [77.149 µs 83.663 µs 87.506 µs]
                        change: [-13.918% -6.6219% +0.5961%] (p = 0.11 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Warming u
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Collecti
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Analyzin
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64)
                        time:   [81.026 µs 85.534 µs 88.489 µs]
                        change: [-12.386% +7.2346% +33.525%] (p = 0.53 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Warming
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Collect
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Analyzi
PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [78.110 µs 84.782 µs 88.659 µs]
                        change: [-8.4617% -2.1484% +4.6004%] (p = 0.55 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Warming
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Collect
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Analyzi
PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [77.775 µs 84.335 µs 88.183 µs]
                        change: [+19.068% +32.326% +47.176%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Warmi
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Colle
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Analy
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [77.605 µs 84.089 µs 87.950 µs]
                        change: [+27.872% +43.726% +62.440%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 
PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64)
                        time:   [14.963 ms 17.532 ms 20.169 ms]
                        change: [-60.617% -44.886% -14.646%] (p = 0.03 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 
PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64)
                        time:   [23.312 ms 26.549 ms 29.825 ms]
                        change: [-6.4059% +21.120% +63.971%] (p = 0.19 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64)
                        time:   [13.641 ms 15.963 ms 18.293 ms]
                        change: [-30.829% -12.771% +8.9778%] (p = 0.29 > 0.05)
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
