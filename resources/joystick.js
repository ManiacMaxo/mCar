const socket = new WebSocket(
  (() => {
    const protocol = window.location.protocol === "https:" ? "wss" : "ws";
    const host = window.location.host;
    return `${protocol}://${host}/control`;
  })()
);

const mapNumber = (num, in_min, in_max, out_min, out_max) =>
  ((num - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min;

const size = Math.min(window.innerWidth * 0.8, 200);
const radius = Math.floor(size / 2);

const joystickElement = document.querySelector("#joystick");

const joystick = nipplejs.create({
  zone: joystickElement,
  color: "blue",
  mode: "static",
  position: { left: "50%", top: "70%" },
  dynamicPage: true,
  size,
});

joystick.on("move", (ev) => {
  const { x, y } = ev.target.nipples[0].frontPosition;

  const rotation = mapNumber(x, -radius, radius, -1, 1);
  const speed = mapNumber(-y, -radius, radius, -1, 1);

  socket.send(
    JSON.stringify({
      event: "control",
      data: { x: rotation, y: speed },
    })
  );
});

joystick.on("end", () => {
  socket.send(JSON.stringify({ event: "stop" }));
});

socket.addEventListener("message", (event) => {
  console.log("Message from server: ", event.data);
});
