#!/usr/bin/env zsh
# Production build script for Tailwind CSS
# This script compiles Tailwind CSS once for production use

echo "🎨 Building Tailwind CSS for production..."
tailwindcss -i ./input.css -o ./static/style.css --minify

if [ $? -eq 0 ]; then
    echo "✅ Tailwind CSS production build complete!"
    echo "📁 Output: ./static/style.css"
else
    echo "❌ Tailwind CSS build failed!"
    exit 1
fi
