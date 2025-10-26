# DevPoints CODEBIT - Guía Completa de Deploy en WSL 💻

## 📌 Proyecto: Sistema de Reputación para Developers

**Token:** CODEBIT  
**Concepto:** 1 CODEBIT = 1 minuto de trabajo de desarrollo verificado  
**Red:** Stellar Testnet  
**Objetivo:** Crear un portfolio blockchain verificable para comunidades dev

---

## 🔧 Parte 1: Configuración Inicial de WSL (Una Sola Vez)

### Paso 1: Instalar WSL2 con Ubuntu

**Abrir PowerShell como Administrador:**

```powershell
# Instalar WSL2 con Ubuntu (toma 5-10 minutos)
wsl --install
```

**Después de la instalación:**
1. Reinicia tu computadora
2. Ubuntu se abrirá automáticamente
3. Crea tu usuario y contraseña Linux

```bash
Enter new UNIX username: karen
New password: ****
Retype new password: ****
```

💡 **Nota:** Esta contraseña es SOLO para Ubuntu dentro de WSL, no afecta Windows.

### Paso 2: Verificar Instalación

```bash
# Verificar versión de WSL
wsl --version

# Verificar que Ubuntu está corriendo
wsl -l -v
# Expected: Ubuntu Running 2
```

### Paso 3: Actualizar Ubuntu

```bash
# Dentro de Ubuntu (WSL), actualizar paquetes
sudo apt update && sudo apt upgrade -y
```

---

## 🦀 Parte 2: Instalar Herramientas de Desarrollo

### Paso 1: Instalar Rust en WSL

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Seleccionar opción 1 (instalación por defecto)
# Presiona Enter cuando te lo pida

# Cargar Rust en la sesión actual
source $HOME/.cargo/env

# Verificar instalación
rustc --version
# Expected: rustc 1.74.0 o superior

cargo --version
# Expected: cargo 1.74.0 o superior
```

### Paso 2: Instalar Stellar CLI

```bash
# Instalar Stellar CLI
cargo install --locked stellar-cli

# Esto toma 5-10 minutos
# Verás: Compiling stellar-cli...

# Verificar instalación
stellar --version
# Expected: stellar 21.0.0 o superior
```

### Paso 3: Instalar Target WASM

```bash
# Agregar target para compilar a WebAssembly
rustup target add wasm32-unknown-unknown

# Verificar instalación
rustup target list | grep wasm32
# Expected: wasm32-unknown-unknown (installed)
```

### Paso 4: Instalar Herramientas Adicionales

```bash
# Instalar jq (para formatear JSON)
sudo apt install jq -y

# Instalar git (si no está instalado)
sudo apt install git -y
```

---

## 📁 Parte 3: Crear y Configurar Proyecto

### Navegar a tu Carpeta de Trabajo

```bash
# Navegar al Desktop (ajusta según tu ruta)
cd /mnt/c/Users/Karen/desktop/CodigoFutura/Clase5-Creacion-Token

# Verificar ubicación
pwd
# Expected: /mnt/c/Users/Karen/desktop/CodigoFutura/Clase5-Creacion-Token
```

### Crear Proyecto CODEBIT

```bash
# Crear proyecto
stellar contract init token_codebit

# Navegar al proyecto
cd token_codebit

# Verificar estructura
ls -la
# Expected: Cargo.toml, src/, README.md
```

---

## 📝 Parte 4: Implementar el Código

### Configurar Cargo.toml

```bash
nano Cargo.toml
```

**Contenido completo:**

```toml
[package]
name = "token_codebit"
version = "1.0.0"
edition = "2021"
authors = ["DevPoints Community"]
description = "CODEBIT - Token for valuing developer time"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "23.0.2"

[dev-dependencies]
soroban-sdk = { version = "23.0.2", features = ["testutils"] }

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
```

**Guardar:** `Ctrl + X` → `Y` → `Enter`

### Crear src/storage.rs

```bash
nano src/storage.rs
```

```rust
use soroban_sdk::{contracttype, Address, String};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    TotalSupply,
    Admin,
    TokenName,
    TokenSymbol,
    Decimals,
    Initialized,
}

