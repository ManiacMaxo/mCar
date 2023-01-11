<script>
    import Joystick from './lib/Joytstick.svelte'

    const getSocketUrl = () => {
        const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws'
        return `${protocol}://${window.location.host}/control`
    }

    const socket = new WebSocket(getSocketUrl())
</script>

<main>
    <!-- svelte-ignore a11y-media-has-caption -->
    <video class="video-feed" src="/api/feed" autoplay />
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
        aspect-ratio: 4 / 3;
        max-width: 100%;
    }
</style>
