# grpc-server

## 概要

tonic を利用した rust 製の grpc サーバ実装のサンプル

自作した MyService と HealthCheckService を実装した grpc サーバを実装している

## 実行方法

### server 

```bash
cargo run --package grpc-server --bin server
```

### client

```bash
cargo run --package grpc-server --bin client
```