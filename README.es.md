# Telegram My ID

<img src="assets/icons/telegram-my-id-128.png" width="112" alt="Icono de Telegram My ID">

[English](README.md) · [Русский](README.ru.md) · [Español](README.es.md)

**Un bot pequeño de Telegram, con una aplicación de escritorio para Mac.**

Inicio y parada manual, estado de conexión, registro de eventos, configuración del token
y vista previa local de los comandos. La interfaz de escritorio está en ruso; el bot
responde en ruso, inglés, español y coreano.

![Aplicación nativa de Telegram My ID](assets/screenshots/overview.png)

El bot está detenido en la captura. Las vistas previas utilizan al usuario ficticio
Alex Example, ID `100000001`, y no envían mensajes a Telegram.

## Primeros pasos

Se necesitan Rust 1.94+ y Xcode Command Line Tools para compilar en macOS.

```sh
make setup
make doctor
make local
```

1. Abra **Настройки**. Seleccione un archivo `.env` con `TELOXIDE_TOKEN` o introduzca
   el token temporalmente. En el selector de macOS, Cmd+Shift+. muestra archivos ocultos.
2. Elija la carpeta de datos. Para usar la base anterior, seleccione `src/db/data`.
3. Guarde las rutas con **Сохранить пути** y pulse **Запустить бота**.
4. Tras confirmar la conexión, abra el bot en Telegram y envíe `/start`.

La aplicación no inicia el bot automáticamente. Esc lo detiene; Cmd+Enter lo inicia
cuando está detenido. Active **Настройки → Закрытие окна → Продолжать работу бота после закрытия приложения** para continuar en segundo plano al cerrar la ventana. Sin esta opción, el bot se detiene. Al volver a abrir, la ventana controla el mismo proceso.
No ejecute el mismo token en varios clientes de long polling simultáneamente.
El programa no elimina automáticamente un webhook existente.

## Comandos del bot

| Comando | Función |
| --- | --- |
| `/start` | Bienvenida y selección del idioma |
| `/info_user` | Su propio ID y perfil de Telegram |
| `/language` | Cambiar entre RU, EN, ES y KO |
| `/help` | Lista de comandos |
| `/random_number` | Número aleatorio dentro de un intervalo |
| `/random_choice` | Elegir una opción de una lista |
| `/test` | Diagnóstico y ejemplos de traducción |

![Vista previa local](assets/screenshots/commands.png)

## Credenciales y datos

La configuración guarda únicamente las rutas del `.env` y de los datos. El token
introducido manualmente permanece en la memoria del proceso de trabajo hasta detener el bot. Las credenciales
no se incluyen en el paquete `.app`. Los últimos 150 eventos se conservan en memoria.

Los archivos predeterminados en macOS son
`~/Library/Application Support/TelegramMyID/settings.json` y
`~/Library/Application Support/TelegramMyID/data/languages.json`.
No hay migración automática. Una base JSON dañada se informa sin sobrescribirla.

## Desarrollo

`make` abre la versión de desarrollo; `make build` genera la aplicación y un ZIP;
`make run` abre la versión existente; `make check` ejecuta formato, Clippy estricto y pruebas.
Node.js solo es necesario para regenerar iconos con `make icons`.

El modo sin interfaz sigue disponible:

```sh
RUST_LOG=info cargo run --locked --bin telegram-bot
```

Utiliza `src/db/data` por defecto. `TELEGRAM_MY_ID_DATA_DIR` permite indicar otra carpeta.
Ctrl+C lo detiene. El paquete usa una firma ad hoc local; no se publica ni se notariza
automáticamente. Windows y Linux no se han verificado.

Las pruebas del ciclo de conexión usan una API aislada en localhost, sin enviar mensajes
a Telegram. El propietario debe comprobar por separado el funcionamiento con su token real.

[Documentación completa](README.md) · [Licencia del proyecto](LICENSE) ·
[Licencias de las fuentes](assets/fonts)


## Instaladores nativos

Ejecute `make setup`, `make doctor` y `make local` para compilar los fuentes actuales
y abrir el instalador. `make run` abre la versión existente. En Windows también:
`./package.ps1 setup`, `./package.ps1 doctor`, `./package.ps1 local`, sin GNU Make.
Se incluyen GUI y headless; fuera del repositorio, CLI usa datos del usuario
si no existe `src/db/data` ni una ruta explícita. Consulte dependencias, paquetes
y plataformas verificadas en [docs/INSTALLERS.md](docs/INSTALLERS.md).


Modo de segundo plano y parada sin ventana: [docs/BACKGROUND.md](docs/BACKGROUND.md).
