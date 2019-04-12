run `cargo run --example demo`

after

To create bob's wallet

```
curl -H "Content-Type: application/json" -X POST -d @examples/create-wallet-1.json  \
    http://127.0.0.1:8000/api/explorer/v1/transactions
```

To create alice's wallet
```

curl -H "Content-Type: application/json" -X POST -d @examples/create-wallet-2.json  \
         http://127.0.0.1:8000/api/explorer/v1/transactions
```

To transfer money

```
curl -H "Content-Type: application/json" -X POST -d @examples/transfer-funds.json  \
         http://127.0.0.1:8000/api/explorer/v1/transactions
```
