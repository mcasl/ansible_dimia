# Administración máquinas laboratorios F5/F6/G7/E5 - Ansible

## Descripción del sistema

### Tailscale - red de máquinas

### Máquinas incluidas en la instalación

### Instalación inicial (ejemplo para máquina )
La instalación inicial está ya presente ya en todas las máquinas. La información relativa a la primera instalación se encuentra recogida en el directorio `disco`.

La configuración de `ansible`, `ansible-pull`, `ansible-playbook`, etc. está en el archivo `ansible.cfg` e incluye las rutas de los archivos de claves de acceso y de claves `ssh`, además del fichero que incluye los nombres de las máquinas que va a afectar (`inventory`).

### crontab
El `crontab` de cada máquina ejecuta cada hora un `ansible-pull` sobre el repositorio `https://github.com/mcasl/ansible.git pasando como parámetro el archivo `local.yml`. Se aconseja incluir en este archivo los archivos yaml del directorio `tasks`. 

### Contraseñas
La información contenida en el repositorio que está cifrada con `ansible` se puede descifrar con la contraseña contenida en `.ansible_vault_passwd` que se encuentra en el gestor de contraseñas del grupo.
La contraseña de administrador y de la cuenta `practicas` 

### Claves ssh
Existe un único par de claves ssh compartida entre todas las máquinas para facilitar el compartir archivos entre ellas. Las claves se copian desde el repositorio en las máquinas en la instalación inicial.

## Instalación de nuevos paquetes

Para programar la instalación de nuevos paquetes se debe añadir el nombre del paquete al final del fichero `tasks/paquetes_apt.yml`

A continuación podemos esperar a la siguiente ejecución del cron para su ejecución, o forzarla con `ansible-pull -U https://github.com/mcasl/ansible_dimia.git local.yml`

## Instalación de nuevas aplicaciones no empaquetadas

La instalación de software que NO sea a través de apt requiere la creación de un nuevo fichero `yaml` que refleje el proceso completo (ver `anaconda.yml`). 