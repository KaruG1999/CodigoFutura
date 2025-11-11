# Clase 2 - Prototipo, Pitch y Demo para Hackathon

## Introducción: De la Idea a lo Visible

Hoy convertimos todo lo planeado en algo **VISIBLE y PRESENTABLE**.

### Al final de esta clase tendrás:

✅ Un prototipo navegable (aunque sea básico)  
✅ La identidad visual de tu proyecto  
✅ Un pitch estructurado de 3-5 minutos  
✅ Una estrategia para la demo

---

## Wireframe y Prototipo

### ¿Qué es un Wireframe?

Es un **BOSQUEJO** de cómo se verá tu app. Es la estructura, el esqueleto, antes de agregar colores, imágenes y estilos.

### ¿Por qué hacer wireframes?

✅ Organizan las ideas visualmente  
✅ Permiten iterar rápido sin programar  
✅ Alinean al equipo sobre qué construir  
✅ Son la base para el desarrollo

> **Clave:** En una hackathon, el wireframe ES tu guía para dividir tareas.

### Wireframe ≠ Diseño Final

El wireframe NO tiene:
- Colores definitivos
- Imágenes reales
- Tipografías finales
- Microinteracciones

El wireframe SÍ tiene:
- Estructura de contenido
- Ubicación de elementos
- Flujo entre pantallas
- Jerarquía visual básica

---

## Anatomía de una Pantalla

### 📱 HEADER (Encabezado)

- Logo/nombre de la app
- Menú de navegación (si aplica)
- Botón de wallet/usuario

### 📦 MAIN CONTENT (Contenido principal)

- Título de la sección
- Formularios/inputs
- Botones de acción (Call to Action)
- Información clave

### 📊 FEEDBACK/STATUS

- Mensajes de error/éxito
- Loaders (mientras procesa)
- Confirmaciones

### 🔗 FOOTER (Pie de página, opcional)

- Links secundarios
- Información de contacto

> **💡 Para el MVP, enfócate en el MAIN CONTENT. El resto puede ser mínimo.**

---

## Las 3 Pantallas Mínimas

### PANTALLA 1: LANDING/HOME

**¿Qué es? ¿Para qué sirve?**

- Título claro
- Subtítulo explicativo (1-2 líneas)
- Call to Action principal (ej: "Conectar Wallet", "Empezar")
- (Opcional) Imagen hero o ilustración

### PANTALLA 2: INTERACCIÓN PRINCIPAL

**Donde pasa la magia**

- Formulario/inputs principales
- Botón de acción (ej: "Enviar", "Crear Token", "Comprar")
- Preview de lo que va a pasar
- Estado de wallet conectada

### PANTALLA 3: CONFIRMACIÓN/RESULTADO

**El 'wow moment'**

- Mensaje de éxito
- Información de la transacción
- Link al explorador de Stellar
- Próximas acciones (ej: "Enviar otra", "Ver historial")

---

## Wireframe: Papel vs. Digital

### ✏️ OPCIÓN A: Papel (5-10 minutos)

**Cómo hacerlo:**
- Dibuja las 3 pantallas en papel
- Usa rectángulos para imágenes
- Usa líneas para texto
- Marca los botones

**Ventajas:** Súper rápido, sin herramientas  
**Desventajas:** No se puede compartir fácil digitalmente

### 💻 OPCIÓN B: Digital en Figma

**Cómo hacerlo:**
- Usa herramientas como Figma, Balsamiq, o Slides
- Usa UI Kits prearmados (aceleran MUCHO)
- Conecta pantallas para simular navegación

**Ventajas:** Se puede iterar, compartir, conectar  
**Desventajas:** Curva de aprendizaje

---

## UI Kits: No Diseñes Desde Cero

### ¿Qué son los UI Kits?

Bibliotecas de componentes pre-diseñados: botones, formularios, cards, menús, etc.

### UI Kits Recomendados para Web3/Crypto

**En Figma:**
- "Web3 UI Kit" (buscar en Community)
- "Crypto Wallet UI Kit"
- "DeFi Dashboard UI Kit"
- UI Kits básicos de Material Design o iOS

**Dónde encontrarlos:**
- Figma Community (gratis)
- Freebies de Dribbble

### Componentes Clave que Necesitas:

- Botón "Connect Wallet"
- Card de transacción
- Inputs de dirección de wallet
- Inputs de monto (con selección de token)
- Estados de loading/success/error
- Modal de confirmación

---

## Figma Básico: 5 Pasos

### 1. Crear un Frame

```
Clic en Frame (F)
Elegir el tamaño:
👉 Desktop: 1440x1024
👉 Mobile: 375x812

Renombrar el frame: "Home", "Main", "Confirmación"
```

### 2. Agregar Elementos Básicos

- **Rectángulo (R):** cajas, botones, fondos
- **Texto (T):** títulos, subtítulos, etiquetas
- **Formas:** círculos o íconos para avatares

💡 **Tip:** Mantén todo simple, sin colores ni imágenes todavía.

### 3. Usar UI Kits

```
Ir a Resources (Shift + I)
Buscar "Web3 UI Kit"
Arrastrar componentes al frame
Cambiar textos y colores a gusto
```

### 4. Conectar Pantallas

```
Ir a pestaña Prototype
Seleccionar un botón → arrastrar el círculo azul al siguiente frame
En "Interaction": On Click → Navigate to [pantalla]
```

### 5. Probar el Prototipo

```
Clic en el botón ▶ (arriba a la derecha)
Probar hacer clic y navegar entre pantallas
✅ Listo: ya tienes un wireframe navegable
```

---

## Tips para Wireframes en Hackathon

💡 **No te preocupes por colores todavía**  
(pueden ser grises/blancos/negros)

💡 **Usa placeholder text**  
"Lorem ipsum" o "Título aquí"

💡 **Marca qué botón va a qué pantalla**

💡 **En Figma, conéctalos con Prototype**

---

## Identidad Visual Express

### Identidad Visual ≠ Branding Completo

En una hackathon, **NO necesitas** un manual de marca de 50 páginas.

### Necesitas Solo:

✅ Nombre claro y memorable  
✅ 2-3 colores consistentes  
✅ Logo simple (o al menos un ícono/símbolo)  
✅ Tagline de 1 línea

**Eso es TODO. Con eso ya te ves profesional.**

---

## El Nombre: 5 Estrategias

### 1. METÁFORA/ANALOGÍA
Basado en algo conocido

**Ejemplos:** Anchor (relacionado a Stellar), Bridge, Vault

### 2. DESCRIPTIVO
Dice qué hace

**Ejemplos:** SendFast, TokenizeIt, PayBridge

### 3. INVENTADO
Palabras nuevas

**Ejemplos:** Solara, Lumify, Credix

### 4. ACRÓNIMO
Siglas con significado

**Ejemplo:** SWIFT (Society for Worldwide Interbank Financial Telecommunication)

### 5. COMBINACIÓN
Stellar + concepto

**Ejemplos:** StellarPay, StellarBridge

---

## Los Colores: Psicología para Blockchain

### 🔵 AZUL
**Significado:** Confianza, seguridad, profesionalismo  
**Usado por:** Coinbase, PayPal, Stellar  
**Cuándo:** Apps financieras, pagos, institucionales

### 🟢 VERDE
**Significado:** Crecimiento, dinero, éxito, ecológico  
**Usado por:** Robinhood, Cash App  
**Cuándo:** Inversiones, remesas, impacto positivo

### 🟣 MORADO
**Significado:** Innovación, lujo, creatividad  
**Usado por:** Nubank, Twitch  
**Cuándo:** Productos disruptivos, para audiencia joven

### ⚫ NEGRO
**Significado:** Premium, elegancia, minimalismo  
**Usado por:** Apple Card, Revolut  
**Cuándo:** Productos sofisticados, audiencia tech-savvy

---

## Paleta de Colores Completa

Tu paleta debe incluir:

1. **1 color principal** (el que domina: botones, logo, identidad)
2. **1 color secundario** (acentos, highlights)
3. **Grises para textos y fondos** (negro puro cansa la vista)
4. **Color de éxito** (verde para confirmaciones)
5. **Color de error** (rojo para alertas)

### Herramientas:

- **Coolors.co** - Generador de paletas
- **Dribbble.com/colors** - Inspiración
- Copiar paletas de apps que admires

---

## El Logo: 3 Opciones Rápidas

### OPCIÓN 1: Logotipo (solo texto)

