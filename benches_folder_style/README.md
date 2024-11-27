# Result

- `cargo bench && cargo t -- --nocapture`


```bash
$ cargo bench && cargo t -- --nocapture
    Finished `bench` profile [optimized] target(s) in 0.40s
     Running unittests src/lib.rs (target/release/deps/benches_folder_style-73e4440cc4813b45)

running 1 test
test tests::sigmoid_test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/sigmoid_fn.rs (target/release/deps/sigmoid_fn-08778db54c204d6b)
Gnuplot not found, using plotters backend
atan fn                 time:   [7.5495 ns 7.5710 ns 7.5943 ns]
                        change: [+0.2069% +0.8091% +1.7645%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

exp fn                  time:   [5.6985 ns 5.7150 ns 5.7350 ns]
                        change: [-0.0078% +0.4889% +0.9684%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe

sqrt fn                 time:   [2.4961 ns 2.4988 ns 2.5018 ns]
                        change: [+0.3538% +0.5528% +0.7418%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

erf fn                  time:   [3.1371 ns 3.1430 ns 3.1493 ns]
                        change: [-0.4280% +0.0917% +0.5390%] (p = 0.73 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

fabs fn                 time:   [1.4441 ns 1.4468 ns 1.4503 ns]
                        change: [-0.1596% +0.3985% +1.1734%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

fast sigmoid tan fn     time:   [3.9472 ns 3.9694 ns 3.9967 ns]
                        change: [-0.2587% +0.2579% +0.8997%] (p = 0.35 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

    Finished `test` profile [optimized] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/benches_folder_style-73e4440cc4813b45)

running 1 test
atan(pi*x/2)*2/pi    6.4 ns
atan(x)              5.2 ns
1/(1+exp(-x))        0.0 ns
1/sqrt(1+x^2)        0.0 ns
erf(sqrt(pi)*x/2)    4.1 ns
tanh(x)             13.7 ns
x/(1+|x|)            0.0 ns
FastSigmoid(1): 0.7869864
test tests::sigmoid_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.43s

   Doc-tests benches_folder_style

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


cargo_bench_training/benches_folder_style on  main [!] is �📦 v0.1.0 via🦀🦀 v1.82.0 took 1m31s
```


- `cargo nextest run --nocapture`

```bash
$ cargo nextest run --nocapture
    Finished `test` profile [optimized] target(s) in 0.14s
    Starting 1 test across 1 binary
       START             benches_folder_style tests::sigmoid_test

running 1 test
atan(pi*x/2)*2/pi    6.4 ns
atan(x)              5.3 ns
1/(1+exp(-x))        0.0 ns
1/sqrt(1+x^2)        0.0 ns
erf(sqrt(pi)*x/2)    4.2 ns
tanh(x)             13.9 ns
x/(1+|x|)            0.0 ns
FastSigmoid(1): 0.7869864
test tests::sigmoid_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.79s

        PASS [  30.170s] benches_folder_style tests::sigmoid_test
------------
     Summary [  30.172s] 1 test run: 1 passed, 0 skipped

```
