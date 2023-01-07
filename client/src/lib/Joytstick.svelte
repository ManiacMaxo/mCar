<script lang="ts">
    import nipplejs from 'nipplejs'
    import { onMount } from 'svelte'

    export let socket: WebSocket

    const mapNumber = (
        num: number,
        in_min: number,
        in_max: number,
        out_min: number,
        out_max: number
    ) => ((num - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min

    const size = Math.min(window.innerWidth * 0.8, 200)
    const radius = Math.floor(size / 2)

    let joystickElement: HTMLElement

    onMount(() => {
        const joystick = nipplejs.create({
            zone: joystickElement,
            color: 'blue',
            mode: 'static',
            position: { left: '50%', top: '70%' },
            dynamicPage: true,
            size
        })

        joystick.on('move', (ev) => {
            const { x, y } = ev.target.nipples[0].frontPosition

            const rotation = mapNumber(x, -radius, radius, -1, 1)
            const speed = mapNumber(-y, -radius, radius, -1, 1)

            socket.send(
                JSON.stringify({
                    event: 'control',
                    data: { x: rotation, y: speed }
                })
            )
        })

        joystick.on('end', () => {
            socket.send(JSON.stringify({ event: 'stop' }))
        })
    })
</script>

<div id="joystick" bind:this={joystickElement} />

<style>
    #joystick {
        position: relative;
        height: 100%;
        width: 100%;
    }
</style>