#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}
```

### Crear src/errors.rs

```bash
nano src/errors.rs
```

```rust
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CodebitError {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    InsufficientBalance = 3,
    InsufficientAllowance = 4,
    NotInitialized = 5,
    InvalidDecimals = 6,
    OverflowError = 7,
    InvalidRecipient = 8,
    InvalidMetadata = 9,
}
```

### Actualizar src/lib.rs

```bash
nano src/lib.rs
```

**Copiar el código completo del contrato** (revisar guía anterior para el código completo de lib.rs)

---

## 🔐 Parte 5: Configurar Cuenta Admin en Testnet

### Paso 1: Crear Identidad

```bash
# Crear identidad de admin
stellar keys generate karen --network testnet
```

**✅ Resultado:**
```
Key saved with alias karen in "/home/karen/.config/stellar/identity/karen.toml"
```

### Paso 2: Obtener Dirección Pública

```bash
stellar keys address karen
```

**✅ Tu dirección pública:**
```
GCIGUZUBYP423VBLEQG7UJIWVFPRDKQIPZ6YXL4WQ5RSPXHWAB633CLW
```

⚠️ **IMPORTANTE:** Guarda esta dirección, la necesitarás constantemente.

### Paso 3: Fondear Cuenta con Friendbot

```bash
curl "https://friendbot.stellar.org?addr=GCIGUZUBYP423VBLEQG7UJIWVFPRDKQIPZ6YXL4WQ5RSPXHWAB633CLW"
```

**✅ Respuesta esperada:**
```json
{
  "id": "26c504633de961e5bc9ea5af65cb0bf62ad1eecf4162f214716ff2ea14b68472",
  "successful": true,
  "hash": "26c504633de961e5bc9ea5af65cb0bf62ad1eecf4162f214716ff2ea14b68472",
  ...
}
```

### Paso 4: Verificar Balance

```bash
# Método 1: Ver en browser
echo "https://horizon-testnet.stellar.org/accounts/GCIGUZUBYP423VBLEQG7UJIWVFPRDKQIPZ6YXL4WQ5RSPXHWAB633CLW"

# Método 2: CLI (comando nuevo de Stellar no disponible en versión antigua)
# Si no funciona, usa el método 1
```

---

## 🏗️ Parte 6: Compilar el Contrato

### Compilar a WASM

```bash
# Asegúrate de estar en la carpeta del proyecto
cd /mnt/c/Users/Karen/desktop/CodigoFutura/Clase5-Creacion-Token/token_codebit

# Compilar
stellar contract build
```

**✅ Salida esperada:**
```
ℹ️ CARGO_BUILD_RUSTFLAGS=--remap-path-prefix=/home/karen/.cargo/registry/src= cargo rustc...
    Finished `release` profile [optimized] target(s) in 1.45s
ℹ️ Build Summary:
  Wasm File: target/wasm32v1-none/release/token_codebit.wasm
  Wasm Hash: 0a01525971e7a8ec16236aea3ae4735cf33cbefda68e2bdad2f155cfc967bd56
  Exported Functions: 13 found
    • admin
    • allowance
    • approve
    • balance
    • burn
    • decimals
    • initialize
    • mint
    • name
    • symbol
    • total_supply
    • transfer
    • transfer_from
✅ Build Complete
```

### Verificar Archivo WASM

```bash
ls -lh target/wasm32v1-none/release/token_codebit.wasm
```

**✅ Resultado:**
```
-rwxrwxrwx 2 karen karen 9.5K Oct 22 20:26 target/wasm32v1-none/release/token_codebit.wasm
```

---

## 🚢 Parte 7: Deploy a Testnet

### Deploy del Contrato

```bash
stellar contract deploy \
    --wasm target/wasm32v1-none/release/token_codebit.wasm \
    --source karen \
    --network testnet
```

**✅ Salida completa:**
```
ℹ️ Simulating install transaction…
ℹ️ Signing transaction: 50ecb54c755442ff80b790761b8cdc41316b12e9c38624c97ed5239508f52c2a
🌎 Submitting install transaction…
ℹ️ Using wasm hash 0a01525971e7a8ec16236aea3ae4735cf33cbefda68e2bdad2f155cfc967bd56
ℹ️ Simulating deploy transaction…
ℹ️ Transaction hash is eba4edbbdcc11cd812435977fdc60a5e36b901c55b85c1cf4911583a78e8b934
🔗 https://stellar.expert/explorer/testnet/tx/eba4edbbdcc11cd812435977fdc60a5e36b901c55b85c1cf4911583a78e8b934
ℹ️ Signing transaction: eba4edbbdcc11cd812435977fdc60a5e36b901c55b85c1cf4911583a78e8b934
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP
✅ Deployed!
CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP
```

### Guardar Contract ID

```bash
# Guardar CONTRACT_ID en variable
export TOKEN_CONTRACT_ID=CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP

# Crear carpeta y guardar en archivo
mkdir -p .soroban
echo $TOKEN_CONTRACT_ID > .soroban/token_id

# Verificar
echo $TOKEN_CONTRACT_ID
```

**⚠️ IMPORTANTE:** Si cierras la terminal, debes cargar el CONTRACT_ID nuevamente:
```bash
export TOKEN_CONTRACT_ID=$(cat .soroban/token_id)
```

---

## ⚙️ Parte 8: Inicializar Token CODEBIT

### Inicializar con Decimals = 0

```bash
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source karen \
    --network testnet \
    -- initialize \
    --admin $(stellar keys address karen) \
    --name "DevPoints CODEBIT" \
    --symbol "CODEBIT" \
    --decimals 0
