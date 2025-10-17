# Soroban Project - Hello Tiburona

## 📚 Contenido del Proyecto

Este repositorio documenta mi aprendizaje en desarrollo de contratos inteligentes con Soroban, incluyendo la implementación profesional de un contrato con manejo de errores, storage organizado y control de acceso.

## 🏗️ Estructura del Proyecto

```text
.
├── contracts
│   └── hello-tiburona
│       ├── src
│       │   └── lib.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## 🚀 Comandos Básicos

### Crear un nuevo contrato

```bash
soroban contract init hello-tiburona
```

### Compilar el contrato

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Optimizar el WASM

```bash
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/hello_tiburona.wasm
```

### Ejecutar tests

```bash
cargo test
```

## 🔧 Errores Encontrados y Soluciones

### Error 1: Método `to_string()` no encontrado en `Symbol`

**Problema:**

```rust
let nombre_str = nombre.to_string();
// error[E0599]: no method named `to_string` found for struct `soroban_sdk::Symbol`
```

**Causa:** El trait `ToString` no está en el scope para el tipo `Symbol` de Soroban.

**Solución:** Usar métodos específicos de Soroban SDK para trabajar con `Symbol`.

---

### Error 2: Global memory allocator no encontrado

**Problema:**

```
error: no global memory allocator found but one is required
```

**Solución:**

1. Agregar en `lib.rs`:

```rust
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

2. Agregar en `Cargo.toml`:

```toml
[dependencies]
wee_alloc = "0.4"
```

## 📖 Conceptos Aprendidos

### 1. Traits en Rust

Los **traits** son como un "idioma universal" en Rust - funcionan como contratos de comportamiento que definen funciones reutilizables.

**Analogía:** Son como componentes en React, proporcionan funcionalidad que puede reutilizarse en diferentes contextos.

**Características:**

- Usan nombres consistentes por convención
- Permiten polimorfismo
- Definen comportamientos compartidos

**Ejemplo - Patrón Ownable:**
Un patrón común que protege el contrato asegurando que solo el propietario pueda ejecutar ciertas funciones críticas.

---

### 2. Manejo de Errores: Result y Option

En Soroban es fundamental **controlar los panics** para evitar fallos inesperados.

#### Option<T>

Representa un valor que **puede no existir**.

**Retorna:**

- `Some(T)` → El valor existe
- `None` → El valor no existe

**Métodos útiles:**

- `unwrap_or(default)` → Si es None, devuelve el valor por defecto
- `map(fn)` → Transforma el valor si existe
- `and_then(fn)` → Encadena operaciones
- `ok_or(error)` → Convierte Option en Result

**Ejemplo:**

```rust
pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<Symbol> {
    env.storage()
        .persistent()
        .get(&DataKey::UltimoSaludo(usuario))
    // Puede retornar Some(nombre) o None si nunca saludó
}
```

#### Result<T, E>

Representa una operación que **puede fallar**.

**Retorna:**

- `Ok(T)` → Operación exitosa con valor
- `Err(E)` → Error específico

**Ejemplo:**

```rust
pub fn hello(env: Env, usuario: Address, nombre: Symbol) -> Result<Symbol, Error> {
    if nombre_str.len() == 0 {
        return Err(Error::NombreVacio);
    }
    Ok(Symbol::new(&env, "Hola"))
}
```

#### El operador `?`

Operador mágico que simplifica el manejo de errores.

**Qué hace:**

1. Evalúa la expresión
2. Si es `Ok(T)` → Devuelve el valor y continúa
3. Si es `Err(E)` → Sale inmediatamente con ese error

**Ejemplo:**

```rust
let admin: Address = env.storage()
    .instance()
    .get(&DataKey::Admin)
    .ok_or(Error::NoInicializado)?;
// Si get() retorna None, se convierte en Err y la función termina
```

---

### 3. Storage Patterns

El **storage** es como una base de datos distribuida en la blockchain.

#### Tres Tipos de Storage

**1. Instance Storage**

