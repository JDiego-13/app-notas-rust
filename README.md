Integrantes:
- Espinoza López Juan Diego
- Guerrero Dávila Juan Carlos
- Ibarra Martínez Ximena Estefania
- Galaviz Flores Christian Darío


-----------------------------------------------
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

3. Luego hacer:
kubectl rollout restart deployment bulletin-board-app


