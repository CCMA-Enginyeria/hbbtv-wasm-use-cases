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
  window.startDebugTimer("parse");
  subtitles.parse(text);
  window.endDebugTimer("parse");

  const video = document.querySelector("video");
  //https://api-media.ccma.cat/pvideo/media.jsp?media=video&versio=vast&idint=6266073&profile=apptv_tv3&format=dm
  video.addEventListener("timeupdate", function () {
    const ms = video.currentTime * 1000;
    subtitles.updateSubtitlesForTimecode(ms);
  });
};
runWasm();
