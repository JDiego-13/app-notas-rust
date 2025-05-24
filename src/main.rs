use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
    routing::get_service,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::{trace::TraceLayer, services::ServeDir};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
    id: usize,
    title: String,
    content: String,
}

type Notes = Arc<Mutex<Vec<Note>>>;

#[tokio::main]
async fn main() {
    let notes: Notes = Arc::new(Mutex::new(vec![]));

    // Servicio para archivos estáticos sin manejo personalizado de error
    let serve_static = get_service(ServeDir::new("static"));

    let app = Router::new()
        // API
        .route("/notes", get(list_notes).post(create_note))
        // Archivos estáticos (index.html, style.css, etc) en carpeta ./static
        .route("/", serve_static)
        .with_state(notes.clone())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor escuchando en http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn list_notes(State(notes): State<Notes>) -> impl IntoResponse {
    let notes = notes.lock().unwrap();
    Json(notes.clone())
}

#[derive(Debug, Deserialize)]
struct NewNote {
    title: String,
    content: String,
}

async fn create_note(
    State(notes): State<Notes>,
    Json(payload): Json<NewNote>,
) -> impl IntoResponse {
    let mut notes = notes.lock().unwrap();
    let id = notes.len() + 1;
    let note = Note {
        id,
        title: payload.title,
        content: payload.content,
    };
    notes.push(note);
    (StatusCode::CREATED, "Nota creada correctamente")
}
