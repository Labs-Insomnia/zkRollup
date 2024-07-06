So there is a batcher that we will fund:

```shell
aligned deposit-to-batcher \
    --batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003 \
    --rpc https://ethereum-holesky-rpc.publicnode.com \
    --chain holesky \
    --keystore_path ~/.aligned_keystore/keystore0 \
    --amount 0.1ether
```

and then we submit proof:
```shell
aligned submit \
    --proving_system SP1 \
    --proof <proof_file> \
    --vm_program <vm_program_file> \
    --conn wss://batcher.alignedlayer.com \
    --keystore_path ~/.aligned_keystore/keystore0
```

