#!/bin/bash
# ===============================================================
# 🌟 PROYECTO CODEBIT - Verificación final de balances y totales
# ===============================================================
# Este script muestra:
#   🪙 Total supply del token CODEBIT
#   👩‍💻 Balance del admin (Karen)
#   👧 Balance de Alice (junior developer)
#   👨 Balance de Bob (senior developer)
# ===============================================================

echo ""
echo "📊 Estadísticas DevPoints - CODEBIT"
echo "==================================="
echo ""

# ✅ Total supply del token CODEBIT
TOTAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- total_supply)

echo "🪙 Total Supply: $TOTAL CODEBIT"

if [ -n "$TOTAL" ]; then
  echo "⏱️ Equivalente a: $(($TOTAL / 60)) horas de trabajo"
else
  echo "⚠️ No se pudo obtener el total supply. Verificá el contrato o la red."
fi

# ✅ Balance del administrador (Karen)
ADMIN_BAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address karen))

echo ""
echo "👩‍💻 Balance Admin (Karen): $ADMIN_BAL CODEBIT"

# ✅ Balance de Alice
ALICE_BAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior))

echo "👧 Balance Alice (Junior Dev): $ALICE_BAL CODEBIT"

# ✅ Balance de Bob
BOB_BAL=$(stellar contract invoke \
  --id $TOKEN_CONTRACT_ID \
  --source karen \
  --network testnet \
  -- balance \
  --account $(stellar keys address bob_senior))

echo "👨 Balance Bob (Senior Dev): $BOB_BAL CODEBIT"

# ✅ Resumen visual final
echo ""
echo "==================================="
echo "🏁 Verificación completa de balances"
echo "==================================="

