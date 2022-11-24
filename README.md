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
Ademas existe un [repo con ejemplos de uso de las instrucciones](https://github.com/WebAssembly/spec/tree/main/test/core) que sirvio,
en particular el ejemplo de la [funcion factorial](https://github.com/WebAssembly/spec/blob/main/test/core/fac.wast) escrita en
varios estilos que ilustra como organizar ciertas cosas.
Tambien revise el codigo de varias librerias, las cuales termine no usando por desacuerdos con el modelo que usaban.
### ¿Cómo implementarías arrays de largo fijo en este target?
WASM soporta bloques de memoria por lo cual se puede usar esto directamente en caso de que la cantidad de arreglos sea estatica
o bien armarme un heap e ir distribuyendo los arreglos en tiempo de compilacion.
### ¿Cómo implementarías una interfaz con la plataforma (uso de syscalls, librerías standard, etc) en este target?
Agregaria funciones que sean importadas (instruccion import), incluyendo la dependencia.
Un [ejemplo](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#web-assembly-text-example) de esto seria [WASI](https://wasi.dev/)
que permite realizar varias operaciones comunes como lectura de archivos.
### ¿Cuán facil fue aprender esta plataforma o VM? ¿Por qué?
Aprender el lenguaje tiene algunas complicaciones al principio por la formalidad del standard y ciertos nombres poco claros (ej "loop" no vuelve al principio por si solo, hay que hacerlo manualmente). Otro tema es referencias a acronimos antes definidos, ej inn.itestop se refiere a cosas como i32.eqz
Ver los ejemplos despeja bastante esas dudas.
### ¿Recomendarías esta plataforma o VM a futuros estudiantes de la materia? ¿Por qué?
La recomendaria, por su robustez, simplez y utilidad. Con una cantidad relativamente pequeña de instrucciones se puede armar programas que luego
pueden ser utilizados en todo tipo de contextos. Es bastante util que si tengo un algoritmo implementado en mi lenguaje pueda usarlo en otro lugar
casi sin dificultad.
### Liste ventajas y desventajas de trabajar en esta plataforma o VM.
Ventajas:
- El lenguaje soporta nativamente constuctos de alto nivel como funciones y estructuras de control. Esto simplifica trabajar con el codigo.
- Una vez que se tiene el archivo .wasm hay varias herramientas disponibles para analizarlo e incluso optimizarlo.
- Al tener verificacion de tipos hay varios errores que se detectan facilmente porque el interprete rechaza el programa.
  Esto es una gran ventaja comparado con ensambladores donde ciertos errores pueden pasar desapercibidos en ciertas condiciones y fallar catastroficamente en otras.
- Esta verificacion tambien garantiza por adelantado seguridad: el programa nunca puede acceder a memoria que no debe.
- Por esto los programas WASM se pueden ejecutar con poco costo extra en plataformas seguras como el buscador, sistemas de plugin o plataformas serverless
- Es muy facil interacturar entre el archivo compilado y otros lenguajes, por lo que es muy facil hacer pruebas y poner en uso el resultado.
  No habria problema que parte de mi programa este en jsonlang, una parte en rust y otra en pascal.
- Poca cantidad de instrucciones, por lo que es facil de entender
- Al ser tanto compilable como interpretable (JIT o estatico) se puede obtener rapidez, flexibildad o un intermedio segun convenga

Desventajas:
- Tener que respetar los tipos de los enteros requiere hacer analisis extra, ya que no todos los tipos se pueden usar para lo que sea.
- La interaccion con funciones externas tiene ciertas limitaciones, en particular que la funcion debe escribir sobre el espacio de memoria propio para
  devolver datos complejos. Esto se piensa arreglar en versiones futuras.