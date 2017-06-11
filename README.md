# 4clojure exercises in Rust

An exercise in understanding the cost of everything _as well as_ its value[1]. The original 4clojure problems can be found [here](http://4clojure.com).

To run the tests:
```sh
cargo test
```

Some tests output debug info, which are normally suppressed by the runner. To see those:
```sh
cargo test -- --nocapture
```

1. "A Lisp programmer knows the value of everything, but the cost of nothing." (Alan Perlis, _Epigrams on Programming_, ACM SIGPLAN Notices 17 (**9**), September 1982, pp. 7–13.)