# Clase 8 - Assets Nativos en Stellar

## Proyecto: dApp de Gestión de Stablecoins

Una aplicación descentralizada para gestionar USDC en Stellar sin necesidad de contratos inteligentes.

**Funcionalidades principales:**
- Conectar wallet (Freighter)
- Ver balance de USDC en tiempo real
- Crear trustlines para aceptar assets
- Guardar datos de transacciones
- Deploy en Vercel

---

## La Gran Diferencia: Ethereum vs Stellar

### En Ethereum (ERC-20)
Para crear un token necesitas escribir un contrato inteligente completo:

```solidity
contract MyToken {
    mapping(address => uint256) balances;
    uint256 public totalSupply;
    
    function transfer(address to, uint256 amount) {
        require(balances[msg.sender] >= amount);
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
    // ... +100 líneas más
}
```

**Costos y complejidad:**
- 100-200 líneas de código en Solidity
- Compilación y auditoría necesaria
- Deploy: $500-$5,000 USD
- Gas fees: $5-$50 por transacción
- Mantenimiento constante

### En Stellar (Assets Nativos)
Los tokens están integrados en el protocolo. Solo defines el asset:

```javascript
const USDC = new Asset(
    'USDC',
    'GBBD4J7F6LWK7P7MDEVSCWR7DPUVW3NY3DTQEVFL4NAT4AQH3ZLLFLAS'
);
// ¡Listo! El protocolo maneja el resto.
```

**Ventajas:**
- 3 líneas de código
- Sin compilación ni auditoría
- Deploy: $0 USD
- Fees fijos: $0.000005 por transacción
- Sin mantenimiento (gestionado por el protocolo)

> **Dato clave:** El 80% de los casos de uso no requieren contratos inteligentes en Stellar.

---

## ¿Qué son los Assets Nativos?

Los Assets Nativos son tokens que existen directamente en el protocolo de Stellar, no son contratos inteligentes.

### Componentes de un Asset

**1. Asset Code**
- Máximo 12 caracteres alfanuméricos
- Case-sensitive: `USDC` ≠ `usdc`
- Ejemplos: USDC, EUROC, BRL, GOLD, AAPL

**2. Issuer (Public Key)**
- Cuenta de Stellar que emite el asset
- Formato: 56 caracteres (comienza con G...)
- Puede ser una empresa, banco o fintech (ej: Circle)

**3. Identificador Único**
- Combinación: Asset Code + Issuer
- Pueden existir varios "USDC", cada uno con diferente issuer

### Analogía Bancaria

Piensa en Stellar como un sistema bancario global:

```
Asset Code → Tipo de moneda (USD, EUR)
Issuer → Banco emisor (Chase, BoA, Circle)
Trustline → Abrir una cuenta en ese banco
```

---

## ¿Cuándo NO necesitas Soroban (Smart Contracts)?

✅ **Usa Assets Nativos para:**
- Crear una stablecoin
- Enviar pagos entre cuentas
- Intercambiar assets (DEX nativo)
- Crear trustlines
- Transferencias básicas de tokens

❌ **Usa Soroban solo para:**
- Yield farming o rendimiento automático
- Votaciones con reglas personalizadas
- NFTs con metadatos dinámicos
- Lending protocols con tasas variables
- Lógica personalizada que el protocolo base no cubre

> **Regla de oro:** Usa Soroban solo cuando necesites lógica avanzada o personalizada.

---

## ¿Qué es una Trustline?

Antes de recibir cualquier asset (excepto XLM), debes crear una **trustline**.

### Analogía Simple

```
XLM = Efectivo
→ Aceptado automáticamente por todos

USDC = Cheque
→ Requiere validación del emisor

Trustline = Cuenta Bancaria
→ Abrir cuenta para recibir cheques de ese banco
```

**Propósito:** Mecanismo de seguridad para evitar spam de tokens no deseados.

### Costo de las Trustlines

| Concepto | XLM | Descripción |
|----------|-----|-------------|
| **Base Reserve** | 1.0 | Mínimo para que la cuenta exista |
| **Por Trustline** | 0.5 | XLM "congelados" por cada trustline |
| **Total Mínimo** | 1.5 | Cuenta activa con 1 trustline |

### Ejemplo Práctico

```
1. Tienes: 10 XLM en tu cuenta
2. Creas: 1 trustline para USDC
3. Quedas con: 9.5 XLM disponibles
4. Congelados: 0.5 XLM (recuperables)
5. Si eliminas la trustline: vuelves a tener 10 XLM
```

⚠️ **Importante:** Los XLM congelados no se pierden. Se recuperan automáticamente cuando eliminas la trustline y el balance del asset es 0.

---

## Arquitectura de la dApp

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  Frontend   │────▶│  Freighter   │────▶│   Stellar   │
│  (Next.js)  │◀────│   Wallet     │◀────│   Network   │
└─────────────┘     └──────────────┘     └─────────────┘
       │
       ▼
