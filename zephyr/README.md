L'idée est de mettre son projet dans un workspace
puis le projet fournit un manifest pour configurer le workspace

west init -l fw-blink-1

west zephyr-export
west build -b rpi_pico fw-blink-1