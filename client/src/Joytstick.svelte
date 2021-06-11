<script>
    import nipplejs from 'nipplejs'

    export let socket = null

    Number.prototype.map = function (in_min, in_max, out_min, out_max) {
        return (
            ((this - in_min) * (out_max - out_min)) / (in_max - in_min) +
            out_min
        )
    }

    const size = Math.min(window.innerWidth * 0.8, 200)
    const radius = parseInt(size / 2)

    const joystick = nipplejs.create({
        create: document.querySelector('#joystick'),
        color: 'blue',
        mode: 'static',
        position: { left: '50%', top: '70%' },
        dynamicPage: true,
        size,
    })

    joystick.on('move', () => {
        const { x, y } = joystick[0].frontPosition
        socket.emit(
            'control',
            -x.map(-radius, radius, -1, 1),
            -y.map(-radius, radius, -1, 1)
        )
    })

    joystick.on('end', () => {
        socket.emit('stop')
    })
</script>

<div id="joystick" />

<style>
    #joystick {
        position: relative;
        height: 100%;
        width: 100%;
    }
</style>
