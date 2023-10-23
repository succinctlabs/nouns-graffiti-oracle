source ../.env
forge verify-contract \
    0x1182095aa9A9496B4C657AFFB9931123e582F264 \
    NounsRaffle \
    --chain-id 5 \
    --verifier etherscan \
    --etherscan-api-key $ETHERSCAN_API_5 \
    --constructor-args-path ./script/constructor.txt