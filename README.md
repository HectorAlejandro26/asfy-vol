# asfy-vol

`asfy-vol` es un indicador de volumen visual desarrollado en Rust utilizando GTK4 y gtk4-layer-shell. Está diseñado para entornos de escritorio que soportan el protocolo Layer Shell (como Wayland/Sway/Hyprland), proporcionando una barra de volumen minimalista que responde a eventos del sistema.

## Características
- Interfaz construida con GTK4
- Soporte para Layer Shell (flota sobre otras ventanas)
- Configurable mediante archivos TOML
- Manejo de umbrales de iconos dinámicos según el nivel de volumen

## Instalación
Para compilar el proyecto desde el código fuente, asegúrate de tener instalado el toolchain de Rust y las dependencias de desarrollo de GTK4.

```bash
git clone https://github.com/HectorAlejandro26/asfy-vol.git
cd asfy-vol
makepkg -si
```
## Dependencias principales
- `wireplumber`
- `gtk4`
- `gtk4-layer-shell`
- `glibc`
- `gcc-libs`

## Configuración
El programa busca su archivo de configuración en `$XDG_CONFIG_HOME/asfy/asfy-vol/config.toml`. Ejemplo de `config.toml`:

```toml
use_percent = true
muted_text = ""

style_path = "$XDG_CONFIG_HOME/asfy/asfy-vol/style.css"

[[thresholds]]
icon = ""
level = 0.15

[[thresholds]]
icon = ""
level = 0.425

[[thresholds]]
icon = ""
level = 1.0
```

**Parámetros:**
- `use_percent`: Determina si se muestra el porcentaje de volumen.
- `thresholds`: Lista de iconos y sus respectivos niveles de volumen (0.0 a 1.0) para cambiar el icono según el nivel actual.
- `muted_text`: Texto que se mostrará al mutear el dispositivo.
- `style_path`: Ruta del archivo CSS.