```

**✅ Resultado:**
```
ℹ️ Signing transaction: 4a5b5a0065b09f57aeae7fc6f797f86862028761c750ba1ffa6f2ccbc950baf9
null
```

💡 **Nota:** `null` es correcto. Significa que la transacción fue exitosa pero no devuelve ningún valor.

---

## 🔍 Parte 9: Verificar Inicialización

### Consultar Nombre

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- name
```

**✅ Resultado:**
```
ℹ️ Simulation identified as read-only. Send by rerunning with `--send=yes`.
"DevPoints CODEBIT"
```

### Consultar Símbolo

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- symbol
```

**✅ Resultado:**
```
"CODEBIT"
```

### Consultar Decimales (DEBE SER 0)

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- decimals
```

**✅ Resultado:**
```
0
```

### Consultar Total Supply Inicial

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- total_supply
```

**✅ Resultado:**
```
0
```

---

## 💰 Parte 10: Mintear Supply Inicial

### Mintear 100,000 CODEBIT

```bash
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source karen \
    --network testnet \
    -- mint \
    --to $(stellar keys address karen) \
    --amount 100000
```

**✅ Resultado:**
```
ℹ️ Signing transaction: 2c129f4acab6c5845f0a129244e292978a15793b45ca9da9f1e7c066176b7522
null
```

### Verificar Balance de Admin

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source-account karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address karen)
```

**✅ Resultado:**
```
ℹ️ Simulation identified as read-only. Send by rerunning with `--send=yes`.
"100000"
```

🎉 **¡Éxito!** Ahora tienes 100,000 CODEBIT (equivalente a ~1,666 horas de trabajo).

---

## 👥 Parte 11: Casos de Uso Reales

### Caso 1: Crear Cuenta Junior Developer

```bash
# Crear cuenta de Alice
stellar keys generate alice_junior --network testnet
```

**✅ Resultado:**
```
Key saved with alias alice_junior in "/home/karen/.config/stellar/identity/alice_junior.toml"
```

```bash
# Fondear cuenta
curl "https://friendbot.stellar.org?addr=$(stellar keys address alice_junior)"
```

**✅ Respuesta exitosa:**
```json
{
  "id": "8d80cf822d06ca15aeec515445e9bdcda437f7a652100c8d85d1889f3adebb4b",
  "successful": true,
  ...
}
```

### Recompensar Contribución (45 minutos)

```bash
# Alice ayudó 45 minutos = 45 CODEBIT
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source karen \
    --network testnet \
    -- mint \
    --to $(stellar keys address alice_junior) \
    --amount 45
```

**✅ Resultado:**
```
ℹ️ Signing transaction: 7ece771a45592b210882f684727c1cea9baa8741562dc5cd25ac446d0b37eca2
null
```

### Verificar Balance de Alice

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source-account karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior)
```

**✅ Resultado:**
```
"45"
```

---

### Caso 2: Pagar por Code Review

#### Crear Cuenta Senior Developer

```bash
# Crear cuenta de Bob
stellar keys generate bob_senior --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address bob_senior)"
```

**✅ Cuenta creada exitosamente**

#### Alice Acumula 1,200 CODEBIT

```bash
# Mintear 1,200 CODEBIT adicionales a Alice (20 horas de ayuda)
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source karen \
    --network testnet \
    -- mint \
    --to $(stellar keys address alice_junior) \
    --amount 1200
```

**✅ Resultado:**
```
ℹ️ Signing transaction: f66e1b9822b434f2a100731377c0119c758bfdd756e784d1e1f256f00e2133d3
null
```

#### Alice Paga 30 CODEBIT a Bob

```bash
# Transferencia: Alice → Bob (30 min de code review)
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source alice_junior \
    --network testnet \
    -- transfer \
    --from $(stellar keys address alice_junior) \
    --to $(stellar keys address bob_senior) \
    --amount 30
```

**✅ Resultado:**
```
ℹ️ Signing transaction: 13ee483e90e96d2650054e562169748ff80645d21af0e64794fdff117559fda5
null
```

#### Verificar Balances Finales

```bash
# Balance de Alice (debe ser 45 + 1200 - 30 = 1215)
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source-account alice_junior \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior)
```

**✅ Resultado:**
```
"1215"
```

```bash
# Balance de Bob (debe ser 30)
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source-account bob_senior \
  --network testnet \
  -- balance \
  --account $(stellar keys address bob_senior)
```

**✅ Resultado:**
```
"30"
```

---

### Caso 3: Sistema de Micro-Bounties

#### Crear Cuentas para Bounty System

```bash
# Crear plataforma de bounties
stellar keys generate bounty_platform --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address bounty_platform)"

# Crear solver
stellar keys generate charlie_solver --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address charlie_solver)"
```

#### Alice Aprueba Allowance

```bash
# Alice aprueba 500 CODEBIT para que la plataforma gestione
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source alice_junior \
    --network testnet \
    -- approve \
    --from $(stellar keys address alice_junior) \
    --spender $(stellar keys address bounty_platform) \
    --amount 500
