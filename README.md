STEP1: deploy target-contract

```bash
casper-client put-deploy --chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key "/home/jh/keys/jdk2/secret_key.pem" \
--session-path "/home/jh/mywork/callstack/target/wasm32-unknown-unknown/release/targetcontract.wasm" \
--payment-amount 50000000000
```

STEP2: deploy caller-contract

```bash
casper-client put-deploy --chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key "/home/jh/keys/jdk2/secret_key.pem" \
--session-path "/home/jh/mywork/callstack/target/wasm32-unknown-unknown/release/callercontract.wasm" \
--payment-amount 40000000000
```

STEP3: call target-contract

```bash
casper-client put-deploy --chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key "/home/jh/keys/jdk2/secret_key.pem" \
--session-hash "hash-8a86d5078527567014dd78f7d3a670b062ac2dc0fbcf142ddd6551d85c61b907" \
--session-entry-point "callcontract" \
--session-arg "targethash:string='contract-9e91c68f5e1b8c020a056f037dc669dc1d5a385ff7bf7594587fd2cefca8ff71'" \
--payment-amount 50000000000
```
