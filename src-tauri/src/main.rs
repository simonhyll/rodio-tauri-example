// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde::Serialize;
use std::{
    fs::File,
    io::BufReader,
    sync::{mpsc::Sender, Mutex},
};
use tauri::{Manager, State};

#[derive(Serialize)]
pub(crate) struct PlaybackState {
    #[serde(skip_serializing)]
    stream: Option<OutputStreamHandle>,
    #[serde(skip_serializing)]
    sinks: Vec<Sink>,
    #[serde(skip_serializing)]
    sender: Sender<String>,
}

impl PlaybackState {
    pub fn new(stream: Option<OutputStreamHandle>, tx: Sender<String>) -> Self {
        let _ = tx.send("default device".to_string());
        Self {
            stream,
            sinks: vec![],
            sender: tx,
        }
    }
}

#[tauri::command]
async fn play(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    println!("Playing!");
    let path = "test.mp3";
    let mut state = playback_mutex.lock().unwrap();
    let stream = state.stream.as_ref().unwrap().clone();
    let sink = Sink::try_new(&stream).unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    state.sinks.push(sink);
    Ok(())
}

#[tauri::command]
async fn stop(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    println!("Stopping!");
    let mut state = playback_mutex.lock().unwrap();
    state.sinks.clear();
    Ok(())
}

#[tauri::command]
async fn change_device(
    device: String,
    playback_mutex: State<'_, Mutex<PlaybackState>>,
) -> Result<(), ()> {
    println!("Changing device!");
    let state = playback_mutex.lock().unwrap();
    let _ = state.sender.send(device);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play, stop, change_device])
        .setup(move |app| {
            let app = app.handle();
            let (tx, rx) = std::sync::mpsc::channel::<String>();
            app.manage(Mutex::new(PlaybackState::new(None, tx)));
            std::thread::spawn(move || {
                let mut stream = OutputStream::try_default().unwrap();
                // Receive messages in a blocking loop
                while let Ok(message) = rx.recv() {
                    println!("Received: {}", message);
                    stream = OutputStream::try_default().unwrap();
                    let state_mutex = app.state::<Mutex<PlaybackState>>();
                    let mut state = state_mutex.lock().unwrap();
                    state.stream = Some(stream.1);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