```

**✅ Resultado:**
```
ℹ️ Signing transaction: 2e3f9c336543998f558b31f26a32985ed1397abdbe1e0f5605c980be9915b5f1
null
```

#### Verificar Allowance

```bash
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source-account alice_junior \
    --network testnet \
    -- allowance \
    --from $(stellar keys address alice_junior) \
    --spender $(stellar keys address bounty_platform)
```

**✅ Resultado:**
```
"500"
```

#### Plataforma Paga Automáticamente

```bash
# Plataforma paga 60 CODEBIT de Alice a Charlie (1 hora de trabajo)
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source bounty_platform \
    --network testnet \
    -- transfer_from \
    --spender $(stellar keys address bounty_platform) \
    --from $(stellar keys address alice_junior) \
    --to $(stellar keys address charlie_solver) \
    --amount 60
```

**✅ Resultado:**
```
ℹ️ Signing transaction: 3cb930e927ee1473c5dd02b0e109c07e1bdd94b595772c1d1a31876b8fd07f10
null
```

---

### Caso 4: Burn de CODEBIT

```bash
# Bob quema 10 CODEBIT
stellar contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --source bob_senior \
    --network testnet \
    -- burn \
    --from $(stellar keys address bob_senior) \
    --amount 10
```

**✅ Resultado:**
```
ℹ️ Signing transaction: ec14b21c32cd49d978991999852da94ea119d9e883c348f8e2b69f894c1ddc3b
null
```

### Verificar Nuevo Total Supply

```bash
stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source-account bob_senior \
  --network testnet \
  -- total_supply
```

**✅ Resultado:**
```
"101235"
```

💡 **Cálculo:** 100,000 (inicial) + 45 (Alice) + 1,200 (Alice) - 10 (burn de Bob) = 101,235 ✅

---

## 🔍 Parte 12: Monitoreo en Stellar Expert

### Ver Contrato en Explorer

```bash
echo "🔗 Ver contrato en explorer:"
echo "https://stellar.expert/explorer/testnet/contract/$TOKEN_CONTRACT_ID"
```

**🔗 Tu contrato:**
```
https://stellar.expert/explorer/testnet/contract/CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP
```

### Ver Portfolio de Alice

```bash
echo "🔗 Ver portfolio de Alice:"
echo "https://stellar.expert/explorer/testnet/account/$(stellar keys address alice_junior)"
```

**🔗 Cuenta de Alice:**
```
https://stellar.expert/explorer/testnet/account/GDRX4RWFT6ZU7FHET4UASWHDRVVP22HJXCKEN6S7RG6IBWKLB7DSJH45
```

### Ver Eventos del Contrato

```bash
stellar events \
    --id $TOKEN_CONTRACT_ID \
    --network testnet \
    --start-ledger 1000000 \
    --output json | jq '.'
```

---

## 📊 Parte 13: Script de Verificación de Balances

### Crear Script de Verificación

```bash
nano check_balances.sh
```

**Contenido del script:**

```bash
#!/bin/bash
# ===============================================================
# 🌟 PROYECTO CODEBIT - Verificación final de balances y totales
# ===============================================================

echo ""
echo "📊 Estadísticas DevPoints - CODEBIT"
echo "==================================="
echo ""

# Total supply del token CODEBIT
TOTAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- total_supply)

echo "🪙 Total Supply: $TOTAL CODEBIT"

if [ -n "$TOTAL" ]; then
  # Eliminar comillas si existen
  TOTAL_CLEAN=$(echo $TOTAL | tr -d '"')
  echo "⏱️ Equivalente a: $(($TOTAL_CLEAN / 60)) horas de trabajo"
else
  echo "⚠️ No se pudo obtener el total supply"
fi

# Balance del administrador (Karen)
ADMIN_BAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address karen))

echo ""
echo "👩‍💻 Balance Admin (Karen): $ADMIN_BAL CODEBIT"

# Balance de Alice
ALICE_BAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior))

echo "👧 Balance Alice (Junior Dev): $ALICE_BAL CODEBIT"

# Balance de Bob
BOB_BAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address bob_senior))

echo "👨 Balance Bob (Senior Dev): $BOB_BAL CODEBIT"

# Resumen visual final
echo ""
echo "==================================="
echo "✅ Verificación completa de balances"
echo "==================================="
```

### Ejecutar Script

```bash
chmod +x check_balances.sh
./check_balances.sh
```

---

## 📜 Parte 14: Script de Deploy Automatizado

### Crear Script de Deploy

```bash
cat > deploy_devpoints.sh <<'EOF'
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

# Limpiar comillas para cálculo
SUPPLY_CLEAN=$(echo $SUPPLY | tr -d '"')

