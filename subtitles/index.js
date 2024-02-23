// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, * as subtitles from "./pkg/subtitles.js";

const subtitleContainerEl = document.querySelector("#subtitles");

window.showSubtitle = function (id, subtitleString) {
  subtitleContainerEl.innerHTML += subtitleString;
};

window.hideSubtitle = function (id) {
  var subtElement = document.getElementById(id);
  if (subtElement) {
    subtitleContainerEl.removeChild(subtElement.parentNode);
  }
};

window.existSubtitle = function (id) {
  return !!subtitleContainerEl.querySelector(`#${id}`);
};

const runWasm = async () => {
  await init();
  const res = await fetch("./subtitles.xml");
  const text = await res.text();
  subtitles.setElementHeight(
    subtitleContainerEl.offsetWidth,
    subtitleContainerEl.offsetHeight
  );
  window.startDebugTimer("parse");
  subtitles.parse(text);
  window.endDebugTimer("parse");
  const video = document.querySelector("video");
  const checkSubtitles = () => {
    const ms = video.currentTime * 1000;
    subtitles.updateSubtitlesForTimecode(ms);
    setTimeout(checkSubtitles, 0);
  };
  checkSubtitles();
  video.addEventListener("seeked", function () {
    subtitleContainerEl.innerHTML = "";
  });
};
runWasm();
