# Clase 10 - Desarrollo de dApps en Stellar

## Introducción: Tu Guía Completa

La guía completa para crear aplicaciones descentralizadas con Soroban y Stellar.

**Aprenderás a:**
- Diseñar aplicaciones descentralizadas
- Desplegar smart contracts
- Optimizar para producción
- Construir experiencias de usuario rápidas, seguras e increíbles

---

## Tu Viaje de Aprendizaje

### 01. Fundamentos del Proyecto
Comprende la arquitectura completa del template monorepo y cómo se integran frontend, backend y blockchain.

### 02. Configuración del Entorno
Instala y prepara las herramientas esenciales: Stellar CLI, Supabase, Next.js y dependencias clave.

### 03. Arquitectura y Conexiones
Domina el flujo de datos desde smart contracts hasta componentes React mediante bindings TypeScript.

### 04. Desarrollo de Contratos
Crea smart contracts personalizados en Rust: desde la escritura hasta el deployment en testnet.

### 05. Optimización Avanzada
Aplica técnicas de performance, caching y mejores prácticas para entornos de producción.

### 06. Trucos y Secretos
Descubre tips profesionales, debugging avanzado y estrategias para escalar tu aplicación con confianza.

---

## El Template: Tu Ferrari Listo para Arrancar

### ¿Qué es este proyecto?

Un **template monorepo fullstack** que integra todo lo necesario para construir dApps profesionales en Stellar.

Sin configuraciones eternas: **todo está listo para que empieces a crear**.

Con este template puedes construir aplicaciones descentralizadas completas:
- Backend persistente
- Frontend moderno
- Smart contracts optimizados

Todo trabajando en armonía.

---

## 🧩 Casos de Uso Perfectos

✅ Sistemas de votación descentralizados con transparencia total  
✅ Marketplaces de NFTs con transacciones seguras  
✅ Plataformas DeFi para trading o staking  
✅ Sistemas de identidad digital verificable  
✅ DAOs con gobernanza on-chain  
✅ Aplicaciones de gaming con economía tokenizada

---

## Estructura del Proyecto

### 1. apps/backend

**Supabase** con:
- Base de datos PostgreSQL
- Autenticación de usuarios
- Almacenamiento de archivos
- Funcionalidades real-time

### 2. apps/web

**Frontend** en Next.js 16 con:
- React 18
- TypeScript
- Tailwind CSS
- Integración nativa con Stellar Wallets Kit (Freighter, xBull y Albedo)

### 3. contracts/

**Smart contracts** en Rust compilados a WASM.

Incluye ejemplos:
- hello-world
- increment
- fungible-token
- NFT
- starter template personalizable

### 4. packages/

**Bindings TypeScript** auto-generados desde tus contratos Rust.

Asegura type-safety completo entre blockchain y frontend sin esfuerzo manual.

---

## ⚠️ Checkpoint Importante

**No avances hasta que todas estas partes funcionen.**

---

## Stack Tecnológico Completo

### 🖥 Frontend & UI

- **Next.js 16:** Framework React con App Router y Server Components
- **React 18:** Biblioteca UI con Concurrent Features
- **TypeScript:** Type safety en toda la aplicación
- **Tailwind CSS:** Utility-first styling moderno
- **Stellar Wallets Kit:** Integración multi-wallet lista

### ⚙ Blockchain & Contratos

- **Stellar / Soroban:** Layer 1 blockchain escalable
- **Rust:** Lenguaje para smart contracts seguros
- **WASM:** Target de compilación optimizado
- **Stellar CLI v23+:** Herramientas de desarrollo

### 🗄 Backend & Database

- **Supabase:** Backend-as-a-Service completo
- **PostgreSQL:** Base de datos relacional robusta
- **Supabase Auth:** Sistema de autenticación
- **Supabase Storage:** Almacenamiento de archivos
- **Realtime:** Subscripciones en tiempo real

### 🔧 DevOps & Tools

