<script>
    let isHovered = false;

    function handleHover() {
        isHovered = true;
    }

    function handleMouseLeave() {
        isHovered = false;
    }
</script>

<div
    role="img"
    aria-label="Company Logo"
    on:mouseenter={handleHover}
    on:mouseleave={handleMouseLeave}
>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-full h-full"
        viewBox="0 0 55 55"
    >
        <!-- Big circle with blue gradient -->
        <defs>
            <radialGradient
                id="blueGradient"
                cx="50%"
                cy="50%"
                r="50%"
                fx="50%"
                fy="50%"
            >
                <stop offset="0%" stop-color="#777fff" />
                <stop offset="100%" stop-color="#404bbb" />
            </radialGradient>
        </defs>
        <circle cx="30" cy="30" r="20" fill="url(#blueGradient)" />

        <!-- Small circle with pink gradient -->
        <defs>
            <radialGradient
                id="pinkGradient"
                cx="50%"
                cy="50%"
                r="50%"
                fx="50%"
                fy="50%"
            >
                <stop offset="0%" stop-color="#c72a73" />
                <stop offset="100%" stop-color="#0195ffff" />
            </radialGradient>
        </defs>
        <circle cx="30" cy="30" r="10" fill="url(#pinkGradient)" />

        <!-- Small circle with yellow gradient -->
        <defs>
            <radialGradient
                id="yellowGradient"
                cx="30%"
                cy="30%"
                r="60%"
                fx="60%"
                fy="60%"
            >
                <stop offset="0%" stop-color="#e46800" />
                <stop offset="100%" stop-color="#ffba50" />
            </radialGradient>
        </defs>
        <circle
            cx="32%"
            cy="35%"
            r="10"
            fill="url(#yellowGradient)"
            class:animateHover={isHovered}
        />

        <!-- Inner white circle -->
        <circle
            cx="34%"
            cy="36%"
            r="4"
            fill="#ffffff"
            class:animateHover={isHovered}
        />

        <!-- Small white oval -->
        <ellipse
            cx="45%"
            cy="45%"
            rx="3"
            ry="2"
            fill="#ffffff"
            transform="rotate(30 25 25)"
            class:animateHover={isHovered}
        />
    </svg>
</div>

<style>
    .animateHover {
        animation: growCircle 0.3s ease-in-out forwards;
    }

    @keyframes growCircle {
        0% {
            transform: scale(1);
        }
        100% {
            transform: scale(1.1);
        }
    }
</style>
