# RTML

## Todos:

[] Componente textarea.

[x] Componente párrafo.

[] He hecho un componente párrafo pero hay que revisarlo. Se está creando el Paragraph de ratatui dos veces, una para calcular el número de líneas y otra para renderizarlo.

[] Que se puedan definir estilos que tengan en cuenta varias propiedades, por ejemplo "a.blue", quiere decir todos los a que tengan la clase blue.

[] Componente para tablas. 

[x] Poder enviar el value de un nodo a los comandos

[x] Tener un atributo data-from-node (le he puesto cdata) que pueda tener una lista de ids separados por coma. Cuando se ejecute un comando se le podrán pasar los data de todos los nodos especificados.

[] Que se puedan modificar los atributos data de un nodo desde la respuesta de un comando.

[x] Tener un atributo value-from-node (le he puesto cvalue) que pueda tener una lista de ids separados por coma. Cuando se ejecute un comando se le podrán pasar los values de todos los nodos especificados.

[] Componente formulario

[] Mover enum StyleSelector de xml a rtml. También la función merge_styles

[] También dar la opción de que los comandos puedan estar limitados al directorio root.

[] Ver si se pueden lanzar los comandos con timeout.

[] Poder reemplazar el value de un line. Seguramente habrá que hacerlo desde el doc para reemplazar el valor de su primer hijo y borrar el resto. Habrá que hacerlo así para que se conserven los estilos del span. Si no tiene ningún hijo se creará un nuevo span.

[] Cuando lanzo un comando desde un botón no estoy enviando el id del botón. Tengo que ver si aporta algo enviarlo.

[] Que se puedan mostrar y ocultar nodos

[x] Que haya un evento para poder refrescar nodos command que no tengan el atributo refresh

[] Que los layouts puedan ser scrolleables si el contenido es mayor que ellos

[] Pasar métodos comunes de xml a xml_util

[] sse para los command y que el comando se ejecute cada vez que llegue un evento. El evento hay que pasarlo como parámetro al comando. Quizá con un atributo refresh-sse.

[] Poder definir estilos directamente en los nodos con atributos fg, bg, etc...

[] Añadir eventos focus y blur para todos los nodos que sean focusables.

[] Hay que rerenderizar la aplicación cuando se redimensione la consola

[] Ventanas flotantes

[] Mensajes toast

[] Añadir más atributos en los estilos. Ahora los line y los span pueden tener padding left y right, estaría bien poder poner eso en los estilos. También pueden tener padding todos los elementos contenedores.

[x] Mejorar el atributo padding, cuando se ponga un solo número debe aplicarlo a los cuatro, etc... Ver doc de css https://developer.mozilla.org/es/docs/Web/CSS/Reference/Properties/padding

[] Añadir focus_style para el Paragraph

[] Añadir alignment para el Paragraph

[] Barra de scroll para el Paragraph

[x] Crear componente select

[] Barra de scroll para el select

[x] Añadir atributo enter-src 

[] Hay que hacer que un elemento pueda tener el foco al cargar la página