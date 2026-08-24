# RTML

## Todos:

[] No me gusta mucho la lógica del input, hay que refinarla.

[] Componente textarea.

[x] Estilos por defecto y que los componentes focusables tengan mínimo el estilo normal y otra para cuando tengan el foco.

[] Componente párrafo.

[x] Al definir estilos poder especificar de algún modo si el estilo es para focus u otros usos. Quizá poniendo el estilo y al final :focus, por ejemplo "a:focus".

[x] ¿Estilos para los layouts?, ¿color de fondo y quizá también bordes?. Ya tengo estilos para el layout, los bordes están en otro punto y hay que hacerlos para todos los elementos.

[x] Bordes para cualquier elemento. (Se crea un elemento border que puede contener cualquier otro elemento)

[] Que se puedan definir estilos teniendo en cuenta los elementos que contienen a otro. Igual que en css puedes hacer "p .red", quiere decir todos los elementos que estén dentro de un p y tengan la clase red.

[] Que se puedan definir estilos que tengan en cuenta varias propiedades, por ejemplo "a.blue", quiere decir todos los a que tengan la clase blue.

[x] Que se pueda definir código de programación en los xmls o en ficheros separados y que lo ejecute el intérprete que el usuario defina. Empezar soportando bash y nodejs. O quizá hacerlo genérico y que el usuario defina el comando del intérprete con sus argumentos.

[] Componente para tablas. 

[x] Componente botón

[x] Que los botones puedan ejecutar comandos

[x] Hacer que los comandos puedan cambiar el valor de un nodo o reemplazar el nodo entero. Si el comando se lanza desde un botón hay que poder definir si afecta a algún nodo y si el resultado debe reemplazar el valor del nodo o el nodo entero.

[x] Que los nodos tengan atributos data que se puedan utilizar para enviar a los comandos

[] Poder enviar el value de un nodo a los comandos

[] Tener un atributo data-from-node que pueda tener una lista de ids separados por coma. Cuando se ejecute un comando se le podrán pasar los data de todos los nodos especificados.

[] Que se puedan modificar los atributos data de un nodo desde la respuesta de un comando.

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

[x] Cuando haya un contexto poder utilizar algún tipo de sintaxis para coger el valor del contexto. Algo como {nombre} que se pueda utilizar en atributos, en textos y también de forma intercalada con el texto.

[x] Que pueda haber condicionales en el xml.

[x] Que pueda haber loops en el xml.

[] Que se puedan mostrar y ocultar nodos

[x] Hacer un dtd o xml schema para el xml

[x] El nodo command debería poder tener contenido. Ese contenido se usaría como plantilla para representar la salida del comando.

[] Que haya un evento para poder refrescar nodos command que no tengan el atributo refresh

[] Que los layouts puedan ser scrolleables si el contenido es mayor que ellos

[] Que se puedan concatenar comandos como en bash comando1 | comando2... La salida de uno será la entrada del otro. Habrá que ver como especificar si la salida se pone en una variable de entorno del siguiente comando o en un argumento.

[] Pasar métodos comunes de xml a xml_util

[x] Hay que revisar la implementación que permite reemplazar nodos desde un callback. Hay que poder utilizar templates.

[] sse para los command y que el comando se ejecute cada vez que llegue un evento. El evento hay que pasarlo como parámetro al comando. Quizá con un atributo refresh-sse.

[] Poder definir estilos directamente en los nodos con atributos fg, bg, etc...

[] Añadir eventos focus y blur para todos los nodos que sean focusables.

[] Hay que rerenderizar la aplicación cuando se redimensione la consola

[] Ventanas flotantes

[] Mensajes toast

[] Añadir más atributos en los estilos. Ahora los line y los span pueden tener padding left y right, estaría bien poder poner eso en los estilos.