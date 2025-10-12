# Apuntes Clase 1 - Introducción a Stellar 🦈

## ¿Qué es Stellar?

Stellar es una **red blockchain descentralizada** diseñada para facilitar transacciones financieras rápidas, seguras y de bajo costo a nivel global.

### Características Principales

- **Rápida:** Transacciones confirmadas en 3-5 segundos
- **Económica:** Tarifas extremadamente bajas (0.00001 XLM por transacción)
- **Global:** Permite transferencias transfronterizas sin intermediarios
- **Escalable:** Puede procesar miles de transacciones por segundo
- **Open Source:** Código abierto y transparente

---

## Conceptos Fundamentales

### 1. XLM (Lumens)
- Criptomoneda nativa de Stellar
- Se usa para pagar tarifas de transacción
- Actúa como puente entre diferentes activos

### 2. Cuentas en Stellar
Cada cuenta en Stellar tiene:
- **Public Key (Clave Pública):** Dirección visible que comienza con "G"
- **Secret Key (Clave Privada):** Clave secreta que comienza con "S" (nunca compartir)

**Formato:**
```
Public Key:  GBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Secret Key:  SBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### 3. Red de Pruebas (Testnet)
- Ambiente seguro para experimentar
- XLM gratis mediante Friendbot
- Sin riesgo financiero real

---

## Componentes de la Red Stellar

### Horizon
- API RESTful para interactuar con la red Stellar
- Permite consultar cuentas, transacciones y operaciones
- Punto de conexión entre aplicaciones y la blockchain

### Stellar Core
- Motor de consenso de la red
- Valida y procesa transacciones
- Mantiene la integridad del ledger

### Friendbot
- Servicio para fondear cuentas en testnet
- Proporciona XLM gratis para desarrollo
- URL: https://friendbot.stellar.org

---

## Primeros Pasos Prácticos

### 1. Crear tu Primera Cuenta

**Usando Stellar Laboratory:**
1. Ir a https://laboratory.stellar.org
2. Seleccionar "Testnet"
3. Ir a "Account > Generate Keypair"
4. Guardar ambas keys de forma segura

### 2. Fondear la Cuenta

**Opciones:**
- Usar Friendbot en Laboratory
- Hacer petición HTTP a Friendbot
- Usar herramientas CLI

**Resultado:** Tu cuenta recibe 10,000 XLM de prueba

### 3. Consultar Balance

En Stellar Laboratory:
1. Ir a "Endpoints > Accounts"
2. Ingresar tu Public Key
3. Ver detalles de la cuenta (balance, sequence number, etc.)

---

## Tipos de Operaciones en Stellar

### Operaciones Básicas

1. **Create Account:** Crear nueva cuenta en la red
2. **Payment:** Enviar activos de una cuenta a otra
3. **Path Payment:** Pago con conversión automática de activos
4. **Manage Offer:** Crear/modificar ofertas en el DEX
5. **Set Options:** Configurar parámetros de la cuenta
6. **Change Trust:** Establecer confianza en un activo
7. **Account Merge:** Fusionar cuentas

### Estructura de una Transacción

Toda transacción en Stellar contiene:
- **Source Account:** Cuenta que origina la transacción
- **Sequence Number:** Número único e incremental
- **Operations:** Lista de operaciones a ejecutar (máx. 100)
- **Fee:** Tarifa a pagar (base: 100 stroops = 0.00001 XLM)
- **Memo:** Nota opcional (máx. 28 bytes)
- **Time Bounds:** Ventana de validez temporal

---

## Herramientas Utilizadas

### Stellar Laboratory
**URL:** https://laboratory.stellar.org

**Funcionalidades:**
- Generar keypairs
- Fondear cuentas
- Construir y firmar transacciones
- Explorar la red
- Probar operaciones

### Stellar Expert
**URL:** https://stellar.expert/explorer/testnet

**Funcionalidades:**
- Explorador de blockchain
- Ver transacciones en tiempo real
- Analizar cuentas y activos
- Estadísticas de la red

---

## Tu Primera Transacción

### Pasos para Enviar un Pago

1. **Obtener información de la cuenta origen**
   - Consultar sequence number actual

2. **Construir la transacción**
   - Definir cuenta origen
   - Agregar operación de pago
   - Establecer destinatario y monto
   - Añadir memo (opcional)

3. **Firmar la transacción**
   - Usar la secret key de la cuenta origen

4. **Enviar a la red**
   - Submit a través de Horizon

5. **Verificar resultado**
   - Consultar hash de transacción
   - Verificar balances actualizados

---

## Conceptos Importantes

### Sequence Number
- Contador único por cuenta
- Se incrementa con cada transacción
- Previene transacciones duplicadas
- Debe ser exactamente el siguiente número

### Stroops
- Unidad mínima de XLM
- 1 XLM = 10,000,000 stroops
- Se usa para cálculos precisos

### Memos
Tipos de memo:
- **MEMO_TEXT:** Texto plano (máx. 28 bytes)
- **MEMO_ID:** Identificador numérico
- **MEMO_HASH:** Hash de 32 bytes
- **MEMO_RETURN:** Para devoluciones

---

## Buenas Prácticas

### Seguridad
✅ **NUNCA** compartas tu Secret Key
✅ Guarda tus keys en lugar seguro
✅ Usa testnet para aprender
✅ Verifica addresses antes de enviar
✅ Usa memos para identificar transacciones

### Desarrollo
✅ Comienza en testnet siempre
✅ Prueba cada operación antes de producción
✅ Maneja errores adecuadamente
✅ Documenta tu código
✅ Consulta la documentación oficial

---

## Recursos Fundamentales

### Documentación Oficial
- **Stellar Developers:** https://developers.stellar.org
- **API Reference:** https://developers.stellar.org/api
- **Stellar SDK:** https://stellar.github.io/js-stellar-sdk/

### Comunidad
- **Discord:** Canal #soroban-dev
- **Stack Overflow:** Tag [stellar]
- **GitHub:** https://github.com/stellar

### Herramientas
- **Laboratory:** https://laboratory.stellar.org
- **Stellar Expert:** https://stellar.expert
- **Friendbot:** https://friendbot.stellar.org

---

## Logros de la Clase 1

✅ Entendiste qué es Stellar y para qué sirve
✅ Creaste tu primera cuenta en testnet
✅ Fondeaste tu cuenta con Friendbot
✅ Consultaste el balance de tu cuenta
✅ Enviaste tu primera transacción
✅ Exploraste transacciones en Stellar Expert
✅ Te familiarizaste con Stellar Laboratory

---

## Próximos Pasos

**En la Clase 2 aprenderás:**
- JavaScript + Stellar SDK
- Crear cuentas programáticamente
- Enviar transacciones con código
- Automatizar operaciones
- Consultar información de la red

---

## Glosario Rápido

| Término | Definición |
|---------|------------|
| **Ledger** | Estado actual de la blockchain |
| **Account** | Entidad que puede realizar transacciones |
| **Asset** | Cualquier tipo de activo en Stellar |
| **Trustline** | Permiso para recibir un activo específico |
| **Operations** | Acciones individuales en una transacción |
| **Fee** | Costo de procesar una transacción |
| **Horizon** | API para interactuar con Stellar |
| **Testnet** | Red de pruebas para desarrollo |

---

## Notas Importantes

💡 **Recuerda:** Stellar es diferente a otras blockchains:
- No hay gas fees variables
- Las transacciones son casi instantáneas
- Puedes enviar múltiples operaciones en una transacción
- Cada cuenta necesita un balance mínimo (2 XLM en mainnet)

🔒 **Seguridad Crítica:**
- Tu Secret Key = acceso total a tu cuenta
- Si pierdes tu Secret Key, pierdes acceso a la cuenta
- No hay forma de recuperar una Secret Key perdida

---

*Apuntes creados para las Tiburonas Builders - Codigo Futura 2025* 🦈⚡

**Próxima clase:** Fundamentos de Programación Stellar