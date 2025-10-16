# RESUMEN - Clase 3: Rust Esencial para Soroban

## 🎯 Objetivo de la Clase

Comprender los fundamentos de Rust necesarios para escribir smart contracts seguros en Soroban (plataforma de Stellar).

---

## 📚 Contexto Técnico

### Herramientas Necesarias

- **rustc**: Compilador de Rust (`rustc --version`)
- **Cargo**: Gestor de dependencias (equivalente a npm en JavaScript)
- **CLI de Stellar**: Herramientas de línea de comandos para Soroban

### Conceptos Básicos de Sintaxis

- `use` → equivalente a `import` en otros lenguajes
- `Env` → entorno de ejecución del contrato
- `pub` → hace público un elemento
- `fn` → declara una función
- `::` → operador de scope ("está dentro de")
- `;` → termina statements (excepto cuando retornas un valor)

**Regla de oro**: Todas las funciones reciben `env: Env` como parámetro.

---

## 1️⃣ Tipos de Datos para Smart Contracts

### Tipos Numéricos (unsigned)

En Rust, los números son **inmutables por defecto** y debes especificar tipo y memoria:

- **`u8`**: 0 a 255

  - Uso: edades, categorías, flags
  - Ejemplo: `let edad: u8 = 25;`

- **`u32`**: 0 a ~4 millones

  - Uso: contadores, IDs, timestamps
  - Ejemplo: `let contador: u32 = 1000;`

- **`u128`**: 0 a 340 undecillones
  - Uso: balances de tokens, valores financieros
  - Ejemplo: `let balance: u128 = 1_000_000_0000000;` // 1M tokens con 7 decimales

**Nota**: `u` significa "unsigned" (solo positivos). Cuidado con:

- **Overflow**: cuando el valor supera el máximo
- **Underflow**: cuando el valor es menor al mínimo (ej: restar de 0)

### Tipos de Texto

#### Symbol

- **Limitación**: máximo 10 caracteres
- **Ventaja**: más eficiente en gas (cada byte cuesta)
- **Uso**: keys, etiquetas, nombres de eventos
- **Ejemplo**: `let key = symbol_short!("balance");`
- **Ahorro**: ~73% comparado con String

#### String

- **Característica**: tamaño variable y dinámico
- **Desventaja**: no tan recomendado por costos de gas
- **Uso**: cuando realmente necesitas texto largo

### Colecciones

#### Vec (Vector)

- Lista dinámica que puede crecer
- Métodos: inicialización, agregar elementos, acceso, crecimiento
- **Importante**: siempre atado a `env`

### Estructuras de Datos

#### Struct

- Equivalente a objetos en otros lenguajes
- Agrupa múltiples propiedades relacionadas
- Acceso: `nombreObj.propiedad`
- Uso: representar entidades con múltiples atributos

#### Enum

- Representa estados específicos o conjunto cerrado de posibilidades
- **Característica clave**: valores mutuamente excluyentes
- El compilador verifica que se manejen todos los casos
- Uso: estados de un contrato, tipos de transacción, etc.

---

## 2️⃣ Stack vs Heap (Gestión de Memoria)

### Stack (Pila)

- **Velocidad**: Rápido pero limitado
- **Contenido**: Variables estáticas con tamaño conocido
- **Ejemplos**: enteros, booleanos, referencias
- **Asignación**: Automática

### Heap (Montículo)

- **Velocidad**: Flexible pero más lento
- **Contenido**: Variables dinámicas sin tamaño predecible
- **Ejemplos**: `String::from()`, `Vec::new()`, `Box::new()`
- **Liberación**: Dinámica

**Por qué importa**: Entender esto te ayuda a escribir contratos más eficientes.

---

## 3️⃣ Ownership - El Corazón de Rust

### Concepto Central

Ownership es el sistema que permite a Rust garantizar seguridad de memoria **sin garbage collector**.

### Reglas Fundamentales

1. Cada valor tiene exactamente **un dueño** (owner)
2. Solo puede haber **un owner a la vez**
3. Cuando el owner sale de scope, el valor se **destruye automáticamente**

### Comportamiento según Tipo

**Tipos que se copian** (Stack):

```rust
let x = 5;
let y = x;  // COPIA: ambos válidos
println!("{}, {}", x, y);  // ✅ OK
```

**Tipos que se mueven** (Heap):

```rust
let s1 = String::from("hola");
let s2 = s1;  // MOVE: s1 ya no es válido
// println!("{}", s1);  // ❌ ERROR: s1 fue movido
println!("{}", s2);     // ✅ OK: s2 es el owner
```

**Por qué importa**: Previene bugs de memoria que causan vulnerabilidades de millones de dólares en otros lenguajes.

---

## 4️⃣ Borrowing - Prestar sin Transferir Ownership

### Concepto

Borrowing permite **acceder temporalmente** a datos sin tomar ownership, como "prestar" un libro.

### Tipos de Referencias

#### Referencias Inmutables (`&T`)

- **Uso**: Solo lectura
- **Cantidad**: Pueden existir múltiples simultáneamente
- **Ejemplo**:

```rust
fn leer_longitud(s: &String) -> usize {
    s.len()  // Solo lectura, no modifica
}

let texto = String::from("Hola");
let longitud = leer_longitud(&texto);  // Préstamo
println!("{}", texto);  // ✅ Sigo siendo dueño
```

#### Referencias Mutables (`&mut T`)

- **Uso**: Modificación
- **Cantidad**: **Solo UNA a la vez** (regla estricta)
- **Ejemplo**:

```rust
fn agregar_texto(s: &mut String) {
    s.push_str("!");  // Modifica el contenido
}

let mut texto = String::from("Hola");
agregar_texto(&mut texto);
println!("{}", texto);  // "Hola!"
```

