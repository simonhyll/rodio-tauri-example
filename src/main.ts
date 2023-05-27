import { invoke } from "@tauri-apps/api/tauri";
import { emit } from "@tauri-apps/api/event";

window.addEventListener("DOMContentLoaded", () => {
  let playButton = document.querySelector("#play-button");
  playButton?.addEventListener("click", () => {
    invoke("play")
  })
  let stopButton = document.querySelector("#stop-button");
  stopButton?.addEventListener("click", () => {
    invoke("stop")
  })
  let changeButton = document.querySelector("#change-button");
  changeButton?.addEventListener("click", () => {
    invoke('change_device', { device: 'something else' })
  })
});
