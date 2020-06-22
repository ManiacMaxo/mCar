const joy = new JoyStick('joydiv')
const req = new XMLHttpRequest()
let currentX = 0
let currentY = 0

const x = document.querySelector('#X')
const y = document.querySelector('#Y')

setInterval(() => {
    x.value = joy.GetX()
    y.value = joy.GetY()
    if (x.value !== currentX || y.value !== currentY) {
        currentX = x.value
        currentY = y.value
        req.open('GET', '/control/' + currentX + ',' + currentY)
        req.send()
    }
}, 50)