- **Docker:** Contenedores para desarrollo local
- **pnpm / npm:** Gestión de dependencias
- **Cargo:** Build system de Rust
- **Git:** Control de versiones

---

## Requisitos Previos del Sistema

### 1. Node.js 20+

Runtime JavaScript necesario para ejecutar Next.js y las herramientas del ecosistema frontend.

**Verifica tu versión:**
```bash
node --version
# Debe mostrar v20.0.0 o superior
```

### 2. Docker Desktop

Requerido para ejecutar Supabase localmente. Docker levanta PostgreSQL, Auth y Storage en contenedores.

**Verifica instalación:**
```bash
docker --version
```

### 3. Rust & Cargo

Lenguaje y build system para compilar smart contracts.

**Instala con el script oficial:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 4. Stellar CLI v23+

Herramienta esencial para build, deploy e invocar contratos Soroban.

**Instala vía Cargo:**
```bash
cargo install --locked stellar-cli --features opt
```

### 5. WASM Target

Target de compilación para generar binarios WebAssembly desde Rust. Indispensable para contracts Soroban.

**Agrega el target:**
```bash
rustup target add wasm32-unknown-unknown
```

### 6. Supabase CLI

CLI para gestionar tu instancia local de Supabase.

**Instala con brew (macOS):**
```bash
brew install supabase/tap/supabase
```

**O con npm (otros sistemas):**
```bash
npm install -g supabase
```

---

## Instalación: Primeros Pasos

### 1. Clonar el Repositorio

```bash
git clone https://github.com/tu-repo/stellar-startup-template.git
cd stellar-startup-template
```

### 2. Instalar Dependencias

Este comando instalará todas las dependencias del monorepo usando workspaces.

```bash
npm install
# o si prefieres:
pnpm install
```

#### ¿Qué está sucediendo?

Durante la instalación, npm/pnpm:
1. Lee el archivo `package.json` de la raíz y de cada workspace
2. Descarga las dependencias especificadas
3. Crea symlinks entre los workspaces locales
4. Genera el archivo `package-lock.json`
5. Prepara el entorno para desarrollo

⏱️ **Tiempo estimado:** 2–5 minutos, según la velocidad de tu conexión.

---

## Configuración de Supabase (Backend)

### 1. Navegar a Backend

```bash
cd apps/backend
```

### 2. Inicializar Supabase

Crea la estructura de carpetas y archivos de configuración para Supabase local.

```bash
npx supabase init
```

### 3. Levantar Servicios

⚠️ **Importante:** Docker Desktop debe estar corriendo.

Este comando levanta PostgreSQL, Auth, Storage y Studio en contenedores.

```bash
npm run dev
```

### 4. Obtener Credenciales

Abre una nueva terminal y ejecuta:

```bash
npm run status
```

💾 **Guarda estos valores:**
- API URL (por ejemplo: `http://localhost:54321`)
- Key `anon`
- Key `service_role`

Necesitarás estos valores para configurar el frontend.

---

## Configuración del Frontend

### 1. Setup Inicial

Copia el template de variables de entorno:

```bash
cd apps/web
cp .env.local.example .env.local
nano .env.local  # o code .env.local
```

### 2. Variables Críticas

🔑 **Nota:** Las variables con prefijo `NEXT_PUBLIC_` estarán disponibles en el browser. Las que no lo tienen solo existen en el servidor.

```bash
# Supabase (del paso anterior)
NEXT_PUBLIC_SUPABASE_URL=http://localhost:54321
NEXT_PUBLIC_SUPABASE_ANON_KEY=eyJ...

# Stellar Network
NEXT_PUBLIC_STELLAR_NETWORK=testnet
NEXT_PUBLIC_STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
NEXT_PUBLIC_STELLAR_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
```

### 3. Levantar el Servidor

```bash
npm run dev
```

🌐 Abre `http://localhost:3000` y verifica que el frontend se conecta correctamente con:
- Tu instancia local de Supabase
- La red de Stellar

---

## Arquitectura: Del Contrato al Usuario

