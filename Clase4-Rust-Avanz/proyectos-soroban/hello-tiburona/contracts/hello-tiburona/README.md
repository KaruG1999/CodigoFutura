# 🦈 Hello Tiburona - Tarea Clase 4

## 📋 Resumen de la Implementación

Este proyecto implementa un contrato inteligente completo en Soroban con manejo profesional de errores, control de acceso, validaciones y gestión de storage.

### ✅ Funcionalidades Implementadas

**Core del Contrato:**
- ✅ Sistema de inicialización con verificación anti-doble-init
- ✅ Función `hello()` con validaciones de input (nombre vacío y longitud máxima)
- ✅ Contador global de saludos
- ✅ Almacenamiento de último saludo por usuario
- ✅ Control de acceso basado en admin
- ✅ Gestión correcta de TTL (instance y persistent storage)

**Retos Adicionales:**
- ✅ **Reto 1:** Estadísticas por usuario - cada Tiburona tiene su propio contador
- ✅ **Reto 2:** Transfer de admin - ownership transferible
- ✅ **Reto 3:** Límite configurable - el admin puede ajustar la longitud máxima de nombres

### 🔧 Funciones del Contrato

```rust
// Inicialización
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>

// Función principal
pub fn hello(env: Env, usuario: Address, nombre: String) -> Result<Symbol, Error>

// Consultas
pub fn get_contador(env: Env) -> u32
pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String>
pub fn get_contador_usuario(env: Env, usuario: Address) -> u32
pub fn get_limite(env: Env) -> u32

// Administrativas
pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error>
pub fn transfer_admin(env: Env, caller: Address, nuevo_admin: Address) -> Result<(), Error>
pub fn set_limite(env: Env, caller: Address, limite: u32) -> Result<(), Error>
```

### 🐛 Desafíos Principales

**1. Cambio de Symbol a String en `hello()`**

El mayor obstáculo fue trabajar con la versión actualizada de soroban-sdk (v23). Inicialmente el código usaba `Symbol` para el parámetro `nombre`, pero `Symbol` no tiene el método `.to_string()` ni `.len()` necesarios para las validaciones.

**Solución:** Cambiar el tipo del parámetro a `String`:
```rust
// ❌ Versión anterior (no funciona)
pub fn hello(env: Env, usuario: Address, nombre: Symbol) -> Result<Symbol, Error> {
    if nombre.to_string().len() == 0 {          // Symbol no tiene .to_string()!
```

```rust
// ✅ Versión correcta
pub fn hello(env: Env, usuario: Address, nombre: String) -> Result<Symbol, Error> {
    if nombre.len() == 0 {                       // String sí tiene .len() directo
```

**2. Deprecación de `register_contract()`**

En los tests, el método `.register_contract()` está deprecado en versiones recientes del SDK, pero aún funciona. Se mantuvo para compatibilidad con la guía de la clase.

### 🧪 Tests Implementados

Se implementaron 6 tests comprehensivos que cubren:

1. ✅ Inicialización exitosa
2. ✅ Prevención de doble inicialización
3. ✅ Saludo exitoso con validaciones
4. ✅ Rechazo de nombre vacío
5. ✅ Reset de contador solo por admin
6. ✅ Prevención de reset por usuarios no autorizados

---

## 🚀 Instrucciones para Ejecutar

### Prerequisitos

```bash
# Verificar instalaciones
rustc --version
soroban --version
```

### 1. Clonar y Preparar

```bash
# Navegar al directorio del proyecto
cd hello-tiburona

# Verificar estructura
ls -la
```

### 2. Compilar el Contrato

```bash
# Build estándar
cargo build --target wasm32-unknown-unknown --release

# O usar el comando de Soroban
soroban contract build
```

### 3. Ejecutar Tests

```bash
# Correr todos los tests
cargo test

# Correr tests con output detallado
cargo test -- --nocapture

# Correr un test específico
cargo test test_hello_exitoso
```

### 4. Optimizar WASM (Opcional)

```bash
# Optimizar el archivo WASM para deploy
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/hello_tiburona.wasm
```

### 5. Verificar el Build Final

```bash
# Ver el tamaño del WASM generado
ls -lh target/wasm32-unknown-unknown/release/*.wasm
```

---

## 📸 Resultados

[Compilación lib.rs](./img/compilacionLib.jpeg)

### Build Final

**Screenshot de la compilación exitosa:**

```
[Pendiente -> Insertar aquí captura de pantalla del output de: soroban contract build]

Debería mostrar:
✅ Compiling hello-tiburona v0.0.0
   Finished release [optimized] target(s) in X.XXs
```

---

### Tests Pasando

**Screenshot de todos los tests exitosos:**

```
[Pendiente -> Insertar aquí captura de pantalla del output de: cargo test]

Debería mostrar:
running 6 tests
test test::test_hello_exitoso ... ok
test test::test_initialize ... ok
test test::test_no_reinicializar ... ok
test test::test_nombre_vacio ... ok
test test::test_reset_solo_admin ... ok
test test::test_reset_no_autorizado ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

---

## 🎓 Aprendizajes Clave

1. **Manejo de Errores:** Uso de `Result<T, Error>` y el operador `?` para propagación elegante de errores

2. **Storage Organizado:** Distinción clara entre `instance` (datos globales del contrato) y `persistent` (datos específicos de usuario)

3. **Validaciones Profesionales:** Siempre validar inputs ANTES de modificar el estado del contrato

4. **Control de Acceso:** Implementación de roles (admin) con verificación de permisos

5. **Types en Soroban:** Entender cuándo usar `String` vs `Symbol` - `String` para datos con validaciones, `Symbol` para identificadores fijos

6. **TTL Management:** Extender el tiempo de vida del storage de manera apropiada

---

## 📚 Estructura del Código

```
hello-tiburona/
├── contracts/
│   └── hello-tiburona/
│       ├── src/
│       │   └── lib.rs          # Contrato completo con retos
│       └── Cargo.toml
├── Cargo.toml
└── README.md                    # Este archivo
```

---

## 🔍 Recursos Adicionales

- [Documentación Oficial de Soroban](https://soroban.stellar.org/docs)
- [Soroban SDK Reference](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Ejemplos Oficiales](https://github.com/stellar/soroban-examples)

---

## 👩‍💻 Autora

Tarea completada como parte de la Clase 4 - Construyendo Contratos Profesionales

**Desafíos superados:**
- ✅ Adaptación a soroban-sdk v23
- ✅ Implementación de los 3 retos adicionales
- ✅ Tests comprehensivos con casos edge
- ✅ Storage management correcto

---

**Nota:** Este contrato es production-ready con manejo profesional de errores, validaciones completas y control de acceso robusto. 🦈