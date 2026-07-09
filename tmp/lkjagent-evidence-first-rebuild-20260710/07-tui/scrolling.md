# Scrolling

## Visual Rows

Compute scrolling from terminal-width wrapped display rows, including Unicode
display width, borders, and the actual transcript viewport height. Do not use
logical newline count.

## Bounds

    max_top = total_visual_rows - viewport_rows, saturating at zero
    top = clamp(requested_top, zero, max_top)

No state or renderer may display rows after max_top.

## Follow

Before applying new content or resize, record whether the viewport is at bottom.
If following, recompute and set top to new max_top. If manually scrolled, keep
an anchor message and visual-row offset.

Scrolling up from follow begins one visual row above the bottom. Scrolling down
to max_top re-enables follow. A new row while manual does not steal position.

## Resize

Rewrap all visible messages. Following remains bottom-anchored. Manual mode
preserves the anchor message where possible and clamps if content shrinks.

## Empty Space

The transcript area fills only its bounded viewport. Footer and side panes never
increase transcript scroll range.
