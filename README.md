# pokescheme

Select pokemon sprites that match your wallpaper's colorscheme!

## Usage:

Pokescheme tries to find your desktop wallpaper using two methods:

1. Pywal: by grabbing the colorscheme from the autogenerated colors-rgb template located in your ~/.cache/wal directory
2. Wallpaper.rs: the rust wallpaper crate

### Flags

-k              the k nearest sprites to your desktop's colorscheme
<br>
--no-shiny      removes shiny sprites from query
<br>
--no-female     removes female sprites from query
<br>
--no-mega       removes mega sprites from query
<br>
--no-regional   removes regional variant sprites from query
<br>
--verbose       verbose output includes the current wallpaper colorscheme and the other k selected sprites' colorschemes + their distances to the wallpaper scheme

### Enhancements

1. Improve colorscheme generation for more accurate results. By default, Pokescheme generates a colorscheme using 
8 colors (due to them being grabbed from pywal). Some sprites don't have enough colors to create a colorscheme
with 8 dominant colors. To correct this, I used the naive approach of padding color schemes. This could be improved
in many ways. Will have to do some research and see. 
2. Use command line argument library to fix some cli bugs
3. Add a feature to create a colorscheme from any image
4. Add custom errors for clearer err messages

