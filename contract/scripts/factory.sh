CONTRACT=dev-1685724938241-33288113782137

# near call $CONTRACT create_new_nft_contract '{"prefix":"test"}' --accountId test412ock.testnet --gas 50000000000000
echo $CONTRACT

near view $CONTRACT get_nfts_contract_accounts