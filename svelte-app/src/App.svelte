<script>
    import io from 'socket.io-client'
    import Joystick from './Joytstick.svelte'
    const socket = io()

    socket.on('connect', () => {
        socket.emit('connection', { data: "I'm connected!" })
    })
</script>

<main>
    <img class="video-feed" src="/video-feed" alt="video feed" />
    <Joystick {socket} />
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        height: 100%;
        justify-content: center;
        align-items: center;
    }

    .video-feed {
        height: 400px;
        aspect-ratio: 16 / 10;
    }
</style>