Nombre en una tipografía distintiva

**Ejemplo:** STELLAR (en Geometos)  
**Herramienta:** Google Fonts + Figma

### OPCIÓN 2: Ícono + texto

Un símbolo simple + nombre

**Ejemplo:** Círculo con inicial + nombre al lado  
**Herramienta:** Canva, Looka.com

### OPCIÓN 3: Símbolo abstracto

Forma geométrica relacionada al concepto

**Ejemplo:** Triángulos para "bridge", cadena para "link"  
**Herramienta:** Figma, Illustrator

---

## Tagline: La Frase que Explica Todo

### Fórmula:

```
{Verbo} + {Qué} + {Para quién/cómo}
```

### ✅ Ejemplos Buenos:

- "Enviá dinero a casa en segundos" (remesas)
- "Tokenizá facturas y liberá flujo de efectivo" (factoring)
- "Cobrá a nivel global, retiralo localmente" (freelancers)
- "Tus NFTs, tu prueba de certificación" (ownership)

---

## Recursos de Diseño

### Colores:
- coolors.co
- colormind.io

### Logo:
- looka.com
- canva.com

### Inspiración:
- dribbble.com
- pinterest.com

---

## El Pitch de Hackathon

### Pitch ≠ Presentación Corporativa

En una hackathon, el pitch es **RÁPIDO y DEMO-CÉNTRICO**.

**Duración típica:** 5-8 minutos

> **Clave:** El 60% del pitch es la DEMO. El resto es contexto.

---

## Estructura del Pitch Perfecto

### ⏱️ 30 segundos: PROBLEMA
Cuál es el problema que existe hoy

### ⏱️ 30 segundos: SOLUCIÓN + Por qué Stellar
Qué construiste y por qué usas Stellar

### ⏱️ 2-3 minutos: DEMO (la estrella)
Mostrar el producto funcionando

### ⏱️ 30 segundos: TECH STACK
Qué tecnologías usaron

### ⏱️ 30 segundos: VISIÓN/IMPACTO
Qué impacto puede tener a futuro

### (Opcional) Q&A
Responder preguntas del jurado

---

## Los 5 Errores Fatales del Pitch

### ❌ ERROR 1: Empezar con "Somos un equipo de..."
A nadie le importa (todavía). Empiecen con el PROBLEMA.

### ❌ ERROR 2: Explicar blockchain en lugar de mostrar valor
Digan "enviamos dinero en 5 segundos por $0.01", no "usamos tecnología blockchain descentralizada..."

### ❌ ERROR 3: Demo que falla
Tengan un **video backup**. SIEMPRE. Murphy's Law.

### ❌ ERROR 4: Leer slides con texto
Las slides son APOYO. Ustedes son la estrella.

### ❌ ERROR 5: Quedarse sin tiempo para la demo
La demo es el 60% del pitch. Si tienen que elegir, salten slides, **NUNCA salten la demo**.

---

## Ejemplo: Problema + Solución

### PROBLEMA (30 seg):

> "Hoy, trabajadores migrantes en USA envían $50B anuales a LATAM. Las remesas tradicionales cobran 8-15% y tardan 3-5 días. Western Union y bancos son lentos, caros e inaccesibles en áreas rurales."

### SOLUCIÓN (30 seg):

> "Creamos SendFast, una app móvil que permite enviar dinero internacionalmente en 5 segundos. Usando Stellar y USDC, eliminamos intermediarios y reducimos fees a $0.01. El resultado: familias reciben 15% más dinero, en minutos en lugar de días."

---

## Guía de Feedback

### ✅ Al dar feedback, enfócate en:

- "El problema quedó claro/no quedó claro porque..."
- "La demo mostró bien X, pero faltó mostrar Y"
- "El timing estuvo bien/se pasaron en X parte"
- "Sugerencia: podrían simplificar/agregar/cambiar..."

### ❌ NO des feedback de:

- Estilo personal ("habla más lento" solo si es grave)
- Detalles mínimos
- Cosas que no pueden cambiar en 24 horas

---

## Checklist Final: ¿Estás Lista para la Hackathon?

### DISEÑO:
□ POV claro y validado  
□ MVP definido (3 funcionalidades máximo)  
□ Taskflow documentado  
□ 3 pantallas wireframe (mínimo)  
□ Identidad visual (nombre + colores + logo + tagline)

