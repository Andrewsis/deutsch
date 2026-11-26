// Loads the YouTube IFrame Player API once and resolves with the global `YT` object.
let apiPromise: Promise<any> | null = null;

export function loadYouTubeApi(): Promise<any> {
    if (apiPromise) return apiPromise;
    apiPromise = new Promise((resolve) => {
        const w = window as any;
        if (w.YT?.Player) {
            resolve(w.YT);
            return;
        }
        const prev = w.onYouTubeIframeAPIReady;
        w.onYouTubeIframeAPIReady = () => {
            prev?.();
            resolve(w.YT);
        };
        const script = document.createElement("script");
        script.src = "https://www.youtube.com/iframe_api";
        script.async = true;
        document.head.appendChild(script);
    });
    return apiPromise;
}
