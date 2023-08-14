<svg
    xmlns="http://www.w3.org/2000/svg"
    width="100"
    height="100"
    viewBox="0 0 100 100"
>
    <!-- Big blue circle with reversed gradient -->
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
    <circle cx="20" cy="20" r="20" fill="url(#blueGradient)" />

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
    <circle cx="20" cy="20" r="10" fill="url(#pinkGradient)" />

    <!-- Small yellow circle with gradient -->
    <defs>
        <radialGradient
            id="yellowGradient"
            cx="50%"
            cy="50%"
            r="60%"
            fx="60%"
            fy="60%"
        >
            <stop offset="0%" stop-color="#e46800" />
            <stop offset="100%" stop-color="#ffba50" />
        </radialGradient>
    </defs>
    <circle cx="10" cy="10" r="10" fill="url(#yellowGradient)" />

    <!-- Inner white circle -->
    <circle cx="11" cy="11" r="4" fill="#ffffff" />

    <!-- Additional circles and ovals -->
    <ellipse
        cx="17"
        cy="18"
        rx="3"
        ry="2"
        fill="#ffffff"
        transform="rotate(30 18 18)"
    />
</svg>
