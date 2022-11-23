# JSONLang to WASM compiler
Compila JSONLang a archivos .wat (WASM text format) que luego se pueden convertir a WASM con wat2wasm o similares.

## Dependencias
cargo, [wabt](https://github.com/WebAssembly/wabt), node.js
## Ejecucion
`cargo run entrada.jsonlang salida.wat`
## Testing
Ejecutar compiler_integration.test.js en el directorio test.

## Preguntas
### ¿Cómo se traduce el if a esta plataforma o VM?
El if se traduce como la instruccion de control if-else-end
### ¿Cómo se traduce el while a esta plataforma o VM?
El while se traduce como un bloque que contiene un loop que contiene un if(condicion) cuerpo else break.
El break y el continue son implementados como branches relativos llevando al final del bloque (afuera del loop) o al comienzo del loop respectivamente.
### ¿Cómo se traduce call a esta plataforma o VM?
Se traduce como la instruccion call.
### ¿Cómo se traduce return a esta plataforma o VM?
Se traduce como la instruccion return.
### ¿Cómo se traduce DeclarationStatement (declaración de funciones) a esta plataforma o VM?
Se analiza la cantidad parametros y se agrega un tipo funcion con esta firma.
Para una funcion de un parametro esto es `(type (func (param f64) (result f64)))`
Luego se analiza la cantidad de variables locales y se declara el cuerpo de la funcion con el tipo encontrado y la cantidad de variables locales correcta.
Continuando el ejemplo anterior, si la funcion tiene una local se agrega `(func (type 0) (param f64) (result f64)(local f64) cuerpo)`

### ¿Cómo se traducen las expresiones a esta plataforma o VM?
WASM utiliza una maquina de pila por lo que el paso principal de la traduccion es aplanar al arbol, colocando los argumentos y luego la operacion.
Un detalle importante al hacer esto es que los tipos de las variables coincidan.
Por ejemplo no es valido hacer la negacion bitwise de un flotante, por lo cual primero es necesario convertirlo a entero.
Tampoco es valido usar un flotante como condicion de un if (debe ser un entero).
Para solucionar esto se analiza los tipos que requiere cada expresion y se agregan conversiones de tipo donde sea necesario.
### Listar él o los links que resultaron más útiles para responder esas preguntas.
El estandard fue lo mas util al respecto https://www.w3.org/TR/wasm-core-1/