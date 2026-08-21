# RTML

## Todos:

[-] Extraer la lógica del input a un componente genérico que puedan utilizar otros. Seguramente sea buena idea hacer un textarea genérico y el input sería un caso específico de una sola fila. 
He hecho el componente pero de momento solo tengo el input, me falta crear el textarea y ver si se comporta bien.

[x] Estilos por defecto y que los componentes focusables tengan mínimo el estilo normal y otra para cuando tengan el foco.

[] Mejorar la implementación de los spans que están dentro del line. Seguramente sea mejor crear un componente genérico que se pueda utilizar tanto para líneas como para párrafos.

[] Hacer que los datos de los componentes puedan venir de algún fichero. Si es un pipe se puede tener un hilo escuchando que nos devuelva línea a línea y esa línea procesarla de alguna manera que actualice el valor de uno o varios inputs. Si es un fichero dar opción para que se lea solo una vez o cada x tiempo.

[] Igual que en el todo anterior pero con urls y también sse.

[] Que los lines y los links tengan como constraint por defecto algo relativo al texto que contienen. Mejor, que todos los elementos puedan tener constraints por defecto solo para ellos aunque quizá la mayoría tiren de la misma función. (Los links ya no lo necesitan, el estilo se lo he puesto al span y así no ocupa todo el tamaño del line).

[x] Al definir estilos poder especificar de algún modo si el estilo es para focus u otros usos. Quizá poniendo el estilo y al final :focus, por ejemplo "a:focus".

[x] ¿Estilos para los layouts?, ¿color de fondo y quizá también bordes?. Ya tengo estilos para el layout, los bordes están en otro punto y hay que hacerlos para todos los elementos.

[] Bordes para cualquier elemento

[] Que se puedan definir estilos teniendo en cuenta los elementos que contienen a otro. Igual que en css puedes hacer "p .red", quiere decir todos los elementos que estén dentro de un p y tengan la clase red.

[] Que se puedan definir estilos que tengan en cuenta varias propiedades, por ejemplo "a.blue", quiere decir todos los a que tengan la clase blue.

[x] Que se pueda definir código de programación en los xmls o en ficheros separados y que lo ejecute el intérprete que el usuario defina. Empezar soportando bash y nodejs. O quizá hacerlo genérico y que el usuario defina el comando del intérprete con sus argumentos.

[] Componente para tablas. 

[x] Componente botón

[x] Que los botones puedan ejecutar comandos

[x] Hacer que los comandos puedan cambiar el valor de un nodo o reemplazar el nodo entero. Si el comando se lanza desde un botón hay que poder definir si afecta a algún nodo y si el resultado debe reemplazar el valor del nodo o el nodo entero.

[x] Que los nodos tengan atributos data que se puedan utilizar para enviar a los comandos

[] Poder enviar el value de un nodo a los comandos

[] Poder recuperar información de la aplicación desde un comando

[] Tener un atributo data-from-node que pueda tener una lista de ids separados por coma. Cuando se ejecute un comando se le podrán pasar los data de todos los nodos especificados.

[] Tener un atributo value-from-node que pueda tener una lista de ids separados por coma. Cuando se ejecute un comando se le podrán pasar los values de todos los nodos especificados.

[] Componente formulario

[] Mover enum StyleSelector de xml a rtml

[x] Hay que afinar un poco más la detención de los comandos. Ahora solo se detienen cuando se carga un documento nuevo pero puede ocurrir que se reemplace un nodo o los hijos de un nodo y quizá algunos hijos sean comandos, en ese caso no se están deteniendo.
Por otro lado si cargas parte de un documento desde un comando tampoco se están iniciando los comandos de los nodos tipo comando.

[x] Hacer que la aplicación tenga un directorio root y que no pueda cargar xmls desde fuera de ese directorio. 

[] También dar la opción de que los comandos puedan estar limitados al directorio root.

[] Ver si se pueden lanzar los comandos con timeout.

[] Poder reemplazar el value de un line. Seguramente habrá que hacerlo desde el doc para reemplazar el valor de su primer hijo y borrar el resto. Habrá que hacerlo así para que se conserven los estilos del span. Si no tiene ningún hijo se creará un nuevo span.

[] Cuando lanzo un comando desde un botón no estoy enviando el id del botón. Tengo que ver si aporta algo enviarlo.

[] Que la aplicación pueda tener un contexto que se alimente con comandos ejecutados al inicio y que pueda cambiar con eventos.

[] Cuando haya un contexto poder utilizar algún tipo de sintaxis para coger el valor del contexto. Algo como {nombre} que se pueda utilizar en atributos, en textos y también de forma intercalada con el texto.

[] Que pueda haber condicionales en el xml.

[] Que pueda haber loops en el xml.

[] Que se puedan mostrar y ocultar nodos

[] Hacer un dtd o xml schema para el xml