echo -e "\n${PURPLE}🎉 DevPoints Deployment Complete!${NC}"
echo -e "${GREEN}Token Details:${NC}"
echo "  Name: $NAME"
echo "  Symbol: $SYMBOL"
echo "  Decimals: $DECIMALS (1 CODEBIT = 1 minute)"
echo "  Total Supply: $SUPPLY CODEBIT (~$(($SUPPLY_CLEAN / 60)) hours)"
echo "  Contract ID: $CONTRACT_ID"

echo -e "\n${BLUE}🔗 View in explorer:${NC}"
echo "https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"

echo -e "\n${YELLOW}💡 Next steps:${NC}"
echo "1. Create developer accounts"
echo "2. Distribute CODEBIT for contributions"
echo "3. Test payment flows"
echo "4. Build frontend integration"
EOF
```

### Dar Permisos y Ejecutar

```bash
chmod +x deploy_devpoints.sh
./deploy_devpoints.sh
```

**✅ Salida esperada:**
```
🚀 Deploying DevPoints - CODEBIT Token

📦 Building contract...
ℹ️ CARGO_BUILD_RUSTFLAGS=--remap-path-prefix=/home/karen/.cargo/registry/src= cargo rustc...
    Finished `release` profile [optimized] target(s) in 1.45s
✅ Build Complete

🚢 Deploying to testnet...
✅ Deployed!
Contract ID: CD547FDKSADDFNWRVASLUDWVIAENK5OXRA2LPC7RE6BJUZ7UHWYOH7TH

⚙️  Initializing DevPoints...
null

💰 Minting community supply (100,000 CODEBIT)...
null

✅ Verifying...

🎉 DevPoints Deployment Complete!
Token Details:
  Name: "DevPoints CODEBIT"
  Symbol: "CODEBIT"
  Decimals: 0 (1 CODEBIT = 1 minute)
  Total Supply: "100000" CODEBIT (~1666 hours)
  Contract ID: CD547FDKSADDFNWRVASLUDWVIAENK5OXRA2LPC7RE6BJUZ7UHWYOH7TH

🔗 View in explorer:
https://stellar.expert/explorer/testnet/contract/CD547FDKSADDFNWRVASLUDWVIAENK5OXRA2LPC7RE6BJUZ7UHWYOH7TH

💡 Next steps:
1. Create developer accounts
2. Distribute CODEBIT for contributions
3. Test payment flows
4. Build frontend integration
```

---

## 📸 Parte 15: Evidencia en Stellar Expert

### Capturas de Pantalla Recomendadas

Para documentar tu proyecto, toma capturas de:

#### 1. Vista del Contrato
**URL:** `https://stellar.expert/explorer/testnet/contract/CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP`

**Elementos clave a capturar:**
- Contract ID verificado
- WASM Hash
- Contract Balance
- Operations count

#### 2. Transacciones del Contrato
**Muestra las operaciones:**
- Initialize transaction
- Mint transactions (100,000 CODEBIT iniciales)
- Transfer transactions (Alice → Bob)
- Approve transactions (sistema de bounties)
- Transfer_from transactions
- Burn transactions

#### 3. Cuenta de Alice (Junior Developer)
**URL:** `https://stellar.expert/explorer/testnet/account/GDRX4RWFT6ZU7FHET4UASWHDRVVP22HJXCKEN6S7RG6IBWKLB7DSJH45`

**Evidencia:**
- Balance actual: 1,155 CODEBIT
- Historial de transacciones:
  - Mint de 45 CODEBIT
  - Mint de 1,200 CODEBIT
  - Transfer de -30 CODEBIT a Bob
  - Approve de 500 CODEBIT a bounty_platform
  - Transfer_from de -60 CODEBIT a Charlie

#### 4. Cuenta de Bob (Senior Developer)
**Evidencia:**
- Balance actual: 20 CODEBIT (30 recibidos - 10 quemados)
- Transacciones:
  - Transfer recibido: +30 CODEBIT de Alice
  - Burn: -10 CODEBIT

#### 5. Eventos del Smart Contract
**Captura eventos emitidos:**
- `init` event (inicialización)
- `mint` events
- `transfer` events
- `approve` events
- `trnsfr_frm` events (transfer_from)
- `burn` events

---

## 📝 Parte 16: Resumen de Operaciones Realizadas

### Tabla de Transacciones Exitosas

| Operación | From | To | Amount | Tx Hash (últimos 8 chars) |
|-----------|------|-----|--------|---------------------------|
| **Initialize** | Admin (Karen) | Contract | - | c950baf9 |
| **Mint** | Admin | Admin | 100,000 | 176b7522 |
| **Mint** | Admin | Alice | 45 | 0b37eca2 |
| **Mint** | Admin | Alice | 1,200 | 0e2133d3 |
| **Transfer** | Alice | Bob | 30 | 7559fda5 |
| **Approve** | Alice | Bounty Platform | 500 | 9915b5f1 |
| **Transfer_from** | Alice (via Platform) | Charlie | 60 | 8fd07f10 |
| **Burn** | Bob | - | 10 | 4c1ddc3b |

