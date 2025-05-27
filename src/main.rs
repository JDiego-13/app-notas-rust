use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Router,
    routing::get_service,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::SocketAddr,
    path::Path as FsPath,
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
    let notes: Notes = Arc::new(Mutex::new(load_notes_from_file()));

    // Servicio de archivos estáticos
    let serve_static = get_service(ServeDir::new("static"));

    let app = Router::new()
        // API
        .route("/notes", get(list_notes).post(create_note))
        .route("/notes/:id", delete(delete_note))
        // Archivos estáticos
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
    let id = notes.last().map_or(1, |n| n.id + 1);
    let note = Note {
        id,
        title: payload.title,
        content: payload.content,
    };
    notes.push(note);
    save_notes_to_file(&notes);
    (StatusCode::CREATED, "Nota creada correctamente")
}

async fn delete_note(
    State(notes): State<Notes>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let mut notes = notes.lock().unwrap();
    if let Some(pos) = notes.iter().position(|note| note.id == id) {
        notes.remove(pos);
        save_notes_to_file(&notes);
        (StatusCode::OK, "Nota eliminada")
    } else {
        (StatusCode::NOT_FOUND, "Nota no encontrada")
    }
}

fn save_notes_to_file(notes: &Vec<Note>) {
    let json = serde_json::to_string_pretty(notes).unwrap();
    fs::write("notes.json", json).unwrap();
}

fn load_notes_from_file() -> Vec<Note> {
    if FsPath::new("notes.json").exists() {
        let data = fs::read_to_string("notes.json").unwrap();
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}
