# Makepad Text Rotation Limitation

## Summary

Makepad's `DrawText` API does not support per-glyph rotation. True radial text (where each character is rotated to point outward from a center point) is not achievable with the current Makepad architecture without modifying Makepad core.

## Technical Analysis

### DrawText Architecture

Makepad's text rendering system (`draw/src/shader/draw_text.rs`) uses:

1. **Font Atlas Textures**: Two textures managed internally:
   - `grayscale_texture` - For standard text rendering with SDF
   - `color_texture` - For emoji and color glyphs

2. **SDF Rendering**: Signed Distance Field rendering for anti-aliased text at any scale

3. **Vertex Shader**: Positions each glyph quad without rotation support:
   ```glsl
   fn vertex(self) -> vec4 {
       let p = mix(self.rect_pos, self.rect_pos + self.rect_size, self.geom_pos);
       let p_clipped = clamp(p, self.draw_clip.xy, self.draw_clip.zw);
       // ... no rotation applied
   }
   ```

### Why Per-Glyph Rotation is Difficult

1. **No Instance Rotation Variable**: Unlike `RotatedImage` which has `instance rotation: 0.0`, DrawText lacks this capability

2. **Internal Texture Management**: Font textures are bound internally through `draw_vars.texture_slots` and not easily accessible for custom shaders

3. **Complex Layout System**: Text layout involves the `Layouter` system which computes glyph positions without rotation consideration

### Reference: How RotatedImage Implements Rotation

The `RotatedImage` widget (`widgets/src/rotated_image.rs`) shows the pattern for rotation in Makepad:

```rust
live_design! {
    pub DrawRotatedImage = {{DrawRotatedImage}} {
        instance rotation: 0.0
        instance pivot_x: 0.0
        instance pivot_y: 0.0

        fn vertex(self) -> vec4 {
            // Calculate rotation expansion for bounds
            let rot_expansion = rotation_vertex_expansion(self.rotation, ...);

            // Apply rotation around pivot
            let cos_a = cos(self.rotation);
            let sin_a = sin(self.rotation);
            let rotated = vec2(
                centered.x * cos_a - centered.y * sin_a,
                centered.x * sin_a + centered.y * cos_a
            );
            // ...
        }
    }
}
```

## What Would Be Required

To add per-glyph rotation to Makepad's DrawText:

1. **Add Instance Variable**: Add `instance rotation: 0.0` to DrawText shader
2. **Modify Vertex Shader**: Apply rotation transformation around glyph center
3. **Expand Bounds**: Account for rotated glyph bounds (like RotatedImage does)
4. **API Changes**: Add rotation parameter to `draw_abs()` and related methods

This would be a **Makepad core change**, not achievable in user code.

## Workarounds

### Option 1: Radial Character Positioning (Limited)
Position each character along a radius, but characters remain horizontal:
```rust
for (i, ch) in text.chars().enumerate() {
    let char_radius = start_radius + (i as f64) * char_spacing;
    let char_x = center_x + char_radius * angle.cos();
    let char_y = center_y + char_radius * angle.sin();
    draw_text.draw_abs(cx, dvec2(char_x, char_y), &ch.to_string());
}
```

### Option 2: Horizontal Labels (Recommended)
Position entire labels horizontally at arc centers - most readable approach:
```rust
let mid_radius = (inner_r + outer_r) / 2.0;
let label_x = center_x + mid_radius * mid_angle.cos();
let label_y = center_y + mid_radius * mid_angle.sin();
draw_text.draw_abs(cx, dvec2(label_x - text_width/2.0, label_y), text);
```

### Option 3: Custom Texture Approach
Pre-render rotated text to a texture, then display as rotated image. Performance-heavy and complex.

## Recommendation

For sunburst and similar radial visualizations, use **horizontal labels** positioned at arc centers. This is:
- The most readable approach
- How most production D3 sunburst implementations handle labels
- Compatible with Makepad's current architecture

## Files Investigated

- `/Users/yuechen/.cargo/git/checkouts/makepad-721ba110953b28bc/53b2e5c/draw/src/shader/draw_text.rs` - DrawText implementation
- `/Users/yuechen/.cargo/git/checkouts/makepad-721ba110953b28bc/53b2e5c/draw/src/text/geom.rs` - Transform utilities (no rotation)
- `/Users/yuechen/.cargo/git/checkouts/makepad-721ba110953b28bc/53b2e5c/widgets/src/rotated_image.rs` - Reference for rotation pattern
- `/Users/yuechen/.cargo/git/checkouts/makepad-721ba110953b28bc/53b2e5c/draw/src/draw_list_2d.rs` - View transform (list-level, not per-glyph)

## Date

January 2026
