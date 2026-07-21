# Running benchmarks

Use `cargo bench` to run benchmarks.

# Example benchmark output

```
% cargo bench
Running benches/benchmark.rs (target/release/deps/benchmark-b2a89f5f4c1a5769)
Gnuplot not found, using plotters backend
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize:
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 352.4ms.
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 352.43 ms (10 iteratio
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize:
PathOsam::initialization/(Capacity: 16384 Blocksize: 4096)
                        time:   [32.570 ms 36.540 ms 41.969 ms]
                        change: [+16.153% +31.329% +50.239%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize:
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 2.0s.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 1.9553 s (10 iteration
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize:
PathOsam::initialization/(Capacity: 65536 Blocksize: 4096)
                        time:   [151.71 ms 173.72 ms 197.95 ms]
                        change: [+33.213% +56.596% +79.060%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksiz
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 44.7s.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 44.749 s (10 iterati
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksiz
PathOsam::initialization/(Capacity: 1048576 Blocksize: 4096)
                        time:   [3.7047 s 3.8331 s 3.9607 s]
                        change: [+47.569% +54.651% +61.875%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): W
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): C
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 4096): A
PathOsam::alloc/(Capacity: 16384 Blocksize: 4096)
                        time:   [39.268 ns 53.937 ns 68.157 ns]
                        change: [-23.938% +5.3794% +45.760%] (p = 0.79 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): W
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): C
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 4096): A
PathOsam::alloc/(Capacity: 65536 Blocksize: 4096)
                        time:   [32.171 ns 32.355 ns 32.690 ns]
                        change: [+3.5037% +4.3461% +5.4634%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096):
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096):
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096):
PathOsam::alloc/(Capacity: 1048576 Blocksize: 4096)
                        time:   [36.502 ns 37.075 ns 38.417 ns]
                        change: [+6.8925% +11.424% +16.908%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 132.4ms or enable flat sampling.
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): Co
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096): An
PathOsam::read/(Capacity: 16384 Blocksize: 4096)
                        time:   [1.6912 ms 1.7641 ms 1.8515 ms]
                        change: [+103.80% +129.29% +172.53%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low mild
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 104.2ms or enable flat sampling.
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): Co
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096): An
PathOsam::read/(Capacity: 65536 Blocksize: 4096)
                        time:   [1.5493 ms 1.6556 ms 1.7177 ms]
                        change: [+61.533% +77.743% +96.150%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 113.5ms or enable flat sampling.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): 
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096): 
PathOsam::read/(Capacity: 1048576 Blocksize: 4096)
                        time:   [1.7885 ms 1.9384 ms 2.0941 ms]
                        change: [+39.529% +50.466% +60.626%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2:
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2:
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2:
PathOsam::read/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [1.1260 ms 1.1644 ms 1.2082 ms]
                        change: [+42.911% +49.186% +56.815%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2:
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2:
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2:
PathOsam::read/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [1.3317 ms 1.4099 ms 1.5079 ms]
                        change: [+52.435% +60.279% +70.895%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2: Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 9.1s.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #
PathOsam::read/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [368.81 ms 397.52 ms 424.73 ms]
                        change: [+29239% +31624% +33670%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): W
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): C
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096): A
PathOsam::write/(Capacity: 16384 Blocksize: 4096)
                        time:   [1.6039 ms 2.1318 ms 2.8598 ms]
                        change: [-0.7190% +34.711% +80.376%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 183.0ms or enable flat sampling.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): C
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096): A
PathOsam::write/(Capacity: 65536 Blocksize: 4096)
                        time:   [1.4509 ms 1.7220 ms 2.4236 ms]
                        change: [-12.222% +36.339% +91.152%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 195.9ms or enable flat sampling.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096):
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096):
PathOsam::write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [2.3462 ms 3.4381 ms 4.6396 ms]
                        change: [+74.353% +196.09% +363.77%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2: Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 121.1ms or enable flat sampling.
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
PathOsam::write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [3.1749 ms 4.0392 ms 4.7839 ms]
                        change: [+42.268% +77.548% +118.28%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2: Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 136.2ms or enable flat sampling.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
PathOsam::write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [1.8824 ms 1.9824 ms 2.1249 ms]
                        change: [-19.806% -4.9697% +18.673%] (p = 0.66 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) 
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2: Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 134.2ms or enable flat sampling.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) 
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 4096) 
PathOsam::write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [2.0404 ms 2.3000 ms 2.4686 ms]
                        change: [-15.790% -3.6991% +10.258%] (p = 0.61 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096): Collecting 10 samples in estimated 103.30 ms (660 iterations
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 40
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096)
                        time:   [358.06 µs 711.41 µs 955.37 µs]
                        change: [-2.6865% +49.269% +124.96%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096): Collecting 10 samples in estimated 113.84 ms (440 iterations
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 40
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096)
                        time:   [684.67 µs 755.76 µs 818.71 µs]
                        change: [+72.802% +117.18% +168.77%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096): Collecting 10 samples in estimated 105.06 ms (440 iteratio
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096)
                        time:   [344.63 µs 391.80 µs 438.31 µs]
                        change: [-12.066% +8.9268% +29.943%] (p = 0.43 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2: Collecting 10 samples in estimated 103.02 ms (550 iterati
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 40
PathOsam::local_write/(Capacity: 16384 Blocksize: 4096) #2
                        time:   [351.21 µs 386.03 µs 409.29 µs]
                        change: [-4.7505% +8.8341% +24.186%] (p = 0.24 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 40
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2: Collecting 10 samples in estimated 107.10 ms (605 iterati
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 40
PathOsam::local_write/(Capacity: 65536 Blocksize: 4096) #2
                        time:   [346.03 µs 391.18 µs 417.51 µs]
                        change: [-8.6138% +5.4702% +22.216%] (p = 0.50 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2: Collecting 10 samples in estimated 107.19 ms (440 itera
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
PathOsam::local_write/(Capacity: 1048576 Blocksize: 4096) #2
                        time:   [552.16 µs 732.06 µs 941.52 µs]
                        change: [+43.091% +105.37% +185.35%] (p = 0.01 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksi
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 530.6ms.
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 530.64 ms 
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksi
PathOsam::random_operations/(Capacity: 16384 Blocksize: 4096, Ops: 64)
                        time:   [83.993 ms 103.70 ms 125.85 ms]
                        change: [+40.301% +82.906% +130.47%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksi
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 1.1s.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 1.0658 s(
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksi
PathOsam::random_operations/(Capacity: 65536 Blocksize: 4096, Ops: 64)
                        time:   [60.648 ms 62.671 ms 65.310 ms]
                        change: [+25.545% +29.841% +35.920%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Block
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 2.2s.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64): Collecting 10 samples in estimated 2.1659s
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Block
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 4096, Ops: 64)
                        time:   [87.686 ms 96.532 ms 110.28 ms]
                        change: [-13.922% +0.2797% +20.403%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize:
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize:
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize: 64): Collecting 10 samples in estimated 104.92 ms (330 iteration
Benchmarking PathOsam::initialization/(Capacity: 16384 Blocksize:
PathOsam::initialization/(Capacity: 16384 Blocksize: 64)
                        time:   [298.06 µs 303.35 µs 313.77 µs]
                        change: [-12.579% -7.5923% -1.4939%] (p = 0.02 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize:
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 115.8ms or enable flat sampling.
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize: 64): Collecting 10 samples in estimated 115.76 ms (55 iterations
Benchmarking PathOsam::initialization/(Capacity: 65536 Blocksize:
PathOsam::initialization/(Capacity: 65536 Blocksize: 64)
                        time:   [1.9853 ms 2.0533 ms 2.1704 ms]
                        change: [-5.7211% -0.7279% +4.6029%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksiz
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Warming up for 100.00 ms
Warning: Unable to complete 10 samples in 100.0ms. You may wish to increase target time to 348.9ms.
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 348.92 ms (10 iteratio
Benchmarking PathOsam::initialization/(Capacity: 1048576 Blocksiz
PathOsam::initialization/(Capacity: 1048576 Blocksize: 64)
                        time:   [31.593 ms 31.828 ms 32.090 ms]
                        change: [-2.9296% -1.3206% +0.2303%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): War
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): Col
Benchmarking PathOsam::alloc/(Capacity: 16384 Blocksize: 64): Ana
PathOsam::alloc/(Capacity: 16384 Blocksize: 64)
                        time:   [33.965 ns 34.131 ns 34.495 ns]
                        change: [-0.2105% +1.3025% +2.7800%] (p = 0.12 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): War
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): Col
Benchmarking PathOsam::alloc/(Capacity: 65536 Blocksize: 64): Ana
PathOsam::alloc/(Capacity: 65536 Blocksize: 64)
                        time:   [32.040 ns 32.320 ns 32.806 ns]
                        change: [+3.1839% +4.3398% +5.4697%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): W
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): C
Benchmarking PathOsam::alloc/(Capacity: 1048576 Blocksize: 64): A
PathOsam::alloc/(Capacity: 1048576 Blocksize: 64)
                        time:   [34.180 ns 34.433 ns 34.944 ns]
                        change: [+0.2480% +1.5251% +2.9656%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Warm
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Coll
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64): Anal
PathOsam::read/(Capacity: 16384 Blocksize: 64)
                        time:   [65.782 µs 65.988 µs 66.366 µs]
                        change: [+31.590% +32.567% +34.022%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Warm
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Coll
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64): Anal
PathOsam::read/(Capacity: 65536 Blocksize: 64)
                        time:   [79.812 µs 80.179 µs 80.999 µs]
                        change: [+34.587% +36.442% +38.385%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): Wa
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): Co
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64): An
PathOsam::read/(Capacity: 1048576 Blocksize: 64)
                        time:   [113.54 µs 114.43 µs 115.52 µs]
                        change: [+23.691% +25.164% +27.164%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: W
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: C
Benchmarking PathOsam::read/(Capacity: 16384 Blocksize: 64) #2: A
PathOsam::read/(Capacity: 16384 Blocksize: 64) #2
                        time:   [65.888 µs 66.245 µs 66.931 µs]
                        change: [+33.036% +34.794% +37.499%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: W
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: C
Benchmarking PathOsam::read/(Capacity: 65536 Blocksize: 64) #2: A
PathOsam::read/(Capacity: 65536 Blocksize: 64) #2
                        time:   [79.378 µs 80.154 µs 81.471 µs]
                        change: [+36.764% +37.897% +39.250%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2:
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2:
Benchmarking PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2:
PathOsam::read/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [114.67 µs 116.42 µs 118.49 µs]
                        change: [+28.124% +29.699% +31.716%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): War
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): Col
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64): Ana
PathOsam::write/(Capacity: 16384 Blocksize: 64)
                        time:   [207.84 µs 231.40 µs 252.61 µs]
                        change: [-69.284% -66.998% -64.372%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): War
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): Col
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64): Ana
PathOsam::write/(Capacity: 65536 Blocksize: 64)
                        time:   [162.27 µs 193.64 µs 215.28 µs]
                        change: [-78.133% -75.826% -73.032%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): W
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): C
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64): A
PathOsam::write/(Capacity: 1048576 Blocksize: 64)
                        time:   [218.15 µs 234.27 µs 247.23 µs]
                        change: [-58.062% -52.743% -46.887%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: 
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: 
Benchmarking PathOsam::write/(Capacity: 16384 Blocksize: 64) #2: 
PathOsam::write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [458.26 µs 542.21 µs 588.52 µs]
                        change: [-47.326% -38.861% -29.025%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: 
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: 
Benchmarking PathOsam::write/(Capacity: 65536 Blocksize: 64) #2: 
PathOsam::write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [475.37 µs 571.89 µs 627.92 µs]
                        change: [-15.439% +1.4043% +22.052%] (p = 0.89 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
Benchmarking PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
PathOsam::write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [542.22 µs 638.12 µs 693.14 µs]
                        change: [-17.133% -1.4884% +16.052%] (p = 0.87 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
PathOsam::local_write/(Capacity: 16384 Blocksize: 64)
                        time:   [78.185 µs 84.333 µs 88.040 µs]
                        change: [-5.4185% +2.1096% +10.672%] (p = 0.62 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
PathOsam::local_write/(Capacity: 65536 Blocksize: 64)
                        time:   [78.428 µs 84.691 µs 88.130 µs]
                        change: [-6.2568% +1.9905% +10.187%] (p = 0.64 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64): Collecting 10 samples in estimated 101.44 ms (2915 iteration
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64)
                        time:   [77.626 µs 84.146 µs 87.925 µs]
                        change: [-4.6843% +2.5216% +10.950%] (p = 0.56 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2: Collecting 10 samples in estimated 101.27 ms (2860 iteratio
Benchmarking PathOsam::local_write/(Capacity: 16384 Blocksize: 64
PathOsam::local_write/(Capacity: 16384 Blocksize: 64) #2
                        time:   [78.111 µs 84.474 µs 88.355 µs]
                        change: [-5.3050% +2.0826% +9.8569%] (p = 0.63 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2: Collecting 10 samples in estimated 101.70 ms (2915 iteratio
Benchmarking PathOsam::local_write/(Capacity: 65536 Blocksize: 64
PathOsam::local_write/(Capacity: 65536 Blocksize: 64) #2
                        time:   [78.336 µs 84.987 µs 88.868 µs]
                        change: [-8.8116% -1.4672% +6.5900%] (p = 0.73 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2: Collecting 10 samples in estimated 100.19 ms (2860 iterat
Benchmarking PathOsam::local_write/(Capacity: 1048576 Blocksize: 
PathOsam::local_write/(Capacity: 1048576 Blocksize: 64) #2
                        time:   [77.807 µs 85.997 µs 90.751 µs]
                        change: [-5.4522% +2.6309% +11.892%] (p = 0.58 > 0.05)
                        No change in performance detected.

Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksi
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksi
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 116.00 ms (2
Benchmarking PathOsam::random_operations/(Capacity: 16384 Blocksi
PathOsam::random_operations/(Capacity: 16384 Blocksize: 64, Ops: 64)
                        time:   [14.644 ms 17.205 ms 19.814 ms]
                        change: [-31.451% -14.097% +6.2964%] (p = 0.20 > 0.05)
                        No change in performance detected.
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksi
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksi
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 145.09 ms (2
Benchmarking PathOsam::random_operations/(Capacity: 65536 Blocksi
PathOsam::random_operations/(Capacity: 65536 Blocksize: 64, Ops: 64)
                        time:   [10.378 ms 12.987 ms 15.614 ms]
                        change: [-49.203% -33.131% -13.023%] (p = 0.01 < 0.05)
                        Performance has improved.
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Block
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Block
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64): Collecting 10 samples in estimated 143.20 ms 
Benchmarking PathOsam::random_operations/(Capacity: 1048576 Block
PathOsam::random_operations/(Capacity: 1048576 Blocksize: 64, Ops: 64)
                        time:   [13.461 ms 15.744 ms 18.005 ms]
                        change: [-35.281% -19.894% -0.9824%] (p = 0.07 > 0.05)
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
