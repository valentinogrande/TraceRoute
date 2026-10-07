# traceroute

Proyecto educativo: un `traceroute` mínimo en Rust para entender cómo se descubre el camino que siguen los paquetes hasta un host.

## Cómo funciona

Cada paquete IP lleva un **TTL** (*Time To Live*). Cada router que lo reenvía le resta 1, y si llega a 0 lo descarta y le avisa al emisor con un mensaje ICMP *Time Exceeded*, que incluye su propia IP.

El programa aprovecha eso:

1. Envía un datagrama UDP al destino con TTL = 1 → el primer router lo descarta y responde.
2. Lee esa respuesta ICMP con un socket raw y muestra la IP del router y el tiempo que tardó.
3. Repite con TTL = 2, 3, … y así va apareciendo cada salto del camino.

## `unsafe` y syscalls

La librería estándar de Rust no ofrece sockets raw, así que el socket ICMP se maneja llamando directamente a las syscalls de Linux a través del crate `libc`:

| Syscall | Para qué |
|---------|----------|
| `socket(AF_INET, SOCK_RAW, IPPROTO_ICMP)` | Crea el socket raw que recibe los mensajes ICMP |
| `setsockopt(SO_RCVTIMEO)` | Pone un timeout de 1 s para no quedarse bloqueado si un router no responde |
| `recv` | Lee el paquete ICMP en un buffer |
| `close` | Cierra el socket al terminar |

Estas llamadas son funciones de C (FFI), y Rust no puede verificar lo que hacen con la memoria. Por eso van dentro de bloques `unsafe`: ahí el programador se hace responsable de pasar punteros válidos (por ejemplo `rec_buff.as_mut_ptr()` junto con su largo real) y de revisar los valores de retorno (`< 0` indica error).

El socket UDP, en cambio, usa `std::net::UdpSocket`, que es seguro; su TTL se cambia con `set_ttl`.

## Uso

Necesita Linux y root (o `CAP_NET_RAW`) para abrir el socket raw ICMP.

```sh
cargo build --release
sudo ./target/release/traceroute
```

```
Introduce tu url: example.com
The IP Address of example.com is: 93.184.215.14
[*] ROUTER IP: 192.168.1.1 TOTAL DELAY: 2ms
[*] ROUTER IP: 10.0.0.1 TOTAL DELAY: 9ms
...
```
