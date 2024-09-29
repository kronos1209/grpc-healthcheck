# grpc-healthcheck

## 概要

RUST での gRPC のヘルスチェックの実装サンプルおよび k8s での grpc ヘルスチェックのサンプル

## Prerequisites

- Rust 
- Docker 
- k8s クラスタ作成ツール（minikube, kind など）
    - ダッシュボードのデプロイが簡単な minikube がおすすめ
- kubectl

## ビルド & デプロイ

1. サンプルイメージの作成

```bash
docker build . -f ./grpc-server/dockerfile  -t test:v1
```

1. k8s クラスタの用意

```bash
# クラスタ作成
minikube start

#クラスタ内にイメージを取り込む
minikube image load test:v1

# ダッシュボードの起動
minikube dashboard
```

1. k8s リソース追加

```bash
kubectl apply -f ./mamifest/pod.yaml
```

## Readiness の挙動確認

k8s の pod を作成してからおよそ 30 秒後にサービスが準備完了となっていることを確認する

確認方法としては k8s のダッシュボードの Pod のページから myapp のステータスを確認する。
正しく動作していれば,イベントのログに以下のような readiness に3回失敗したログが出力された後ポッドは準備完了状態になるはず

> Readiness probe failed: service unhealthy (responded with "NOT_SERVING")