### TÉCNICO:
□ Viabilidad técnica confirmada  
□ Stack definido  
□ Tareas divididas entre el equipo  
□ Repositorio creado

### PITCH:
□ Script de pitch estructurado  
□ Demo practicada (mínimo 3 veces)  
□ Video backup de la demo  
□ Slides (máximo 6)  
□ Cronometrado (3-5 minutos)

### ANTES DE LA HACKATHON:
□ Validar el problema con 2-3 usuarios reales  
□ Investigar competencia/proyectos similares  
□ Resolver dudas técnicas de Stellar  
□ Dormir bien la noche anterior 😴

---

## El Rol de la IA en Diseño de Producto

### Durante esta clase usamos IA para:

✅ Generar ideas de nombres  
✅ Refinar textos y taglines  
✅ Crear paletas de colores  
✅ Sugerir funcionalidades  
✅ Estructurar el pitch

### Pero la IA NO hizo:

❌ Validar si el problema es real  
❌ Definir el MVP (ustedes decidieron qué es prioritario)  
❌ Diseñar el flujo pensando en SUS usuarios  
❌ Practicar la demo  
❌ Tener criterio sobre qué funciona y qué no

---

## La Pregunta Ética

> **¿Dónde está la línea entre "usar IA como herramienta" y "dejar que la IA decida por nosotros"?**

### En diseño de producto, la IA puede:

✅ Acelerar tareas repetitivas  
✅ Dar opciones para elegir  
✅ Generar primeros drafts

### Pero USTEDES deben:

✅ Tener empatía real con usuarios  
✅ Validar las sugerencias  
✅ Decidir con criterio  
✅ Iterar basándose en feedback humano

---

## Regla de Oro:

> **Si no puedes explicar POR QUÉ tomaste una decisión de diseño, probablemente estás usando IA sin criterio.**

---

## Para Tener en Cuenta

En estas 2 clases hicimos un **sprint de diseño de producto**. Normalmente esto toma semanas.

### Lo importante NO es que tu diseño esté perfecto.

### Lo importante es que ahora tienes:

✅ Claridad sobre qué problema resuelves  
✅ Estructura para construir con foco  
✅ Herramientas para comunicar tu idea  
✅ Criterio para tomar decisiones de diseño

> **En la hackathon vas a improvisar, cambiar cosas e iterar. Eso es NORMAL.**

---

## Criterios de Evaluación Típicos

### 🎯 IMPACTO
¿Cuántas personas beneficia? ¿Qué tan significativo es el problema?

### 💡 INNOVACIÓN
¿Es una idea nueva o un enfoque único a un problema existente?

### ⚙️ IMPLEMENTACIÓN TÉCNICA
¿Qué tan bien usaron Stellar? ¿El código funciona?

### 🎨 DISEÑO Y UX
¿Es intuitivo? ¿Es accesible?

### 🎤 PRESENTACIÓN
¿Comunicaron claramente? ¿La demo fue convincente?

### 🏆 COMPLETITUD
¿Qué tan terminado está el MVP? ¿Funciona de punta a punta?

---

## 💡 Tips de Oro para Hackathons

✨ **No trates de ganar, trata de APRENDER y CONECTAR**

✨ **Un MVP simple y funcional > un proyecto ambicioso a medias**

✨ **La demo es el 50% de tu score, practícala**

✨ **Duerme** (al menos 4-6 horas en hackathons de 48h+)

✨ **Pide ayuda a mentores temprano**, no a las 2am del último día

✨ **Diviértete** - es una hackathon, no una auditoría 😊

---

## Resumen de la Clase

### ✅ Lo que logramos:

1. Creamos wireframes de las 3 pantallas mínimas
2. Definimos la identidad visual (nombre, colores, logo, tagline)
3. Estructuramos un pitch de 3-5 minutos
4. Entendimos la importancia de la demo
5. Reflexionamos sobre el uso ético de IA en diseño

---

**Curso:** Código Futura - Buen Día Builders  
**Clase:** 2 - Prototipo, Pitch y Demo para Hackathon  
**Enfoque:** Diseño Visual y Presentación

---

## ¡Mucha Suerte en la Hackathon! 🚀