### Flujo de Datos Unidireccional

Tu aplicación sigue un flujo que garantiza consistencia y type-safety en cada paso:

```
1. Smart Contract (Rust)
        ↓
2. TypeScript Bindings
        ↓
3. API Routes (Next.js)
        ↓
4. Custom Hooks
        ↓
5. React Components
```

---

## Componentes del Flujo

### 1️⃣ Smart Contract (Rust)

Lógica de negocio compilada a WASM, desplegada en la blockchain de Stellar.

### 2️⃣ TypeScript Bindings

Código auto-generado que provee types e interfaces para invocar los contratos.

### 3️⃣ API Routes (Next.js)

Endpoints backend que cargan los bindings dinámicamente y procesan las transacciones.

### 4️⃣ Custom Hooks

Lógica reutilizable que consume APIs y gestiona el estado en React.

### 5️⃣ React Components

Interfaz que conecta wallets, muestra datos y permite la interacción del usuario.

---

## Crear un Smart Contract desde Cero

Vas a construir un **contrato de votación completo** que demuestra conceptos clave:

📦 **Storage**  
🔐 **Authorization**  
⚡ **Eventos**  
🧩 **Estructuras de datos complejas**

Este ejemplo cubre el flujo completo: desde Rust hasta la UI.

---

## 🚀 Funcionalidades Clave del Contrato

- **initialize:** Setup inicial del contrato
- **create_proposal:** Crear nuevas propuestas
- **vote:** Votar por propuestas existentes
- **get_proposal:** Consultar propuesta específica
- **get_all_proposals:** Listar todas las propuestas

🗳️ Este contrato usa **persistent storage** para registrar propuestas y votos, asegurando que cada dirección solo pueda votar una vez por propuesta.

---

## ⚙️ Inicializar Proyecto

```bash
cd contracts/contracts
mkdir voting && cd voting
cargo init --lib --name voting
```

---

## 🧱 Estructura del Contrato

```
contracts/contracts/voting/
│
├── Cargo.toml
├── src/
│   └── lib.rs
└── target/
```

### Configurar Cargo.toml

Asegúrate de que tu `Cargo.toml` tenga las dependencias necesarias:

```toml
[package]
name = "voting"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "21.0.0"

[dev-dependencies]
soroban-sdk = { version = "21.0.0", features = ["testutils"] }

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true

[profile.release-with-logs]
inherits = "release"
debug-assertions = true
```

### Implementar el Contrato (lib.rs)

El contrato de votación incluye estructuras de datos, storage y funciones principales.

**Puntos clave del código:**
- Usa `#[contract]` para definir el contrato
- Usa `#[contracttype]` para estructuras de datos
- Usa `storage::Persistent` para datos que persisten entre llamadas
- Implementa autorización con `env.require_auth()`
- Emite eventos para registrar acciones importantes

---

## Deploy y Generación de Bindings

### 1️⃣ Compilar

Genera el archivo WASM optimizado en `target/wasm32-unknown-unknown/release/`

```bash
stellar contract build
```

### 2️⃣ Deploy a Testnet

Despliega tu contrato en la testnet y guarda el `CONTRACT_ID` generado.

```bash
CONTRACT_ID=$(stellar contract deploy \
  --wasm target/.../voting.wasm \
  --network testnet \
  --source $SECRET_KEY)
  
echo $CONTRACT_ID
```

### 3️⃣ Inicializar

Ejecuta la función de setup inicial del contrato.

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  --source $SECRET_KEY \
  --initialize
```

### 4️⃣ Generar Bindings

Crea el package TypeScript con types e interfaces automáticas para tu contrato.

```bash
stellar contract bindings typescript \
  --network testnet \
  --contract-id $CONTRACT_ID \
  --output-dir packages/voting
  