┌─────────────┐
│  Supabase   │
│   (Base de  │
│    datos)   │
└─────────────┘
```

**Componentes:**
- **Frontend:** Next.js (Vercel) - Interfaz de usuario
- **Freighter:** Wallet para firmar transacciones
- **Stellar Network:** Blockchain descentralizada
- **Supabase:** Almacenamiento de metadata

---

## Flujo de Transacción

### 1. Iniciación
El usuario hace clic en "Crear Trustline". El frontend construye la transacción con Stellar SDK.

### 2. Firma
Se abre popup de Freighter donde el usuario autoriza con su clave privada (la clave nunca sale de la wallet).

### 3. Envío
La transacción firmada se envía a Stellar Network, que la valida y ejecuta en 3-5 segundos.

### 4. Confirmación
La red devuelve confirmación:
- Frontend guarda metadata en Supabase
- Muestra mensaje de éxito al usuario

---

## Setup del Proyecto

### 1. Requisitos Previos
- Node.js v18+
- VS Code
- Freighter Wallet instalada
- Cuentas gratuitas en Supabase y Vercel

### 2. Crear Proyecto Next.js

```bash
npx create-next-app@latest dapp-stellar-assets
```

**Opciones:**
- TypeScript: NO
- ESLint: YES
- Tailwind CSS: YES
- src directory: YES
- App Router: YES

### 3. Instalar Dependencias

```bash
npm install stellar-sdk @supabase/supabase-js @stellar/freighter-api
```

### 4. Estructura de Carpetas
```
src/
├── components/     # Componentes React
└── lib/           # Utilidades y configuración
```

---

## Configuración de Supabase

### Crear Proyecto
1. Ve a supabase.com
2. New Project → "dapp-stellar-assets"
3. Región: South America
4. Plan: Free Tier

### Crear Tablas

**Tabla: trustlines**
- `user_id`: Public key del usuario
- `asset_code`: Código del asset (USDC)
- `asset_issuer`: Issuer del asset
- `status`: Estado (active, removed)
- `created_at`: Timestamp

**Tabla: transactions**
- `user_id`: Public key
- `tx_hash`: Hash de la transacción
- `asset_code`: Asset involucrado
- `created_at`: Timestamp

### Obtener Credenciales
Settings → API:
- Project URL
- anon public key

Guardar en `.env.local`

⚠️ **Importante:** Activa Row Level Security (RLS) para proteger datos.

---

## Componentes Principales

### 1. WalletConnect
Gestiona la conexión con Freighter Wallet.

**Funcionalidades:**
- 🔍 Detectar si Freighter está instalado (`window.freighter`)
- 🔑 Solicitar acceso con `getPublicKey()`
- 💾 Guardar public key en estado de React
- 📋 Copiar public key al portapapeles

**UX:** Muestra dirección abreviada (`GABC…XYZ9`) pero permite copiar la versión completa.

### 2. AssetBalance
Consulta el balance de un asset específico desde Horizon API.

**Funcionalidades clave:**
- Conexión con Stellar Horizon API (Testnet)
- Carga datos completos de cuenta
- Filtra asset específico
- Excluye XLM nativo
- Botón de actualización manual
- Manejo de errores

🧩 **Corrección crítica:** Siempre verifica `asset.asset_type !== 'native'` para excluir XLM.

💡 **Mejora:** Tooltip con información completa del issuer al hacer hover.

### 3. CreateTrustline
Componente más avanzado. Crea, firma y envía la trustline.

**Proceso:**

**1. Verificación**
- Comprueba si la trustline ya existe (blockchain + DB)
- Previene duplicados

**2. Construcción**
- Usa `TransactionBuilder` para crear operación `ChangeTrust`
- Define asset y límite

**3. Firma**
- Convierte transacción a XDR
- Envía a Freighter para firma con secret key

**4. Envío**
- Envía transacción firmada con `submitTransaction()`
- Stellar Network la valida y ejecuta

**5. Persistencia**
- Guarda en Supabase: hash, user_id, asset info, status

---

## Puntos Clave a Recordar

✅ Assets Nativos son parte del protocolo, no contratos inteligentes

✅ Una trustline es necesaria para recibir cualquier asset (excepto XLM)

✅ Los XLM congelados en trustlines son recuperables

✅ Stellar es más simple y económico que Ethereum para casos de uso básicos

✅ Freighter maneja la seguridad: las claves privadas nunca salen de la wallet

✅ Supabase almacena metadata, la blockchain almacena transacciones

---

## Próximos Pasos

En la siguiente clase continuaremos con la implementación práctica del código de estos componentes.

---

**Curso:** Código Futura - Buen Día Builders  
**Clase:** 8 - Assets Nativos en Stellar  
**Continuación de:** Clase 7 (Código dApp)