<script lang="ts">
    // Plays only a short fragment of a YouTube video (one subtitle line) and pauses at its end.
    import { onDestroy, onMount } from "svelte";
    import { loadYouTubeApi } from "$lib/youtube";

    export let videoId: string;
    /** Subtitle line start, seconds. */
    export let startSec: number;
    /** Start of the next subtitle line, seconds (null if this is the last line). */
    export let endSec: number | null = null;
    /** True once the fragment has finished and the player was paused. */
    export let ended = false;

    const LEAD = 0.8; // start a bit earlier so the phrase is not cut
    const TAIL = 4.6; // and stop later: subtitle timings are rough, phrases got cut
    const FALLBACK_LEN = 6; // when there is no next line
    const MIN_LEN = 2.5;
    const MAX_LEN = 12;

    $: clipStart = Math.max(0, startSec - LEAD);
    $: lineLen = Math.min(
        MAX_LEN,
        Math.max(MIN_LEN, (endSec ?? startSec + FALLBACK_LEN) - startSec),
    );
    $: clipEnd = startSec + lineLen + TAIL;

    let host: HTMLDivElement;
    let player: any = null;
    let ready = false;
    // Enforce the end only after playback of the current clip has actually started,
    // so a stale getCurrentTime() from the previous video can't pause the new one.
    let armed = false;
    let timer: ReturnType<typeof setInterval> | undefined;

    onMount(async () => {
        const YT = await loadYouTubeApi();
        if (!host) return; // unmounted while the API was loading
        player = new YT.Player(host, {
            width: "100%",
            height: "100%",
            playerVars: { autoplay: 1, rel: 0, playsinline: 1, modestbranding: 1 },
            events: {
                onReady: () => (ready = true),
                onStateChange: (e: any) => {
                    if (e.data === YT.PlayerState.PLAYING) armed = true;
                },
            },
        });
        timer = setInterval(checkEnd, 100);
    });

    onDestroy(() => {
        clearInterval(timer);
        player?.destroy?.();
    });

    $: if (ready) loadClip(videoId, clipStart);

    function loadClip(id: string, from: number) {
        ended = false;
        armed = false;
        player.loadVideoById({ videoId: id, startSeconds: from });
    }

    function checkEnd() {
        if (!ready || !armed || ended) return;
        if (player.getCurrentTime() >= clipEnd) {
            player.pauseVideo();
            ended = true;
        }
    }

    export function replay() {
        if (!ready) return;
        ended = false;
        armed = false;
        player.seekTo(clipStart, true);
        player.playVideo();
    }
</script>

<div class="w-full h-full">
    <div bind:this={host}></div>
</div>
