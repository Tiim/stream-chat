const src = new EventSource("/sse");

const elements = [];
src.onmessage = (event) => {
    const newElement = document.createElement("div");
    const eventList = document.getElementById("chat");

    const msg = JSON.parse(event.data);

    switch (msg.type) {
        case "Chat": chat(newElement, msg); break;
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

src.onerror = () => {
    const newElement = document.createElement("div");
    const eventList = document.getElementById("chat");
    newElement.textContent = `! Connection Error\n`;
    eventList.appendChild(newElement);
}
src.onopen = () => {
    const newElement = document.createElement("div");
    const eventList = document.getElementById("chat");
    newElement.textContent = `> Connected`;
    eventList.appendChild(newElement);
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
