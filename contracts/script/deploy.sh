source ../.env
forge create src/NounsRaffle.s.sol:NounsRaffle.s.sol \
    --rpc-url $RPC_5 \
    --private-key $PRIVATE_KEY \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_5