# Result

- `cargo bench`

```bash
 cargo bench

    Finished `bench` profile [optimized] target(s) in 4.27s

running 1 test
test tests::sigmoid_test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

Gnuplot not found, using plotters backend

atan fn                 time:   [7.5049 ns 7.5425 ns 7.5911 ns]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

exp fn                  time:   [5.6396 ns 5.6505 ns 5.6632 ns]
Found 31 outliers among 100 measurements (31.00%)
  4 (4.00%) low severe
  10 (10.00%) low mild
  3 (3.00%) high mild
  14 (14.00%) high severe

sqrt fn                 time:   [2.4946 ns 2.4983 ns 2.5022 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

erf fn                  time:   [3.1148 ns 3.1196 ns 3.1253 ns]
Found 12 outliers among 100 measurements (12.00%)
  9 (9.00%) high mild
  3 (3.00%) high severe

fabs fn                 time:   [1.4440 ns 1.4480 ns 1.4522 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

fast sigmoid tan fn     time:   [3.9152 ns 3.9289 ns 3.9438 ns]
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
  1 (1.00%) high mild
  11 (11.00%) high severe

```

