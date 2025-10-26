#!/bin/bash
# deploy_devpoints.sh - Deploy completo de DevPoints CODEBIT

set -e  # Salir si hay error

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

echo -e "${PURPLE}🚀 Deploying DevPoints - CODEBIT Token${NC}\n"

# 1. Build
echo -e "${GREEN}📦 Building contract...${NC}"
stellar contract build

if [ $? -ne 0 ]; then
    echo -e "${YELLOW}⚠️  Build failed${NC}"
    exit 1
fi

# 2. Deploy
echo -e "${GREEN}🚢 Deploying to testnet...${NC}"
CONTRACT_ID=$(stellar contract deploy \
    --wasm target/wasm32v1-none/release/token_codebit.wasm \
    --source karen \
    --network testnet)

echo -e "Contract ID: ${BLUE}$CONTRACT_ID${NC}"

# Guardar CONTRACT_ID
mkdir -p .soroban
echo $CONTRACT_ID > .soroban/codebit_id
export TOKEN_CONTRACT_ID=$CONTRACT_ID

# 3. Initialize
echo -e "${GREEN}⚙️  Initializing DevPoints...${NC}"
stellar contract invoke \
    --id $CONTRACT_ID \
    --source karen \
    --network testnet \
    -- initialize \
    --admin $(stellar keys address karen) \
    --name "DevPoints CODEBIT" \
    --symbol "CODEBIT" \
    --decimals 0

# 4. Mint initial supply
echo -e "${GREEN}💰 Minting community supply (100,000 CODEBIT)...${NC}"
stellar contract invoke \
    --id $CONTRACT_ID \
    --source karen \
    --network testnet \
    -- mint \
    --to $(stellar keys address karen) \
    --amount 100000





# 5. Verify
echo -e "${GREEN}✅ Verifying...${NC}"
NAME=$(stellar contract invoke --id $CONTRACT_ID --source karen --network testnet -- name)
SYMBOL=$(stellar contract invoke --id $CONTRACT_ID --source karen --network testnet -- symbol)
DECIMALS=$(stellar contract invoke --id $CONTRACT_ID --source karen --network testnet -- decimals)
SUPPLY=$(stellar contract invoke --id $CONTRACT_ID --source karen --network testnet -- total_supply)

echo -e "\n${PURPLE}🎉 DevPoints Deployment Complete!${NC}"
echo -e "${GREEN}Token Details:${NC}"
echo "  Name: $NAME"
echo "  Symbol: $SYMBOL"
echo "  Decimals: $DECIMALS (1 CODEBIT = 1 minute)"
echo "  Total Supply: $SUPPLY CODEBIT (~$(($SUPPLY / 60)) hours)"
echo "  Contract ID: $CONTRACT_ID"

echo -e "\n${BLUE}🔗 View in explorer:${NC}"
echo "https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"

echo -e "\n${YELLOW}💡 Next steps:${NC}"
echo "1. Create developer accounts"
echo "2. Distribute CODEBIT for contributions"
echo "3. Test payment flows"
echo "4. Build frontend integration"
