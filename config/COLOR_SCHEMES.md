# Piko Color Schemes

Piko offers several predefined color palettes to customize the appearance of the output. Each scheme has been designed to be visually pleasing and readable.

## 🎨 Available Schemes

### 1. Default (Dracula-inspired)
**File**: `default_config.toml`

A modern palette inspired by the Dracula theme, with vibrant and contrasting colors:

- **OS**: Vibrant Pink (#FF79C6)
- **Username**: Fresh Green (#50FA7B)
- **Hostname**: Warm Yellow (#F1FA8C)
- **Desktop Environment**: Elegant Purple (#BD93F9)
- **Memory**: Technical Cyan (#8BE9FD)
- **Disks**: Attention-grabbing Red (#FF5555)
- **CPU Model**: Professional Blue-grey (#6272A4)
- **GPU Model**: Energetic Orange (#FFB86C)
- **Kernel Version**: Subtle Dark Grey (#44475A)
- **Display Server**: Modern Light Cyan (#A4FFFF)
- **Uptime**: Pleasant Light Pink (#FF92DF)

**Ideal for**: Dark background terminals, users who prefer vibrant colors.

### 2. Pastel
**File**: `pastel_config.toml`

A delicate and modern pastel palette, perfect for a soft and elegant look:

- **OS**: Soft Pink (#FFB3BA)
- **Username**: Soft Mint (#BAFFC9)
- **Hostname**: Soft Yellow (#FFFFBA)
- **Desktop Environment**: Soft Lavender (#E8BAFF)
- **Memory**: Soft Blue (#BAE1FF)
- **Disks**: Soft Rose (#FFB3D9)
- **CPU Model**: Soft Sage (#B8E6B8)
- **GPU Model**: Soft Peach (#FFD4B3)
- **Kernel Version**: Soft Grey (#D4D4D4)
- **Display Server**: Soft Sky Blue (#B3E6FF)
- **Uptime**: Soft Cream (#FFE6B3)

**Ideal for**: Light background terminals, users who prefer delicate colors.

### 3. Dark
**File**: `dark_config.toml`

A dark and elegant palette, perfect for professional environments:

- **OS**: Soft Red (#E06C75)
- **Username**: Soft Green (#98C379)
- **Hostname**: Soft Yellow (#E5C07B)
- **Desktop Environment**: Soft Purple (#C678DD)
- **Memory**: Soft Cyan (#56B6C2)
- **Disks**: Bright Red (#FF6B6B)
- **CPU Model**: Soft Blue (#61AFEF)
- **GPU Model**: Light Salmon (#FFA07A)
- **Kernel Version**: Light Grey (#ABB2BF)
- **Display Server**: Turquoise (#4ECDC4)
- **Uptime**: Light Pink (#F8BBD9)

**Ideal for**: Dark background terminals, professional work environments.

## 🚀 Usage

### Using Predefined Schemes

```bash
# Default scheme (Dracula-inspired)
piko

# Pastel scheme
piko --config config/pastel_config.toml

# Dark scheme
piko --config config/dark_config.toml
```

### Creating Your Own Scheme

1. Copy one of the existing schemes:
   ```bash
   cp config/default_config.toml ~/.config/piko/my_scheme.toml
   ```

2. Modify the colors in the `[colors]` section:
   ```toml
   [colors]
   OS = "#YOUR_COLOR_HERE"
   Username = "#YOUR_COLOR_HERE"
   # ... other colors
   ```

3. Use your scheme:
   ```bash
   piko --config ~/.config/piko/my_scheme.toml
   ```

## 🎯 Tips for Creating Schemes

### Contrast and Readability
- Ensure colors have sufficient contrast with the terminal background
- Always test colors in your work environment

### Color Harmony
- Use complementary colors for related elements
- Maintain chromatic consistency throughout the scheme
- Consider accessibility for colorblind users

### Color Codes
- Use 6-digit hexadecimal codes (#RRGGBB)
- Find inspiration on sites like:
  - [Coolors](https://coolors.co/)
  - [Color Hunt](https://colorhunt.co/)
  - [Adobe Color](https://color.adobe.com/)

## 🔧 Advanced Customization

Beyond colors, you can also customize:

- **Symbols**: Modify emojis in the `[symbols]` section
- **Layout**: Change the order of elements in `info_keys`
- **Font**: Modify `font_size` in the `[customization]` section

Complete customization example:
```toml
[customization]
color_scheme = "custom"
font_size = 14
alignment = "center"

[colors]
OS = "#FF6B6B"
Username = "#4ECDC4"
# ... other colors

[symbols]
OS = "🖥️"
Username = "👤"
# ... other symbols
```
