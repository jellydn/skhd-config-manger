#!/bin/bash
# Generate DMG background images using ImageMagick or Python PIL
# This creates simple, professional backgrounds for the DMG installer

set -e

# Check if ImageMagick is available
if command -v convert &> /dev/null; then
    echo "Using ImageMagick to generate backgrounds..."

    # Standard resolution (660x400)
    convert -size 660x400 \
        -background "#f5f5f7" \
        -fill "#1d1d1f" \
        -pointsize 24 \
        -font "Helvetica-Bold" \
        -gravity center \
        -annotate +0-60 "skhd GUI" \
        -fill "#86868b" \
        -pointsize 14 \
        -font "Helvetica" \
        -annotate +0-30 "Keyboard Shortcuts Manager" \
        -annotate +0+100 "Drag to Applications to install" \
        background.png

    # Retina resolution (1320x800)
    convert -size 1320x800 \
        -background "#f5f5f7" \
        -fill "#1d1d1f" \
        -pointsize 48 \
        -font "Helvetica-Bold" \
        -gravity center \
        -annotate +0-120 "skhd GUI" \
        -fill "#86868b" \
        -pointsize 28 \
        -font "Helvetica" \
        -annotate +0-60 "Keyboard Shortcuts Manager" \
        -annotate +0+200 "Drag to Applications to install" \
        background@2x.png

    echo "✅ DMG backgrounds generated successfully"

elif command -v python3 &> /dev/null; then
    echo "Using Python PIL to generate backgrounds..."

    python3 << 'EOF'
from PIL import Image, ImageDraw, ImageFont
import os

def create_background(width, height, output_file, title_size, subtitle_size, instruction_size):
    """Create DMG background image with PIL"""
    # Create image with light gray background (Apple style)
    img = Image.new('RGB', (width, height), color='#f5f5f7')
    draw = ImageDraw.Draw(img)

    # Calculate center positions
    center_x = width // 2

    # Try to use system fonts, fallback to default
    try:
        title_font = ImageFont.truetype('/System/Library/Fonts/Helvetica.ttc', title_size)
        subtitle_font = ImageFont.truetype('/System/Library/Fonts/Helvetica.ttc', subtitle_size)
        instruction_font = ImageFont.truetype('/System/Library/Fonts/Helvetica.ttc', instruction_size)
    except:
        title_font = ImageFont.load_default()
        subtitle_font = ImageFont.load_default()
        instruction_font = ImageFont.load_default()

    # Draw title
    title = "skhd GUI"
    title_bbox = draw.textbbox((0, 0), title, font=title_font)
    title_width = title_bbox[2] - title_bbox[0]
    draw.text((center_x - title_width//2, height//2 - 60), title, fill='#1d1d1f', font=title_font)

    # Draw subtitle
    subtitle = "Keyboard Shortcuts Manager"
    subtitle_bbox = draw.textbbox((0, 0), subtitle, font=subtitle_font)
    subtitle_width = subtitle_bbox[2] - subtitle_bbox[0]
    draw.text((center_x - subtitle_width//2, height//2 - 20), subtitle, fill='#86868b', font=subtitle_font)

    # Draw instruction
    instruction = "Drag to Applications to install"
    instruction_bbox = draw.textbbox((0, 0), instruction, font=instruction_font)
    instruction_width = instruction_bbox[2] - instruction_bbox[0]
    draw.text((center_x - instruction_width//2, height//2 + 80), instruction, fill='#86868b', font=instruction_font)

    # Save image
    img.save(output_file)
    print(f"✅ Created {output_file}")

# Generate both resolutions
create_background(660, 400, 'background.png', 24, 14, 12)
create_background(1320, 800, 'background@2x.png', 48, 28, 24)

print("✅ DMG backgrounds generated successfully")
EOF

else
    echo "❌ Neither ImageMagick nor Python3 found. Creating placeholder backgrounds..."

    # Create simple placeholder files
    # These are tiny 1x1 pixel placeholders - replace with proper graphics later
    printf "\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xb4\x00\x00\x00\x00IEND\xaeB\x60\x82" > background.png
    printf "\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xb4\x00\x00\x00\x00IEND\xaeB\x60\x82" > background@2x.png

    echo "⚠️  Placeholder backgrounds created. Install ImageMagick or use Python PIL for proper backgrounds."
fi

# Verify files were created
if [ -f background.png ] && [ -f background@2x.png ]; then
    ls -lh background*.png
    exit 0
else
    echo "❌ Failed to create background images"
    exit 1
fi