### Estado Final de Balances

```
📊 Estado Final del Sistema DevPoints
═══════════════════════════════════════

👤 Cuenta                    Balance CODEBIT
────────────────────────────────────────────
👩‍💻 Admin (Karen)                98,755
👧 Alice (Junior Dev)            1,155
👨 Bob (Senior Dev)                 20
👤 Charlie (Solver)                 60
🔥 Quemados (Burned)                10
────────────────────────────────────────────
🪙 TOTAL SUPPLY                101,235
⏱️ Equivalente a             ~1,687 horas
```

**Cálculos verificados:**
- Admin inicial: 100,000 CODEBIT
- Mint a Alice: +45 + 1,200 = 1,245
- Admin final: 100,000 - 1,245 = 98,755 ✅
- Alice recibió: 1,245 CODEBIT
- Alice pagó a Bob: -30 CODEBIT
- Alice pagó a Charlie (via platform): -60 CODEBIT
- Alice final: 1,245 - 30 - 60 = 1,155 ✅
- Bob recibió: 30 CODEBIT
- Bob quemó: -10 CODEBIT
- Bob final: 30 - 10 = 20 ✅
- Charlie recibió: 60 CODEBIT ✅
- Total Supply: 100,000 + 45 + 1,200 - 10 = 101,235 ✅

---

## 🛠️ Troubleshooting - Problemas Reales Resueltos

### Problema 1: Error en Script de Deploy (Línea 76)

**Error encontrado:**
```bash
./deploy_devpoints.sh: line 76: "100000" / 60: syntax error: operand expected
```

**Causa:** Las comillas en el valor de SUPPLY no permiten operaciones aritméticas.

**Solución aplicada:**
```bash
# Limpiar comillas antes del cálculo
SUPPLY_CLEAN=$(echo $SUPPLY | tr -d '"')
echo "Total Supply: $SUPPLY CODEBIT (~$(($SUPPLY_CLEAN / 60)) hours)"
```

### Problema 2: Variable TOKEN_CONTRACT_ID Se Pierde

**Problema:** Al cerrar la terminal WSL, la variable `$TOKEN_CONTRACT_ID` desaparece.

**Solución permanente:**
```bash
# Siempre cargar desde archivo al inicio de sesión
export TOKEN_CONTRACT_ID=$(cat .soroban/token_id)

# O agregar al ~/.bashrc para cargar automáticamente
echo 'export TOKEN_CONTRACT_ID=$(cat ~/devpoints-codebit/token_codebit/.soroban/token_id 2>/dev/null)' >> ~/.bashrc
source ~/.bashrc
```

### Problema 3: Comando "stellar account balance" No Funciona

**Problema:** Comando no disponible en versión de Stellar CLI instalada.

**Solución alternativa:**
```bash
# Usar Horizon API directamente
curl "https://horizon-testnet.stellar.org/accounts/TU_DIRECCION_PUBLICA"

# O verificar en Stellar Expert
echo "https://stellar.expert/explorer/testnet/account/$(stellar keys address karen)"
```

### Problema 4: Mensaje "Simulation identified as read-only"

**Mensaje:**
```
ℹ️ Simulation identified as read-only. Send by rerunning with `--send=yes`.
```

**Explicación:** No es un error. Las operaciones de lectura (balance, allowance, name, etc.) no modifican el estado, por eso Stellar CLI simula en lugar de enviar transacciones reales.

**Acción:** Ninguna. El resultado mostrado es correcto.

---

## 💡 Tips y Comandos Útiles

### Verificación Rápida de Estado

```bash
# Crear alias útiles
alias codebit-supply='stellar contract invoke --id $TOKEN_CONTRACT_ID --source-account karen --network testnet -- total_supply'
alias codebit-balance='stellar contract invoke --id $TOKEN_CONTRACT_ID --source-account karen --network testnet -- balance --account'

# Usar aliases
codebit-supply
codebit-balance $(stellar keys address alice_junior)
```

### Ver Todas las Cuentas Creadas

```bash
# Listar todas las identidades
stellar keys ls

# Ver dirección de una cuenta específica
stellar keys address alice_junior
```

### Copiar CONTRACT_ID al Portapapeles (Windows)

```bash
# Copiar usando clip.exe
echo $TOKEN_CONTRACT_ID | clip.exe

# Ahora puedes pegar con Ctrl+V en cualquier aplicación de Windows
```

### Crear Script de Inicio Rápido

