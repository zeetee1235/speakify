#!/bin/bash

# Speakify - Quality presets script
# Usage: ./speakify.sh <quality> <input_image>
# Quality: low, mid, high

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPEAKIFY="$SCRIPT_DIR/target/release/speakify"

if [ ! -f "$SPEAKIFY" ]; then
    echo "Error: speakify binary not found at $SPEAKIFY"
    echo "Please build first: cargo build --release"
    exit 1
fi

show_usage() {
    echo "Speakify - Image to 스핔이 Transformer"
    echo ""
    echo "Usage: $0 <quality> <input_image>"
    echo ""
    echo "Quality presets:"
    echo "  low    - 64x64,  50 frames  (~1s,  fast preview)"
    echo "  mid    - 128x128, 100 frames (~5s,  balanced)"
    echo "  high   - 256x256, 150 frames (~30s, high quality)"
    echo ""
    echo "Examples:"
    echo "  $0 low image.png       # Quick preview"
    echo "  $0 high photo.jpg      # High quality output"
    echo ""
    exit 1
}

if [ $# -lt 2 ]; then
    show_usage
fi

QUALITY="$1"
INPUT="$2"

if [ ! -f "$INPUT" ]; then
    echo "Error: Input file not found: $INPUT"
    exit 1
fi

case "$QUALITY" in
    low|l)
        RESOLUTION=64
        FRAMES=50
        PROXIMITY=13
        LABEL="저화질"
        ;;
    mid|m)
        RESOLUTION=128
        FRAMES=100
        PROXIMITY=13
        LABEL="고화질"
        ;;
    high|h)
        RESOLUTION=256
        FRAMES=150
        PROXIMITY=13
        LABEL="초화질"
        ;;
    *)
        echo "Error: Invalid quality preset: $QUALITY"
        echo "Available: low, mid, high"
        exit 1
        ;;
esac

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Speakify - $LABEL mode"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "입력: $INPUT"
echo "해상도: ${RESOLUTION}x${RESOLUTION}"
echo "프레임: $FRAMES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

"$SPEAKIFY" -i "$INPUT" -r "$RESOLUTION" -f "$FRAMES" -p "$PROXIMITY"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Cuayo~~!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
