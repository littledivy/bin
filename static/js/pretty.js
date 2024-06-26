const homePage = document.location.origin;
const pasteContentDiv = document.getElementById('pasteContent');

let wrap_level = 0;

function wrapClicked() {
    wrap_level = (wrap_level === 2) ? 0 : wrap_level + 1;

    switch (wrap_level) {
        case 0:
            pasteContentDiv.style.width = 'max-content';
            break;
        case 1:
            pasteContentDiv.style.width = 'auto';
            break;
        case 2:
            pasteContentDiv.style.width = '80ch';
            break;
    }
}

function forkClicked() {
    let text = pasteContentDiv.innerText;
    localStorage["forkText"] = text;
    let name = document.location.href.split('/').pop();
    localStorage["forkName"] = name;

    console.log(text);

    window.location = homePage;
}

function pinClicked() {
    let text = pasteContentDiv.innerText;
    let pastes = localStorage["pinned"] ? JSON.parse(localStorage["pinned"]) : [];

    const name = document.location.href.split('/').pop();
    pastes.push(name);

    localStorage["pinned"] = JSON.stringify(pastes);

    window.location = homePage;
}

function newPasteClicked() {
    window.location = homePage;
}

function rawClicked() {
    window.location = window.location.href.replace('/p/', '/');
}