- **Uso:** Configuración global del contrato
- **Ejemplo:** Admin, contador total, configuraciones
- **Características:** Datos compartidos por todo el contrato

```rust
env.storage().instance().set(&DataKey::Admin, &admin);
```

**2. Persistent Storage**

- **Uso:** Datos críticos de usuario que deben perdurar
- **Ejemplo:** Último saludo por usuario, balances, ownership
- **Características:** Datos importantes que no deben perderse

```rust
env.storage()
    .persistent()
    .set(&DataKey::UltimoSaludo(usuario), &nombre);
```

**3. Temporary Storage**

- **Uso:** Datos temporales que pueden regenerarse
- **Ejemplo:** Cachés, datos calculados, información transitoria
- **Características:** Más económico, pero puede expirar

#### TTL (Time To Live)

El **TTL** es el tiempo de vida de los datos en la blockchain antes de expirar permanentemente.

**Sintaxis:**

```rust
env.storage().instance().extend_ttl(100, 100);
//                                   ↑    ↑
//                                   |    └─ Extensión: añade 100 ledgers más
//                                   └────── Umbral: cuando quedan 100 ledgers
```

**Interpretación:** "Cuando queden 100 ledgers de vida, extiende por 100 ledgers más"

**Importante:** Cada tipo de storage funciona diferente con el TTL y tiene costos distintos.

---

### 4. Enum DataKey - Práctica Esencial

Usar un **enum para las keys del storage** es una práctica fundamental en Soroban.

**Ventajas:**

- ✅ Organización clara de todas las keys
- ✅ Type safety (el compilador verifica los tipos)
- ✅ Evita errores de typos en strings
- ✅ Documentación automática de qué se guarda

**Ejemplo:**

```rust
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,                      // Sin parámetros - dato único
    ContadorSaludos,           // Sin parámetros - dato único
    UltimoSaludo(Address),     // Con parámetro - dato por usuario
}
```

---

### 5. Optimización: Validaciones y Gas

**Principio clave:** Validar lo barato primero.

**Objetivo:** Ahorrar gas fallando rápido en validaciones simples antes de operaciones costosas.

**Ejemplo de orden óptimo:**

```rust
pub fn hello(env: Env, usuario: Address, nombre: Symbol) -> Result<Symbol, Error> {
    // 1. Validación barata: revisar longitud (solo memoria)
    let nombre_str = nombre.to_string();
    if nombre_str.len() == 0 {
        return Err(Error::NombreVacio);  // Sale rápido
    }

    // 2. Validación barata: revisar límite (solo memoria)
    if nombre_str.len() > 32 {
        return Err(Error::NombreMuyLargo);  // Sale rápido
    }

    // 3. Operación costosa: leer storage (requiere I/O)
    let contador: u32 = env.storage()
        .instance()
        .get(&key_contador)
        .unwrap_or(0);

    // 4. Operación costosa: escribir en storage
    env.storage().instance().set(&key_contador, &(contador + 1));

    Ok(Symbol::new(&env, "Hola"))
}
```

**Lógica:** Si el nombre está vacío o es muy largo, fallamos sin tocar storage, ahorrando gas.

---

## 📐 Arquitectura del Contrato

### Estructura Base

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address
};

// 1. Definir errores personalizados
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

// 2. Definir keys de storage
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}

// 3. Definir el contrato
#[contract]
pub struct HelloContract;

