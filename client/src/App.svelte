<script>
    import io from 'socket.io-client'
    import Joystick from './Joytstick.svelte'

    const socket = io('192.168.2.120:3000')
</script>

<main>
    <img
        class="video-feed"
        src="http://192.168.2.120:3000/api/video_feed"
        alt="video feed"
    />
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
