Apuntes de clase 

Construir un token en Stellar desde 0

### Estandar CAP-46 para crear token fungible 

- Define como deben comportarse los tokens en Stellar (Similar a ERC-20 Ethereum)
Tiene :
- inicialize
- mint & burn 
- transfer
- approve & transfer_from
- Funciones consulta: name, symbol, decimals, total_supply. balance, allowance 

### Dif entre tokens ERC-20 de Ethereum y token Stellar 

Token -> Representacion de valor digital que vive en Blockchain, tiene reglas para su uso 

### Seguridad en tokens

1) Autorización estricta -> requiere.auth()
2) Proteccion contra overflow -> checked_add y checked_sub
3) Gestión de TTL -> Extender en cada operación (datos importantes)
4) Eventos Ricos -> Detallar código para fácil entendimiento 

### Estructura proyecto

Igual a diapo 

Dependencias 
-> Sdk soroban
-> opt-level
-> overflow-checks
-> lto
-> 

Siempre comando ini -> stellar contact new token ...

Importante -> cada vez que modificamos el contrato cuesta plata yq ueda registrado. Siempre corroborar TODO 

Funcion mint -> crear un nuevo token (Solo Admin) 

### Test unitarios 

Garantizan calidad  

comando -> cargo test 