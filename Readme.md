# Otras herramientas para el manejo de errores

**Autor:** Kevin Alan Martinez Virgen - 219294382

**Clase:** Computación tolerante a fallas

A continuación se expondrán los mecanismos de manejo de errores de un lenguaje
de programación elegido, en particular, Rust. Rust es un caso interesante
porque no utiliza los métodos tradicionales de los otros lenguajes de
programación

## Manejo de errores en Rust

Los errores son parte escencial de la programación, ya que no siempre se tiene
el estado deseado y es importante reconocer esto para poder instalar sistemas
que permitan reconocer estos errores y tomar las acciones correspondientes.

Elegí Rust ya que me parece un caso bastante interesante de estudio, muchos de
los lenguajes, utilizan un sistema de errores similar (throw, try, catch).
Rust por otra parte, tiene un acercamiento diferente a los errores, no trata de
ignorar los errores, por el contrario, siempre intenta que el programador esté
conciente de ellos.

### Tipos de errores en Rust

Rust reconoce dos tipos de errores, los recuperables y los irrecuperrables.
Los recuperables son aquellos errores de los que el sistema puede elegir que hacer.
Por ejemplo, un archivo no encontrado se le notifica al usuario y puede volver
a intentarlo. Por otra parte, los errores irrecuperables, son aquellos que no 
dan oportunidad al programa de maniobrar, por ejemplo, acceder a una posición
inválida de un arreglo.

#### Errores irrecuperables

Estos son los primeros errores a cubrir, ya que son bastante similares a los
errores en otros lenguajes. A continuación se muestra la misma declaración
de una función en _python_ y _rust_ 

```py
def divide(a, b):
    if b == 0:
       rise Exception("Division by 0")
    return a/b
```

```rs
fn divide(a:f32, b:f32)->f32{
    if b == 0:
        panic!("Division by 0")
    return a/b
}
```

Ambas funciones realizan una división y comprueban que el divisor no sea 0
en caso de que sea 0, se denota un error. Si bien, ambos ejemplos pueden parecer
similares de forma superficial, no lo son. En caso de error, el código de _python_ 
devolverá a quien llamó la función una excepción, si esta no es "manejada" entonces
se irá pasando de función en función, hasta que alguien la maneje o llegue al
script principal y termine el programa.

La versión de _rust_ por otra parte, termina inmediatamente el programa al llegar 
a la instrucción panic, con el mensaje de error. No da oportunidad a las funciones
que la llamaron de procesarla. Si bien, esto parece inútil o cuando menos raro,
es parte de los beneficios de _rust_, ya que a diferencia de _python_, nos está
asegurando una cosa, esta función siempre retornará un flotante de 32 bits.

Sin embargo, habrá veces donde estaremos abiertos a una respuesta menos drástica.
Es decir, queremos manejar el error y muy probrablemente queremos informar al 
usuario de dicho error, usualmente para volverlo a intentar. Para estos casos 
tenemos

### Errores recuperables
Los errores recuperables, son la segunda clase de errores de _rust_ y son la
forma deseada de manejar errores en este lenguaje de programación. Esta sintaxis,
si bien existe en otros lenguajes, no es la opción por defecto. El ejemplo anterior
podemos recrearlo usando errores recuperables

```py
def divide(a, b):
    if b == 0:
       rise Exception("Division by 0")
    return a/b
```

```rs
fn divide(a:f32, b:f32)->Result(f32, String){
    if b == 0:
        Err("Division by 0")
    Ok(a/b)
}
```

Como podemos ver, hay dos cambios radicales en esta forma de manejar errores,
primero tenemos el tipo de valor de retorno de la función, cambió de un simple
flotante 32 a algo llamado _Result_ y tenemos ahora algo que parecen funciones
llamadas _Err_ y _Ok_

Antes de explicar que es _Result_, centremonos en porqué se cambió el tipo de 
retorno de la función. Como había mencionado anterior mente, _rust_ busca que 
los programadores estén concientes de los posibles errores que cada parte del
código pude producir y para esto utiliza simplemente el sistema de tipos.
Esto hace que aún sin saber el lenguaje, podemos entender que esta función va a 
devolver un Resultado, y que ese resultado puede ser un error (_Err_) o un valor
válido (_OK_)

#### El enum _Result_

_Result_ no es más que un enum, un tipo de dato que delimita un conjunto de opciones
a escoger y que solo puede ser una a la vez. En este caso las opciones de _Result__
son _Err_ para indicar un error y _Ok_ para indicar que todo está bien y el código
se ejecutó correctamente.

Otro factor interesante de _Result_ (y los enum de _rust_ en general) es que 
puden ser genéricos, es decir, utilizar diferentes tipos de datos, en el caso 
del ejemplo anterior, se indica que el resultado correcto es de tipo f32 (flotante)
y el error de tipo String. Lo que permite al que llamó la función no solo sabe
que tiene que manejar un posible error, si no de que tipo es cada posible resultado

```rs
fn divide(a:f32, b:f32)->Result(f32, String){
```

Los beneficios del enum _Result_ no acaban ahí, al ser un tipo estándar de _rust_
tiene métodos asociados que vuelven incluso más poderoso a este método de manejo
de errores

##### Match

El operador de match es uno de los más utilizados en _rust_ y es una muy buena
forma de manejar errores

```rs
match divide(0,2){
        Ok(res)->println!("El resultado de la operación es: {}", res),
        Err(e)->println!("Error! {}", e)
    }
```

Match ejecuta una instrucción u otra dependiendo del tipo de dato de retorno de
la función, en este caso imprimiría el error o el resultado

##### Unwrap

Match es excelente cuando se quiere realizar operaciones diferentes en caso de 
error u ok, sin embargo, existen "acortadores" que reducen el código necesario
en escenarios similares.

Unwrap devuelve el resultado si fue de tipo Ok, e invoca a panic (error irrecuperable) 
en caso de recibir un Err. Útil para rápido prototipado o si el no recibir un
valor adecuado representa irrecuperabilidad en el programa

```rs
let numeric_result = divide(0,2).unwrap();
```

Existe otro método similar, llamado expect, que permite enviar una cadena a modo
de mensaje de error, este método se llama expect

```rs
let numeric_result = divide(0,2).expect("Error al dividir");
```

##### Propagar el error. El operador '?'

El último patrón que tiene rust para manejar errores, es, similar a otros lenguajes, 
dejar que funciones superiores lo manejen, con la diferencia que _rust_ requiere
que el progamador lo haga de forma consciente, mediante el operador '?'.
Este operador provoca que, en caso de que una llamada a función retorne un Err,
este Err se retorne de inmediato.

Para poder utilizar este operador, la función que lo use deberá tambien tener 
como tipo de retorno un _Result_


### Bibliografía
[The Rust Programing Language](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
