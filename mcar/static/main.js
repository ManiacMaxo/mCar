const socket = io()
socket.on('connect', () => {
    socket.emit('connection', { data: "I'm connected!" })
})

Number.prototype.map = function (in_min, in_max, out_min, out_max) {
    return ((this - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min
}

const joystick = nipplejs.create({
    create: document.querySelector('#wrapper'),
    color: 'blue',
    mode: 'static',
    position: { left: '50%', top: '50%' },
    size: 200,
})

joystick.on('move', () => {
    const { x, y } = joystick[0].frontPosition
    socket.emit('control', -x.map(0, 100, 0, 1), -y.map(0, 100, 0, 1))
})

joystick.on('end', () => {
    socket.emit('stop')
})
