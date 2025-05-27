Integrantes:
- Espinoza López Juan Diego
- Guerrero Dávila Juan Carlos
- Ibarra Martínez Ximena Estefania
- Galaviz Flores Christian Darío


-------------------------------------------------------------
Para hacer pruebas locales.
1. Compilar el proyecto
cargo build

2. Ejecutarmo localmente:
cargo run

3. Visitar desde el navegador de forma local:
http://localhost:3000


-------------------------------------------------------------
PROBAR LA API CON HERRAMIENTAS.

1. Desde la terminal, probar:
curl http://localhost:3000/notes

2. Agregar nota:
curl -X POST http://localhost:3000/notes -H "Content-Type: application/json" -d '{"title": "Nota 1", "content": "Papel"}'

3. Eliminar la nota:
curl -X DELETE http://localhost:3000/notes/1

-------------------------------------------------------------

Dentro de la carpteta cd ~/Descargas app-notas-rust: (Tu carpeta donde tienes el proyecto)


1. Iniciar el cluster Minikube (solo si no está iniciado)
minikube start

2. Desplegar la app (deployment + ingress)
kubectl apply -f deployment.yaml
kubectl apply -f ingress.yaml

3. Agregar al hosts(una sola vez si no se ha hecho)
echo "$(minikube ip) notas.local" | sudo tee -a /etc/hosts

4. Abrir la app en el navegador.
xdg-open http://notas.local


5. Parar / eliminar los recursos en Kubernetes (pero NO borra la imagen en Docker Hub)
kubectl delete -f deployment.yaml
kubectl delete -f ingress.yaml

-------------------------------------------------------------
Si cambio mi codigo tengo que hacer esto:
1. Volver a compilar con:
cargo build --release

2. Volver a crear la imagen Docker y subirla a Docker Hub:
docker build -t jdino13/bulletin-board-app:latest .
docker push jdino13/bulletin-board-app:latest

2.1. Puede que se necesite hacer un inicio del minikube:
minikube start

3. Luego hacer:
kubectl rollout restart deployment bulletin-board-app

--------------------------------------------------------------
LIBERAR ESPACIO.
1. Limpia imágenes y contenedores Docker que no uses:
docker system prune -a

