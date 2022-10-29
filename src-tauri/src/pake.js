window.addEventListener('DOMContentLoaded', (_event) => {
    const style = document.createElement('style');
    style.innerHTML = `

    *::-webkit-scrollbar {
      display: none;
    }

    .panel.give_me .nav_view {
      top: 154px !important;
    }

    .columns .column #header{
      padding-top: 30px;
    }

    #page .main_header {
      padding-top: 20px;
    }

    #page #footer-wrapper,
    .drawing-board .toolbar .toolbar-action,
    .c-swiper-container,
    .download_entry,
    .lang, .copyright {
      display: none !important;
    }

    .container-with-note #home, .container-with-note #switcher{
      top: 30px;
    }

    .geist-page nav.dashboard_nav__PRmJv {
      padding-top:10px;
    }

    .geist-page .submenu button{
      margin-top:24px;
    }

    #pack-top-dom:active {
      cursor: grabbing;
      cursor: -webkit-grabbing;
    }

    #pack-top-dom{
      position:fixed;
      background:transparent;
      top:0;
      width: 100%;
      height: 20px;
      cursor: grab;
      cursor: -webkit-grab;
    }
  `;
    document.head.append(style);
    const topDom = document.createElement("div");
    topDom.id = "pack-top-dom"
    document.body.appendChild(topDom);

    const domEl = document.getElementById('pack-top-dom');

    domEl.addEventListener('mousedown', (e) => {
        if (e.buttons === 1 && e.detail !== 2) {
            window.ipc.postMessage('drag_window');
        }
    })

    domEl.addEventListener('touchstart', (e) => {
        window.ipc.postMessage('drag_window');
    })

    domEl.addEventListener('dblclick', (e) => {
        window.ipc.postMessage('fullscreen');
    })

    document.addEventListener('keyup', function (event) {
        if (event.key === "ArrowUp" && event.metaKey){
            scrollTo(0,0);
        }
        if (event.key === "ArrowDown" && event.metaKey){
            window.scrollTo(0, document.body.scrollHeight);
        }
        if (event.key === "[" && event.metaKey){
            window.history.go(-1);
        }
        if (event.key === "]" && event.metaKey){
            window.history.go(1);
        }
        if (event.key === "r" && event.metaKey){
            window.location.reload();
        }
        if (event.key === "-" && event.metaKey){
            zoomOut();
        }
        if (event.key === "=" && event.metaKey){
            zoomIn();
        }
        if (event.key === "0" && event.metaKey){
            document.getElementsByTagName('html')[0].style.zoom = '100%';
            window.localStorage.setItem('htmlZoom', '100%');
        }
    })

    const pakeLinks = document.links;
    for (let linkIndex = 0; linkIndex < pakeLinks.length; linkIndex++) {
        pakeLinks[linkIndex].target = '_self';
    }

});

setDefaultZoom();

function setDefaultZoom() {
  const htmlZoom = window.localStorage.getItem('htmlZoom');
  if (htmlZoom) {
    document.getElementsByTagName('html')[0].style.zoom = htmlZoom;
  }
}

function zoomIn() {
  const htmlZoom = window.localStorage.getItem('htmlZoom') || '100%';
  const html = document.getElementsByTagName('html')[0];
  const zoom = parseInt(htmlZoom) < 200 ? (parseInt(htmlZoom) + 10 +'%') : '200%';
  html.style.zoom = zoom;
  window.localStorage.setItem('htmlZoom', zoom);
}

function zoomOut() {
  const htmlZoom = window.localStorage.getItem('htmlZoom') || '100%';
  const html = document.getElementsByTagName('html')[0];
  const zoom = parseInt(htmlZoom) > 30 ? (parseInt(htmlZoom) - 10 +'%') : '30%';
  html.style.zoom = zoom;
  window.localStorage.setItem('htmlZoom', zoom);
}