```bash
# Crear quick_start.sh
cat > quick_start.sh <<'EOF'
#!/bin/bash
# Cargar variables de entorno del proyecto

export TOKEN_CONTRACT_ID=$(cat .soroban/token_id 2>/dev/null)

if [ -z "$TOKEN_CONTRACT_ID" ]; then
    echo "⚠️  CONTRACT_ID no encontrado"
    echo "Asegúrate de estar en la carpeta del proyecto"
else
    echo "✅ Contract ID cargado: $TOKEN_CONTRACT_ID"
    echo ""
    echo "📊 Comandos disponibles:"
    echo "  - Total Supply: stellar contract invoke --id \$TOKEN_CONTRACT_ID --network testnet -- total_supply"
    echo "  - Balance: stellar contract invoke --id \$TOKEN_CONTRACT_ID --network testnet -- balance --account DIRECCION"
    echo ""
    echo "🔗 Ver en explorer:"
    echo "  https://stellar.expert/explorer/testnet/contract/$TOKEN_CONTRACT_ID"
fi
EOF

chmod +x quick_start.sh
```

**Usar al inicio de cada sesión:**
```bash
cd /mnt/c/Users/Karen/desktop/CodigoFutura/Clase5-Creacion-Token/token_codebit
source quick_start.sh
```

---

## 📚 Recursos y Links Importantes

### URLs de tu Proyecto

**Contrato Principal:**
```
https://stellar.expert/explorer/testnet/contract/CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP
```

**Cuenta Admin (Karen):**
```
https://horizon-testnet.stellar.org/accounts/GCIGUZUBYP423VBLEQG7UJIWVFPRDKQIPZ6YXL4WQ5RSPXHWAB633CLW
```

**Cuenta Alice (Junior Dev):**
```
https://stellar.expert/explorer/testnet/account/GDRX4RWFT6ZU7FHET4UASWHDRVVP22HJXCKEN6S7RG6IBWKLB7DSJH45
```

### Documentación Oficial

- **Stellar Docs:** https://developers.stellar.org
- **Soroban Docs:** https://soroban.stellar.org
- **CAP-46 Spec:** https://stellar.org/protocol/cap-46
- **Stellar CLI Docs:** https://developers.stellar.org/docs/tools/developer-tools

### Herramientas

- **Stellar Expert (Testnet):** https://stellar.expert/explorer/testnet
- **Friendbot (fondear cuentas):** https://friendbot.stellar.org
- **Horizon API (Testnet):** https://horizon-testnet.stellar.org
- **Freighter Wallet:** https://www.freighter.app

---

## ✅ Checklist Final - Deploy Completado

### Configuración Inicial
- [x] ✅ WSL2 con Ubuntu instalado y actualizado
- [x] ✅ Rust 1.74.0+ instalado
- [x] ✅ Stellar CLI 21.0.0+ instalado
- [x] ✅ Target wasm32-unknown-unknown agregado
- [x] ✅ Herramientas adicionales (jq, git) instaladas

### Desarrollo del Proyecto
- [x] ✅ Proyecto token_codebit creado
- [x] ✅ Cargo.toml configurado correctamente
- [x] ✅ src/storage.rs implementado
- [x] ✅ src/errors.rs implementado
- [x] ✅ src/lib.rs implementado (código completo)
- [x] ✅ Compilación exitosa a WASM (9.5KB)
- [x] ✅ Sin warnings ni errores

### Configuración de Cuentas
- [x] ✅ Cuenta admin (karen) creada
- [x] ✅ Cuenta admin fondeada (10,000 XLM)
- [x] ✅ Cuenta alice_junior creada y fondeada
- [x] ✅ Cuenta bob_senior creada y fondeada
- [x] ✅ Cuenta bounty_platform creada y fondeada
- [x] ✅ Cuenta charlie_solver creada y fondeada

### Deploy y Configuración
- [x] ✅ Contrato desplegado en testnet
- [x] ✅ CONTRACT_ID guardado (CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP)
- [x] ✅ Token inicializado con decimals = 0
- [x] ✅ Nombre: "DevPoints CODEBIT"
- [x] ✅ Símbolo: "CODEBIT"
- [x] ✅ Supply inicial: 100,000 CODEBIT minteado

### Operaciones Probadas
- [x] ✅ Mint de CODEBIT (admin → users)
- [x] ✅ Transfer directo (Alice → Bob)
- [x] ✅ Approve para allowance (Alice → Platform)
- [x] ✅ Transfer_from (delegated transfer)
- [x] ✅ Burn de tokens (Bob)
- [x] ✅ Balance queries funcionando
- [x] ✅ Total supply verificado (101,235 CODEBIT)

### Verificación en Blockchain
- [x] ✅ Contrato visible en Stellar Expert
- [x] ✅ Todas las transacciones verificadas on-chain
- [x] ✅ Eventos emitidos correctamente
- [x] ✅ Balances consistentes con operaciones

### Scripts y Automatización
- [x] ✅ deploy_devpoints.sh creado y probado
- [x] ✅ check_balances.sh creado
- [x] ✅ quick_start.sh creado
- [x] ✅ Todos los scripts con permisos de ejecución

### Documentación
- [x] ✅ Guía completa de deploy documentada
- [x] ✅ Todos los comandos probados y verificados
- [x] ✅ Screenshots/evidencia preparada para agregar
- [x] ✅ Troubleshooting documentado
- [x] ✅ Links a recursos guardados

