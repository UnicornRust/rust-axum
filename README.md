
# Axum

- [axum](https://docs.rs/axum/latest/axum/)

## tracing 实现日志的输出

- [tracing](https://docs.rs/tracing/0.1.37/tracing/)
- [tracing-subscriber](https://docs.rs/tracing-subscriber/0.3.16/tracing_subscriber/)

```shell
# 日志级别
RUST_LOG=debug cargo run
```

## config 构建配置

- [config](https://docs.rs/config/latest/config/)

## tokio 异步运行时

- [tokio](https://docs.rs/tokio/latest/tokio/)

## anyhow 错误处理

- [anyhow](https://docs.rs/anyhow/latest/anyhow/)

## sqlx 数据库操作

- [sqlx](https://docs.rs/sqlx/latest/sqlx/)

## serde 序列化和反序列化

- [serde](https://docs.rs/serde/latest/serde/)

## Sea-ORM

- [Sea-ORM](https://docs.rs/sea-orm/latest/sea_orm/)

- 安装seo-orm-cli 进行数据库实体的生成等固定模式的代码

```shell
  cargo install sea-orm-cli
```

## thiserror 自定义错误

- [thiserror](https://docs.rs/thiserror/latest/thiserror/)

## tower-http 中间件

- [tower-http](https://docs.rs/tower-http/latest/tower_http/)

可以处理常见的例如：请求的追踪日志，跨域，限流, 超时，路径处理等中间层

### trace

- trace 特性可以追踪请求的调试日志，结合 tracing-subscriber 可以实现日志的输出

### validator 验证中间件

- [validator](https://docs.rs/validator/latest/validator/)

进行请求参数的校验，提前校验一些异常的参数,防止后续的逻辑出错