cd packages/voting && npm install
```

---

## 💡 Pro Tip

Agrega el `CONTRACT_ID` a tus variables de entorno:

```bash
NEXT_PUBLIC_VOTING_BINDING=packages/voting
```

Así el frontend puede importar dinámicamente el binding correcto.

---

## Optimización y Performance

### ⚡ Estrategias de Caché

Implementa un sistema de caché en memoria con TTL configurable para evitar llamadas RPC costosas.

**Ideal para:** Datos que no cambian con frecuencia.

```javascript
const cached = getCached(key);
if (cached) return cached;

const data = await fetchContract();
setCached(key, data, 30000); // 30s TTL
```

### 🧠 Optimistic Updates

Actualiza la UI de inmediato mientras la transacción se procesa en blockchain.

Si la transacción falla, se revierte el cambio.

```javascript
// Actualización optimista
setVotes(prev => prev + 1);

try {
  await contractVote(id); // Confirmar con data real
} catch {
  setVotes(prev => prev - 1); // Revertir si falla
}
```

### 🔄 Connection Pooling

Reutiliza conexiones RPC ya existentes en lugar de crear nuevas.

### ⌨️ Debouncing de Inputs

Evita llamadas innecesarias mientras el usuario escribe.

El hook `useDebounce` espera unos milisegundos antes de ejecutar búsquedas o transacciones.

---

## Tips Profesionales y Debugging

### 🧪 Testing Exhaustivo

Escribe tests unitarios para tus contratos Rust usando `cargo test`, y tests de integración para tus hooks con React Testing Library.

**Simula:** Wallets y RPC responses para obtener tests determinísticos.

### 🛡️ Error Boundaries

Implementa `React.ErrorBoundary` para capturar errores de UI y mostrar mensajes de fallback elegantes.

**En producción:** Envía errores a sistemas como Sentry o New Relic.

### 🧾 Logging Estratégico

Usa logs con niveles (`debug`, `info`, `warn`, `error`).

**En desarrollo:** Logea todo  
**En producción:** Solo errores y advertencias

Centraliza tus logs con servicios como LogRocket o Datadog.

### 🔐 Variables de Entorno

Valida las env vars al inicio de la app.

- Usa TypeScript types para evitar errores
- Nunca comitees secrets
- Separa archivos `.env` para dev, staging y prod

### ⚙️ Performance Monitoring

Mide métricas críticas:
- Tiempo de respuesta de contratos
- Render time de componentes
- Tiempos de carga

Usa dashboards de datos para detectar cuellos de botella y optimizar.

### 🔁 Circuit Breakers

Protege tu app de servicios externos caídos.

Si un endpoint falla varias veces, el circuit breaker lo marca como "open" y pausa las llamadas hasta que el servicio se recupere.

---

## Lo que Has Aprendido

✅ Arquitectura completa de dApps en Stellar / Soroban  
✅ Setup profesional de entorno de desarrollo  
✅ Flujo de datos: desde smart contracts hasta la UI  
✅ Creación y deploy de contratos personalizados  
✅ Estrategias de optimización y performance  
✅ Técnicas avanzadas de debugging y buenas prácticas

---

## Próximos Pasos

1. Practica con el contrato de votación del tutorial
2. Adapta tu smart contract
3. Implementa optimizaciones de performance
4. ¡Despliega tu dApp a producción!

---

## Recursos Útiles

🟣 **Stellar Docs** – Documentación oficial  
💬 **Discord** – Comunidad global  
🌟 **Stellar Quest** – Tutoriales interactivos  
🧪 **Stellar Laboratory** – Playground web  
💻 **GitHub Examples** – Código de referencia

---

## Resumen: ¡Estás Lista para Construir!

Has completado el recorrido completo desde la configuración del entorno hasta el deployment de smart contracts.

Ahora tienes todas las herramientas y conocimientos para:
- Construir dApps profesionales en Stellar
- Optimizar para producción
- Debuggear eficientemente
- Escalar tu aplicación con confianza

---

**Curso:** Código Futura - Buen Día Builders  
**Clase:** 10 - Desarrollo de dApps en Stellar  
**Enfoque:** Arquitectura Fullstack y Smart Contracts