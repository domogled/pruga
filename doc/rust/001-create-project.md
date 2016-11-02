# Programování v jazyce Rust

## vytvoření projektu

[cargo](http://doc.crates.io/guide.html)

```shell
cargo new ${project_name} --bin
```

## spuštění

```shell
cargo run
```

## testy

## logování

Loger je hen [log](https://docs.rs/crate/log) ale nic nevypisuje, nastavování vypisování dělá modul [env_logger](https://doc.rust-lang.org/log/env_logger)