// 4. Implementar funciones
#[contractimpl]
impl HelloContract {
    // Funciones aquí
}
```

### Funciones Implementadas

#### 1. `initialize()` - Inicialización del contrato

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

- Verifica que no esté ya inicializado
- Guarda el admin
- Inicializa el contador en 0
- Extiende TTL

#### 2. `hello()` - Función principal

```rust
pub fn hello(env: Env, usuario: Address, nombre: Symbol) -> Result<Symbol, Error>
```

- Valida que el nombre no esté vacío
- Valida que el nombre no sea muy largo (>32 chars)
- Incrementa el contador global
- Guarda el último saludo del usuario
- Extiende TTL
- Retorna "Hola"

#### 3. `get_contador()` - Consulta de contador

```rust
pub fn get_contador(env: Env) -> u32
```

- Retorna el número total de saludos
- Nunca falla (retorna 0 si no existe)

#### 4. `get_ultimo_saludo()` - Consulta de último saludo

```rust
pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<Symbol>
```

- Retorna el último nombre usado por el usuario
- Puede retornar `None` si nunca saludó

#### 5. `reset_contador()` - Función administrativa

```rust
pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error>
```

- Verifica que el caller sea el admin
- Resetea el contador a 0
- Control de acceso: solo admin puede ejecutar

---

## 🧪 Tests Implementados

### Suite de 6 Tests

1. **test_initialize** - Verifica inicialización correcta
2. **test_no_reinicializar** - Asegura que no se pueda inicializar dos veces
3. **test_hello_exitoso** - Prueba flujo completo de hello()
4. **test_nombre_vacio** - Valida error cuando nombre está vacío
5. **test_reset_solo_admin** - Verifica que admin puede resetear
6. **test_reset_no_autorizado** - Verifica que no-admin no puede resetear

---

## 💡 Mejores Prácticas Aprendidas

### 1. Manejo de Errores

- ✅ Definir errores específicos con `contracterror`
- ✅ Usar `Result<T, Error>` para operaciones que pueden fallar
- ✅ Usar `Option<T>` para valores que pueden no existir
- ✅ Aprovechar el operador `?` para código más limpio

### 2. Storage

- ✅ Usar `DataKey` enum para organizar las keys
- ✅ Elegir el tipo de storage apropiado (Instance vs Persistent vs Temporary)
- ✅ Siempre extender TTL después de operaciones importantes
- ✅ Usar `has()` en lugar de `get()` cuando solo necesitas verificar existencia

### 3. Validaciones

- ✅ Validar inputs ANTES de tocar storage
- ✅ Validar lo barato primero (fail fast)
- ✅ Usar early returns para salir rápido en caso de error

### 4. Control de Acceso

- ✅ Implementar patrón de ownership/admin
- ✅ Verificar permisos antes de operaciones críticas
- ✅ Usar funciones de inicialización para configurar admin

### 5. Testing

- ✅ Escribir tests para flujos exitosos
- ✅ Escribir tests para cada tipo de error
- ✅ Usar `#[should_panic]` para tests de errores esperados
- ✅ Verificar estado después de cada operación

---

## 🎯 Diferencias Clave: Storage Types

| Característica  | Instance        | Persistent          | Temporary         |
| --------------- | --------------- | ------------------- | ----------------- |
| **Uso**         | Config global   | Datos críticos      | Datos temporales  |
| **Costo**       | Medio           | Alto                | Bajo              |
| **Durabilidad** | Alta            | Muy alta            | Baja              |
| **Ejemplo**     | Admin, contador | Balances, ownership | Cachés            |
| **TTL**         | Debe extenderse | Debe extenderse     | Expira más rápido |

---

## 🚀 Próximos Pasos

### Retos Adicionales

1. **Estadísticas por usuario** - Rastrear cuántas veces cada usuario ha saludado
2. **Transfer admin** - Implementar función para transferir ownership
3. **Límite configurable** - Hacer que el límite de caracteres sea modificable

---

## 📚 Recursos

- [Documentación oficial de Soroban](https://soroban.stellar.org/docs)
- [Soroban Examples en GitHub](https://github.com/stellar/soroban-examples)
- [Manejo de errores en Soroban](https://soroban.stellar.org/docs/learn/errors)
- [Storage en Soroban](https://soroban.stellar.org/docs/learn/storage)

---

## 🦈 Notas Finales

Este proyecto representa mi primera implementación profesional de un contrato inteligente en Soroban, aplicando:

- ✅ Manejo de errores profesional
- ✅ Storage organizado y eficiente
- ✅ Validaciones robustas
- ✅ Control de acceso seguro
- ✅ Tests comprehensivos
- ✅ Optimización de gas

**Lección más importante:** El orden importa. Validar barato primero, tocar storage después.
