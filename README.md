# xmltui

xmltui es un framework para crear aplicaciones de terminal de forma similar a como haríamos una página web. 

Se utiliza ratatui para dibujar en la terminal, tokio para lanzar comandos de forma asíncrona y minijinja para interpretar las plantillas con jinja2.

## Ejemplo:

Vamos a hacer una pequeña aplicación que nos muestre la fecha y la hora en el centro de la pantalla.
Este ejemplo solo funciona si tienes instalado el comando date en tu sistema.

```xml
<rtml xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" 
      xsi:noNamespaceSchemaLocation="schema.xsd">

    <!--
    En el nodo head podemos definir comandos, estilos y templates. También se pueden definir en ficheros externos y cargarlos desde el head. 
    -->
    <head>

        <!-- 
        El nodo code se utiliza para definir comandos que queremos ejecutar. 
        En el atributo command ponemos el comando a ejecutar y en name ponemos un nombre que luego utilizaremos para referirnos a este comando.
        También se pueden definir argumentos y variables de entorno, como en este ejemplo no son necesarios dejamos el nodo code vacío.
        -->
        <code command="date" name="date-command" />

        <!-- 
        style se utiliza para definir los estilos de la aplicación. 
        Se escriben en formato json, aunque se pueden omitir las llaves de inicio y fin el resto debe ser un json válido.
        Para indicar a que nodo se aplicarán los estilos se pueden utilizar nombres de etiquetas, clases precedidas por un punto e ids precedidos de # al igual que se hace en css.
        -->
        <style>
            "body" : {
                "flex" : "center",
                "dir" : "vertical"
            },
            "command" : {
                "length" : "1"
            },
            "line" : {
                "align" : "center"
            }
        </style>
    </head>

    <!-- 
    En el body pondremos lo que queramos que se vea por pantalla
    -->
    <body>
        <!-- 
        El nodo command se utiliza para ejecutar un comando y renderizar un template con la salida del comando.
        El template se puede definir dentro del nodo command pero también se puede poner el atributo template para utilizar alguno que tengamos en el head o en algún fichero externo.
        En exec debemos poner el name de algún nodo code.
        Con refresh-sec="1" le decimos que el comando se ejecute cada segundo. Por defecto los comandos se ejecutan solo una vez cuando se carga la página aunque se pueden relanzar desde diferentes eventos.
        -->
        <command exec="date-command" refresh-sec="1"> 
            <template>
                <!-- 
                line es el widget Line de ratatui. Pueden contener texto y/o nodos span. 
                En la variable ctx está la salida del comando. 
                Por defecto la salida de los comandos es texto pero también se puede interpretar como json si ponemos el atributo output="json" y la salida del comando es un json válido.
                Las dobles llaves pertenecen a la sintaxis de jinja2.
                Como en este caso la salida del comando es texto ponemos directamente {{ ctx }} y se renderizará el texto devuelto por el comando.
                -->
                <line>{{ ctx }}</line>
            </template>
        </command>
    </body>
</rtml>
```


