# PokePalette

Find pokemon that match your image's color palette! Upload any image and discover which Pokemon share similar colors using computer vision and color theory.

![Smeargle](smeargle.png)

---

## What it does

Ever wondered which Pokemon would blend perfectly with your favorite photo? This CLI tool analyzes the dominant colors in any image and finds Pokemon with the most similar color palettes.

```bash
# Find Pokemon that match your sunset photo
pokepalette sunset.jpg -t 5

# Results:
# 1. Charizard
# 2. Moltres
# 3. Arcanine
# 4. Rapidash
# 5. Flareon
```

## Usage:

```bash
pokepalette <IMAGE> [OPTIONS]
```

---

### Options

| Option                 | Description                               |
| ---------------------- | ----------------------------------------- |
| `-t, --top-k <NUMBER>` | Number of Pokemon to return (default: 10) |

### Examples

```bash
# Basic usage - find top 10 matching Pokemon
pokepalette my_photo.jpg

# Get more results
pokepalette landscape.png -t 20
```

### How it works

The tool uses several computer vision and color theory techniques to find accurate matches:

**Color Quantization**: Extracts dominant colors from your image using uniform quantization <br>
**LAB Color Space**: Converts colors to LAB space for perceptually uniform color comparisons <br>
**Euclidean Distance**: Calculates color similarity using euclidean distance in LAB space <br>

### Supported Formats

Common image formats: JPG, JPEG, PNG, BMP, GIF
Any image size (larger images may take longer to process)

### Contributing & Feedback

This project started as a fun way to learn Rust, so I'd love to hear your thoughts! Especially:

Accuracy improvements: Know better color matching algorithms? <br>
Performance optimizations: Ideas for faster color extraction? <br>
Feature suggestions: What would make this more useful or fun? <br>

Feel free to open issues or PRs!

### Acknowledgments

Pokemon data sourced from [krabby](https://github.com/yannjor/krabby). This is an awesome project, check it out!

### License

This project is licensed under the [MIT License](../LICENSE)

---

_Made with 🦀 Rust - because even Pokemon deserve memory safety!_
