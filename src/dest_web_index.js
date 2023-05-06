const src = new EventSource("/sse");

const elements = [];
src.onmessage = (event) => {
    const newElement = document.createElement("div");
    const eventList = document.getElementById("chat");
    console.log(event);

    const msg = JSON.parse(event.data);

    switch (msg.type) {
        case "Chat": chat(newElement, msg); break;
        case "Command": command(newElement, msg.cmd); break;
        default:
            //TODO: format the other event types as well. 
            //See source::Event for all the event types.
            newElement.textContent = `> ${event.data}\n`;
            break;
    }

    eventList.appendChild(newElement);

    elements.push(newElement);

    while (elements.length > 60) {
        element = elements.shift();
        element.remove();
    }
};

src.onerror = (evt) => {
    const newElement = document.createElement("div");
    const eventList = document.getElementById("chat");
    newElement.textContent = `! Connection Error: ${evt}\n`;
    eventList.appendChild(newElement);
}


function command(elem, cmd) {
    switch (cmd.cmd) {
        case "TTS": tts(elem, cmd.value); break;
    }
}

function tts(_, msg) {
    const voices = speechSynthesis.getVoices();
    let voice = null;
    let langs = ["en-gb", "en-us", "en-"]
    while (!voice && langs.length) {
        const lang = langs.shift();
        voice = voices.find(v => v.lang.toLowerCase().startsWith(lang));
    }
    if (!voice) {
        voice = voices[0];
    }
    const utter = new SpeechSynthesisUtterance(msg);
    utter.voice = voice;
    utter.lang = "en";
    speechSynthesis.speak(utter);
}

function chat(elem, msg) {
    const author = document.createElement("span");
    author.textContent = `<${msg.chat.author}> `;
    author.className = "chat-author";
    const src = document.createElement("span");
    src.textContent = `${msg.chat.src.substring(0, 3)} `;
    src.className = "chat-source";
    const message = document.createElement("span");
    message.textContent = msg.chat.message;

    elem.appendChild(src);
    elem.appendChild(author);
    elem.appendChild(message);
}