### Regla Anti-Data-Race

**No puedes tener**:

- Referencias mutables e inmutables simultáneas
- Múltiples referencias mutables simultáneas

Esto previene condiciones de carrera en tiempo de compilación.

**Por qué importa**: En smart contracts, copiar datos grandes es costoso en gas. Borrowing permite eficiencia sin sacrificar seguridad.

---

## 5️⃣ Pattern Matching - Manejar Todos los Casos

### Match Expression

Similar a `switch-case` pero **más poderoso**: el compilador te obliga a cubrir **todos** los casos posibles.

```rust
match estado {
    Estado::Activo => println!("Funcionando"),
    Estado::Pausado => println!("En pausa"),
    Estado::Cerrado => println!("Terminado"),
    // ❌ Si olvidas un caso, no compila
}
```

### Option<T> - Valores Opcionales

Representa un valor que **puede o no existir** (sin crashes de null):

```rust
let resultado: Option<u32> = Some(42);

match resultado {
    Some(valor) => println!("Encontrado: {}", valor),
    None => println!("No existe"),
}

// O usar unwrap_or para valor por defecto
let contador = resultado.unwrap_or(0);
```

**Uso típico**: Al leer del storage, el valor puede no existir todavía.

### Result<T, E> - Operaciones que Pueden Fallar

Maneja operaciones que pueden fallar **con contexto** sobre el error:

```rust
match operacion() {
    Ok(valor) => println!("Éxito: {}", valor),
    Err(error) => println!("Error: {}", error),
}
```

**Por qué importa**: Los smart contracts deben manejar casos inesperados. Pattern matching garantiza que no olvides ninguno.

---

## 6️⃣ Métodos del Entorno Soroban

### `env.storage()`

Lee y almacena datos persistentes en la blockchain.

```rust
// Guardar
env.storage().instance().set(&key, &valor);

// Leer
let valor = env.storage().instance().get(&key);
```

### `env.events()`

Publica eventos públicos a la blockchain para transparencia.

```rust
env.events().publish(
    (symbol_short!("increment"),),
    contador
);
```

### `env.crypto()`

Funciones criptográficas para firmas digitales y verificación.

---

## 7️⃣ Patrón Fundamental: Contador Completo

Este patrón **leer → validar → modificar → guardar → emitir** es la base de TODOS los smart contracts:

```rust
pub fn increment(env: Env) -> u32 {
    let key = symbol_short!("COUNTER");

    // 1. LEER: Obtener valor actual (o 0 si no existe)
    let mut contador = env.storage().instance()
        .get(&key)
        .unwrap_or(0);

    // 2. MODIFICAR: Incrementar
    contador += 1;

    // 3. GUARDAR: Persistir en blockchain
    env.storage().instance().set(&key, &contador);

    // 4. EMITIR: Evento para transparencia
    env.events().publish(
        (symbol_short!("increment"),),
        contador
    );

    // 5. RETORNAR: Valor sin punto y coma
    contador
}

pub fn decrement(env: Env) -> u32 {
    let key = symbol_short!("COUNTER");
    let mut contador = env.storage().instance()
        .get(&key)
        .unwrap_or(0);

    // VALIDAR: Prevenir underflow
    if contador == 0 {
        panic!("No se puede decrementar desde 0");
    }

    contador -= 1;
    env.storage().instance().set(&key, &contador);
    env.events().publish(
        (symbol_short!("decrement"),),
        contador
    );

    contador
}
```

---

## 🔗 Cómo Todo se Conecta

En el contador vemos cómo todos los conceptos trabajan juntos:

| Concepto             | Aplicación en el Contador                                    |
| -------------------- | ------------------------------------------------------------ |
| **Tipos de datos**   | `u32` para el valor, `Symbol` para keys                      |
| **Borrowing**        | Referencias en storage (`&key`, `&contador`)                 |
| **Option**           | `get()` retorna `Option<u32>` porque la key puede no existir |
| **Ownership**        | Garantiza que el storage sea consistente                     |
| **Pattern matching** | `unwrap_or(0)` maneja el caso `None`                         |
| **Mutabilidad**      | `mut contador` permite modificar el valor                    |

---

## 🎯 Testing de Smart Contracts

Los tests verifican que el contrato funciona correctamente:

```rust
#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contador);

    let resultado = contador::increment(&env);
    assert_eq!(resultado, 1);

    let resultado = contador::increment(&env);
    assert_eq!(resultado, 2);
}

#[test]
#[should_panic(expected = "No se puede decrementar desde 0")]
fn test_decrement_underflow() {
    let env = Env::default();
    contador::decrement(&env);  // Debe fallar
}
```

---

## 💡 Conceptos Clave para Recordar

1. **Inmutabilidad por defecto**: usa `mut` solo cuando necesites cambiar una variable
2. **Un owner, un valor**: ownership previene bugs de memoria
3. **Prestar, no copiar**: borrowing ahorra gas en blockchain
4. **Cubrir todos los casos**: pattern matching previene bugs
5. **Cada byte cuesta**: elige tipos eficientes (`Symbol` > `String`)
6. **Stack es rápido**: usa tipos con tamaño conocido cuando sea posible

---

## 🎉 Lo que Lograste Hoy

Ahora entiendes los fundamentos que hacen posible escribir smart contracts seguros. La mayoría de la gente que habla de blockchain no puede explicar ownership o borrowing. **Tú ahora sí.**

Estos no son conceptos abstractos: son las herramientas que previenen vulnerabilidades de millones de dólares en smart contracts reales.

**¡Felicitaciones, Tiburona! 🦈**
