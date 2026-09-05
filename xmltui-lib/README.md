# RTML

## Todos:

[] Componente textarea.

[] He hecho un componente párrafo pero hay que revisarlo. Se está creando el Paragraph de ratatui dos veces, una para calcular el número de líneas y otra para renderizarlo.

[] Que se puedan definir estilos que tengan en cuenta varias propiedades, por ejemplo "a.blue", quiere decir todos los a que tengan la clase blue.

[] Componente para tablas. 

[] Componente formulario

[] Mover enum StyleSelector de xml a rtml. También la función merge_styles

[] También dar la opción de que los comandos puedan estar limitados al directorio root.

[] Ver si se pueden lanzar los comandos con timeout.

[] Cuando lanzo un comando desde un botón no estoy enviando el id del botón. Tengo que ver si aporta algo enviarlo.

[] Que se puedan mostrar y ocultar nodos

[] Que los layouts puedan ser scrolleables si el contenido es mayor que ellos

[] Pasar métodos comunes de xml a xml_util

[] sse para los command y que el comando se ejecute cada vez que llegue un evento. El evento hay que pasarlo como parámetro al comando. Quizá con un atributo refresh-sse.

[] Añadir eventos focus y blur para todos los nodos que sean focusables.

[] Hay que rerenderizar la aplicación cuando se redimensione la consola

[] Ventanas flotantes

[] Mensajes toast

[] Barra de scroll para el Paragraph

[] Barra de scroll para el select

[] Al deserializar los estilos tener cuidado con el padding, ahora si pones un único atributo vertical u horizontal te mapea los dos. Quiza haya que crear un XMLHorizontalPadding y vertical con los atributos opcionales.

[] Revisar la lógica del padding cuando se ponen solo dos números. Como algunos elementos pueden tener solo padding horizontal quizá haya que cambiar el orden cuando se ponen solo 2 números y hacer que el primer y segundo número sean right y left.

[] Revisar los atributos de border, ahí hay más cosas que se deberían poder meter en los estilos.

[] Tener un estado. Que los componentes que tengan valores que puedan ser cambiados por el usuario se puedan sincronizar con el estado. Que el valor de los atributos (y quizá también el contenido) de los componentes puedan recuperarse del estado.

[] Que todos (o la mayoría) de los atributos de los componentes puedan ejecutar templates para calcular su valor. A esos templates hay que pasarles el estado.

[x] Poder inicializar el estado.

[x] En la respuesta de los comandos también se tiene que poder acceder al estado.

[x] En los eventos en los atributos {prefix}-data y {prefix}-value que se pueda especificar el nombre del parámetro poniendo ":" y después el nombre que se buscará en el comando. Si no se ponen los dos puntos sigue funcionando igual que ahora. (Estoy pensando que lo voy a hacer todo con el estado. Quitaré todo lo que sea atributos {prefix}-data y {prefix}-value).

[x] Cambiar la forma de pintar el contenido del componente line. Que funcione igual que el paragraph o el contenido de los options del select. 

[] Hacer que los button y los link calculen su contenido igual que el line, ahora solo tienen un string.

[] Que los estilos de los spans también se puedan modificar en función del estado con jinja.

[] Cambiar la forma de relanzar los comandos que tengan refresh. De la forma que está hecho ahora se les pasa el estado pero si el estado cambia no se enteran y siguen ejecutando los comandos con valores que no son correctos.

[] Que los comandos que se lanzan para cambiar el estado también tengan refresh.

[] Que se puedan lanzar comandos para cambiar el estado desde un evento.

[] Que el estado se pueda enviar cuando se cambia de página.