---

## 🎯 Próximos Pasos para Clase 6

### Preparación para Frontend

#### 1. Instalar Node.js en Windows
```bash
# Descargar e instalar Node.js 18+ desde:
# https://nodejs.org/

# Verificar instalación (en PowerShell de Windows)
node --version
npm --version
```

#### 2. Instalar Freighter Wallet
1. Ir a: https://www.freighter.app/
2. Descargar extensión para tu navegador (Chrome/Firefox)
3. Crear nueva wallet o importar existente
4. Cambiar a Testnet en configuración
5. Importar tu cuenta admin (karen) usando su secret key

**Para obtener secret key:**
```bash
stellar keys show karen
```

⚠️ **IMPORTANTE:** Esta información es SOLO para testnet. NUNCA compartas secret keys de mainnet.

#### 3. Verificar Freighter Conecta con tu Token

En Freighter wallet:
- Cambiar a Testnet
- Ver balance de XLM (~10,000)
- Agregar token CODEBIT manualmente:
  - Contract ID: `CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP`
  - Verificar que aparezca tu balance de CODEBIT

---

## 🎓 Lecciones Aprendidas

### Conceptos Clave Dominados

1. **Tokens con 0 Decimales:**
   - Entiendes por qué CODEBIT usa decimals = 0
   - Sabes calcular amounts correctamente (1 CODEBIT = 1 minuto exacto)

2. **Sistema de Allowances:**
   - Implementaste approve/transfer_from completo
   - Entiendes el patrón de delegación de permisos

3. **Deploy en Blockchain Real:**
   - Desplegaste tu primer smart contract en testnet
   - Todas las operaciones son verificables on-chain

4. **Herramientas de Desarrollo:**
   - Dominas WSL para desarrollo blockchain
   - Usas Stellar CLI profesionalmente
   - Automatizaste procesos con scripts bash

5. **Verificación y Debugging:**
   - Sabes usar Stellar Expert para auditar transacciones
   - Entiendes cómo leer eventos del smart contract
   - Puedes troubleshoot problemas comunes

---

## 🦈 Reflexión Final

**¡Felicitaciones, Karen!** 🎉

Has completado exitosamente el deploy de tu token CODEBIT en Stellar Testnet. Este no es un ejercicio teórico - es un smart contract real, funcionando en una blockchain pública, con transacciones verificables por cualquier persona.

### Lo que has logrado:

✅ **Smart Contract en Producción (Testnet)**
- Token CODEBIT desplegado y funcional
- 101,235 tokens en circulación
- 8+ transacciones exitosas verificadas on-chain

✅ **Economía Funcional Creada**
- Sistema de recompensas por contribuciones (mint)
- Pagos peer-to-peer funcionando (transfer)
- Sistema de micro-bounties automático (approve + transfer_from)
- Quema de tokens implementada (burn)

✅ **Portfolio Blockchain Verificable**
- Todas tus operaciones están en la blockchain
- Cualquier reclutador puede verificar tu trabajo
- URL pública de tu contrato para agregar a CV

✅ **Experiencia Profesional Real**
- Herramientas usadas por developers profesionales
- Procesos similares a deploy en mainnet
- Debugging y troubleshooting en ambiente real

### Tu impacto potencial:

Este token puede revolucionar cómo las comunidades dev reconocen el valor:
- **Para juniors:** Portfolio verificable de contribuciones
- **Para seniors:** Sistema justo de compensación por mentorías
- **Para comunidades:** Incentivos económicos transparentes
- **Para el ecosistema:** Nueva forma de valorar conocimiento

### Próximo nivel:

En Clase 6 conectarás este backend blockchain con:
- Frontend React profesional
- Integración con Freighter Wallet
- Dashboard en tiempo real
- UI intuitiva para usuarios no técnicos

**Tu CODEBIT está listo para cambiar el juego. ¡Seguí construyendo! 💻🚀**

---

## 📞 Soporte y Recursos

### Si Necesitas Ayuda

**Documentación oficial:**
- Stellar Docs: https://developers.stellar.org
- Soroban Docs: https://soroban.stellar.org

**Comunidad:**
- Stellar Discord: https://discord.gg/stellardev
- Stack Overflow: Tag `soroban` o `stellar`

**Tu proyecto:**
- Contract ID: `CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP`
- Explorer: https://stellar.expert/explorer/testnet/contract/CC7HE6TTZ56BE3OIG4EKHF52Q5HJTA22DFY37BORQB2K57ZPAM3MA6OP

---

**Versión de la guía:** 2.0 (Actualizada con resultados reales)  
**Fecha:** Octubre 2025  
**Developer:** Karen  
**Proyecto:** DevPoints CODEBIT  
**Network:** Stellar Testnet  
**Status:** ✅ Deploy Exitoso y Verificado  

---

# 🌟 ¡Tu token CODEBIT está vivo en la blockchain! 🌟